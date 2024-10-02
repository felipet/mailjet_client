// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

//! Translations of data objects returned by the endpoints of Mailjet's API.

use crate::data_objects::ResponseObject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Data object returned by `/send` (v3.1) as `Messages`. See [`/send`][send].
/// [send]: https://dev.mailjet.com/email/reference/send-emails#v3_1_post_send
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SendResponseObject {
    pub status: ResponseStatus,
    pub errors: Option<Vec<ResponseError>>,
    pub to: Option<Vec<MessageError>>,
    pub cc: Option<Vec<MessageError>>,
    pub bcc: Option<Vec<MessageError>>,
}

impl ResponseObject for SendResponseObject {}

/// Data object for the field `Errors` in the response of `/send`. See [`/send`][send].
/// [send]: https://dev.mailjet.com/email/reference/send-emails#v3_1_post_send
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseError {
    pub error_identifier: Uuid,
    pub error_code: String,
    pub status_code: i64,
    pub error_message: String,
    pub error_related_to: Vec<String>,
}

/// Data object for the field `To`, `Bcc`, `Cc` in the response of `/send`. See [`/send`][send].
/// [send]: https://dev.mailjet.com/email/reference/send-emails#v3_1_post_send
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Success,
    Error,
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
