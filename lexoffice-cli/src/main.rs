pub mod db;
pub mod lexoffice;

use clap::{arg, command, Command};
use db::models::{DbInvoice, DbVoucher};
use dotenvy::dotenv;
use lexoffice::LexofficeApi;
use log::{error, info, warn};
use openapi::models::VoucherlistVoucher;
use simple_logger::SimpleLogger;
use std::env;

use crate::db::{
    models::{DbAddress, DbLineItem, DbProduct},
    LexofficeDb,
};

struct App {
    api: LexofficeApi,
    db: LexofficeDb,
}

async fn sync_vouchers(app: &App, vouchers: Vec<VoucherlistVoucher>) {
    let mut inserted = 0;
    for voucher in &vouchers {
        if !app.db.voucher_exists(voucher.id.to_string()).await {
            let db_voucher = DbVoucher::from(voucher.to_owned());
            match app.db.insert_voucher(db_voucher).await {
                Ok(_) => {
                    inserted += 1;
                    info!("Added new voucher: {}", voucher.voucher_number);
                }
                Err(_) => error!("Error while adding voucher"),
            }
        }
    }

    info!("Synced {} vouchers, inserted: {}", vouchers.len(), inserted);
}

async fn sync_invoice(app: &App, invoice_id: String) {
    if !app.db.invoice_exists(invoice_id.clone()).await {
        match app.api.get_invoice(invoice_id.clone()).await {
            Ok(ref invoice) => {
                // Insert address if not existing
                let db_address = DbAddress::from((*invoice.address).clone());
                match app.db.insert_address(db_address).await {
                    Ok(address_id) => info!("Added new address: {}", address_id),
                    Err(e) => error!("Error while adding address: {:?}", e),
                }

                // Insert invoice
                let db_invoice = DbInvoice::from(invoice.to_owned());
                match app.db.insert_invoice(db_invoice).await {
                    Ok(_) => {
                        info!("Added new invoice: {}", invoice.voucher_number);
                        // Insert line items
                        let mut li_inserted = 0;
                        for item in &invoice.line_items {
                            // Currently only items with a valid ID are inserted
                            if item.id.is_some() && !item.id.unwrap().is_nil() {
                                if !app.db.product_exists(item.id.unwrap().to_string()).await {
                                    let db_product = DbProduct::from(item.to_owned());
                                    match app.db.insert_product(db_product).await {
                                        Ok(product_id) => {
                                            info!("Added new product: {}", product_id)
                                        }
                                        Err(e) => error!("Error while adding product: {:?}", e),
                                    }
                                }
                                let db_lineitem = DbLineItem::from(item.to_owned());
                                match app
                                    .db
                                    .insert_lineitem(
                                        db_lineitem,
                                        item.id.unwrap().to_string(),
                                        invoice.id.to_string(),
                                    )
                                    .await
                                {
                                    Ok(_) => li_inserted += 1,
                                    Err(e) => {
                                        error!(
                                            "Error while adding line item: {:?} - {:?}",
                                            item.id, e
                                        )
                                    }
                                }
                            }
                        }
                        info!("Added {} line items of invoice {}", li_inserted, invoice_id);
                    }
                    Err(e) => error!("Error while adding invoice: {:?}", e),
                }
            }
            Err(e) => error!("Error while fetching invoice: {:?}", e),
        }
    }
}

async fn sync_lexoffice(app: &App, types: Vec<String>) {
    // First get all vouchers from voucherlist endpoint and save into DB
    let mut current_page = 1;
    let mut fetched_vouchers = 0;
    let page_size = 250;
    loop {
        let res = app
            .api
            .get_voucherlist(types.join(","), current_page, page_size)
            .await;
        match res {
            Err(e) => error!("error getting voucherlist: {}", e),
            Ok(voucher_list) => {
                fetched_vouchers += voucher_list.number_of_elements;
                info!(
                    "Fetched {} of {} vouchers from Voucherlist",
                    fetched_vouchers, voucher_list.total_elements
                );
                let vouchers = voucher_list.content;
                sync_vouchers(app, vouchers).await;

                if voucher_list.last {
                    break;
                }
                break;
                current_page += 1;
            }
        }
    }

    // For each voucher type to sync - get from endpoint and save to DB
    for t in types {
        let vouchers_by_type = app.db.get_vouchers_by_type(t).await;
        match vouchers_by_type {
            Ok(vouchers) => {
                for v in vouchers {
                    if v.voucher_type == "invoice" {
                        sync_invoice(app, v.id).await;
                    } else {
                        warn!(
                            "Sync. of voucher type {} currently not supported",
                            v.voucher_type
                        );
                    }
                }
            }
            Err(_) => {}
        }
    }

    let invoices = app
        .db
        .get_vouchers_by_type("invoice".to_string())
        .await
        .unwrap_or(vec![]);
    for voucher in invoices {
        sync_invoice(app, voucher.id).await;
    }
}

async fn db_info(app: &App) {
    let all_vouchers = app.db.get_all_vouchers().await.unwrap_or(vec![]);
    let invoices = app.db.get_all_invoices().await.unwrap_or(vec![]);

    info!("----- DATABASE INFO -----");
    info!("  - Vouchers: {}", all_vouchers.len());
    info!("  - Invoices: {}", invoices.len());
    info!("-------------------------");
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_key = env::var("LEXOFFICE_APIKEY").expect("LEXOFFICE_APIKEY must be set");

    SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("sync")
                .about("Sync a voucher type or all vouchers stored at lexoffice")
                .arg(arg!([VOUCHER_TYPE]).required(false).default_value("all")),
        )
        .subcommand(
            Command::new("show")
                .about("Show locally stored data")
                .arg(arg!([VOUCHER_TYPE]).required(false).default_value("all")),
        )
        .get_matches();

    // Connect + migrate database
    let db = LexofficeDb::new(db_url).await;
    db.migrate().await.expect("Error while migrating database");

    // Create the lexoffice API using the API Key
    let api = LexofficeApi::new(api_key);

    // ... then put together in our app
    let app = App { db, api };

    match matches.subcommand() {
        Some(("sync", sub_matches)) => {
            let types_arg = sub_matches
                .get_one::<String>("VOUCHER_TYPE")
                .unwrap()
                .to_string();
            match types_arg.as_str() {
                "all" => {
                    let voucher_types = [
                        "salesinvoice".to_string(),
                        "salescreditnote".to_string(),
                        "purchaseinvoice".to_string(),
                        "purchasecreditnote".to_string(),
                        "invoice".to_string(),
                        "downpaymentinvoice".to_string(),
                        "creditnote".to_string(),
                        "orderconfirmation".to_string(),
                        "quotation".to_string(),
                        "deliverynote".to_string(),
                    ]
                    .to_vec();
                    info!("Syncing all vouchers...");
                    sync_lexoffice(&app, voucher_types).await;
                }
                "invoices" => {
                    let voucher_types = ["invoice".to_string()].to_vec();
                    info!("Syncing invoices...");
                    sync_lexoffice(&app, voucher_types).await;
                }
                _ => unreachable!(
                    "Unknown or unsupported argument for voucher types: {}",
                    types_arg
                ),
            }
        }
        Some(("show", sub_matches)) => {
            let types_arg = sub_matches
                .get_one::<String>("VOUCHER_TYPE")
                .unwrap()
                .to_string();
            let voucher_types = [types_arg.clone()].to_vec();
            info!("Showing vouchers: {:?}\n", voucher_types);
            db_info(&app).await;
        }
        _ => unreachable!("Cannot parse subcommand"),
    };

    info!("Finished! Exiting...");
}
