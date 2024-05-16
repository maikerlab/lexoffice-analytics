use std::net::Shutdown::Write;
use std::time::Duration;
use futures::TryStreamExt;
use log::error;
use mongodb::{Client, Collection, Database};
use mongodb::bson::{doc, Document};
use mongodb::options::{InsertOneOptions, WriteConcern};
use tokio::time::sleep;
use openapi::apis::configuration::Configuration;
use openapi::apis::{Error, vouchers_api};
use openapi::models::{VoucherList, VoucherlistVoucher};
use lexoffice_models::{Invoice, Product, Customer, Sale, INVOICES_COLLECTION_NAME};
use openapi::apis::invoices_api::{invoices_id_get, InvoicesIdGetError};

pub async fn sync_invoices(api_config: &Configuration, db: &Database) -> mongodb::error::Result<()> {
    let invoice_coll: Collection<Invoice> = db
        .collection(INVOICES_COLLECTION_NAME);

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
    for mut voucher in &voucher_list.content.to_vec()[..10] {
        println!("Voucher: {:?}", voucher);
        let invoice = invoices_id_get(api_config, voucher.id.to_string().as_str())
            .await;
        match invoice {
            Ok(i) => {
                let invoice: Invoice = i.into();
                invoice_coll.insert_one(invoice, None)
                    .await
                    .ok();
            }
            Err(e) => {
                error!("Error while fetching invoice from lexoffice API: {:?}", e);
            }
        }
        sleep(Duration::from_millis(500)).await;
    }

    let mut cursor = invoice_coll.find(
        doc! { "voucher_status": "paid" },
        None
    ).await?;
    while let Some(doc) = cursor.try_next().await? {
        println!("Invoice Number: {:?}", doc.voucher_number);
    }

    Ok(())
}

pub async fn connect(connection_string: &str, db_name: &str) -> mongodb::error::Result<Database> {
    // Create a new client and connect to the server
    let client = Client::with_uri_str(connection_string.to_string()).await?;
    let database = client.database(db_name);
    Ok(database)
}

async fn insert_invoices(db: &Database, voucher_list: VoucherList) -> mongodb::error::Result<()> {
    let my_coll: Collection<VoucherlistVoucher> = db
        .collection("invoices");

    for mut voucher in voucher_list.content {
        println!("Voucher: {:?}", voucher);
        my_coll.insert_one(voucher, None)
            .await?;
    }

    let mut cursor = my_coll.find(
        doc! { "voucherStatus": "paid" },
        None
    ).await?;
    while let Some(doc) = cursor.try_next().await? {
        println!("Voucher status: {:?}", doc.voucher_status);
    }

    Ok(())
}
pub async fn test(db: &Database, collection_name: &str) -> mongodb::error::Result<()> {
    let my_coll: Collection<Document> = db.collection(collection_name);

    // Find a movie based on the title value
    let my_movie = my_coll.find_one(doc! { "title": "The Perils of Pauline" }, None).await?;

    // Print the document
    println!("Found a movie:\n{:#?}", my_movie);
    Ok(())
}