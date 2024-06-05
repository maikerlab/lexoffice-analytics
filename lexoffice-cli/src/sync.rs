use chrono::{DateTime, Utc};
use futures::TryFutureExt;
use indicatif::ProgressBar;
use log::{debug, info};
use mongodb::{Client, Collection, Database};
use mongodb::bson::doc;
use lexoffice_models::{Invoice, INVOICES_COLLECTION_NAME};
use openapi::apis::invoices_api::invoices_id_get;
use openapi::apis::vouchers_api::voucherlist_get;
use crate::{LexofficeClient, MyError};

pub async fn sync_invoices(client: &LexofficeClient, db: &Database, from_date: Option<DateTime<Utc>>, to_date: Option<DateTime<Utc>>) -> Result<(), MyError> {
    let invoice_coll: Collection<Invoice> = db
        .collection(INVOICES_COLLECTION_NAME);
    let doc_count_before = invoice_coll.count_documents(doc! { }, None)
        .map_err(|err| MyError::MongoDbError("Error getting number of documents".to_string(), *err.kind))
        .await?;
    debug!("Before sync: Collection contains {} documents", doc_count_before);

    let mut current_page = 0;
    let mut progress_bar: Option<ProgressBar> = None;
    loop {
        client.limiter.acquire(1).await;
        let voucher_list = voucherlist_get(
            &client.config,
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
        ).await.map_err(|err| MyError::LexofficeApiError(err.to_string()))?;

        debug!("Syncing {} invoices of page no. {}...", voucher_list.content.len(), current_page);
        if progress_bar.is_none() {
            progress_bar = Some(ProgressBar::new(voucher_list.total_elements as u64));
        }

        // Sync all invoices of current page
        for voucher in voucher_list.content {
            client.limiter.acquire(1).await;
            let lexoffice_invoice = invoices_id_get(&client.config, voucher.id.to_string().as_str())
                .await
                .map_err(|_| MyError::LexofficeApiError("Error while fetching invoice from lexoffice API".to_string()))?;
            let invoice: Invoice = lexoffice_invoice.clone().into();

            // TODO: For testing delete old entry first
            let _ = invoice_coll.delete_one(doc! { "voucher_number": &lexoffice_invoice.voucher_number }, None).await;
            debug!("Deleted invoice {}", &lexoffice_invoice.voucher_number);

            invoice_coll.insert_one(invoice.clone(), None)
                .await
                .map_err(|err| MyError::MongoDbError("Error inserting invoice".to_string(), *err.kind))?;
            debug!("Inserted invoice: ID={}, Number={}, Date={}", voucher.id, voucher.voucher_number, voucher.voucher_date);

            if let Some(ref pb) = progress_bar { pb.inc(1); }
        }

        // If this is the last page, we are done!
        if voucher_list.last {
            break;
        }
        // Fetch next page...
        current_page += 1;
    }

    if let Some(ref pb) = progress_bar { pb.finish_and_clear(); }

    let doc_count_after = invoice_coll.count_documents(doc! { }, None)
        .await
        .map_err(|err| MyError::MongoDbError("Error getting number of documents".to_string(), *err.kind))?;

    debug!("After sync: Collection contains {} documents", doc_count_after);
    info!("DONE! Inserted {} new documents.", doc_count_after - doc_count_before);

    Ok(())
}

pub async fn connect_db(connection_string: &str, db_name: &str) -> mongodb::error::Result<Database> {
    // Create a new client and connect to the server
    let client = Client::with_uri_str(connection_string.to_string()).await?;
    let database = client.database(db_name);
    Ok(database)
}
