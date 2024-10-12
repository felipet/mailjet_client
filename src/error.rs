// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

//! Custom error types.

use thiserror::Error;

/// Error types returned by the client.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ClientError {
    #[error("An error was found in the internal HTTP Client")]
    HTTPClient,
    #[error("Wrong API version specified")]
    WrongApiVersion,
    #[error("Missing Mailjet API key token")]
    MissingApiKey,
    #[error("Unknown error")]
    UnknownError(String),
    #[error("External error")]
    ExternalError(String),
    #[error("The given data is formatted wrongly")]
    BadRequest(String),
    #[error("Error found while parsing data")]
    ParseError(String),
}
