use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Message<'a> {
    pub from: NameAndEmail,
    pub sender: Option<NameAndEmail>,
    pub to: &'a [NameAndEmail],
    pub cc: Option<&'a [NameAndEmail]>,
    pub bcc: Option<&'a [NameAndEmail]>,
    pub reply_to: Option<NameAndEmail>,
    pub subject: Option<String>,
    pub text_part: Option<String>,
    pub html_part: Option<String>,
    pub template_id: Option<u64>,
    pub template_language: Option<bool>,
    pub template_error_reporting: Option<NameAndEmail>,
    pub template_error_deliver: Option<bool>,
    pub attachments: Option<&'a [Attachment]>,
    pub inline_attachments: Option<&'a [Attachment]>,
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameAndEmail {
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attachment {
    pub filename: String,
    pub content_type: String,
    pub base_64_content: String,
}

#[derive(Debug, Default, Serialize, PartialEq)]
pub enum Track {
    #[default]
    #[serde(rename = "account_default")]
    AccountDefault,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
}
