use std::time::Duration;
use futures::{StreamExt, TryStreamExt};
use log::{error, info};
use mongodb::{Client, Collection, Database};
use mongodb::bson::{doc, Document};
use mongodb::options::{InsertOneOptions, WriteConcern};
use tokio::time::sleep;
use openapi::apis::configuration::Configuration;
use openapi::apis::{Error, vouchers_api};
use openapi::models::{VoucherList, VoucherlistVoucher};
use lexoffice_models::{Invoice, Product, Address, LineItem, INVOICES_COLLECTION_NAME};
use openapi::apis::invoices_api::{invoices_id_get, InvoicesIdGetError};

pub async fn sync_invoices(api_config: &Configuration, db: &Database) -> mongodb::error::Result<()> {
    let invoice_coll: Collection<Invoice> = db
        .collection(INVOICES_COLLECTION_NAME);
    let doc_count_before = invoice_coll.count_documents(doc! { "voucher_status": "paid" }, None)
        .await?;
    println!("Collection contains {} documents", doc_count_before);

    let voucher_list = vouchers_api::voucherlist_get(
        api_config,
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
        Some(1),
        Some(50),
        Some("voucherDate,DESC"),
    )
        .await
        .expect("Error getting voucherlist");

    // TODO: For testing take only a slice
    for mut voucher in &voucher_list.content.to_vec()[..3] {
        info!("Syncing voucher: {}", voucher.id);
        let invoice = invoices_id_get(api_config, voucher.id.to_string().as_str())
            .await;
        match invoice {
            Ok(i) => {
                let invoice: Invoice = i.clone().into();
                // TODO: For testing delete old entry first
                invoice_coll.delete_one(doc! { "voucher_number": i.voucher_number }, None).await?;
                match invoice_coll.insert_one(invoice.clone(), None).await {
                    Ok(_) => { info!("Inserted new invoice: {:?}", invoice) }
                    Err(err) => { error!("Error inserting invoice - already exists? {:?}", err.kind) }
                }
            }
            Err(e) => {
                error!("Error while fetching invoice from lexoffice API: {:?}", e);
            }
        }
        sleep(Duration::from_millis(500)).await;
    }

    let doc_count_after = invoice_coll.count_documents(doc! { "voucher_status": "paid" }, None)
        .await?;
    println!("Collection contains {} documents", doc_count_after);
    println!(">>> Inserted {} documents!", doc_count_after-doc_count_before);

    Ok(())
}

pub async fn connect(connection_string: &str, db_name: &str) -> mongodb::error::Result<Database> {
    // Create a new client and connect to the server
    let client = Client::with_uri_str(connection_string.to_string()).await?;
    let database = client.database(db_name);
    Ok(database)
}
