mod sync;

use std::env;
use clap::{arg, command, Command};
use dotenvy::dotenv;
use futures::{StreamExt, TryStreamExt};
use log::info;
use mongodb::{Collection, Database};
use mongodb::bson::doc;
use mongodb::options::InsertOneOptions;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use openapi::apis::configuration::Configuration;
use openapi::apis::vouchers_api;
use openapi::models::{Voucher, VoucherList, VoucherlistVoucher};
use crate::sync::{connect, sync_invoices, test};

#[tokio::main]
async fn main() {
    dotenv().ok();

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

    match matches.subcommand() {
        Some(("sync", sub_matches)) => {
            let types_arg = sub_matches
                .get_one::<String>("VOUCHER_TYPE")
                .unwrap()
                .to_string();
            match types_arg.as_str() {
                "all" => {
                    sync_vouchers(Vec::from([
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
                    ])).await;
                }
                "invoices" => {
                    sync_vouchers(Vec::from(["invoice".to_string()])).await;
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
            show_vouchers(Vec::from([types_arg.clone()])).await;
        }
        _ => unreachable!("Cannot parse subcommand"),
    };
}

async fn sync_vouchers(voucher_types: Vec<String>) {
    info!("Syncing vouchers: {:?}\n", voucher_types);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_key = env::var("LEXOFFICE_APIKEY").expect("LEXOFFICE_APIKEY must be set");

    // Get connection string
    let db_name = "lexoffice";

    // Connect to DB and get handle
    let db = connect(db_url.as_str(), db_name)
        .await
        .expect("Connection failed!");

    let mut api_config = Configuration::default();
    api_config.bearer_access_token = Some(api_key);

    sync_invoices(&api_config, &db)
        .await
        .expect("error syncing invoices");
}

async fn show_vouchers(voucher_types: Vec<String>) {
    info!("Showing vouchers: {:?}\n", voucher_types);
}