use std::time::SystemTime;

use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::vouchers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QueryableVoucher {
    pub id: String,
    pub vouchertype: Option<String>,
    pub voucherstatus: Option<String>,
    pub vouchernumber: Option<String>,
    pub voucherdate: Option<SystemTime>,
    pub createddate: Option<SystemTime>,
    pub updateddate: Option<SystemTime>,
    pub duedate: Option<SystemTime>,
    pub contactid: Option<String>,
    pub contactname: Option<String>,
    pub totalamount: Option<f64>,
    pub openamount: Option<f64>,
    pub currency: Option<String>,
    pub archived: Option<bool>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::invoices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QueryableInvoice {
    pub id: String,
    pub organizationid: Option<String>,
    pub createddate: Option<SystemTime>,
    pub updateddate: Option<SystemTime>,
    pub version: Option<i32>,
    pub language: Option<String>,
    pub archived: Option<bool>,
    pub voucherstatus: Option<String>,
    pub vouchernumber: Option<String>,
    pub voucherdate: Option<SystemTime>,
    pub duedate: Option<SystemTime>,
    pub address_id: Option<String>,
    pub address_name: Option<String>,
    pub address_supplement: Option<String>,
    pub address_street: Option<String>,
    pub address_city: Option<String>,
    pub address_zip: Option<String>,
    pub address_countrycode: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::invoices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertableInvoice {
    pub id: String,
    pub organizationid: Option<String>,
    pub createddate: Option<SystemTime>,
    pub updateddate: Option<SystemTime>,
    pub version: Option<i32>,
    pub language: Option<String>,
    pub archived: Option<bool>,
    pub voucherstatus: Option<String>,
    pub vouchernumber: Option<String>,
    pub voucherdate: Option<SystemTime>,
    pub duedate: Option<SystemTime>,
    pub address_id: Option<String>,
    pub address_name: Option<String>,
    pub address_supplement: Option<String>,
    pub address_street: Option<String>,
    pub address_city: Option<String>,
    pub address_zip: Option<String>,
    pub address_countrycode: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::vouchers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertableVoucher {
    pub id: String,
    pub vouchertype: Option<String>,
    pub voucherstatus: Option<String>,
    pub vouchernumber: Option<String>,
    pub voucherdate: Option<SystemTime>,
    pub createddate: Option<SystemTime>,
    pub updateddate: Option<SystemTime>,
    pub duedate: Option<SystemTime>,
    pub contactid: Option<String>,
    pub contactname: Option<String>,
    pub totalamount: Option<f64>,
    pub openamount: Option<f64>,
    pub currency: Option<String>,
    pub archived: Option<bool>,
}
