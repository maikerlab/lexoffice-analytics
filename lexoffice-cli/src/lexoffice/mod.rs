use std::{thread::sleep, time::Duration};
pub mod utils;

use openapi::{
    apis::{
        configuration::Configuration,
        default_api::{invoices_id_get, voucherlist_get, InvoicesIdGetError, VoucherlistGetError},
        Error,
    },
    models::{Invoice, VoucherList},
};

pub const MAX_REQUESTS_PER_SECOND: f32 = 2.0;

pub fn get_config(api_key: String) -> Configuration {
    let mut conf = Configuration::default();
    conf.bearer_access_token = Some(api_key);
    conf
}

fn request_delay() {
    sleep(Duration::from_millis(utils::get_api_rate_ms(
        MAX_REQUESTS_PER_SECOND,
    )));
}

pub async fn get_invoice(
    config: &Configuration,
    id: String,
) -> Result<Invoice, Error<InvoicesIdGetError>> {
    request_delay();
    let response = invoices_id_get(config, id.as_str()).await;
    response
}

pub async fn get_voucherlist(
    config: &Configuration,
    page: i32,
    size: i32,
) -> Result<VoucherList, Error<VoucherlistGetError>> {
    request_delay();
    println!("syncing voucherlist (page {})", page);

    voucherlist_get(
        config,
        "any",
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
    .await
}
