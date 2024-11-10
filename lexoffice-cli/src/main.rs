mod sync;
mod api;
mod utils;

use std::env;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use chrono::{DateTime, Utc};
use clap::{arg, command, Command};
use colored::Colorize;
use dotenvy::dotenv;
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use leaky_bucket::RateLimiter;
use log::info;
use mongodb::Database;
use mongodb::error::ErrorKind;
use once_cell::sync::OnceCell;
use simple_logger::SimpleLogger;
use lexoffice_api::apis::configuration::Configuration;
use crate::sync::{connect_db, sync_invoices};
use crate::utils::parse_date_string;

struct LexofficeClient {
    config: Configuration,
    limiter: RateLimiter,
}

impl LexofficeClient {
    pub fn new(api_key: String, max_api_calls_per_second: usize) -> Self {
        let mut api_config = Configuration::default();
        api_config.bearer_access_token = Some(api_key);
        Self {
            config: api_config,
            limiter: RateLimiter::builder()
                .initial(max_api_calls_per_second)
                .interval(Duration::from_secs(1))
                .build(),
        }
    }
}

impl Debug for LexofficeClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lexoffice Client")
    }
}
static LEXOFFICE_CLIENT: OnceCell<Arc<LexofficeClient>> = OnceCell::new();
static DATABASE: OnceCell<Arc<Database>> = OnceCell::new();

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

async fn init() {
    // Init Client with API Key and max. 2 API calls per second allowed
    let api_key = env::var("LEXOFFICE_APIKEY").expect("LEXOFFICE_APIKEY must be set");
    let client = LexofficeClient::new(api_key, 2);
    LEXOFFICE_CLIENT.set(Arc::new(client)).expect("Error initializing Lexoffice Client");

    // Connect to DB and get handle
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = connect_db(db_url.as_str(), "lexoffice")
        .await
        .expect("Connection failed!");
    DATABASE.set(Arc::new(db)).expect("Error initializing Database");

    info!("Lexoffice and Database successfully initialized");
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

    init().await;

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
        .subcommand(
            Command::new("run")
                .about("Runs the API")
                .args(&[
                    arg!([PORT]).required(false).default_value("8000")
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
        Some(("run", sub_matches)) => {
            let port = sub_matches
                .get_one::<String>("PORT")
                .map(|port_str|
                    u16::from_str(port_str.as_str()).expect("Error parsing port")
                ).expect("Error getting port");
            api::run(port).await;
        },
        _ => unreachable!("Cannot parse subcommand"),
    };
}

async fn sync_vouchers(voucher_types: Vec<String>, from_date: Option<DateTime<Utc>>, to_date: Option<DateTime<Utc>>) {
    info!("Syncing voucher types: {}", voucher_types.join(", ").bright_yellow());

    let client = LEXOFFICE_CLIENT.get().expect("Client not initialized");
    let db = DATABASE.get().expect("Database not initialized");

    sync_invoices(client.clone(), db.clone(), from_date, to_date)
        .await
        .expect("error syncing invoices");
}