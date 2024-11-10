use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use lexoffice_api::models::invoice::Invoice as LexofficeInvoice;
use lexoffice_api::models::{
    LineItem as LexofficeLineItem,
    VoucherAddress as LexofficeVoucherAddress,
    VoucherlistVoucher as LexofficeVoucherlistVoucher
};
use bson;

pub const CUSTOMER_COLLECTION_NAME: &str = "customers";
pub const PRODUCTS_COLLECTION_NAME: &str = "products";
pub const INVOICES_COLLECTION_NAME: &str = "invoices";
pub const SALES_COLLECTION_NAME: &str = "sales";

use serde_with::serde_as;
impl Into<Voucher> for LexofficeVoucherlistVoucher {
    fn into(self) -> Voucher {
        Voucher {
            id: self.id,
            voucher_type: format!("{:?}", self.voucher_type).to_lowercase(),
            voucher_status: format!("{:?}", self.voucher_status).to_lowercase(),
            voucher_number: self.voucher_number,
            voucher_date: DateTime::parse_from_rfc3339(self.voucher_date.as_str()).unwrap().to_utc(),
            created_date: DateTime::parse_from_rfc3339(self.created_date.as_str()).unwrap().to_utc(),
            updated_date: DateTime::parse_from_rfc3339(self.updated_date.as_str()).unwrap().to_utc(),
            due_date: self.due_date.map(|due_date| DateTime::parse_from_rfc3339(due_date.as_str()).unwrap().to_utc()),
            contact_id: self.contact_id.map(|id| id.unwrap()),
            contact_name: self.contact_name,
            total_amount: self.total_amount,
            open_amount: self.open_amount,
            currency: format!("{:?}", self.currency).to_uppercase(),
            archived: self.archived,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Address {
    #[serde(rename = "_id", default, skip_serializing_if = "Option::is_none")]
    contact_id: Option<Uuid>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    supplement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
    country_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_person: Option<String>,
}

impl Into<Address> for LexofficeVoucherAddress {
    fn into(self) -> Address {
        Address {
            contact_id: self.contact_id,
            name: self.name,
            supplement: self.supplement.map(|sup| sup.unwrap()),
            street: self.street,
            zip: self.zip,
            city: self.city,
            country_code: self.country_code,
            contact_person: self.contact_person,
        }
    }
}

#[serde_as]
#[derive(Deserialize, Serialize)]
struct Voucher {
    #[serde(rename = "_id")]
    id: Uuid,
    voucher_type: String,
    voucher_status: String,
    voucher_number: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    voucher_date: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_date: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_date: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<bson::DateTime>")]
    due_date: Option<DateTime<Utc>>,
    contact_id: Option<Uuid>,
    contact_name: String,
    total_amount: Option<f64>,
    open_amount: Option<f64>,
    currency: String,
    archived: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Invoice {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub voucher_date: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_date: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_date: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub due_date: DateTime<Utc>,
    pub archived: bool,
    pub voucher_status: String,
    pub voucher_number: String,
    pub address: Address,
    pub line_items: Vec<LineItem>,
    pub currency: String,
    pub total_net_amount: f64,
    pub total_gross_amount: f64,
    pub total_tax_amount: f64,
    pub total_discount_absolute: Option<f64>,
    pub total_discount_percentage: Option<f64>,
    pub closing_invoice: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineItem {
    product: Product,
    quantity: f64,
    line_item_amount: Option<f64>,
    unit_name: Option<String>,
    unit_net_amount: Option<f64>,
    unit_gross_amount: Option<f64>,
    unit_tax_rate_percentage: Option<f64>,
}

impl Into<LineItem> for LexofficeLineItem {
    fn into(self) -> LineItem {
        LineItem {
            product: self.clone().into(),
            quantity: self.quantity,
            unit_name: self.unit_name,
            unit_net_amount: self.unit_price.clone().map(|up| (*up).net_amount),
            unit_gross_amount: self.unit_price.clone().map(|up| (*up).gross_amount),
            unit_tax_rate_percentage: self.unit_price.map(|up| (*up).tax_rate_percentage),
            line_item_amount: self.line_item_amount,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<Uuid>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    product_type: String,
}

impl Into<Product> for LexofficeLineItem {
    fn into(self) -> Product {
        Product {
            id: self.id,
            name: self.name,
            description: self.description,
            product_type: format!("{:?}", self.r#type).to_lowercase(),
        }
    }
}

impl Into<Invoice> for LexofficeInvoice {
    fn into(self) -> Invoice {
        Invoice {
            id: self.id,
            organization_id: self.organization_id,
            voucher_date: DateTime::parse_from_rfc3339(self.voucher_date.as_str()).unwrap().to_utc(),
            created_date: DateTime::parse_from_rfc3339(self.created_date.as_str()).unwrap().to_utc(),
            updated_date: DateTime::parse_from_rfc3339(self.updated_date.as_str()).unwrap().to_utc(),
            due_date: DateTime::parse_from_rfc3339(self.due_date.as_str()).unwrap().to_utc(),
            archived: self.archived,
            voucher_status: format!("{:?}", self.voucher_status).to_lowercase(),
            voucher_number: self.voucher_number,
            address: (*self.address).into(),
            line_items: self.line_items.into_iter().map(|li| li.into()).collect(),
            currency: format!("{:?}", self.total_price.currency).to_uppercase(),
            total_net_amount: self.total_price.total_net_amount,
            total_gross_amount: self.total_price.total_gross_amount,
            total_tax_amount: self.total_price.total_tax_amount,
            total_discount_absolute: self.total_price.total_discount_absolute,
            total_discount_percentage: self.total_price.total_discount_percentage,
            closing_invoice: self.closing_invoice,
        }
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;
    use lexoffice_api::models::voucherlist_voucher::{Currency, VoucherStatus, VoucherType};
    use lexoffice_api::models::invoice::VoucherStatus as InvoiceStatus;
    use lexoffice_api::models::total_price::Currency as TotalPriceCurrency;
    use lexoffice_api::models::{TotalPrice, VoucherlistVoucher};
    use lexoffice_api::models::line_item::Type;
    use super::*;

    #[test]
    pub fn test_voucher_into() {
        let id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();
        let lo_voucher = VoucherlistVoucher {
            id: id.clone(),
            voucher_type: VoucherType::Invoice,
            voucher_status: VoucherStatus::Draft,
            voucher_number: "ABC-123".to_string(),
            voucher_date: "2023-06-14T00:00:00.000+02:00".to_string(),
            created_date: "2023-06-14T00:00:00.000+02:00".to_string(),
            updated_date: "2023-06-14T00:00:00.000+02:00".to_string(),
            due_date: Some("2023-06-14T00:00:00.000+02:00".to_string()),
            contact_id: Some(Some(contact_id)),
            contact_name: "Test Contact".to_string(),
            total_amount: Some(123.45),
            open_amount: Some(43.21),
            currency: Currency::Eur,
            archived: false,
        };

        let voucher: Voucher = lo_voucher.clone().into();

        assert_eq!(id, voucher.id);
        assert_eq!("invoice", voucher.voucher_type.as_str());
        assert_eq!("2023-06-13 22:00:00".to_string(), voucher.created_date.to_utc().format("%Y-%m-%d %H:%M:%S").to_string());
    }

    #[test]
    pub fn test_voucher_into_none() {
        let id = Uuid::new_v4();
        let lo_voucher = VoucherlistVoucher {
            id,
            voucher_type: VoucherType::Invoice,
            voucher_status: VoucherStatus::Draft,
            voucher_number: "ABC-123".to_string(),
            voucher_date: "2023-06-14T00:00:00.000+02:00".to_string(),
            created_date: "2023-06-14T00:00:00.000+02:00".to_string(),
            updated_date: "2023-06-14T00:00:00.000+02:00".to_string(),
            due_date: None,
            contact_id: None,
            contact_name: "Test Contact".to_string(),
            total_amount: None,
            open_amount: None,
            currency: Currency::Eur,
            archived: false,
        };

        let voucher: Voucher = lo_voucher.clone().into();

        assert_eq!(None, voucher.due_date);
    }

    #[test]
    pub fn test_invoice_into() {
        let invoice_id = Uuid::new_v4();
        let mut invoice = LexofficeInvoice::default();
        invoice.id = invoice_id.clone();
        invoice.voucher_number = "ABC-123".to_string();
        invoice.voucher_date = "2023-06-14T00:00:00.000+02:00".to_string();
        invoice.created_date = "2023-06-14T00:00:00.000+02:00".to_string();
        invoice.updated_date = "2023-06-14T00:00:00.000+02:00".to_string();
        invoice.due_date = "2023-06-14T00:00:00.000+02:00".to_string();
        invoice.voucher_status = InvoiceStatus::Paid;
        invoice.total_price = Box::new(TotalPrice::new(TotalPriceCurrency::Eur, 123.4, 234.5, 12.3));
        invoice.line_items = vec![
            LexofficeLineItem::new(Type::Material, "Nudeln".to_string(), 3.0),
            LexofficeLineItem::new(Type::Service, "Versand".to_string(), 1.0)
        ];

        let db_invoice: Invoice = invoice.clone().into();
        assert_eq!(invoice_id, db_invoice.id);
        assert_eq!("paid", db_invoice.voucher_status);
        assert_eq!(123.4, db_invoice.total_net_amount);
        assert_eq!(234.5, db_invoice.total_gross_amount);
        assert_eq!(12.3, db_invoice.total_tax_amount);
        assert_eq!(2, db_invoice.line_items.len());
        let li_1 = db_invoice.line_items.get(0).unwrap();
        assert_eq!("Nudeln".to_string(), li_1.product.name);
        assert_eq!("material".to_string(), li_1.product.product_type);
    }
}