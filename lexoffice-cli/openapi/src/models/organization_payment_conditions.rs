/*
 * lexoffice Public API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationPaymentConditions {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "organizationDefault", skip_serializing_if = "Option::is_none")]
    pub organization_default: Option<bool>,
    #[serde(rename = "paymentTermLabelTemplate", skip_serializing_if = "Option::is_none")]
    pub payment_term_label_template: Option<String>,
    #[serde(rename = "paymentTermDuration", skip_serializing_if = "Option::is_none")]
    pub payment_term_duration: Option<i32>,
    #[serde(rename = "paymentDiscountConditions", skip_serializing_if = "Option::is_none")]
    pub payment_discount_conditions: Option<Box<crate::models::PaymentConditionsPaymentDiscountConditions>>,
}

impl OrganizationPaymentConditions {
    pub fn new() -> OrganizationPaymentConditions {
        OrganizationPaymentConditions {
            id: None,
            organization_default: None,
            payment_term_label_template: None,
            payment_term_duration: None,
            payment_discount_conditions: None,
        }
    }
}


