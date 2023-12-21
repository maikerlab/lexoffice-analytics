use openapi::apis::{
    configuration::Configuration,
    default_api::{invoices_id_get, voucherlist_get},
};

pub async fn sync_invoice(config: &Configuration, id: uuid::Uuid) {
    let response = invoices_id_get(config, id.to_string().as_str()).await;
    match response {
        Ok(invoice) => println!("Got Invoice: {}", invoice.voucher_number.unwrap()),
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
            for v in vouchers.content.unwrap() {
                println!("Got Voucher: {:?}", v.contact_name.unwrap());
                sync_invoice(config, v.id.unwrap()).await;
                if !vouchers.last.unwrap_or_default() {
                    //sync_voucherlist(config, page + 1, size).await;
                }
            }
        }
    }
}
