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
