use std::env;

use diesel::result::Error;
use diesel::{Connection, PgConnection};

pub mod models;
pub mod schema;

use diesel::prelude::*;
use openapi::models::*;

pub fn connect_db() -> PgConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

pub fn get_all_vouchers() -> Vec<models::Voucher> {
    use self::schema::vouchers::dsl::*;
    let conn = &mut connect_db();

    let results = vouchers.select(models::Voucher::as_select()).load(conn);

    match results {
        Ok(res) => res,
        Err(e) => {
            println!("Error getting vouchers: {}", e);
            vec![]
        }
    }
}

pub fn get_voucher_by_id(voucher_id: String) -> Result<models::Voucher, Error> {
    use self::schema::vouchers::dsl::*;
    let conn = &mut connect_db();

    vouchers.find(voucher_id).first(conn)
}

pub fn get_all_invoices() -> Vec<models::Invoice> {
    use self::schema::invoices::dsl::*;
    let conn = &mut connect_db();

    let results: Result<Vec<models::Invoice>, Error> =
        invoices.select(models::Invoice::as_select()).load(conn);

    match results {
        Ok(res) => res,
        Err(e) => {
            println!("Error getting invoices: {}", e);
            vec![]
        }
    }
}

// Convert Lexoffice Invoice to database entity
impl From<Invoice> for models::Invoice {
    fn from(invoice: Invoice) -> Self {
        models::Invoice {
            id: invoice.id.unwrap_or_default().to_string(),
            organizationid: None,
            createddate: None,
            updateddate: None,
            version: None,
            language: None,
            archived: None,
            voucherstatus: Some(format!("{:?}", invoice.voucher_status.unwrap_or_default())),
            vouchernumber: None,
            voucherdate: None,
            duedate: None,
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
    use self::schema::invoices::dsl::*;
    let conn = &mut connect_db();

    let invoice = models::Invoice::from(invoice);

    let result = diesel::insert_into(invoices).values(invoice).execute(conn);
    result
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
