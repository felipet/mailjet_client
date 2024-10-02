// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

//! Translations of data objects use as params for the endpoints of Mailjet's API.

use crate::data_objects::{Message, MessageProperty};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendEmailParams<'a> {
    pub sandbox_mode: Option<bool>,
    pub advance_error_handling: Option<bool>,
    pub globals: Option<MessageProperty<'a>>,
    pub messages: Vec<Message>,
}

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
