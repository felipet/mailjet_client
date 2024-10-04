// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

//! Module that defines all the objects related to mails.

use super::RequestObject;
use names::Generator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Object that represents the parameters needed to send a message using the API::v3.1.
///
/// # Description
///
/// This object matches the allowed parameters defined in [`/send`](https://dev.mailjet.com/email/reference/send-emails#v3_1_post_send).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Message {
    pub from: NameAndEmail,
    pub sender: Option<NameAndEmail>,
    pub to: Option<Vec<NameAndEmail>>,
    pub cc: Option<Vec<NameAndEmail>>,
    pub bcc: Option<Vec<NameAndEmail>>,
    pub reply_to: Option<NameAndEmail>,
    pub subject: Option<String>,
    pub text_part: Option<String>,
    pub html_part: Option<String>,
    pub template_id: Option<u64>,
    pub template_language: Option<bool>,
    pub template_error_reporting: Option<NameAndEmail>,
    pub template_error_deliver: Option<bool>,
    pub attachments: Option<Vec<Attachment>>,
    pub inline_attachments: Option<Vec<Attachment>>,
    pub priority: Option<u8>,
    pub custom_campaign: Option<String>,
    pub deduplicate_campaign: Option<bool>,
    pub track_opens: Option<Track>,
    pub track_clicks: Option<Track>,
    #[serde(rename = "CustomID")]
    pub custom_id: Option<String>,
    pub event_payload: Option<String>,
    #[serde(rename = "URLTags")]
    pub url_tags: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub variables: Option<HashMap<String, String>>,
}

/// Object that represents the parameters needed to send a message using the API::v3.0.
///
/// # Description
///
/// This object matches the allowed parameters defined in [`/send`](https://dev.mailjet.com/email/reference/send-emails#v3_1_post_send).
/// However, some parameters are missing in this struct. Mostly, all the parameters that start by *Mj-* are
/// skipped due to problems with the request.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SimpleMessage {
    pub from_email: String,
    pub from_name: String,
    pub sender: Option<bool>,
    pub recipients: Vec<NameAndEmail>,
    pub to: Option<String>,
    pub cc: Option<String>,
    pub bcc: Option<String>,
    pub subject: Option<String>,
    #[serde(rename = "Text-part")]
    pub text_part: Option<String>,
    #[serde(rename = "Html-part")]
    pub html_part: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
    pub inline_attachments: Option<Vec<Attachment>>,
    pub event_payload: Option<String>,
    pub vars: Option<String>,
}

impl RequestObject for SimpleMessage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Object that implements a builder construction pattern for [Message].
///
/// # Description
///
/// This object populates the mandatory fields with the default value for the type in case no value was
/// given. This might cause later issues when sending the request to the external API. The main use case
/// is to build a [Message] with default values, and later on, populate only those required. This would
/// be much quicker than calling new with a lot of `None` values.
///
/// ## Example
///
/// ```
/// use mailjet_client::MessageBuilder;
///
/// let message = MessageBuilder::default()
///     .with_from("john_doe@mail.com", Some("John Doe"))
///     .build();
/// ```
#[derive(Debug, Default)]
pub struct MessageBuilder {
    pub from: Option<NameAndEmail>,
    pub sender: Option<NameAndEmail>,
    pub to: Option<Vec<NameAndEmail>>,
    pub cc: Option<Vec<NameAndEmail>>,
    pub bcc: Option<Vec<NameAndEmail>>,
    pub reply_to: Option<NameAndEmail>,
    pub subject: Option<String>,
    pub text_part: Option<String>,
    pub html_part: Option<String>,
    pub template_id: Option<u64>,
    pub template_language: Option<bool>,
    pub template_error_reporting: Option<NameAndEmail>,
    pub template_error_deliver: Option<bool>,
    pub attachments: Option<Vec<Attachment>>,
    pub inline_attachments: Option<Vec<Attachment>>,
    pub priority: Option<u8>,
    pub custom_campaign: Option<String>,
    pub deduplicate_campaign: Option<bool>,
    pub track_opens: Option<Track>,
    pub track_clicks: Option<Track>,
    pub custom_id: Option<String>,
    pub event_payload: Option<String>,
    pub url_tags: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub variables: Option<HashMap<String, String>>,
}

