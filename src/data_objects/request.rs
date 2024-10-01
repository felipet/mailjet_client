// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

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
