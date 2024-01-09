pub mod db;
pub mod lexoffice;

use clap::{arg, command, Command};
use db::models::{DbInvoice, DbVoucher};
use dotenvy::dotenv;
use openapi::{apis::configuration::Configuration, models::VoucherlistVoucher};
use sqlx::PgPool;
use std::{env, error::Error};

struct App {
    api_conf: Configuration,
    db_pool: PgPool,
}

impl App {
    fn new(api_key: String, pool: PgPool) -> Self {
        let mut conf = Configuration::default();
        conf.bearer_access_token = Some(api_key);
        App {
            api_conf: conf,
            db_pool: pool,
        }
    }

    async fn run(self) -> Result<(), Box<dyn Error>> {
        //sync_lexoffice(&self.db_pool).await;
        Ok(())
    }
}

async fn sync_vouchers(pool: &PgPool, vouchers: Vec<VoucherlistVoucher>) {
    for voucher in vouchers {
        if voucher.id.is_none() || !db::voucher_exists(pool, voucher.id.unwrap().to_string()).await
        {
            if voucher.voucher_number.is_some() {
                println!(
                    "Insert voucher into DB: {:?}",
                    voucher.clone().voucher_number.unwrap()
                );
            }
        }

        let db_voucher = DbVoucher::from(voucher);
        db::insert_voucher(pool, db_voucher).await.ok();
    }
}

async fn sync_invoice(config: &Configuration, pool: &PgPool, invoice_id: String) {
    let result = lexoffice::get_invoice(config, invoice_id.clone())
        .await
        .and_then(move |invoice| {
            let db_invoice = DbInvoice::from(invoice);
            Ok(db::insert_invoice(pool, db_invoice))
        });
    match result {
        Ok(_) => println!("Synced invoice {}", invoice_id),
        Err(e) => println!("Error syncing invoice: {:?}", e),
    }
}

async fn sync_lexoffice(pool: &PgPool, types: Vec<String>) {
    let api_key = env::var("LEXOFFICE_APIKEY").expect("'LEXOFFICE_APIKEY' must bet set!");
    let config = lexoffice::get_config(api_key);

    // First get all vouchers from voucherlist endpoint and save into DB
    let mut current_page = 1;
    let page_size = 250;
    loop {
        let res =
            lexoffice::get_voucherlist(&config, types.join(","), current_page, page_size).await;
        match res {
            Err(e) => println!("error getting voucherlist: {}", e),
            Ok(voucher_list) => {
                println!(
                    "Fetched {} of {} vouchers",
                    voucher_list.number_of_elements.unwrap(),
                    voucher_list.total_elements.unwrap()
                );
                let vouchers = voucher_list.content.unwrap_or(vec![]);
                sync_vouchers(pool, vouchers).await;

                if voucher_list.last.unwrap_or(false) {
                    break;
                }
                current_page += 1;
            }
        }
    }

    // Get saved vouchers from DB and get+save invoices
    let invoices = db::get_vouchers_by_type(pool, "invoice".to_string())
        .await
        .unwrap_or(vec![]);
    for voucher in invoices {
        sync_invoice(&config, pool, voucher.id).await;
    }
}

async fn show_vouchers(pool: &PgPool) {
    let all_vouchers = db::get_all_vouchers(pool).await.unwrap_or(vec![]);

    println!("Displaying {} vouchers", all_vouchers.len());
    for voucher in all_vouchers {
        println!("-----------");
        println!("ID: {}", voucher.id);
        println!(
            "Type: {}",
            voucher.voucher_type.unwrap_or("n/a".to_string())
        );
        println!(
            "Contact Name: {}",
            voucher.contact_name.unwrap_or("n/a".to_string())
        );
    }

    let all_invoices = db::get_all_invoices(pool).await.unwrap_or(vec![]);
    println!("\n\nDisplaying {} invoices", all_invoices.len());
    for invoice in all_invoices {
        println!("-----------");
        println!("ID: {}", invoice.id);
        println!(
            "Number: {}",
            invoice.voucher_number.unwrap_or("n/a".to_string())
        );
        println!("Updated at: {:?}", invoice.updated_date);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

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

    let db_pool = db::connect_db().await.unwrap();
    let _ = sqlx::migrate!().run(&db_pool).await;

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
                    sync_lexoffice(&db_pool, voucher_types).await;
                }
                "invoices" => {
                    let voucher_types = ["invoice".to_string()].to_vec();
                    println!("Syncing invoices...");
                    sync_lexoffice(&db_pool, voucher_types).await;
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
            show_vouchers(&db_pool).await;
        }
        _ => unreachable!("Cannot parse subcommand"),
    };

    println!("Finished! Exiting...");
}
