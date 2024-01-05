pub mod models;
use self::models::{DbInvoice, DbVoucher};
use openapi::models::*;
use sqlx::{postgres::PgPoolOptions, Error, PgPool, Pool, Postgres};
use std::{env, vec};

pub async fn connect_db() -> Result<Pool<Postgres>, Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}

pub async fn get_all_vouchers(pool: &PgPool) -> Vec<DbVoucher> {
    let results = sqlx::query_as!(
        DbVoucher,
r#"
    SELECT id, archived, contact_id, contact_name, voucher_date, created_date, due_date, updated_date, voucher_number, voucher_type, voucher_status, total_amount, open_amount, currency FROM vouchers
"#
    )
    .fetch_all(pool).await;

    match results {
        Ok(vouchers) => vouchers,
        Err(_) => vec![],
    }
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

pub async fn insert_voucher(pool: &PgPool, voucher: Voucher) -> Result<usize, Error> {
    todo!("insert voucher into db");
    let _ = sqlx::query!(r#"INSERT INTO vouchers (id, voucher_type) VALUES ('123', 'asd')"#,)
        .bind(voucher.id.unwrap().to_string())
        .bind(format!("{:?}", voucher.voucher_type))
        .fetch_all(pool)
        .await;

    Ok(1)
}

pub fn insert_invoice(pool: &PgPool, invoice: Invoice) -> Result<usize, Error> {
    todo!("insert invoice into db");
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use openapi::models::invoice::*;
    use sqlx::types::Uuid;

    #[test]
    fn test_insert_invoice() {
        let invoice_id = Uuid::new_v4();
        let invoice = Invoice {
            id: Some(invoice_id),
            organization_id: None,
            created_date: None,
            updated_date: None,
            version: None,
            language: None,
            archived: None,
            voucher_status: Some(VoucherStatus::Draft),
            voucher_number: Some("test".to_string()),
            voucher_date: None,
            due_date: None,
            address: None,
            x_rechnung: None,
            line_items: None,
            total_price: None,
            tax_amounts: None,
            tax_conditions: None,
            payment_conditions: None,
            shipping_conditions: None,
            closing_invoice: None,
            claimed_gross_amount: None,
            down_payment_deductions: None,
            recurring_template_id: None,
            related_vouchers: None,
            title: None,
            introduction: None,
            remark: None,
            files: None,
        };
        let result = insert_invoice(invoice);
        assert_eq!(result.unwrap(), 1);
    }
}
