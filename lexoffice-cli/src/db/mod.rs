use std::env;

use diesel::result::Error;
use diesel::{Connection, PgConnection};

pub mod models;
pub mod schema;

use diesel::prelude::*;
use openapi::models::*;

use self::models::*;

pub fn connect_db() -> PgConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

pub fn get_all_vouchers() -> Vec<QueryableVoucher> {
    use self::schema::vouchers::dsl::*;
    let conn = &mut connect_db();

    let results = vouchers.select(QueryableVoucher::as_select()).load(conn);

    match results {
        Ok(res) => res,
        Err(e) => {
            println!("Error getting vouchers: {}", e);
            vec![]
        }
    }
}

pub fn get_voucher_by_id(voucher_id: String) -> Result<QueryableVoucher, Error> {
    use self::schema::vouchers::dsl::*;
    let conn = &mut connect_db();

    vouchers.find(voucher_id).first(conn)
}

pub fn get_all_invoices() -> Vec<QueryableInvoice> {
    use self::schema::invoices::dsl::*;
    let conn = &mut connect_db();

    let results = invoices.select(QueryableInvoice::as_select()).load(conn);

    match results {
        Ok(res) => res,
        Err(e) => {
            println!("Error getting invoices: {}", e);
            vec![]
        }
    }
}

// Convert Lexoffice Invoice to database entity
impl From<Invoice> for InsertableInvoice {
    fn from(invoice: Invoice) -> Self {
        InsertableInvoice {
            id: invoice.id.unwrap_or_default().to_string(),
            organizationid: invoice.organization_id.try_into(),
            createddate: invoice.created_date.try_into(),
            updateddate: invoice.updated_date.try_into(),
            version: invoice.version.try_into(),
            language: invoice.language.try_into(),
            archived: invoice.archived.try_into(),
            voucherstatus: invoice.voucher_status.try_into(),
            vouchernumber: invoice.voucher_number.try_into(),
            voucherdate: invoice.voucher_date.try_into(),
            duedate: invoice.due_date.try_into(),
            address_id: invoice.address.unwrap_or_default().contact_id.try_into(),
            address_name: invoice.address.unwrap_or_default().name.try_into(),
            address_supplement: invoice.address.unwrap_or_default().supplement.try_into(),
            address_street: invoice.address.unwrap_or_default().street.try_into(),
            address_city: invoice.address.unwrap_or_default().city.try_into(),
            address_zip: invoice.address.unwrap_or_default().zip.try_into(),
            address_countrycode: invoice.address.unwrap_or_default().country_code.try_into(),
        }
    }
}

pub fn insert_invoice(invoice: Invoice) -> Result<usize, Error> {
    use self::schema::invoices::dsl::*;
    let conn = &mut connect_db();

    let invoice = InsertableInvoice::from(invoice);

    let result = diesel::insert_into(invoices).values(invoice).execute(conn);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
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
            voucher_status: None,
            voucher_number: None,
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
