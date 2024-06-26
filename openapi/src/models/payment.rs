/*
 * lexoffice Public API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Payment {
    #[serde(rename = "openAmount")]
    pub open_amount: f64,
    #[serde(rename = "currency")]
    pub currency: Currency,
    #[serde(rename = "paymentStatus")]
    pub payment_status: PaymentStatus,
    #[serde(rename = "voucherType")]
    pub voucher_type: VoucherType,
    #[serde(rename = "voucherStatus")]
    pub voucher_status: VoucherStatus,
    #[serde(rename = "paidDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub paid_date: Option<Option<String>>,
    #[serde(rename = "paymentItems")]
    pub payment_items: Vec<models::PaymentItem>,
}

impl Payment {
    pub fn new(open_amount: f64, currency: Currency, payment_status: PaymentStatus, voucher_type: VoucherType, voucher_status: VoucherStatus, payment_items: Vec<models::PaymentItem>) -> Payment {
        Payment {
            open_amount,
            currency,
            payment_status,
            voucher_type,
            voucher_status,
            paid_date: None,
            payment_items,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "EUR")]
    Eur,
}

impl Default for Currency {
    fn default() -> Currency {
        Self::Eur
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentStatus {
    #[serde(rename = "balanced")]
    Balanced,
    #[serde(rename = "openRevenue")]
    OpenRevenue,
    #[serde(rename = "openExpense")]
    OpenExpense,
}

impl Default for PaymentStatus {
    fn default() -> PaymentStatus {
        Self::Balanced
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoucherType {
    #[serde(rename = "salesinvoice")]
    Salesinvoice,
    #[serde(rename = "salescreditnote")]
    Salescreditnote,
    #[serde(rename = "purchaseinvoice")]
    Purchaseinvoice,
    #[serde(rename = "purchasecreditnote")]
    Purchasecreditnote,
    #[serde(rename = "invoice")]
    Invoice,
    #[serde(rename = "downpaymentinvoice")]
    Downpaymentinvoice,
    #[serde(rename = "creditnote")]
    Creditnote,
}

impl Default for VoucherType {
    fn default() -> VoucherType {
        Self::Salesinvoice
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoucherStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "paidoff")]
    Paidoff,
    #[serde(rename = "voided")]
    Voided,
    #[serde(rename = "transferred")]
    Transferred,
    #[serde(rename = "sepadebit")]
    Sepadebit,
}

impl Default for VoucherStatus {
    fn default() -> VoucherStatus {
        Self::Open
    }
}

