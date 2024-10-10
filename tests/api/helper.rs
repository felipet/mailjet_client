// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

use anyhow::{anyhow, Result};
use mailjet_client::{
    data_objects::{ContactQuery, RequestObject, Response},
    ClientError, MailjetClient,
};
use once_cell::sync::Lazy;
use secrecy::SecretString;
use std::sync::Arc;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt, layer::SubscriberExt, registry, util::SubscriberInitExt, Layer};
use wiremock::MockServer;

static TRACING: Lazy<()> = Lazy::new(|| {
    // Build a `Subscriber` when TEST_LOG is set in the terminal session.
    let enable_tracing = std::env::var("TEST_LOG").is_ok();

    if enable_tracing {
        let filter_level = match std::env::var("TEST_LOG").unwrap().as_str() {
            "debug" => LevelFilter::DEBUG,
            "tracing" => LevelFilter::TRACE,
            _ => LevelFilter::INFO,
        };

        // Compose a very detailed layer to help debugging testing errors.
        let layer = fmt::layer()
            .pretty()
            .with_ansi(true)
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_filter(filter_level)
            .boxed();

        registry().with(layer).init();
    }
});

pub struct TestApp {
    pub api_client: MailjetClient,
    pub email_server: Option<Arc<MockServer>>,
}

impl TestApp {
    pub fn new() -> Result<Self> {
        Lazy::force(&TRACING);

        let api_user =
            std::env::var("MAILJET_API_USER").expect("Missing MAILJET_API_USER env variable");
        let api_key =
            std::env::var("MAILJET_API_KEY").expect("Missing MAILJET_API_KEY env variable");

        let email_address = match std::env::var("MAILJET_EMAIL") {
            Ok(email) => email,
            Err(_) => {
                info!("Email used as sender not specified, using a dummy value");
                "jane_doe@mail.com".into()
            }
        };

        let api_client = MailjetClient::new(
            SecretString::new(api_user.into()),
            SecretString::new(api_key.into()),
            Some(&email_address),
            Some("Rust mailjet test agent"),
            Some("Test"),
            None,
            None,
            None,
        )
        .map_err(|_| anyhow!("Failed to build mailjet client"))?;

        Ok(TestApp {
            api_client,
            email_server: None,
        })
    }

    pub async fn spawn_app() -> Result<Self> {
        Lazy::force(&TRACING);

        // Instantiate a new mock HTTP server to handle requests to the external API.
        let mock_server = MockServer::start().await;

        let api_client = MailjetClient::new(
            SecretString::new("None".into()),
            SecretString::new("None".into()),
            Some("jane_doe@mail.com"),
            Some("Jane Doe"),
            Some("Test"),
            Some(&mock_server.uri()),
            None,
            Some(false),
        )
        .map_err(|_| anyhow!("Failed to build Mailjet client"))?;

        Ok(TestApp {
            api_client,
            email_server: Some(Arc::new(mock_server)),
        })
    }

    pub async fn send_email_v3_1(
        &mut self,
        request: &impl RequestObject,
    ) -> Result<Response, ClientError> {
        info!("Test email using API v3.1");
        self.api_client
            .use_api_version(mailjet_client::ApiVersion::V3_1);
        self.api_client.send_email(request).await
    }

    pub async fn send_email_v3(
        &mut self,
        request: &impl RequestObject,
    ) -> Result<Response, ClientError> {
        info!("Test email using API v3");
        self.api_client
            .use_api_version(mailjet_client::ApiVersion::V3);
        self.api_client.send_email(request).await
    }

    pub async fn post_contact(&self, request: &ContactQuery) -> Result<Response, ClientError> {
        self.api_client.add_contact(request).await
    }
}
