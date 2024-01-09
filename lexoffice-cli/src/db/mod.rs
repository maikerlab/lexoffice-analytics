pub mod models;
use self::models::{DbInvoice, DbVoucher};
use crate::lexoffice::EnumToString;
use openapi::models::*;
use sqlx::{
    postgres::PgPoolOptions,
    types::chrono::{DateTime, NaiveDateTime},
    Error, PgPool,
};
use std::env;

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
            address_id: None,
            address_name: invoice.address.name,
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
            id: v.id.to_string(),
            voucher_type: v.voucher_type.enum_to_string(),
            voucher_status: v.voucher_status.enum_to_string(),
            voucher_number: v.voucher_number,
            voucher_date: parse_datetime(v.voucher_date).unwrap(),
            created_date: parse_datetime(v.created_date).unwrap(),
            updated_date: parse_datetime(v.updated_date).unwrap(),
            due_date: parse_datetime(v.due_date.unwrap_or_default()),
            contact_id: match v.contact_id {
                Some(c_id) => Some(c_id.to_string()),
                None => None,
            },
            contact_name: v.contact_name,
            total_amount: v.total_amount.map(|val| f64::from(val)),
            open_amount: v.open_amount.map(|val| f64::from(val)),
            currency: v.currency.map(|val| val.enum_to_string()),
            archived: match v.archived {
                true => 1,
                false => 0,
            },
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
        voucher.archived == 1
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

pub async fn insert_invoice(pool: &PgPool, invoice: DbInvoice) -> Result<String, Error> {
    /*
        pub id: String,
    pub organization_id: Option<String>,
    pub created_date: Option<NaiveDateTime>,
    pub updated_date: Option<NaiveDateTime>,
    pub version: Option<i32>,
    pub language: Option<String>,
    pub archived: Option<bool>,
    pub voucher_status: Option<String>,
    pub voucher_number: Option<String>,
    pub voucher_date: Option<NaiveDateTime>,
    pub due_date: Option<NaiveDateTime>,
    pub address_id: Option<String>,
    pub address_name: Option<String>,
    pub address_supplement: Option<String>,
    pub address_street: Option<String>,
    pub address_city: Option<String>,
    pub address_zip: Option<String>,
    pub address_countrycode: Option<String>,
     */
    let rec = sqlx::query!(
        r#"
INSERT INTO invoices ( id, organization_id, created_date, updated_date, version, language, archived, voucher_status, voucher_number, voucher_date, due_date, address_id, address_name, address_supplement, address_street, address_city, address_zip, address_countrycode )
VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18 )
RETURNING id
        "#,
        invoice.id,
        invoice.organization_id,
        invoice.created_date,
        invoice.updated_date,
        invoice.version,
        invoice.language,
        invoice.archived == 1,
        invoice.voucher_status,
        invoice.voucher_number,
        invoice.voucher_date,
        invoice.due_date,
        invoice.address_id,
        invoice.address_name,
        invoice.address_supplement,
        invoice.address_street,
        invoice.address_city,
        invoice.address_zip,
        invoice.address_countrycode
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use sqlx::types::Uuid;

    #[test]
    fn test_insert_invoice() {
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
        let result = block_on(insert_invoice(&pool, invoice));
        assert_eq!(result.unwrap(), 1);
    }
}
