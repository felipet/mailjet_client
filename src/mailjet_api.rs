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
