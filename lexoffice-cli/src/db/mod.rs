pub mod models;

use self::models::{DbInvoice, DbLineItem, DbProduct, DbVoucher, DbAddress};
use crate::lexoffice::EnumToString;
use log::info;
use openapi::models::*;
use sqlx::{
    postgres::PgPoolOptions,
    types::{chrono::{DateTime, NaiveDateTime}, Uuid},
    Error, PgPool,
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
            total_discount_absolute: invoice.total_price.total_discount_absolute.unwrap_or(0.0) as f64,
            total_discount_percentage: invoice.total_price.total_discount_percentage.unwrap_or(0.0) as f64
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
            product_id: "".to_string(),
            voucher_id: "".to_string(),
            quantity: 1.0,
            unit_name: item.unit_name.unwrap_or("".to_string()),
            currency: "".to_string(),
            net_amount: 2.0,
            gross_amount: 2.0,
            tax_rate_percentage: Some(2.0),
            discount_percentage: Some(2.0),
            line_item_amount: Some(2.0),
        }
    }
}

impl From<LineItem> for DbProduct {
    fn from(item: LineItem) -> Self {
        Self {
            id: item.id.unwrap_or_default().to_string(),
            product_type: item.r#type.enum_to_string(),
            name: item.name,
            description: item.description,
        }
    }
}

impl From<VoucherAddress> for DbAddress {
    fn from(a: VoucherAddress) -> Self {
        Self { contact_id: match a.contact_id {
            Some(c_id) => c_id.to_string(),
            None => "".to_string()
        }, name: a.name, supplement: a.supplement.map(|s| s.unwrap()), street: a.street, city: a.city, zip: a.zip, country_code: a.country_code }
    }
}

pub struct LexofficeDb {
    db_pool: PgPool,
}

impl LexofficeDb {
    pub async fn new(db_url: String) -> Self {
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url.as_str())
            .await
            .expect("Error connecting to database");

