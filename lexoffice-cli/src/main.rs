pub mod db;
pub mod lexoffice;

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
        sync_lexoffice(&self.db_pool).await;
        Ok(())
    }
}

async fn sync_vouchers(pool: &PgPool, vouchers: Vec<VoucherlistVoucher>) {
    for voucher in vouchers {
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
