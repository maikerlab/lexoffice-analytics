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
pub struct TotalPrice {
    #[serde(rename = "currency")]
    pub currency: Currency,
    #[serde(rename = "totalNetAmount")]
    pub total_net_amount: f32,
    #[serde(rename = "totalGrossAmount")]
    pub total_gross_amount: f32,
    #[serde(rename = "totalTaxAmount")]
    pub total_tax_amount: f32,
    #[serde(rename = "totalDiscountAbsolute", skip_serializing_if = "Option::is_none")]
    pub total_discount_absolute: Option<f32>,
    #[serde(rename = "totalDiscountPercentage", skip_serializing_if = "Option::is_none")]
    pub total_discount_percentage: Option<f32>,
}

impl TotalPrice {
    pub fn new(currency: Currency, total_net_amount: f32, total_gross_amount: f32, total_tax_amount: f32) -> TotalPrice {
        TotalPrice {
            currency,
            total_net_amount,
            total_gross_amount,
            total_tax_amount,
            total_discount_absolute: None,
            total_discount_percentage: None,
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

