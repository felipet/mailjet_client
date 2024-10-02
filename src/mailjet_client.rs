// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

//! Client module.

use crate::{
    data_objects::{ResponseObject, SendEmailParams, SendResponse, SendResponses},
    mailjet_api::ApiUrl,
    ApiVersion, ClientError, MailjetClientBuilder,
};
use reqwest_middleware::ClientWithMiddleware;
use reqwest_tracing::TracingMiddleware;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use tracing::{debug, instrument, trace};

/// This object implements a client for [Mailjet's][mapi] REST API.
///
/// # Description
///
/// TODO
///
/// [mapi]: https://dev.mailjet.com/email/reference/overview/
#[derive(Debug)]
pub struct MailjetClient {
    http_client: ClientWithMiddleware,
    pub email_address: Option<String>,
    pub email_name: Option<String>,
    api_user: SecretString,
    api_key: SecretString,
    api_url: String,
    api_version: ApiVersion,
}

impl MailjetClient {
    /// Constructor.
    ///
    /// # Description
    ///
    /// Main constructor of the object [MailjetClient]. This is a fallible call as it relies on [reqwest::Client::new]
    /// which is fallible.
    ///
    /// Internally, [reqwest_middleware::ClientWithMiddleware] is used to handle all the HTTP connections to the
    /// external REST API. Thanks to using that wrapped version of a [reqwest::Client], tracing support could be added
    /// with 0 effort.
    /// Beyond that, the following settings are applied:
    /// - Use the native TLS implementation.
    /// - HTTPS only.
    ///
    /// ## Arguments
    ///
    /// - `api_user` should receive the user token provided by Mailjet. See [Authentication][api_doc].
    /// - `api_key` should receive the user private token provided by Mailjet.
    /// - `email_address` allows to specify the sender address.
    /// - `email_name` allows to specify a name that will identify the sender of an email.
    /// - `user_agent`` allows to specify the `User-agent` content of the cookie that is included on every request made
    ///   to the REST API.
    ///
    /// ## Returns
    ///
    /// - A new instance of the object on success.
    /// - A variant of the `enum` [ClientError] otherwise.
    ///
    /// [api_doc]: https://dev.mailjet.com/email/guides/
    pub fn new(
        api_user: SecretString,
        api_key: SecretString,
        email_address: Option<&str>,
        email_name: Option<&str>,
        user_agent: Option<&str>,
        api_url: Option<&str>,
        api_version: Option<&str>,
    ) -> Result<Self, ClientError> {
        let user_agent: &str = if let Some(agent) = user_agent {
            agent
        } else {
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),)
        };

        let api_url = match api_url {
            Some(url) => url.into(),
            None => "https://api.mailjet.com".into(),
        };

        let api_version = match api_version {
            Some(version) => version.try_into()?,
            None => ApiVersion::V3,
        };

        let http_client = reqwest::ClientBuilder::new()
            .user_agent(user_agent)
            .use_native_tls()
            .https_only(true)
            .build()
            .map_err(|_| ClientError::HTTPClient)?;

        let wrapped_client = reqwest_middleware::ClientBuilder::new(http_client)
            .with(TracingMiddleware::default())
            .build();

        debug!("reqwest client successfully built");

        Ok(MailjetClient {
            http_client: wrapped_client,
            email_address: email_address.map(String::from),
            email_name: email_name.map(String::from),
            api_key,
            api_user,
            api_url,
            api_version,
        })
    }

    /// Change the target external API version (Mailjet).
    pub fn use_api_version(&mut self, version: ApiVersion) {
        self.api_version = version;
    }

    pub async fn send_email<'a>(
        &self,
        request: &SendEmailParams<'a>,
    ) -> Result<SendResponses, ClientError> {
        match self.api_version {
            ApiVersion::V3 => {
                trace!("Sending email to the external API (v3)");
                todo!("send email for V3 is unimplemented")
            }
            ApiVersion::V3_1 => {
                trace!("Sending email to the external API (v3.1)");
                self.send_email_v3_1(request).await
            }
        }
    }

    #[instrument]
    pub async fn send_email_v3_1<'a>(
        &self,
        request: &SendEmailParams<'a>,
    ) -> Result<SendResponses, ClientError> {
        debug!("Request parameters: {:#?}", request);

        // Build a new request using the HTTP client.
        let request = self
            .http_client
            .post(format!(
                "{}/{}",
                self.api_url,
                ApiUrl::send(&self.api_version)
            ))
            .basic_auth(
                self.api_user.expose_secret(),
                Some(&self.api_key.expose_secret()),
            )
            .json(&request)
            .build()
            .unwrap();

        debug!("POST request: {:#?}", request);

        // Send the prepared request to the external API.
        let raw_response = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| ClientError::ExternalError(e.to_string()))?;

        debug!("Received response: {:#?}", raw_response);

        let code = raw_response.status().as_u16();
        // The POST request was successfully executed.
        if code == 201 {
            let payload = raw_response
                .text()
                .await
                .map_err(|e| ClientError::UnknownError(e.to_string()))?;

            // Temporal struct to implement a deserializer.
            #[derive(Deserialize, Debug)]
            #[serde(rename_all = "PascalCase")]
            #[allow(dead_code)]
            struct TempResponse {
                pub messages: Vec<SendResponse>,
            }

            let mut payload: TempResponse = serde_json::from_str(payload.as_str())
                .map_err(|e| ClientError::UnknownError(e.to_string()))?;

            Ok(SendResponses {
                messages: payload
                    .messages
                    .drain(..)
                    .map(|e| Box::<dyn ResponseObject>::from(Box::new(e)))
                    .collect(),
            })
        } else if code == 400 {
            Err(ClientError::BadRequest(format!(
                "status_code: {}, payload: {:#?}",
                code, raw_response
            )))
        } else {
            Err(ClientError::UnknownError(format!(
                "status_code: {}, payload: {:#?}",
                code, raw_response
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::*;
    use secrecy::{ExposeSecret, SecretString};
    use uuid::Uuid;

    #[rstest]
    fn client_builds() {
        // TC: Default builder
        let builder = MailjetClientBuilder::default().build();

        assert!(builder.is_ok());

        // TC: Default builder + keys
        let builder = MailjetClientBuilder::default()
            .with_api_user(SecretString::from(Uuid::new_v4().to_string()))
            .with_api_key(SecretString::from(Uuid::new_v4().to_string()))
            .build();

        assert!(builder.is_ok());

        // TC: Default values given by Mailjet::new
        let api_user = SecretString::from(Uuid::new_v4().to_string());
        let api_key = SecretString::from(Uuid::new_v4().to_string());
        let builder = MailjetClientBuilder::new(api_user, api_key).build();
        assert!(builder.is_ok());

        // TC: Complete builder
        let api_user = SecretString::from(Uuid::new_v4().to_string());
        let api_key = SecretString::from(Uuid::new_v4().to_string());
        let name = "Test User";
        let email = "test_user@mail.com";
        let url = "demo.com";
        let version = ApiVersion::V3_1;

        let builder = MailjetClientBuilder::default()
            .with_api_user(api_user.clone())
            .with_api_key(api_key.clone())
            .with_email_name(name)
            .with_email_address(email)
            .with_user_agent(name)
            .with_api_url(url)
            .with_api_version(version.to_string().as_str())
            .build();

        assert!(builder.is_ok());

        let client = builder.unwrap();
        assert_eq!(client.email_address.unwrap(), email);
        assert_eq!(client.email_name.unwrap(), name);
        assert_eq!(client.api_user.expose_secret(), api_user.expose_secret());
        assert_eq!(client.api_key.expose_secret(), api_key.expose_secret());
        assert_eq!(client.api_url, url);
        assert_eq!(client.api_version, version);
    }

    #[rstest]
    fn change_api_version() {
        let mut client = MailjetClientBuilder::default().build().unwrap();
        assert_eq!(client.api_version, ApiVersion::default());
        client.api_version = ApiVersion::V3_1;
        assert_eq!(client.api_version, ApiVersion::V3_1);
        client.use_api_version(ApiVersion::V3);
        assert_eq!(client.api_version, ApiVersion::V3);
    }
}
