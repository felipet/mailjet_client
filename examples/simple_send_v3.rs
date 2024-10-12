//! Code example to showcase how to send a simple message using [mailjet_client::MailjetClient] and the API version v3.
//!
//! # Description
//!
//! This example builds a new client using two environment variables: **MAILJET_API_USER** and **MAILJET_API_KEY**.
//! These two variables must be defined in order to retrieve the credentials to use the external API.
//!
//! Then a simple client is build using the builder object ([mailjet_client::MailjetClientBuilder]), and an object
//! [mailjet_client::data_objects::SimpleMessage] is created with the content of the email. Mailjet's dummy account
//! is used for the shake of the example. Feel free to modify either or both *FromEmail* and/or *Recipients::email* to
//! test this example with a real email account.
//!
//! Finally the request is sent to the external API, and the returned status code is checked.
use mailjet_client::{data_objects::SimpleMessage, ClientError, MailjetClientBuilder};
use secrecy::SecretString;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    // Read the API user and key hashes from an environment variable and store them using Secrecy
    // to avoid data leaks.
    let api_user = SecretString::from(
        env::var("MAILJET_API_USER").expect("Missing MAILJET_API_USER env variable"),
    );
    let api_key = SecretString::from(
        env::var("MAILJET_API_KEY").expect("Missing MAILJET_API_KEY env variable"),
    );

    let mclient = MailjetClientBuilder::new(api_user, api_key)
        .build()
        .expect("Failed to build a new Mailjet client");

    let message: SimpleMessage = serde_json::from_value(json!({
        "FromEmail": "pilot@mailjet.com",
        "FromName": "Mailjet Client test",
        "Recipients":[
          {
            "Email":"pilot@mailjet.com",
            "Name":"Your Mailjet Pilot"
          }
        ],
        "Subject":"Your email flight plan!",
        "Text-part":"Dear passenger, welcome to Mailjet! May the delivery force be with you!"
    }))
    .unwrap();

    let response = mclient.send_email(&message).await?;
    if response.status_code == 200 {
        println!("The email was successfully delivered.");
    } else {
        println!("The email couldn't be delivered.");
    }

    Ok(())
}
