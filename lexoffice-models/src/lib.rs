use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use openapi::models::invoice::Invoice as LexofficeInvoice;
use openapi::models::{
    LineItem as LexofficeLineItem,
    VoucherAddress as LexofficeVoucherAddress,
    VoucherlistVoucher as LexofficeVoucherlistVoucher
};

pub const CUSTOMER_COLLECTION_NAME: &str = "customers";
pub const PRODUCTS_COLLECTION_NAME: &str = "products";
pub const INVOICES_COLLECTION_NAME: &str = "invoices";
pub const SALES_COLLECTION_NAME: &str = "sales";

#[derive(Deserialize, Serialize)]
struct Voucher {
    #[serde(rename = "_id")]
    id: Uuid,
    voucher_type: String,
    voucher_status: String,
    voucher_number: String,
    voucher_date: DateTime,
    created_date: DateTime,
    updated_date: DateTime,
    due_date: Option<DateTime>,
    contact_id: Option<Uuid>,
    contact_name: String,
    total_amount: Option<f64>,
    open_amount: Option<f64>,
    currency: String,
    archived: bool,
}

impl Into<Voucher> for LexofficeVoucherlistVoucher {
    fn into(self) -> Voucher {
        Voucher {
            id: self.id,
            voucher_type: format!("{:?}", self.voucher_type).to_lowercase(),
            voucher_status: format!("{:?}", self.voucher_status).to_lowercase(),
            voucher_number: self.voucher_number,
            voucher_date: DateTime::parse_rfc3339_str(self.voucher_date).unwrap(),
            created_date: DateTime::parse_rfc3339_str(self.created_date).unwrap(),
            updated_date: DateTime::parse_rfc3339_str(self.updated_date).unwrap(),
            due_date: self.due_date.map(|d| DateTime::parse_rfc3339_str(d).unwrap()),
            contact_id: self.contact_id.map(|id| id.unwrap()),
            contact_name: self.contact_name,
            total_amount: self.total_amount,
            open_amount: self.open_amount,
            currency: format!("{:?}", self.currency).to_lowercase(),
            archived: self.archived,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Address {
    #[serde(rename = "_id", default, skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Invoice {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    pub voucher_date: DateTime,
    pub created_date: DateTime,
    pub updated_date: DateTime,
    pub due_date: DateTime,
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
    invoice_id: Uuid,
    product: Product,
    quantity: f64,
    total_amount: f64,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Product {
    #[serde(rename = "_id")]
    id: Uuid,
    name: String,
    product_type: String,
}

impl Into<Address> for LexofficeVoucherAddress {
    fn into(self) -> Address {
        Address {
            id: self.contact_id.map(|id| id.to_string()),
            name: self.name
        }
    }
}

impl Into<Product> for LexofficeLineItem {
    fn into(self) -> Product {
        Product {
            id: self.id.unwrap_or(Uuid::new_v4()),
            name: self.name,
            product_type: format!("{:?}", self.r#type).to_lowercase(),
        }
    }
}

impl LineItem {
    pub fn new(line_item: LexofficeLineItem, invoice_id: Uuid) -> LineItem {
        LineItem {
            invoice_id,
            product: line_item.clone().into(),
            quantity: line_item.quantity,
            total_amount: line_item.line_item_amount.unwrap_or(0.0),
        }
    }
}

impl Into<Invoice> for LexofficeInvoice {
    fn into(self) -> Invoice {
        Invoice {
            id: self.id,
            organization_id: self.organization_id,
            voucher_date: DateTime::parse_rfc3339_str(self.voucher_date).unwrap(),
            created_date: DateTime::parse_rfc3339_str(self.created_date).unwrap(),
            updated_date: DateTime::parse_rfc3339_str(self.updated_date).unwrap(),
            due_date: DateTime::parse_rfc3339_str(self.due_date).unwrap(),
            archived: self.archived,
            voucher_status: format!("{:?}", self.voucher_status).to_lowercase(),
            voucher_number: self.voucher_number,
            address: (*self.address).into(),
            line_items: self.line_items.into_iter().map(|li| LineItem::new(li, self.id)).collect(),
            currency: format!("{:?}", self.total_price.currency).to_lowercase(),
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
    use openapi::models::voucherlist_voucher::{Currency, VoucherStatus, VoucherType};
    use openapi::models::invoice::VoucherStatus as InvoiceStatus;
    use openapi::models::total_price::Currency as TotalPriceCurrency;
    use openapi::models::{TotalPrice, VoucherlistVoucher};
    use openapi::models::line_item::Type;
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
        assert_eq!(Some("2023-06-13T22:00:00Z".to_string()), voucher.created_date.try_to_rfc3339_string().ok());
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
        let li_2 = db_invoice.line_items.get(1).unwrap();

        assert_eq!(invoice_id, li_1.invoice_id);
        assert_eq!(invoice_id, li_2.invoice_id);

        assert_eq!("Nudeln".to_string(), li_1.product.name);
        assert_eq!("material".to_string(), li_1.product.product_type);
    }
}