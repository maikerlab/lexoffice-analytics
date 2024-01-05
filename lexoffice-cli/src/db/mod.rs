use std::{env, vec};

use sqlx::{postgres::PgPoolOptions, Error, PgPool, Pool, Postgres};

pub mod models;

use openapi::models::*;

pub async fn connect_db() -> Result<Pool<Postgres>, Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}

pub async fn get_all_vouchers(pool: &PgPool) -> Vec<models::Voucher> {
    let results = sqlx::query_as!(
        models::Voucher,
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

pub fn get_voucher_by_id(voucher_id: String) -> Result<models::Voucher, Error> {
    todo!("todo");
}

pub fn get_all_invoices() -> Vec<models::Invoice> {
    todo!("todo");
}

// Convert Lexoffice Invoice to database entity
impl From<Invoice> for models::Invoice {
    fn from(invoice: Invoice) -> Self {
        models::Invoice {
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

pub fn insert_invoice(invoice: Invoice) -> Result<usize, Error> {
    todo!("todo");
}

#[cfg(test)]
mod tests {
    use super::*;
    use openapi::models::invoice::*;
    use uuid::Uuid;

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
        assert_eq!(result, Ok(1));
    }
}
