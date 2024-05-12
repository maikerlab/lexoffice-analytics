use sqlx::{
    self,
    types::{chrono::NaiveDateTime, BigDecimal},
};

#[derive(Debug, sqlx::FromRow)]
pub struct DbAddress {
    pub contact_id: String,
    pub name: String,
    pub supplement: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub zip: Option<String>,
    pub country_code: String,
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
    pub total_net_amount: BigDecimal,
    pub total_gross_amount: BigDecimal,
    pub total_tax_amount: BigDecimal,
    pub total_discount_absolute: BigDecimal,
    pub total_discount_percentage: BigDecimal,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DbLineItem {
    pub id: i32,
    pub product_id: String,
    pub voucher_id: String,
    pub quantity: BigDecimal,
    pub unit_name: String,
    pub currency: String,
    pub net_amount: BigDecimal,
    pub gross_amount: BigDecimal,
    pub tax_rate_percentage: Option<BigDecimal>,
    pub discount_percentage: Option<BigDecimal>,
    pub line_item_amount: Option<BigDecimal>,
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
    pub contact_name: String,
    pub total_amount: BigDecimal,
    pub open_amount: BigDecimal,
    pub currency: String,
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
