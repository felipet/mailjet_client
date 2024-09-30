// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

use anyhow::{anyhow, Result};
use mailjet_client::{ClientError, MailjetClient, Response};
use once_cell::sync::Lazy;
use secrecy::SecretString;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, registry, util::SubscriberInitExt, Layer};

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
}

impl TestApp {
    pub fn new() -> Result<Self> {
        Lazy::force(&TRACING);

        let api_user =
            std::env::var("MAILJET_API_USER").expect("Missing MAILJET_API_USER env variable");
        let api_key =
            std::env::var("MAILJET_API_KEY").expect("Missing MAILJET_API_KEY env variable");

        let api_client = MailjetClient::new(
            SecretString::new(api_user.into()),
            SecretString::new(api_key.into()),
            Some("Rust mailjet test agent"),
            Some("admin@nubecita.eu"),
            Some("Test"),
            None,
            None,
        )
        .map_err(|_| anyhow!("Failed to build mailjet client"))?;

        Ok(TestApp { api_client })
    }

    pub fn send_email(&mut self) -> Result<Response, ClientError> {
        self.api_client
            .use_api_version(mailjet_client::ApiVersion::default());
        self.api_client.send_email()
    }

    pub fn send_email_v3_1(&mut self) -> Result<Response, ClientError> {
        self.api_client
            .use_api_version(mailjet_client::ApiVersion::V3_1);
        self.api_client.send_email()
    }
}
