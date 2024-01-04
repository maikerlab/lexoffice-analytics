pub mod db;
pub mod lexoffice;

use dotenvy::dotenv;
use futures::executor::block_on;
use openapi::apis::configuration::Configuration;
use std::{env, time::SystemTime};

fn sync_lexoffice() {
    let mut conf = Configuration::default();
    let api_key =
        env::var("LEXOFFICE_APIKEY").expect("'LEXOFFICE_APIKEY' must bet set as env var!");
    conf.bearer_access_token = Some(api_key);
    block_on(lexoffice::sync_voucherlist(&conf, 1, 250));
}

fn show_vouchers() {
    let all_vouchers = db::get_all_vouchers();
    println!("Displaying {} vouchers", all_vouchers.len());
    for voucher in all_vouchers {
        println!("-----------");
        println!("ID: {}", voucher.id);
        println!("Type: {}", voucher.vouchertype.unwrap_or("n/a".to_string()));
        println!(
            "Contact Name: {}",
            voucher.contactname.unwrap_or("n/a".to_string())
        );
    }

    let all_invoices = db::get_all_invoices();
    println!("\n\nDisplaying {} invoices", all_invoices.len());
    for invoice in all_invoices {
        println!("-----------");
        println!("ID: {}", invoice.id);
        println!(
            "Number: {}",
            invoice.vouchernumber.unwrap_or("n/a".to_string())
        );
        println!(
            "Updated at: {:?}",
            invoice.updateddate.unwrap_or(SystemTime::now())
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
        sync_lexoffice();
    } else if cmd == "show" {
        println!("Showing database entries...\n");
        show_vouchers();
    } else {
        panic!("Unknown command: {}", cmd);
    }
    println!("Finished! Exiting...");
}
