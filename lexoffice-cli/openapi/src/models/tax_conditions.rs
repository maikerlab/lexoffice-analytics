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
pub struct TaxConditions {
    #[serde(rename = "taxType", skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<TaxType>,
    #[serde(rename = "taxSubType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tax_sub_type: Option<Option<TaxSubType>>,
    #[serde(rename = "taxTypeNote", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tax_type_note: Option<Option<String>>,
}

impl TaxConditions {
    pub fn new() -> TaxConditions {
        TaxConditions {
            tax_type: None,
            tax_sub_type: None,
            tax_type_note: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaxType {
    #[serde(rename = "net")]
    Net,
    #[serde(rename = "gross")]
    Gross,
    #[serde(rename = "vatfree")]
    Vatfree,
    #[serde(rename = "intraCommunitySupply")]
    IntraCommunitySupply,
    #[serde(rename = "constructionService13b")]
    ConstructionService13b,
    #[serde(rename = "externalService13b")]
    ExternalService13b,
    #[serde(rename = "thirdPartyCountryService")]
    ThirdPartyCountryService,
    #[serde(rename = "thirdPartyCountryDelivery")]
    ThirdPartyCountryDelivery,
    #[serde(rename = "photovoltaicEquipment")]
    PhotovoltaicEquipment,
}

impl Default for TaxType {
    fn default() -> TaxType {
        Self::Net
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaxSubType {
    #[serde(rename = "distanceSales")]
    DistanceSales,
    #[serde(rename = "electronicServices")]
    ElectronicServices,
}

impl Default for TaxSubType {
    fn default() -> TaxSubType {
        Self::DistanceSales
    }
}

