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
pub struct File {
    #[serde(rename = "documentFileId", skip_serializing_if = "Option::is_none")]
    pub document_file_id: Option<String>,
}

impl File {
    pub fn new() -> File {
        File {
            document_file_id: None,
        }
    }
}

