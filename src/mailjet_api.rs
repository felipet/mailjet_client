// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

use crate::ClientError;
use core::convert::TryFrom;
use core::fmt;

/// `Enum` to select the API version of Mailjet's REST API.
#[derive(PartialEq, Default)]
pub enum ApiVersion {
    #[default]
    V3,
    V3_1,
}

/// A simple look-up table that builds the URL of a particular endpoint.
///
/// # Description
///
/// This object abstracts the endpoint URLs of the external REST API (Mailjet) from the client's code.
/// To build the full URL of a particular endpoint of the external API, the base URL is needed (set in the client's
/// constructor) along the API version and the endpoint aimed to use. This object builds the internal path of the URL,
/// so updates to the API version or minor changes in the endpoint routes might not affect the client's implementation.
pub struct ApiUrl;

impl ApiUrl {
    /// URL of the endpoint [`/send`][send]
    /// [send]: https://dev.mailjet.com/email/reference/send-emails/
    pub fn send(api_version: &ApiVersion) -> String {
        let endpoint = "send";

        match api_version {
            ApiVersion::V3_1 => format!("{api_version}/{endpoint}"),
            _ => format!("{}/{endpoint}", ApiVersion::default()),
        }
    }

    pub fn contact(api_version: &ApiVersion) -> String {
        let endpoint = "contact";

        // As of today, this endpoint is only supported by the API v3.0.
        #[allow(clippy::match_single_binding)]
        match api_version {
            _ => format!("{}/REST/{endpoint}", ApiVersion::default()),
        }
    }
}

impl TryFrom<&str> for ApiVersion {
    type Error = ClientError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();

        match value.as_str() {
            "v3.1" => Ok(ApiVersion::V3_1),
            "v3" => Ok(ApiVersion::V3),
            _ => Err(ClientError::WrongApiVersion),
        }
    }
}

impl fmt::Debug for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version = match self {
            ApiVersion::V3 => "v3",
            ApiVersion::V3_1 => "v3.1",
        };

        write!(f, "{version}")
    }
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version = match self {
            ApiVersion::V3 => "v3",
            ApiVersion::V3_1 => "v3.1",
        };

        write!(f, "{version}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::*;

    #[rstest]
    #[case("v3", "v3/send")]
    #[case("v3.1", "v3.1/send")]
    fn check_api_send(#[case] input: &str, #[case] expected: &str) {
        let api_version = ApiVersion::try_from(input).expect("Failed to convert str to ApiVersion");

        assert_eq!(ApiUrl::send(&api_version), expected.to_string());
    }

    #[rstest]
    #[case("default", ClientError::WrongApiVersion)]
    fn check_apiversion_fails_when_using_wrong_data(
        #[case] input: &str,
        #[case] expected: ClientError,
    ) {
        let api_version = ApiVersion::try_from(input);
        assert!(api_version.is_err());
        let error = api_version.err().unwrap();
        assert_eq!(error, expected);
    }

    #[rstest]
    fn check_api_contact() {
        // TCs for REST API v3 (default)
        let mut api_version = ApiVersion::default();

        assert_eq!(ApiUrl::send(&api_version), "v3/send".to_string());

        // TCs for REST API v3
        api_version = ApiVersion::V3;

        assert_eq!(ApiUrl::send(&api_version), "v3/send".to_string());

        // TCs for REST API v3.1
        api_version = ApiVersion::V3_1;
        assert_eq!(ApiUrl::send(&api_version), "v3.1/send".to_string());
    }
}
