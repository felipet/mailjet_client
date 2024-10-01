// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

//! Data objects related to the endpoint `/sender` of Mailjet's REST API.
use crate::data_objects::{ObjectType, ResponseObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub enum EmailType {
    Transactional,
    Bulk,
    #[default]
    Unknown,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum Status {
    Inactive,
    #[default]
    Active,
    Deleted,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Sender {
    pub email_type: EmailType,
    pub is_default_sender: bool,
    pub name: String,
    pub created_at: String,
    #[serde(rename = "DNSID")]
    pub dns_id: i64,
    pub email: String,
    pub filename: String,
    #[serde(rename = "ID")]
    pub id: i64,
    pub status: Status,
}

impl ResponseObject for Sender {}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SenderQuery {
    #[serde(rename = "DnsID")]
    pub dns_id: Option<i64>,
    pub domain: Option<String>,
    pub email: Option<String>,
    pub is_domain_sender: Option<bool>,
    pub local_part: Option<String>,
    pub show_deleted: Option<bool>,
    pub status: Option<String>,
    pub limit: Option<u16>,
    pub offset: Option<u16>,
    pub count_only: Option<bool>,
    pub sort: Option<String>,
}

#[derive(Debug)]
pub struct SendResponses {
    pub messages: ObjectType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SendResponse {
    pub status: ResponseStatus,
    pub errors: Option<Vec<ResponseError>>,
    pub to: Option<Vec<MessageError>>,
    pub cc: Option<Vec<MessageError>>,
    pub bcc: Option<Vec<MessageError>>,
}

impl ResponseObject for SendResponse {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseError {
    pub error_identifier: Uuid,
    pub error_code: String,
    pub status_code: i64,
    pub error_message: String,
    pub error_related_to: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MessageError {
    pub email: String,
    #[serde(rename = "MessageUUID")]
    pub message_uuid: Uuid,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub message_href: String,
}