impl MessageBuilder {
    pub fn with_from(mut self, email: &str, name: Option<&str>) -> Self {
        self.from = Some(NameAndEmail::new(email, name));

        self
    }

    pub fn with_to(mut self, email: &str, name: Option<&str>) -> Self {
        self.to = Some(Vec::from([
            NameAndEmail::new(email, name),
            NameAndEmail::new(email, name),
        ]));

        self
    }

    pub fn with_text_body(mut self, body: &str) -> Self {
        self.text_part = Some(body.to_string());

        self
    }

    pub fn build(self) -> Message {
        Message {
            from: self.from.unwrap_or_default(),
            sender: self.sender,
            to: self.to,
            cc: self.cc,
            bcc: self.bcc,
            reply_to: self.reply_to,
            subject: self.subject,
            text_part: self.text_part,
            html_part: self.html_part,
            template_id: self.template_id,
            template_language: self.template_language,
            template_error_reporting: self.template_error_reporting,
            template_error_deliver: self.template_error_deliver,
            attachments: self.attachments,
            inline_attachments: self.inline_attachments,
            priority: self.priority,
            custom_campaign: self.custom_campaign,
            deduplicate_campaign: self.deduplicate_campaign,
            track_opens: self.track_opens,
            track_clicks: self.track_clicks,
            custom_id: self.custom_id,
            event_payload: self.event_payload,
            url_tags: self.url_tags,
            headers: self.headers,
            variables: self.variables,
        }
    }
}

/// Object that represents the object needed to fill the property `Globals` of the `/send` endpoint.
///
/// # Description
///
/// The parameters to send a message using the external API::v3.1 include a field named `Globals`. It is
/// an object that is translated by this struct.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MessageProperty {
    pub from: Option<NameAndEmail>,
    pub sender: Option<NameAndEmail>,
    pub cc: Option<Vec<NameAndEmail>>,
    pub bcc: Option<Vec<NameAndEmail>>,
    pub reply_to: Option<NameAndEmail>,
    pub subject: Option<String>,
    pub text_part: Option<String>,
    pub html_part: Option<String>,
    pub template_id: Option<u64>,
    pub template_language: Option<bool>,
    pub template_error_reporting: Option<NameAndEmail>,
    pub template_error_deliver: Option<bool>,
    pub attachments: Option<Vec<Attachment>>,
    pub inline_attachments: Option<Vec<Attachment>>,
    pub priority: Option<u8>,
    pub custom_campaign: Option<String>,
    pub deduplicate_campaign: Option<bool>,
    pub track_opens: Option<Track>,
    pub track_clicks: Option<Track>,
    #[serde(rename = "CustomID")]
    pub custom_id: Option<String>,
    pub event_payload: Option<String>,
    #[serde(rename = "URLTags")]
    pub url_tags: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub variables: Option<HashMap<String, String>>,
}

/// Simple object that includes an email and a linked name (optional).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameAndEmail {
    pub email: String,
    pub name: Option<String>,
    pub vars: Option<String>,
}

impl Default for NameAndEmail {
    fn default() -> Self {
        let rand_name = Generator::default().next().unwrap();

        NameAndEmail {
            email: format!("{}@mail.com", rand_name.clone()),
            name: Some(rand_name.replace(&rand_name, "-")),
            vars: None,
        }
    }
}

impl NameAndEmail {
    pub fn new(email: &str, name: Option<&str>) -> Self {
        NameAndEmail {
            email: email.into(),
            name: name.map(str::to_string),
            vars: None,
        }
    }
}

/// Attachment object.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attachment {
    pub filename: String,
    pub content_type: String,
    pub base_64_content: String,
}

#[derive(Debug, Default, Serialize, PartialEq, Deserialize)]
pub enum Track {
    #[default]
    #[serde(rename = "account_default")]
    AccountDefault,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
}
