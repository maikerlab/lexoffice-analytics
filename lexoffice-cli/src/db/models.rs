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
    pub created_date: Option<NaiveDateTime>,
    pub updated_date: Option<NaiveDateTime>,
    pub version: Option<i32>,
    pub language: Option<String>,
    pub archived: Option<bool>,
    pub voucher_status: Option<String>,
    pub voucher_number: Option<String>,
    pub voucher_date: Option<NaiveDateTime>,
    pub due_date: Option<NaiveDateTime>,
    pub address_id: Option<String>,
    pub address_name: Option<String>,
    pub address_supplement: Option<String>,
    pub address_street: Option<String>,
    pub address_city: Option<String>,
    pub address_zip: Option<String>,
    pub address_countrycode: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DbLineItem {
    pub id: i32,
    pub product_id: Option<String>,
    pub voucher_id: Option<String>,
    pub quantity: Option<i32>,
    pub unit_name: Option<String>,
    pub currency: Option<String>,
    pub net_amount: Option<f64>,
    pub gross_amount: Option<f64>,
    pub tax_rate_percentage: Option<i32>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DbProduct {
    pub id: String,
    pub type_: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DbVoucher {
    pub id: String,
    pub voucher_type: Option<String>,
    pub voucher_status: Option<String>,
    pub voucher_number: Option<String>,
    pub voucher_date: Option<NaiveDateTime>,
    pub created_date: Option<NaiveDateTime>,
    pub updated_date: Option<NaiveDateTime>,
    pub due_date: Option<NaiveDateTime>,
    pub contact_id: Option<String>,
    pub contact_name: Option<String>,
    pub total_amount: Option<f64>,
    pub open_amount: Option<f64>,
    pub currency: Option<String>,
    pub archived: Option<bool>,
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
