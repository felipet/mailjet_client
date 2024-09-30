// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

//! Client module.

use crate::{ApiVersion, ClientError};
use reqwest_middleware::ClientWithMiddleware;
use reqwest_tracing::TracingMiddleware;
use secrecy::SecretString;
use tracing::debug;

pub struct MailjetClientBuilder {
    email_address: Option<String>,
    email_name: Option<String>,
    user_agent: Option<String>,
    api_user: Option<SecretString>,
    api_key: Option<SecretString>,
    api_url: Option<String>,
    api_version: Option<String>,
}

impl Default for MailjetClientBuilder {
    fn default() -> Self {
        MailjetClientBuilder {
            email_address: None,
            email_name: None,
            user_agent: None,
            api_user: Some(SecretString::new("None".into())),
            api_key: Some(SecretString::new("None".into())),
            api_url: Some("https://api.mailjet.com".into()),
            api_version: Some(ApiVersion::default().to_string()),
        }
    }
}

impl MailjetClientBuilder {
    pub fn with_email_address(mut self, email: &str) -> MailjetClientBuilder {
        self.email_address = Some(email.to_string());

        self
    }

    pub fn with_email_name(mut self, name: &str) -> MailjetClientBuilder {
        self.email_name = Some(name.to_string());

        self
    }

    pub fn with_user_agent(mut self, name: &str) -> MailjetClientBuilder {
        self.user_agent = Some(name.into());

        self
    }

    pub fn with_api_user(mut self, api_user: SecretString) -> MailjetClientBuilder {
        self.api_user = Some(api_user);

        self
    }

    pub fn with_api_key(mut self, api_key: SecretString) -> MailjetClientBuilder {
        self.api_key = Some(api_key);

        self
    }

    pub fn with_api_url(mut self, url: &str) -> MailjetClientBuilder {
        self.api_url = Some(url.to_string());

        self
    }

    pub fn with_api_version(mut self, version: &str) -> MailjetClientBuilder {
        self.api_version = Some(version.into());

        self
    }

    pub fn new(api_user: SecretString, api_key: SecretString) -> MailjetClientBuilder {
        MailjetClientBuilder {
            api_user: Some(api_user),
            api_key: Some(api_key),
            email_address: None,
            email_name: None,
            user_agent: None,
            api_url: None,
            api_version: None,
        }
    }

    pub fn build(self) -> Result<MailjetClient, ClientError> {
        MailjetClient::new(
            self.api_user.unwrap(),
            self.api_key.unwrap(),
            self.email_address.as_deref(),
            self.email_name.as_deref(),
            self.user_agent.as_deref(),
            self.api_url.as_deref(),
            self.api_version.as_deref(),
        )
    }
}

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
            None => "https://api.mailjet.com/".into(),
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
