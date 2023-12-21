pub mod db;
pub mod lexoffice;

use dotenvy::dotenv;
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

fn show_vouchers() {
    let results = db::get_all_vouchers();
    println!("Displaying {} vouchers", results.len());
    for voucher in results {
        println!("-----------");
        println!("ID: {}", voucher.id);
        println!("Type: {}", voucher.vouchertype.unwrap_or("n/a".to_string()));
        println!(
            "Contact Name: {}",
            voucher.contactname.unwrap_or("n/a".to_string())
        );
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: lexoffice-cli [sync|show]");
    }

    let cmd = &args[1];
    if cmd == "sync" {
        println!("Starting lexoffice sync...");
        block_on(sync_lexoffice());
    } else if cmd == "show" {
        println!("Showing voucher entries...");
        show_vouchers();
    } else {
        panic!("Unknown command: {}", cmd);
    }
    println!("Finished! Exiting...");
}
