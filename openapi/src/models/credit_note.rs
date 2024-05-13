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
pub struct CreditNote {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "organizationId", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<uuid::Uuid>,
    #[serde(rename = "createdDate")]
    pub created_date: String,
    #[serde(rename = "updatedDate")]
    pub updated_date: String,
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "language")]
    pub language: Language,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "voucherStatus")]
    pub voucher_status: VoucherStatus,
    #[serde(rename = "voucherNumber")]
    pub voucher_number: String,
    #[serde(rename = "voucherDate")]
    pub voucher_date: String,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<models::VoucherAddress>>,
    #[serde(rename = "lineItems")]
    pub line_items: Vec<models::LineItem>,
    #[serde(rename = "totalPrice", skip_serializing_if = "Option::is_none")]
    pub total_price: Option<Box<models::TotalPrice>>,
    #[serde(rename = "taxAmounts", skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<Vec<models::TaxAmount>>,
    #[serde(rename = "taxConditions", skip_serializing_if = "Option::is_none")]
    pub tax_conditions: Option<Box<models::TaxConditions>>,
    #[serde(rename = "relatedVouchers", skip_serializing_if = "Option::is_none")]
    pub related_vouchers: Option<Vec<models::RelatedVoucher>>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "introduction", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub introduction: Option<Option<String>>,
    #[serde(rename = "remark", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub remark: Option<Option<String>>,
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<models::File>>,
}

impl CreditNote {
    pub fn new(id: uuid::Uuid, created_date: String, updated_date: String, version: i32, language: Language, archived: bool, voucher_status: VoucherStatus, voucher_number: String, voucher_date: String, line_items: Vec<models::LineItem>) -> CreditNote {
        CreditNote {
            id,
            organization_id: None,
            created_date,
            updated_date,
            version,
            language,
            archived,
            voucher_status,
            voucher_number,
            voucher_date,
            address: None,
            line_items,
            total_price: None,
            tax_amounts: None,
            tax_conditions: None,
            related_vouchers: None,
            title: None,
            introduction: None,
            remark: None,
            files: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "de")]
    De,
    #[serde(rename = "en")]
    En,
}

impl Default for Language {
    fn default() -> Language {
        Self::De
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoucherStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "voided")]
    Voided,
}

impl Default for VoucherStatus {
    fn default() -> VoucherStatus {
        Self::Draft
    }
}
