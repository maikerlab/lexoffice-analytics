pub mod db;
pub mod lexoffice;

use clap::{arg, command, Command};
use db::models::{DbInvoice, DbVoucher};
use dotenvy::dotenv;
use lexoffice::LexofficeApi;
use openapi::models::VoucherlistVoucher;
use std::env;

use crate::db::LexofficeDb;

struct App {
    api: LexofficeApi,
    db: LexofficeDb,
}

async fn sync_vouchers(app: &App, vouchers: Vec<VoucherlistVoucher>) {
    for voucher in vouchers {
        if !app.db.voucher_exists(voucher.id.to_string()).await {
            println!(
                "Insert voucher into DB: {:?}",
                voucher.clone().voucher_number
            );
        }

        let db_voucher = DbVoucher::from(voucher);
        app.db.insert_voucher(db_voucher).await.ok();
    }
}

async fn sync_invoice(app: &App, invoice_id: String) {
    let result = app.api.get_invoice(invoice_id.clone()).await;

    match result {
        Ok(ref invoice) => {
            let db_invoice = DbInvoice::from(invoice.to_owned());
            let inserted_id = app.db.insert_invoice(db_invoice).await;
            match inserted_id {
                Ok(invoice_id) => println!("Inserted invoice with ID: {}", invoice_id),
                Err(e) => println!("Error inserting invoice: {:?}", e),
            }
        }
        Err(e) => println!("Error syncing invoice: {:?}", e),
    }
}

async fn sync_lexoffice(app: &App, types: Vec<String>) {
    // First get all vouchers from voucherlist endpoint and save into DB
    let mut current_page = 1;
    let page_size = 250;
    loop {
        let res = app
            .api
            .get_voucherlist(types.join(","), current_page, page_size)
            .await;
        match res {
            Err(e) => println!("error getting voucherlist: {}", e),
            Ok(voucher_list) => {
                println!(
                    "Fetched {} of {} vouchers",
                    voucher_list.number_of_elements, voucher_list.total_elements
                );
                let vouchers = voucher_list.content;
                sync_vouchers(app, vouchers).await;

                if voucher_list.last {
                    break;
                }
                current_page += 1;
            }
        }
    }

    // Get saved vouchers from DB and get+save invoices
    let invoices = app
        .db
        .get_vouchers_by_type("invoice".to_string())
        .await
        .unwrap_or(vec![]);
    for voucher in invoices {
        sync_invoice(app, voucher.id).await;
    }
}

async fn show_vouchers(app: &App) {
    let all_vouchers = app.db.get_all_vouchers().await.unwrap_or(vec![]);

    println!("Displaying {} vouchers", all_vouchers.len());
    for voucher in all_vouchers {
        println!("-----------");
        println!("ID: {}", voucher.id);
        println!("Type: {}", voucher.voucher_type);
        println!(
            "Contact Name: {}",
            voucher.contact_name.unwrap_or("n/a".to_string())
        );
    }

    let all_invoices = app.db.get_all_invoices().await.unwrap_or(vec![]);
    println!("\n\nDisplaying {} invoices", all_invoices.len());
    for invoice in all_invoices {
        println!("-----------");
        println!("ID: {}", invoice.id);
        println!("Number: {}", invoice.voucher_number);
        println!("Updated at: {:?}", invoice.updated_date);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_key = env::var("LEXOFFICE_APIKEY").expect("LEXOFFICE_APIKEY must be set");

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
                    println!("Syncing all vouchers...");
                    sync_lexoffice(&app, voucher_types).await;
                }
                "invoices" => {
                    let voucher_types = ["invoice".to_string()].to_vec();
                    println!("Syncing invoices...");
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
            println!("Showing vouchers: {:?}\n", voucher_types);
            show_vouchers(&app).await;
        }
        _ => unreachable!("Cannot parse subcommand"),
    };

    println!("Finished! Exiting...");
}
