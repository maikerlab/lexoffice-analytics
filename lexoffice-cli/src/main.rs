pub mod db;
pub mod lexoffice;

use dotenvy::dotenv;
use futures::executor::block_on;
use openapi::apis::configuration::Configuration;
use sqlx::PgPool;
use std::{env, fmt::Error, time::SystemTime};

async fn sync_lexoffice(pool: &PgPool) {
    let mut conf = Configuration::default();
    let api_key =
        env::var("LEXOFFICE_APIKEY").expect("'LEXOFFICE_APIKEY' must bet set as env var!");
    conf.bearer_access_token = Some(api_key);
    block_on(lexoffice::sync_voucherlist(&conf, 1, 250));
}

async fn show_vouchers(pool: &PgPool) {
    let all_vouchers = db::get_all_vouchers(pool).await;

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

    let all_invoices = db::get_all_invoices();
    println!("\n\nDisplaying {} invoices", all_invoices.len());
    for invoice in all_invoices {
        println!("-----------");
        println!("ID: {}", invoice.id);
        println!(
            "Number: {}",
            invoice.voucher_number.unwrap_or("n/a".to_string())
        );
        println!("Updated at: {:?}", invoice.updated_date.unwrap());
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: lexoffice-cli [sync|show]");
    }

    let db_pool = db::connect_db().await.unwrap();
    let _ = sqlx::migrate!().run(&db_pool).await;

    let cmd = &args[1];
    if cmd == "sync" {
        println!("Starting lexoffice sync...");
        sync_lexoffice(&db_pool).await;
    } else if cmd == "show" {
        println!("Showing database entries...\n");
        show_vouchers(&db_pool).await;
    } else {
        panic!("Unknown command: {}", cmd);
    }
    println!("Finished! Exiting...");
}