        Self { db_pool }
    }

    pub async fn migrate(&self) -> Result<(), Error> {
        let _ = sqlx::migrate!().run(&self.db_pool).await;
        Ok(())
    }

    pub async fn get_all_vouchers(&self) -> Result<Vec<DbVoucher>, Error> {
        sqlx::query_as!(
        DbVoucher,
r#"
    SELECT id, archived, contact_id, contact_name, voucher_date, created_date, due_date, updated_date, 
    voucher_number, voucher_type, voucher_status, total_amount, open_amount, currency 
    FROM voucherlist
"#
    )
    .fetch_all(&self.db_pool).await
    }

    pub async fn get_voucher_by_id(&self, voucher_id: String) -> Result<DbVoucher, Error> {
        sqlx::query_as!(
            DbVoucher,
            r#"select * FROM voucherlist where id = $1"#,
            voucher_id
        )
        .fetch_one(&self.db_pool)
        .await
    }

    pub async fn get_vouchers_by_type(
        &self,
        voucher_type: String,
    ) -> Result<Vec<DbVoucher>, Error> {
        sqlx::query_as!(
            DbVoucher,
            r#"SELECT * FROM voucherlist where voucher_type = $1"#,
            voucher_type
        )
        .fetch_all(&self.db_pool)
        .await
    }

    pub async fn get_all_invoices(&self) -> Result<Vec<DbInvoice>, Error> {
        sqlx::query_as!(DbInvoice, 
    r#"SELECT id, organization_id, created_date, updated_date, version, language, archived, voucher_status, voucher_number, voucher_date, due_date, address_id, currency, total_net_amount, total_gross_amount, total_tax_amount, total_discount_absolute, total_discount_percentage
        FROM invoices
    "#)
            .fetch_all(&self.db_pool)
            .await
    }

    pub async fn insert_voucher(&self, voucher: DbVoucher) -> Result<String, Error> {
        let rec = sqlx::query!(
            r#"
    INSERT INTO voucherlist ( 
        id, voucher_type, voucher_status, voucher_number, voucher_date, 
        created_date, updated_date, due_date, contact_id, contact_name, 
        total_amount, open_amount, currency, archived 
    )
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
        .fetch_one(&self.db_pool)
        .await?;

        Ok(rec.id)
    }

    pub async fn insert_product(&self, product: DbProduct) -> Result<String, Error> {
        let rec = sqlx::query!(
            r#"
    INSERT INTO products ( id, type, name, description )
    VALUES ( $1, $2, $3, $4 )
    RETURNING id
            "#,
            product.id,
            product.product_type,
            product.name,
            product.description
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(rec.id)
    }

    pub async fn product_exists(&self, product_id: String) -> bool {
        let result = sqlx::query!(
            r#"
SELECT EXISTS(SELECT 1 FROM products WHERE id=$1)
        "#,
            product_id
        )
        .fetch_one(&self.db_pool)
        .await;

        let res = match result {
            Ok(rec) => rec.exists.unwrap(),
            Err(_) => false,
        };

        res
    }

    pub async fn insert_lineitem(
        &self,
        mut item: DbLineItem,
        product_id: String,
        voucher_id: String,
    ) -> Result<i32, Error> {
        item.voucher_id = voucher_id;
        item.product_id = product_id;

        let rec = sqlx::query!(
            r#"
    INSERT INTO line_items (
        product_id, voucher_id, quantity, unit_name, currency, 
        net_amount, gross_amount, tax_rate_percentage, discount_percentage, line_item_amount
    )
    VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10 )
    RETURNING id
            "#,
            item.product_id,
            item.voucher_id,
            item.quantity,
            item.unit_name,
            item.currency,
            item.net_amount,
            item.gross_amount,
            item.tax_rate_percentage,
            item.discount_percentage,
            item.line_item_amount
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(rec.id)
    }

    pub async fn voucher_exists(&self, voucher_id: String) -> bool {
        let result = sqlx::query!(
            r#"
SELECT EXISTS(SELECT 1 FROM voucherlist WHERE id=$1)
        "#,
            voucher_id
        )
        .fetch_one(&self.db_pool)
        .await;

        //println!("voucher_exists: {:?}", rec.exists.unwrap_or(false));
        let res = match result {
            Ok(rec) => rec.exists.unwrap(),
            Err(_) => false,
        };

        res
    }

    pub async fn invoice_exists(&self, invoice_id: String) -> bool {
        let result = sqlx::query!(
            r#"
SELECT EXISTS(SELECT 1 FROM invoices WHERE id=$1)
        "#,
            invoice_id
        )
        .fetch_one(&self.db_pool)
        .await;

        let res = match result {
            Ok(rec) => rec.exists.unwrap(),
            Err(_) => false,
        };

        res
    }

    pub async fn insert_invoice(&self, invoice: DbInvoice) -> Result<String, Error> {
        let rec = sqlx::query!(
        r#"
INSERT INTO invoices ( 
    id, organization_id, created_date, updated_date, version, language, archived, voucher_status, voucher_number, 
    voucher_date, due_date, address_id )
VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12 )
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
        invoice.address_id
    )
    .fetch_one(&self.db_pool)
    .await?;

        Ok(rec.id)
    }

    pub async fn get_address_by_id_or_collective(&self, address: &DbAddress) -> Option<DbAddress> {
        if address.contact_id != "" {
            let result = sqlx::query_as!(
                DbAddress,
            r#"
                SELECT contact_id, name, supplement, street, city, zip, country_code
                FROM addresses WHERE contact_id=$1
            "#,
                address.contact_id
            )
            .fetch_one(&self.db_pool)
            .await;
            result.ok()
        } else {
            let result = sqlx::query_as!(
                DbAddress,
            r#"
                SELECT contact_id, name, supplement, street, city, zip, country_code 
                FROM addresses 
                WHERE type='collective' 
                AND name=$1 AND supplement=$2 AND street=$3 AND city=$4 AND zip=$5 AND country_code=$6
            "#,
                address.name,
                address.supplement,
                address.street,
                address.city,
                address.zip,
                address.country_code
            )
            .fetch_one(&self.db_pool)
            .await;
    
            result.ok()
        }

    }

    pub async fn insert_address(&self, mut address: DbAddress) -> Result<String, Error> {
        let contact = self.get_address_by_id_or_collective(&address).await;
        if contact.is_some() {
            return Ok(contact.unwrap().contact_id);
        }
        
        let address_type: String;
        if address.contact_id == "" {
            // Does not exist and no ID -> collective contact -> generate new ID
            address_type = "collective".to_string();
            address.contact_id = Uuid::new_v4().to_string();
        } else {
            // Has ID -> customer or vendor
            address_type = "customer".to_string();
        }

        let rec = sqlx::query!(
            r#"
        INSERT INTO addresses ( contact_id, type, name, supplement, street, city, zip, country_code )
        VALUES ( $1, $2, $3, $4, $5, $6, $7, $8 )
        RETURNING contact_id
            "#,
            address.contact_id,
            address_type,
            address.name,
            address.supplement,
            address.street,
            address.city,
            address.zip,
            address.country_code
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(rec.contact_id)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use sqlx::types::{
        chrono::{NaiveDate, NaiveTime},
        Uuid,
    };

    #[test]
    fn test_insert_invoice() {
        let invoice_id = Uuid::new_v4();
        let date_time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 12, 24).unwrap(),
            NaiveTime::from_hms_milli_opt(12, 12, 33, 100).unwrap(),
        );
        let invoice = DbInvoice {
            id: invoice_id.to_string(),
            organization_id: Some("myid".to_string()),
            created_date: date_time,
            updated_date: date_time,
            version: 1,
            language: "DE".to_string(),
            archived: 0,
            voucher_status: "draft".to_string(),
            voucher_number: "test".to_string(),
            voucher_date: date_time,
            due_date: Some(date_time),
            address_id: Some("a1".to_string()),
            address_name: "Max Mustermann".to_string(),
            address_street: Some("Musterstr. 1".to_string()),
            address_zip: Some("20095".to_string()),
            address_city: Some("Hamburg".to_string()),
            address_countrycode: Some("DE".to_string()),
            address_supplement: Some("none".to_string()),
        };
        let db = block_on(LexofficeDb::new(
            "postgres://bunu:bunu@localhost:5434/bunu".to_string(),
        ));
        let result = block_on(db.insert_invoice(invoice));
        assert_eq!(result.unwrap(), invoice_id.to_string());
    }
}
