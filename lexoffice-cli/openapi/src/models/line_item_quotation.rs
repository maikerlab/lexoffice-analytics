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
pub struct LineItemQuotation {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "quantity")]
    pub quantity: f32,
    #[serde(rename = "unitName", skip_serializing_if = "Option::is_none")]
    pub unit_name: Option<String>,
    #[serde(rename = "unitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<Box<crate::models::UnitPrice>>,
    #[serde(rename = "discountPercentage", skip_serializing_if = "Option::is_none")]
    pub discount_percentage: Option<f32>,
    #[serde(rename = "lineItemAmount", skip_serializing_if = "Option::is_none")]
    pub line_item_amount: Option<f32>,
    #[serde(rename = "subItems", skip_serializing_if = "Option::is_none")]
    pub sub_items: Option<Vec<crate::models::LineItemQuotation>>,
    #[serde(rename = "optional", skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(rename = "alternative", skip_serializing_if = "Option::is_none")]
    pub alternative: Option<bool>,
}

impl LineItemQuotation {
    pub fn new(r#type: Type, name: String, quantity: f32) -> LineItemQuotation {
        LineItemQuotation {
            id: None,
            r#type,
            name,
            description: None,
            quantity,
            unit_name: None,
            unit_price: None,
            discount_percentage: None,
            line_item_amount: None,
            sub_items: None,
            optional: None,
            alternative: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "material")]
    Material,
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "text")]
    Text,
}

impl Default for Type {
    fn default() -> Type {
        Self::Service
    }
}

