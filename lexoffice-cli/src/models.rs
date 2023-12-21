use std::time::SystemTime;

use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::vouchers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Voucher {
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
