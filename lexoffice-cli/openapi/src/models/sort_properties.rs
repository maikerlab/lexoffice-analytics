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
pub struct SortProperties {
    #[serde(rename = "direction")]
    pub direction: String,
    #[serde(rename = "property")]
    pub property: String,
    #[serde(rename = "ignoreCase")]
    pub ignore_case: bool,
    #[serde(rename = "nullHandling")]
    pub null_handling: String,
    #[serde(rename = "ascending")]
    pub ascending: bool,
}

impl SortProperties {
    pub fn new(direction: String, property: String, ignore_case: bool, null_handling: String, ascending: bool) -> SortProperties {
        SortProperties {
            direction,
            property,
            ignore_case,
            null_handling,
            ascending,
        }
    }
}


