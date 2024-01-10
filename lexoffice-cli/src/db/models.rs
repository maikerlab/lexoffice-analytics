use sqlx::{self, types::chrono::NaiveDateTime};

#[derive(Debug, sqlx::FromRow)]
pub struct DbAddress {
    pub contactid: String,
    pub name: Option<String>,
    pub supplement: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub zip: Option<String>,
    pub countrycode: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DbInvoice {
    pub id: String,
    pub organization_id: Option<String>,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
    pub version: i32,
    pub language: String,
    pub archived: i8,
    pub voucher_status: String,
    pub voucher_number: String,
    pub voucher_date: NaiveDateTime,
    pub due_date: Option<NaiveDateTime>,
    pub address_id: Option<String>,
    pub currency: String,
    pub total_net_amount: f64,
    pub total_gross_amount: f64,
    pub total_tax_amount: f64,
    pub total_discount_absolute: f64,
    pub total_discount_percentage: f64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DbLineItem {
    pub id: i32,
    pub product_id: String,
    pub voucher_id: String,
    pub quantity: f64,
    pub unit_name: String,
    pub currency: String,
    pub net_amount: f64,
    pub gross_amount: f64,
    pub tax_rate_percentage: Option<f64>,
    pub discount_percentage: Option<f64>,
    pub line_item_amount: Option<f64>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DbProduct {
    pub id: String,
    pub product_type: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DbVoucher {
    pub id: String,
    pub voucher_type: String,
    pub voucher_status: String,
    pub voucher_number: String,
    pub voucher_date: NaiveDateTime,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
    pub due_date: Option<NaiveDateTime>,
    pub contact_id: Option<String>,
    pub contact_name: Option<String>,
    pub total_amount: Option<f64>,
    pub open_amount: Option<f64>,
    pub currency: Option<String>,
    pub archived: i8,
}

impl Default for DbVoucher {
    fn default() -> Self {
        Self {
            id: Default::default(),
            voucher_type: Default::default(),
            voucher_status: Default::default(),
            voucher_number: Default::default(),
            voucher_date: Default::default(),
            created_date: Default::default(),
            updated_date: Default::default(),
            due_date: Default::default(),
            contact_id: Default::default(),
            contact_name: Default::default(),
            total_amount: Default::default(),
            open_amount: Default::default(),
            currency: Default::default(),
            archived: Default::default(),
        }
    }
}
