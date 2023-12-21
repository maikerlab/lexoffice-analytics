/*
 * lexoffice Public API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`invoices_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InvoicesIdGetError {
    Status401(crate::models::ErrorMsg),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`voucherlist_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VoucherlistGetError {
    Status401(crate::models::ErrorMsg),
    UnknownValue(serde_json::Value),
}


pub async fn invoices_id_get(configuration: &configuration::Configuration, id: &str) -> Result<crate::models::Invoice, Error<InvoicesIdGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/invoices/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<InvoicesIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn voucherlist_get(configuration: &configuration::Configuration, voucher_type: &str, voucher_status: &str, archived: Option<bool>, contact_id: Option<&str>, voucher_date_from: Option<String>, voucher_date_to: Option<String>, created_date_from: Option<String>, created_date_to: Option<String>, updated_date_from: Option<String>, updated_date_to: Option<String>, voucher_number: Option<&str>, page: Option<i32>, size: Option<i32>, sort: Option<&str>) -> Result<crate::models::VoucherList, Error<VoucherlistGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/voucherlist", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("voucherType", &voucher_type.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("voucherStatus", &voucher_status.to_string())]);
    if let Some(ref local_var_str) = archived {
        local_var_req_builder = local_var_req_builder.query(&[("archived", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = contact_id {
        local_var_req_builder = local_var_req_builder.query(&[("contactId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = voucher_date_from {
        local_var_req_builder = local_var_req_builder.query(&[("voucherDateFrom", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = voucher_date_to {
        local_var_req_builder = local_var_req_builder.query(&[("voucherDateTo", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_date_from {
        local_var_req_builder = local_var_req_builder.query(&[("createdDateFrom", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_date_to {
        local_var_req_builder = local_var_req_builder.query(&[("createdDateTo", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = updated_date_from {
        local_var_req_builder = local_var_req_builder.query(&[("updatedDateFrom", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = updated_date_to {
        local_var_req_builder = local_var_req_builder.query(&[("updatedDateTo", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = voucher_number {
        local_var_req_builder = local_var_req_builder.query(&[("voucherNumber", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = size {
        local_var_req_builder = local_var_req_builder.query(&[("size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<VoucherlistGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

