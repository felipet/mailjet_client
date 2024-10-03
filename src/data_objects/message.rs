use names::Generator;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameAndEmail {
    pub email: String,
    pub name: Option<String>,
}

impl Default for NameAndEmail {
    fn default() -> Self {
        let rand_name = Generator::default().next().unwrap();

        NameAndEmail {
            email: format!("{}@mail.com", rand_name.clone()),
            name: Some(rand_name.replace(&rand_name, "-")),
        }
    }
}

impl NameAndEmail {
    pub fn new(email: &str, name: Option<&str>) -> Self {
        NameAndEmail {
            email: email.into(),
            name: name.map(str::to_string),
        }
    }
}

/// Attachment object.
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
