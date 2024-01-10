use log::*;
use openapi::models::*;
use sqlx::types::chrono::{DateTime, NaiveDateTime};

use crate::{
    db::{models::*, LexofficeDb},
    lexoffice::{EnumToString, LexofficeApi},
};

fn parse_datetime(datetime_str: String) -> Option<NaiveDateTime> {
    let parsed_result = DateTime::parse_from_rfc3339(datetime_str.as_str());
    match parsed_result {
        Ok(dt) => Some(dt.naive_utc()),
        Err(_) => None,
    }
}

// Convert Lexoffice Invoice to database entity
impl From<Invoice> for DbInvoice {
    fn from(invoice: Invoice) -> Self {
        //let _address = invoice.address.map(|a| );
        DbInvoice {
            id: invoice.id.to_string(),
            organization_id: invoice.organization_id.map(|val| val.to_string()),
            created_date: parse_datetime(invoice.created_date).unwrap(),
            updated_date: parse_datetime(invoice.updated_date).unwrap(),
            version: invoice.version,
            language: invoice.language.enum_to_string(),
            archived: match invoice.archived {
                true => 1,
                false => 0,
            },
            voucher_status: invoice.voucher_status.enum_to_string(),
            voucher_number: invoice.voucher_number,
            voucher_date: parse_datetime(invoice.voucher_date).unwrap(),
            due_date: parse_datetime(invoice.due_date),
            address_id: invoice.address.contact_id.map(|id| id.to_string()),
            currency: invoice.total_price.currency.enum_to_string(),
            total_net_amount: invoice.total_price.total_net_amount as f64,
            total_gross_amount: invoice.total_price.total_gross_amount as f64,
            total_tax_amount: invoice.total_price.total_tax_amount as f64,
            total_discount_absolute: invoice.total_price.total_discount_absolute.unwrap_or(0.0)
                as f64,
            total_discount_percentage: invoice.total_price.total_discount_percentage.unwrap_or(0.0)
                as f64,
        }
    }
}

impl From<VoucherlistVoucher> for DbVoucher {
    fn from(v: VoucherlistVoucher) -> Self {
        DbVoucher {
            id: v.id.to_string(),
            voucher_type: v.voucher_type.enum_to_string(),
            voucher_status: v.voucher_status.enum_to_string(),
            voucher_number: v.voucher_number,
            voucher_date: parse_datetime(v.voucher_date).unwrap(),
            created_date: parse_datetime(v.created_date).unwrap(),
            updated_date: parse_datetime(v.updated_date).unwrap(),
            due_date: parse_datetime(v.due_date.unwrap_or_default()),
            contact_id: match v.contact_id {
                Some(c_id) => Some(c_id.unwrap().to_string()),
                None => None,
            },
            contact_name: v.contact_name,
            total_amount: v.total_amount as f64,
            open_amount: v.open_amount as f64,
            currency: v.currency.enum_to_string(),
            archived: match v.archived {
                true => 1,
                false => 0,
            },
        }
    }
}

impl From<LineItem> for DbLineItem {
    fn from(item: LineItem) -> Self {
        Self {
            id: 1,
            product_id: item
                .id
                .map(|id| id.to_string())
                .unwrap_or_else(|| "".to_string()),
            voucher_id: "".to_string(),
            quantity: item.quantity as f64,
            unit_name: item.unit_name.unwrap_or("".to_string()),
            currency: item
                .unit_price
                .clone()
                .map(|up| up.currency.enum_to_string())
                .unwrap_or_else(|| "".to_string()),
            net_amount: item
                .unit_price
                .clone()
                .map(|up| up.net_amount as f64)
                .unwrap_or_else(|| 0.0),
            gross_amount: item
                .unit_price
                .clone()
                .map(|up: Box<UnitPrice>| up.gross_amount as f64)
                .unwrap_or_else(|| 0.0),
            tax_rate_percentage: item
                .unit_price
                .clone()
                .map(|up| up.tax_rate_percentage as f64),
            discount_percentage: item.discount_percentage.map(|p| p as f64),
            line_item_amount: item.line_item_amount.map(|a| a as f64),
        }
    }
}

impl From<LineItem> for DbProduct {
    fn from(item: LineItem) -> Self {
        Self {
            id: item
                .id
                .map(|id| id.to_string())
                .unwrap_or_else(|| "".to_string()),
            product_type: item.r#type.enum_to_string(),
            name: item.name,
            description: item.description,
        }
    }
}

impl From<VoucherAddress> for DbAddress {
    fn from(a: VoucherAddress) -> Self {
        Self {
            contact_id: match a.contact_id {
                Some(c_id) => c_id.to_string(),
                None => "".to_string(),
            },
            name: a.name,
            supplement: a.supplement.map(|s| s.unwrap()),
            street: a.street,
            city: a.city,
            zip: a.zip,
            country_code: a.country_code,
        }
    }
}

