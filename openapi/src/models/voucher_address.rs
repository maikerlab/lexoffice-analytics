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

/// VoucherAddress : Address for CreditNote/DeliveryNote/Invoice
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoucherAddress {
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<uuid::Uuid>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "supplement", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub supplement: Option<Option<String>>,
    #[serde(rename = "street", skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "zip", skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "contactPerson", skip_serializing_if = "Option::is_none")]
    pub contact_person: Option<String>,
}

impl VoucherAddress {
    /// Address for CreditNote/DeliveryNote/Invoice
    pub fn new(name: String, country_code: String) -> VoucherAddress {
        VoucherAddress {
            contact_id: None,
            name,
            supplement: None,
            street: None,
            city: None,
            zip: None,
            country_code,
            contact_person: None,
        }
    }
}
