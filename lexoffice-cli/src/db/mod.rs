pub mod models;
use self::models::{DbInvoice, DbVoucher};
use openapi::models::*;
use sqlx::{postgres::PgPoolOptions, types::chrono::NaiveDateTime, Error, PgPool};
use std::{env, str::FromStr};

// Convert Lexoffice Invoice to database entity
impl From<Invoice> for DbInvoice {
    fn from(invoice: Invoice) -> Self {
        DbInvoice {
            id: invoice.id.unwrap_or_default().to_string(),
            organization_id: None,
            created_date: None,
            updated_date: None,
            version: None,
            language: None,
            archived: None,
            voucher_status: Some(format!("{:?}", invoice.voucher_status.unwrap_or_default())),
            voucher_number: None,
            voucher_date: None,
            due_date: None,
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
            voucher_type: Some(format!("{:?}", v.voucher_type).to_lowercase()),
            voucher_status: Some(format!("{:?}", v.voucher_status).to_lowercase()),
            voucher_number: v.voucher_number,
            voucher_date: NaiveDateTime::from_str(v.voucher_date.unwrap_or_default().as_str()).ok(),
            created_date: NaiveDateTime::from_str(v.created_date.unwrap().as_str()).ok(),
            updated_date: NaiveDateTime::from_str(v.updated_date.unwrap().as_str()).ok(),
            due_date: NaiveDateTime::from_str(v.due_date.unwrap_or_default().as_str()).ok(),
            contact_id: None,
            contact_name: v.contact_name,
            total_amount: None,
            open_amount: None,
            currency: None,
            archived: None,
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
    let _rec = sqlx::query!(
        r#"
INSERT INTO vouchers ( id, voucher_type, voucher_status, voucher_number )
VALUES ( $1, $2, $3, $4 )
RETURNING id
        "#,
        voucher.id,
        voucher.voucher_type,
        voucher.voucher_status,
        voucher.voucher_number
    )
    .fetch_one(pool)
    .await?;

    Ok(1)
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
