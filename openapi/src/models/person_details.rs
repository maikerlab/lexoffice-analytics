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
pub struct PersonDetails {
    #[serde(rename = "salutation", skip_serializing_if = "Option::is_none")]
    pub salutation: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl PersonDetails {
    pub fn new() -> PersonDetails {
        PersonDetails {
            salutation: None,
            first_name: None,
            last_name: None,
        }
    }
}