pub struct App {
    pub api: LexofficeApi,
    pub db: LexofficeDb,
}

pub async fn sync_lexoffice(app: &App, types: Vec<String>) {
    // First get all vouchers from voucherlist endpoint and save into DB
    let mut current_page = 1;
    let mut fetched_vouchers = 0;
    let page_size = 250;
    loop {
        let res = app
            .api
            .get_voucherlist(types.join(","), current_page, page_size)
            .await;
        match res {
            Err(e) => error!("error getting voucherlist: {}", e),
            Ok(voucher_list) => {
                fetched_vouchers += voucher_list.number_of_elements;
                info!(
                    "Fetched {} of {} vouchers from Voucherlist",
                    fetched_vouchers, voucher_list.total_elements
                );
                let vouchers = voucher_list.content;
                sync_vouchers(app, vouchers).await;

                if voucher_list.last {
                    break;
                }
                break; // TODO: just for during development - remove this!
                current_page += 1;
            }
        }
    }

    // For each voucher type to sync - get from endpoint and save to DB
    for t in types {
        let vouchers_by_type = app.db.get_vouchers_by_type(t).await;
        match vouchers_by_type {
            Ok(vouchers) => {
                for v in vouchers {
                    if v.voucher_type == "invoice" {
                        sync_invoice(app, v.id).await;
                    } else {
                        warn!(
                            "Sync. of voucher type {} currently not supported",
                            v.voucher_type
                        );
                    }
                }
            }
            Err(_) => {}
        }
    }

    let invoices = app
        .db
        .get_vouchers_by_type("invoice".to_string())
        .await
        .unwrap_or(vec![]);
    for voucher in invoices {
        sync_invoice(app, voucher.id).await;
    }
}

pub async fn sync_vouchers(app: &App, vouchers: Vec<VoucherlistVoucher>) {
    let mut inserted = 0;
    for voucher in &vouchers {
        if !app.db.voucher_exists(voucher.id.to_string()).await {
            let db_voucher = DbVoucher::from(voucher.to_owned());
            match app.db.insert_voucher(db_voucher).await {
                Ok(_) => {
                    inserted += 1;
                    info!("Added new voucher: {}", voucher.voucher_number);
                }
                Err(_) => error!("Error while adding voucher"),
            }
        }
    }

    info!("Synced {} vouchers, inserted: {}", vouchers.len(), inserted);
}

pub async fn sync_invoice(app: &App, invoice_id: String) {
    if !app.db.invoice_exists(invoice_id.clone()).await {
        match app.api.get_invoice(invoice_id.clone()).await {
            Ok(ref invoice) => {
                // Insert address if not existing
                let db_address = DbAddress::from((*invoice.address).clone());
                match app.db.insert_address(db_address).await {
                    Ok(address_id) => info!("Added new address: {}", address_id),
                    Err(e) => error!("Error while adding address: {:?}", e),
                }

                // Insert invoice
                let db_invoice = DbInvoice::from(invoice.to_owned());
                match app.db.insert_invoice(db_invoice).await {
                    Ok(_) => {
                        info!("Added new invoice: {}", invoice.voucher_number);
                        // Insert line items
                        let mut li_inserted = 0;
                        for item in &invoice.line_items {
                            // Currently only items with a valid ID are inserted
                            if item.id.is_some() && !item.id.unwrap().is_nil() {
                                if !app.db.product_exists(item.id.unwrap().to_string()).await {
                                    let db_product = DbProduct::from(item.to_owned());
                                    match app.db.insert_product(db_product).await {
                                        Ok(product_id) => {
                                            info!("Added new product: {}", product_id)
                                        }
                                        Err(e) => error!("Error while adding product: {:?}", e),
                                    }
                                }
                                let db_lineitem = DbLineItem::from(item.to_owned());
                                match app
                                    .db
                                    .insert_lineitem(
                                        db_lineitem,
                                        item.id.unwrap().to_string(),
                                        invoice.id.to_string(),
                                    )
                                    .await
                                {
                                    Ok(_) => li_inserted += 1,
                                    Err(e) => {
                                        error!(
                                            "Error while adding line item: {:?} - {:?}",
                                            item.id, e
                                        )
                                    }
                                }
                            }
                        }
                        info!("Added {} line items of invoice {}", li_inserted, invoice_id);
                    }
                    Err(e) => error!("Error while adding invoice: {:?}", e),
                }
            }
            Err(e) => error!("Error while fetching invoice: {:?}", e),
        }
    }
}
