use std::time::Duration;
use chrono::{DateTime, Utc};
use futures::TryFutureExt;
use log::{debug, info};
use mongodb::{Client, Collection, Database};
use mongodb::bson::doc;
use tokio::time::sleep;
use openapi::apis::configuration::Configuration;
use openapi::apis::vouchers_api;
use lexoffice_models::{Invoice, INVOICES_COLLECTION_NAME};
use openapi::apis::invoices_api::invoices_id_get;
use crate::MyError;

pub async fn sync_invoices(api_config: &Configuration, db: &Database, from_date: Option<DateTime<Utc>>, to_date: Option<DateTime<Utc>>) -> Result<(), MyError> {
    let invoice_coll: Collection<Invoice> = db
        .collection(INVOICES_COLLECTION_NAME);
    let doc_count_before = invoice_coll.count_documents(doc! { }, None)
        .map_err(|err| MyError::MongoDbError("Error getting number of documents".to_string(), *err.kind))
        .await?;
    debug!("Before sync: Collection contains {} documents", doc_count_before);

    let mut current_page = 0;
    loop {
        let voucher_list = vouchers_api::voucherlist_get(
            api_config,
            "invoice",
            "any",
            None,
            None,
            from_date.map(|date| format!("{}", date.format("%Y-%m-%d"))),
            to_date.map(|date| format!("{}", date.format("%Y-%m-%d"))),
            None,
            None,
            None,
            None,
            None,
            Some(current_page),
            Some(250),
            Some("voucherDate,ASC"),
        )
            .await
            .map_err(|err| MyError::LexofficeApiError(err.to_string()))?;

        info!("Syncing {} invoices...", voucher_list.content.len());
        for voucher in voucher_list.content {
            let lexoffice_invoice = invoices_id_get(api_config, voucher.id.to_string().as_str())
                .await
                .map_err(|_| MyError::LexofficeApiError("Error while fetching invoice from lexoffice API".to_string()))?;
            let invoice: Invoice = lexoffice_invoice.clone().into();

            // TODO: For testing delete old entry first
            invoice_coll.delete_one(doc! { "voucher_number": lexoffice_invoice.voucher_number }, None).await.map_err(|err| MyError::MongoDbError("Error deleting old entry".to_string(), *err.kind))?;
            debug!("Deleted old entry");

            invoice_coll.insert_one(invoice.clone(), None)
                .await
                .map_err(|err| MyError::MongoDbError("Error inserting invoice".to_string(), *err.kind))?;
            info!("Inserted invoice: ID={}, Number={}, Date={}", voucher.id, voucher.voucher_number, voucher.voucher_date);
            sleep(Duration::from_millis(500)).await;
        }

        if voucher_list.last {
            break;
        }
        current_page += 1;
    }

    let doc_count_after = invoice_coll.count_documents(doc! { }, None)
        .await
        .map_err(|err| MyError::MongoDbError("Error getting number of documents".to_string(), *err.kind))?;

    debug!("After sync: Collection contains {} documents", doc_count_after);
    info!("DONE! Inserted {} new documents.", doc_count_after - doc_count_before);

    Ok(())
}

pub async fn connect(connection_string: &str, db_name: &str) -> mongodb::error::Result<Database> {
    // Create a new client and connect to the server
    let client = Client::with_uri_str(connection_string.to_string()).await?;
    let database = client.database(db_name);
    Ok(database)
}
