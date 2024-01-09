pub mod models;
use self::models::{DbInvoice, DbVoucher};
use crate::lexoffice::EnumToString;
use openapi::models::*;
use sqlx::{
    postgres::PgPoolOptions,
    types::chrono::{DateTime, NaiveDateTime},
    Error, PgPool,
};
use std::{borrow::Borrow, env, str::FromStr};

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
            id: invoice.id.unwrap_or_default().to_string(),
            organization_id: invoice.organization_id.map(|val| val.to_string()),
            created_date: parse_datetime(invoice.created_date.unwrap_or_default()),
            updated_date: parse_datetime(invoice.updated_date.unwrap_or_default()),
            version: None,
            language: None,
            archived: None,
            voucher_status: Some(invoice.voucher_status.unwrap().enum_to_string()),
            voucher_number: None,
            voucher_date: parse_datetime(invoice.voucher_date.unwrap_or_default()),
            due_date: parse_datetime(invoice.due_date.unwrap_or_default()),
            address_id: None,
            address_name: None,
            address_supplement: None,
            address_street: None,
            address_city: None,
            address_zip: None,
            address_countrycode: None,
        }
    }
}

impl From<VoucherlistVoucher> for DbVoucher {
    fn from(v: VoucherlistVoucher) -> Self {
        DbVoucher {
            id: v.id.unwrap_or_default().to_string(),
            voucher_type: Some(v.voucher_type.enum_to_string()),
            voucher_status: Some(v.voucher_status.enum_to_string()),
            voucher_number: v.voucher_number,
            voucher_date: parse_datetime(v.voucher_date.unwrap_or_default()),
            created_date: parse_datetime(v.created_date.unwrap_or_default()),
            updated_date: parse_datetime(v.updated_date.unwrap_or_default()),
            due_date: parse_datetime(v.due_date.unwrap_or_default()),
            contact_id: match v.contact_id {
                Some(c_id) => Some(c_id.to_string()),
                None => None,
            },
            contact_name: v.contact_name,
            total_amount: v.total_amount.map(|val| f64::from(val)),
            open_amount: v.open_amount.map(|val| f64::from(val)),
            currency: v.currency.map(|val| val.enum_to_string()),
            archived: v.archived,
        }
    }
}

pub async fn connect_db() -> Result<PgPool, Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}

pub async fn get_all_vouchers(pool: &PgPool) -> Result<Vec<DbVoucher>, Error> {
    sqlx::query_as!(
        DbVoucher,
r#"
    SELECT id, archived, contact_id, contact_name, voucher_date, created_date, due_date, updated_date, voucher_number, voucher_type, voucher_status, total_amount, open_amount, currency FROM vouchers
"#
    )
    .fetch_all(pool).await
}

pub async fn get_voucher_by_id(pool: &PgPool, voucher_id: String) -> Result<DbVoucher, Error> {
    sqlx::query_as!(
        DbVoucher,
        r#"select * from vouchers where id = $1"#,
        voucher_id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_vouchers_by_type(
    pool: &PgPool,
    voucher_type: String,
) -> Result<Vec<DbVoucher>, Error> {
    sqlx::query_as!(
        DbVoucher,
        r#"select * from vouchers where voucher_type = $1"#,
        voucher_type
    )
    .fetch_all(pool)
    .await
}

pub async fn get_all_invoices(pool: &PgPool) -> Result<Vec<DbInvoice>, Error> {
    sqlx::query_as!(DbInvoice, r#"select * from invoices"#)
        .fetch_all(pool)
        .await
}

pub async fn insert_voucher(pool: &PgPool, voucher: DbVoucher) -> Result<usize, Error> {
    let rec = sqlx::query!(
        r#"
INSERT INTO vouchers ( id, voucher_type, voucher_status, voucher_number, voucher_date, created_date, updated_date, due_date, contact_id, contact_name, total_amount, open_amount, currency, archived )
VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14 )
RETURNING id
        "#,
        voucher.id,
        voucher.voucher_type,
        voucher.voucher_status,
        voucher.voucher_number,
        voucher.voucher_date,
        voucher.created_date,
        voucher.updated_date,
        voucher.due_date,
        voucher.contact_id,
        voucher.contact_name,
        voucher.total_amount,
        voucher.open_amount,
        voucher.currency,
        voucher.archived
    )
    .fetch_one(pool)
    .await?;

    println!("Inserted voucher with ID: {:?}", rec.id);

    Ok(1)
}

pub async fn voucher_exists(pool: &PgPool, voucher_id: String) -> bool {
    let result = sqlx::query!(
        r#"
SELECT EXISTS(SELECT 1 FROM vouchers WHERE id=$1)
        "#,
        voucher_id
    )
    .fetch_one(pool)
    .await;

    //println!("voucher_exists: {:?}", rec.exists.unwrap_or(false));
    let res = match result {
        Ok(rec) => rec.exists.unwrap(),
        Err(_) => false,
    };

    res
}

pub async fn insert_invoice(pool: &PgPool, invoice: DbInvoice) -> Result<usize, Error> {
    let _rec = sqlx::query!(
        r#"
INSERT INTO invoices ( id, voucher_status, voucher_number, voucher_date )
VALUES ( $1, $2, $3, $4 )
RETURNING id
        "#,
        invoice.id,
        invoice.voucher_status,
        invoice.voucher_number,
        invoice.voucher_date
    )
    .fetch_one(pool)
    .await?;
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use sqlx::types::Uuid;

    #[test]
    async fn test_insert_invoice() {
        let invoice_id = Uuid::new_v4();
        let invoice = DbInvoice {
            id: invoice_id.to_string(),
            organization_id: None,
            created_date: None,
            updated_date: None,
            version: None,
            language: None,
            archived: None,
            voucher_status: Some("draft".to_string()),
            voucher_number: Some("test".to_string()),
            voucher_date: None,
            due_date: None,
            address_id: Some("a1".to_string()),
            address_name: Some("Max Mustermann".to_string()),
            address_street: Some("Musterstr. 1".to_string()),
            address_zip: Some("20095".to_string()),
            address_city: Some("Hamburg".to_string()),
            address_countrycode: Some("DE".to_string()),
            address_supplement: Some("none".to_string()),
        };
        let pool = block_on(connect_db()).unwrap();
        let result = insert_invoice(&pool, invoice).await;
        assert_eq!(result.unwrap(), 1);
    }
}
