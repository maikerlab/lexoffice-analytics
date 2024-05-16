use serde::{Deserialize, Serialize};
use uuid::Uuid;
use openapi::models::invoice::Invoice as LexofficeInvoice;
use openapi::models::{LineItem, VoucherAddress, VoucherlistVoucher};

pub const CUSTOMER_COLLECTION_NAME: &str = "customers";
pub const PRODUCTS_COLLECTION_NAME: &str = "products";
pub const INVOICES_COLLECTION_NAME: &str = "invoices";
pub const SALES_COLLECTION_NAME: &str = "sales";

#[derive(Deserialize, Serialize, Default)]
struct Voucher {
    #[serde(rename = "_id")]
    id: String,
    voucher_type: String
}

impl Into<Voucher> for VoucherlistVoucher {
    fn into(self) -> Voucher {
        Voucher {
            id: self.id.to_string(),
            voucher_type: format!("{:?}", self.voucher_type).to_lowercase(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Customer {
    id: Option<String>,
    name: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Invoice {
    #[serde(rename = "_id")]
    pub id: String,
    pub voucher_number: String,
    pub customer: Customer,
    pub line_items: Vec<Sale>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Sale {
    invoice: Invoice,
    product: Product,
    quantity: u32,
    total_amount: f32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Product {
    #[serde(rename = "_id")]
    id: String,
    name: String,
}

impl Into<Customer> for VoucherAddress {
    fn into(self) -> Customer {
        Customer {
            id: self.contact_id.map(|id| id.to_string()),
            name: self.name
        }
    }
}

impl Into<Invoice> for LexofficeInvoice {
    fn into(self) -> Invoice {
        Invoice {
            id: self.id.to_string(),
            voucher_number: self.voucher_number,
            customer: (*self.address).into(),
            line_items: vec![],
        }
    }
}

impl Into<Product> for LineItem {
    fn into(self) -> Product {
        Product {
            id: self.id.unwrap_or(Uuid::new_v4()).to_string(),
            name: self.name,
        }
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;
    use openapi::models::voucherlist_voucher::VoucherType;
    use openapi::models::VoucherlistVoucher;
    use super::*;

    #[test]
    pub fn test_voucher_into() {
        let id = Uuid::new_v4();
        let mut lo_voucher = VoucherlistVoucher::default();
        lo_voucher.id = id.clone();
        lo_voucher.voucher_type = VoucherType::Invoice;

        let voucher: Voucher = lo_voucher.clone().into();

        assert_eq!(id.to_string(), voucher.id);
        assert_eq!("invoice", voucher.voucher_type.as_str());
    }
}