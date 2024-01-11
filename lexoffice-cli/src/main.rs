mod db;
mod lexoffice;
mod sync;

use clap::{arg, command, Command};
use dotenvy::dotenv;
use lexoffice::LexofficeApi;
use log::*;
use simple_logger::SimpleLogger;
use std::env;

use crate::{db::LexofficeDb, sync::*};

async fn show_info(db: &LexofficeDb) {
    let all_vouchers = db.get_all_vouchers().await.unwrap_or(vec![]);
    let invoices = db.get_all_invoices().await.unwrap_or(vec![]);

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
            show_info(&app.db).await;
        }
        _ => unreachable!("Cannot parse subcommand"),
    };

    info!("Finished! Exiting...");
}
