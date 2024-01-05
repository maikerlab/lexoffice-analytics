pub mod db;
pub mod lexoffice;

use dotenvy::dotenv;
use openapi::{
    apis::configuration::Configuration,
    models::{
        voucher::{VoucherStatus, VoucherType},
        Voucher,
    },
};
use sqlx::{types::Uuid, PgPool};
use std::env;

async fn sync_vouchers(pool: &PgPool, vouchers: Vec<Voucher>) {
    for voucher in vouchers {
        db::insert_voucher(pool, voucher).await.ok();
    }
}

async fn sync_invoice(config: &Configuration, pool: &PgPool, invoice_id: String) {
    let result = lexoffice::get_invoice(config, invoice_id.clone())
        .await
        .and_then(move |invoice| Ok(db::insert_invoice(pool, invoice)));
    match result {
        Ok(_) => println!("Synced invoice {}", invoice_id),
        Err(e) => println!("Error syncing invoice: {:?}", e),
    }
}

async fn sync_lexoffice(pool: &PgPool) {
    let api_key = env::var("LEXOFFICE_APIKEY").expect("'LEXOFFICE_APIKEY' must bet set!");
    let config = lexoffice::get_config(api_key);

    // First get all vouchers from voucherlist endpoint and save into DB
    let mut current_page = 1;
    let page_size = 250;
    loop {
        let res = lexoffice::get_voucherlist(&config, current_page, page_size).await;
        match res {
            Err(e) => println!("error getting voucherlist: {}", e),
            Ok(voucher_list) => {
                println!(
                    "Fetched {} of {} vouchers",
                    voucher_list.number_of_elements.unwrap(),
                    voucher_list.total_elements.unwrap()
                );
                let vouchers = voucher_list.content.unwrap_or(vec![]);
                sync_vouchers(pool, vouchers.clone()).await;

                for v in &vouchers[..3] {
                    println!("Got Voucher: {:?}", v.contact_name.as_ref());
                    println!(" - Voucher Date: {:?}", v.voucher_date.as_ref());
                }

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

    let all_invoices = db::get_all_invoices(pool).await.unwrap_or(vec![]);
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
