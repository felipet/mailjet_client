// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

//! Client builder module.

use crate::{ApiVersion, ClientError, MailjetClient};
use secrecy::SecretString;

/// A builder object for [MailjetClient].
///
/// # Description
///
/// This object implements a builder creation pattern for the [MailjetClient] object. Use this object to simplify the
/// construction of a new object `MailjetClient`.
///
/// ## Example
///
/// ```rust
/// use mailjetclient::{MailjetClientBuilder};
/// use secrecy::SecretString;
///
/// let client = MailjetClientBuilder::default()
///     .with_api_user(SecretString::from("A hash"))
///     .with_api_key(SecretString::from("A hash"))
///     .build();
///
/// ```
///
/// If you don't provide valid keys for the access of the external API, the build step won't fail. However, the client
/// will raise an error whenever you attempt to access the external API.
/// The rest of the values are optional, and the lowest API version will be used as default choice.
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
    /// Default values for a client builder.
    ///
    /// # Description
    ///
    /// Be aware that if you don't provide a valid hash for the API user and key, a dummy value will be used to
    /// populate those fields. If you don't change those values before attempting a call that interacts with the
    /// external API using the client, an error will be returned.
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::*;
    use secrecy::ExposeSecret;
    use uuid::Uuid;

    struct Keys {
        pub user: String,
        pub key: String,
    }

    #[fixture]
    fn keys() -> Keys {
        Keys {
            user: Uuid::new_v4().to_string(),
            key: Uuid::new_v4().to_string(),
        }
    }

    #[rstest]
    fn client_from_new(keys: Keys) {
        let api_user = SecretString::from(keys.user.clone());
        let api_key = SecretString::from(keys.key.clone());

        let client = MailjetClientBuilder::new(api_user, api_key).build();
        assert!(client.is_ok());
    }

    #[rstest]
    fn client_from_builder(keys: Keys) {
        let api_user = SecretString::from(keys.user.clone());
        let api_key = SecretString::from(keys.key.clone());
        let email = "janedoe@mail.com";
        let name = "jane doe";
        let agent = "test agent";

        let client_builder = MailjetClientBuilder::default()
            .with_api_key(api_key.clone())
            .with_api_user(api_user.clone())
            .with_email_address(email)
            .with_email_name(name)
            .with_user_agent(agent)
            .with_api_version(&ApiVersion::default().to_string());

        assert_eq!(client_builder.api_user.unwrap().expose_secret(), keys.user);
        assert_eq!(client_builder.api_key.unwrap().expose_secret(), keys.key);
        assert_eq!(client_builder.email_name.unwrap(), name);
        assert_eq!(client_builder.user_agent.unwrap(), agent);
        assert_eq!(
            client_builder.api_version.unwrap(),
            ApiVersion::default().to_string()
        );

        let client_builder = MailjetClientBuilder::default()
            .with_api_key(api_key)
            .with_api_user(api_user)
            .with_email_address(email)
            .with_email_name(name)
            .with_user_agent(agent)
            .with_api_version(&ApiVersion::default().to_string())
            .build();

        assert!(client_builder.is_ok());
    }
}
