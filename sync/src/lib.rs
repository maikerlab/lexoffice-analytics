pub mod db;
pub mod lexoffice;

use log::*;
use num_traits::cast::FromPrimitive;
use openapi::models::*;
use sqlx::{
    types::{
        chrono::{DateTime, NaiveDateTime},
        BigDecimal,
    },
    Error,
};

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
            total_net_amount: BigDecimal::from_f64(invoice.total_price.total_net_amount).unwrap(),
            total_gross_amount: BigDecimal::from_f64(invoice.total_price.total_gross_amount)
                .unwrap(),
            total_tax_amount: BigDecimal::from_f64(invoice.total_price.total_tax_amount).unwrap(),
            total_discount_absolute: BigDecimal::from_f64(
                invoice.total_price.total_discount_absolute.unwrap_or(0.0),
            )
            .unwrap(),
            total_discount_percentage: BigDecimal::from_f64(
                invoice.total_price.total_discount_percentage.unwrap_or(0.0),
            )
            .unwrap(),
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
            total_amount: BigDecimal::from_f64(v.total_amount.unwrap_or(0.0)).unwrap(),
            open_amount: BigDecimal::from_f64(v.open_amount.unwrap_or(0.0)).unwrap(),
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
            product_id: item.id.map(|id| id.to_string()).unwrap_or("".to_string()),
            voucher_id: "".to_string(),
            quantity: BigDecimal::from_f64(item.quantity).unwrap(),
            unit_name: item.unit_name.unwrap_or("".to_string()),
            currency: item
                .unit_price
                .clone()
                .map(|up| up.currency.enum_to_string())
                .unwrap_or("".to_string()),
            net_amount: item
                .unit_price
                .clone()
                .map(|up| BigDecimal::from_f64(up.net_amount).unwrap())
                .unwrap_or(BigDecimal::from(0)),
            gross_amount: item
                .unit_price
                .clone()
                .map(|up: Box<UnitPrice>| BigDecimal::from_f64(up.gross_amount).unwrap())
                .unwrap_or(BigDecimal::from(0)),
            tax_rate_percentage: item
                .unit_price
                .clone()
                .map(|up| BigDecimal::from_f64(up.tax_rate_percentage).unwrap()),
            discount_percentage: item
                .discount_percentage
                .map(|p| BigDecimal::from_f64(p).unwrap()),
            line_item_amount: item
                .line_item_amount
                .map(|a| BigDecimal::from_f64(a).unwrap()),
        }
    }
}

impl From<LineItem> for DbProduct {
    fn from(item: LineItem) -> Self {
        Self {
            id: item.id.map(|id| id.to_string()).unwrap_or("".to_string()),
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
    // For each voucher type to sync - get from endpoint and save to DB
    for voucher_type in types {
        info!("Start syncing {} ...", voucher_type.clone());
        let vouchers_by_type = app.db.get_vouchers_by_type(voucher_type).await;
        match vouchers_by_type {
            Ok(vouchers) => {
                for v in vouchers {
                    if v.voucher_type == "invoice" {
                        let _ = sync_invoice(app, v.id.clone()).await;
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
}

pub async fn sync_voucherlist(app: &App) {
    // Get all vouchers from voucherlist endpoint and save into DB
    let available_vouchers = app.db.get_all_vouchers().await.unwrap_or(vec![]).len() as i32;
    info!("Already available vouchers: {}", available_vouchers);

    let mut current_page = 1;
    let mut fetched_vouchers = 0;
    let page_size = 250;
    loop {
        let res = app
            .api
            .get_voucherlist("any".to_string(), current_page, page_size)
            .await;
        //.expect("Error while fetching voucherlist");
        match res {
            Err(e) => {
                error!("error getting voucherlist: {}", e);
                current_page += 1;
            }
            Ok(voucher_list) => {
                if available_vouchers >= voucher_list.total_elements {
                    info!("Voucherlist is up-to-date");
                    break;
                }
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
                //break; // TODO: just for during development - remove this!
                current_page += 1;
            }
        }
    }

    info!("Finished syncing Voucherlist!");
}

pub async fn sync_vouchers(app: &App, vouchers: Vec<VoucherlistVoucher>) {
    let mut inserted = 0;
    for voucher in &vouchers {
        if !app.db.voucher_exists(voucher.id.to_string()).await {
            let db_voucher = DbVoucher::from(voucher.to_owned());
            match app.db.add_voucher(db_voucher).await {
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

async fn add_address_if_not_existing(
    db: &LexofficeDb,
    address: &VoucherAddress,
) -> Option<DbAddress> {
    let db_address = DbAddress::from((*address).clone());
    if !db.address_exists_by_id_or_collective(&db_address).await {
        let inserted = db.add_address(db_address).await;
        match inserted {
            Ok(address) => {
                info!(
                    "Added new address: {}, {}, {} {}",
                    address.name,
                    address.street.clone().unwrap_or("".to_string()),
                    address.zip.clone().unwrap_or("".to_string()),
                    address.city.clone().unwrap_or("".to_string())
                );
                return Some(address);
            }
            Err(e) => {
                error!("Error while adding address: {:?}", e);
            }
        }
    }
    return None;
}

async fn add_line_items(
    db: &LexofficeDb,
    invoice_id: String,
    line_items: &Vec<LineItem>,
) -> Result<u64, Error> {
    let mut li_inserted = 0;
    for item in line_items {
        // Currently only items with a valid ID are inserted
        if item.id.is_some() && !item.id.unwrap().is_nil() {
            if !db.product_exists(item.id.unwrap().to_string()).await {
                let db_product = DbProduct::from(item.to_owned());
                match db.add_product(db_product).await {
                    Ok(_) => {
                        let i = item.clone();
                        info!(
                            "Added new product: {}   {}",
                            i.name,
                            i.description.unwrap_or("".to_string())
                        );
                    }
                    Err(e) => error!("Error while adding product: {:?}", e),
                }
            }
            let db_lineitem = DbLineItem::from(item.to_owned());
            match db
                .add_lineitem(
                    db_lineitem,
                    item.id.unwrap().to_string(),
                    invoice_id.clone(),
                )
                .await
            {
                Ok(_) => li_inserted += 1,
                Err(e) => {
                    error!("Error while adding line item: {:?} - {:?}", item.id, e)
                }
            }
        }
    }
    Ok(li_inserted)
}

pub async fn sync_invoice(app: &App, invoice_id: String) -> Result<(), Error> {
    // If invoice already exists, skip sync!
    if app.db.invoice_exists(invoice_id.clone()).await {
        return Ok(());
    }

    // Get invoice from API and insert into DB
    match app.api.get_invoice(invoice_id.clone()).await {
        Ok(ref invoice) => {
            // Insert address
            add_address_if_not_existing(&app.db, &invoice.address).await;
            // ... then invoice
            let db_invoice = DbInvoice::from(invoice.to_owned());
            match app.db.add_invoice(db_invoice).await {
                Ok(_) => {
                    // ... then line items
                    match add_line_items(&app.db, invoice.id.to_string(), &invoice.line_items).await
                    {
                        Ok(n_inserted) => {
                            info!(
                                "Added new invoice ({}) with {} line items",
                                invoice.voucher_number, n_inserted
                            );
                        }
                        Err(e) => error!("Error while adding line items: {:?}", e),
                    }
                }
                Err(e) => error!("Error while adding invoice: {:?}", e),
            }
        }
        Err(e) => error!("Error while fetching invoice: {:?}", e),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
