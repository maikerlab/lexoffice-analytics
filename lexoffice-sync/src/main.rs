pub mod lexoffice;

use diesel::prelude::*;
use diesel::PgConnection;
use futures::executor::block_on;
use openapi::apis::configuration::Configuration;
use std::env;

async fn sync_lexoffice() {
    let mut conf = Configuration::default();
    let api_key = env::var("LEXOFFICE_APIKEY");
    match api_key {
        Ok(val) => {
            conf.bearer_access_token = Some(val);
            lexoffice::sync_voucherlist(&conf, 1, 250).await;
        }
        Err(_) => {
            panic!("'LEXOFFICE_APIKEY' must bet set as env var!");
        }
    }
}

fn init_db() {
    let db_url = env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
    let db_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let db_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let db_name = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    println!("Init DB...");
    let conn = PgConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: lexoffice-cli [sync]");
    }

    init_db();

    let cmd = &args[1];
    if cmd == "sync" {
        println!("Starting lexoffice sync...");
        block_on(sync_lexoffice());
    } else {
        panic!("Unknown command: {}", cmd);
    }
    println!("Finished! Exiting...");
}
