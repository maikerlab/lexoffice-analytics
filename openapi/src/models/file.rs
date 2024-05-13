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
pub struct File {
    #[serde(rename = "documentFileId")]
    pub document_file_id: String,
}

impl File {
    pub fn new(document_file_id: String) -> File {
        File {
            document_file_id,
        }
    }
}
