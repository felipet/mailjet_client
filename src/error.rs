use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("An error was found in the internal HTTP Client")]
    HTTPClient,
    #[error("Wrong API version specified")]
    WrongApiVersion,
    #[error("Missing Mailjet API key token")]
    MissingApiKey,
}
