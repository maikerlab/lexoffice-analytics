use core::time;
use std::thread::sleep;
pub mod utils;
use crate::db;
use once_cell::sync::Lazy;
use std::env;

use openapi::apis::{
    configuration::Configuration,
    default_api::{invoices_id_get, voucherlist_get},
};

pub static MAX_REQUESTS_PER_SECOND: Lazy<f32> = Lazy::new(|| match env::var("API_RATE_LIMIT") {
    Ok(value) => value.parse().unwrap_or(2.0),
    Err(_) => 2.0,
});

pub async fn sync_invoice(config: &Configuration, id: uuid::Uuid) {
    let response = invoices_id_get(config, id.to_string().as_str()).await;
    match response {
        Ok(invoice) => {
            let _result = db::insert_invoice(invoice.clone());
            println!("Inserted Invoice: {}", invoice.voucher_number.unwrap());
        }
        Err(e) => println!("Error fetching invoice: {}", e),
    }
}

pub async fn sync_voucherlist(config: &Configuration, page: i32, size: i32) {
    println!("syncing voucherlist (page {})", page);

    let res = voucherlist_get(
        config,
        "invoice",
        "any",
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(page),
        Some(size),
        Some("voucherDate,DESC"),
    )
    .await;

    match res {
        Err(e) => println!("error getting voucherlist: {}", e),
        Ok(vouchers) => {
            println!(
                "Fetched {} of {} vouchers",
                vouchers.number_of_elements.unwrap(),
                vouchers.total_elements.unwrap()
            );
            let test = vouchers.content.clone().unwrap_or(vec![]);
            for v in &test[..3] {
                //for v in vouchers.content.unwrap() {
                println!("Got Voucher: {:?}", v.contact_name.as_ref().unwrap());
                println!(" - Voucher Date: {:?}", v.voucher_date.as_ref().unwrap());
                sync_invoice(config, v.id.unwrap()).await;
                if !vouchers.last.unwrap_or_default() {
                    //sync_voucherlist(config, page + 1, size).await;
                }
                let wait_ms = utils::get_api_rate_ms(*MAX_REQUESTS_PER_SECOND);
                println!("Waiting {} ms for next API call", wait_ms);
                sleep(time::Duration::from_millis(wait_ms));
            }
        }
    }
}
