mod sync;

use std::env;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use chrono::{DateTime, ParseResult, Utc};
use clap::{arg, command, Command};
use colored::Colorize;
use dotenvy::dotenv;
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use leaky_bucket::RateLimiter;
use log::info;
use mongodb::error::ErrorKind;
use simple_logger::SimpleLogger;
use openapi::apis::configuration::Configuration;
use crate::sync::{connect_db, sync_invoices};

struct LexofficeClient {
    config: Configuration,
    limiter: RateLimiter,
}

impl LexofficeClient {
    pub fn new(config: Configuration, max_api_calls_per_second: usize) -> Self {
        Self {
            config,
            limiter: RateLimiter::builder()
                .initial(max_api_calls_per_second)
                .interval(Duration::from_secs(1))
                .build(),
        }
    }
}

#[derive(Debug)]
pub enum MyError {
    LexofficeApiError(String),
    MongoDbError(String, ErrorKind),
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::LexofficeApiError(msg) => write!(f, "Lexoffice API Error: {}", msg),
            MyError::MongoDbError(msg, kind) => write!(f, "MongoDB Error: {} - {:?}", msg, kind),
        }
    }
}

impl std::error::Error for MyError {}

fn parse_date_string(date_str: String) -> ParseResult<DateTime<Utc>> {
    let result = DateTime::parse_from_str(format!("{} 00:00:00.000 +0000", date_str).as_str(), "%Y-%m-%d %H:%M:%S%.3f %z")?;
    Ok(result.to_utc())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let logger = SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Info);
    let multi = MultiProgress::new();
    LogWrapper::new(multi.clone(), logger)
        .try_init()
        .unwrap();

    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("sync")
                .about("Sync a voucher type or all vouchers stored at lexoffice")
                .args(&[
                    arg!([VOUCHER_TYPE]).required(false).default_value("all"),
                    arg!(--from <FROM_DATE> "start date").required(false),
                    arg!(--to <TO_DATE> "end date").required(false)
                ])
        )
        .get_matches();

    match matches.subcommand() {
        Some(("sync", sub_matches)) => {
            let types_arg = sub_matches
                .get_one::<String>("VOUCHER_TYPE")
                .unwrap()
                .to_string();
            let from_date = sub_matches.get_one::<String>("from")
                .map(|from_str|
                    parse_date_string(from_str.to_string())
                        .expect("Failed to parse 'from' date")
                );
            let to_date = sub_matches.get_one::<String>("to")
                .map(|to_str|
                    parse_date_string(to_str.to_string())
                        .expect("Failed to parse 'to' date")
                );
            match types_arg.as_str() {
                "all" => {
                    sync_vouchers(Vec::from([
                        "invoice".to_string(),
                        "salesinvoice".to_string(),
                        "salescreditnote".to_string(),
                        "purchaseinvoice".to_string(),
                        "purchasecreditnote".to_string(),
                        "downpaymentinvoice".to_string(),
                        "creditnote".to_string(),
                        "orderconfirmation".to_string(),
                        "quotation".to_string(),
                        "deliverynote".to_string(),
                    ]), from_date, to_date).await;
                }
                "invoices" => {
                    sync_vouchers(Vec::from(["invoice".to_string()]), from_date, to_date).await;
                }
                _ => unreachable!(
                    "Unknown or unsupported argument for voucher types: {}",
                    types_arg
                ),
            }
        }
        _ => unreachable!("Cannot parse subcommand"),
    };
}

async fn sync_vouchers(voucher_types: Vec<String>, from_date: Option<DateTime<Utc>>, to_date: Option<DateTime<Utc>>) {
    info!("Syncing voucher types: {}", voucher_types.join(", ").bright_yellow());
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_key = env::var("LEXOFFICE_APIKEY").expect("LEXOFFICE_APIKEY must be set");

    // Connect to DB and get handle
    let db = connect_db(db_url.as_str(), "lexoffice")
        .await
        .expect("Connection failed!");

    let mut api_config = Configuration::default();
    api_config.bearer_access_token = Some(api_key);

    // max. 2 API calls per second allowed
    let client = LexofficeClient::new(api_config, 2);

    sync_invoices(&client, &db, from_date, to_date)
        .await
        .expect("error syncing invoices");
}