// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

use crate::helper::TestApp;
use async_std::fs::read_to_string;
use mailjet_client::{
    data_objects::{MessageBuilder, MessageObject, SendEmailParams, SimpleMessage},
    ClientError,
};
use rstest::*;
use serde::{Deserialize, Serialize};
use std::mem::discriminant;
use tracing::{debug, info};
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

#[fixture]
fn empty_email_request_v3_1() -> SendEmailParams {
    let message = MessageBuilder::default().build();

    SendEmailParams {
        sandbox_mode: Some(true),
        advance_error_handling: None,
        globals: None,
        messages: Vec::from([message]),
    }
}

#[fixture]
async fn valid_email_request_v3_1() -> SendEmailParams {
    let valid_mail = read_to_string("tests/api/data/valid_mail_v3_1.json")
        .await
        .expect("Failed to load valid message example from JSON file");

    serde_json::from_str(&valid_mail).expect("Failed to parse valid email JSON")
}

#[fixture]
async fn valid_email_request_v3() -> SimpleMessage {
    let valid_mail = read_to_string("tests/api/data/valid_mail_v3.json")
        .await
        .expect("Failed to load valid message example from JSON file");

    let mut message: SimpleMessage =
        serde_json::from_str(&valid_mail).expect("Failed to parse valid email JSON");

    let from_addr =
        std::env::var("MAILJET_FROM_EMAIL").expect("Missing MAILJET_FROM_EMAIL env variable");

    if let Ok(addr) = std::env::var("MAILJET_TEST_RECIPIENT") {
        message.to = Some(addr);
    }

    message.from_email = from_addr;

    message
}

#[fixture]
async fn valid_response_send_v3() -> ResponseTemplate {
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    #[allow(dead_code)]
    struct TempResponse {
        pub sent: Vec<MessageObject>,
    }

    let example_response: TempResponse = serde_json::from_str(
        &read_to_string("tests/api/data/response_send_v3.json")
            .await
            .expect("Failed to read the response template from a file"),
    )
    .expect("Failed");

    ResponseTemplate::new(200).set_body_json(example_response)
}

/// Simple test case to check that we can send a request to the external API.
///
/// # Description
///
/// This TC sends a dummy request to the external API to send a message. However, the content of the request is
/// empty, thus the POST is expected to fail with a 400 code.
#[rstest]
async fn test_send_empty_email_v3_1(empty_email_request_v3_1: SendEmailParams) {
    let mut test_client = TestApp::new().expect("Failed to build a test client");
    let result = test_client.send_email_v3_1(&empty_email_request_v3_1).await;

    assert!(result.is_err());
    let errors = result.err().unwrap();
    info!("Errors: {:#?}", errors);
    assert_eq!(
        discriminant(&errors),
        discriminant(&ClientError::BadRequest("".to_string()))
    );
}

/// Test case to check whether we are able to send a valid email (sandbox_mode activated).
#[rstest]
async fn test_send_valid_email_v3_1(#[future] valid_email_request_v3_1: SendEmailParams) {
    let mut test_client = TestApp::new().expect("Failed to build a test client");
    let result = test_client
        .send_email_v3_1(&valid_email_request_v3_1.await)
        .await;

    debug!("Result: {:#?}", result);
    assert!(result.is_ok());
}

/// Test case to check sending an email using real fire (no sandbox_mode activated.)
#[rstest]
#[ignore = "Run only on MR, or important reviews"]
async fn test2_send_email_v3_1() {
    // First, try to read confidential variables from the environment.
    let from_addr =
        std::env::var("MAILJET_FROM_EMAIL").expect("Missing MAILJET_FROM_EMAIL env variable");

    let test_recipient =
        std::env::var("MAILJET_TEST_RECIPIENT").unwrap_or("pilot@mailjet.com".to_string());

    let mut test_client = TestApp::new().expect("Failed to build a test client");

    let message = MessageBuilder::default()
        .with_from(&from_addr, Some("Mailjet Rust Client"))
        .with_to(&test_recipient, None)
        .with_text_body("Test message sent from the Mailjet Rust Client")
        .build();

    let request = SendEmailParams {
        sandbox_mode: Some(false),
        advance_error_handling: Some(true),
        globals: None,
        messages: Vec::from([message]),
    };

    let result = test_client.send_email_v3_1(&request).await;

    debug!("Result: {:#?}", result);
    assert!(result.is_ok());
}

#[rstest]
#[ignore = "Run only on MR, or important reviews"]
async fn test_send_email_v3(#[future] valid_email_request_v3: SimpleMessage) {
    let mut test_client = TestApp::new().expect("Failed to build a test client");
    let result = test_client
        .send_email_v3(&valid_email_request_v3.await)
        .await;

    assert!(result.is_ok());
    debug!("Response: {:#?}", result.unwrap());
}

#[rstest]
async fn mocktest_send_email_v3(
    #[future] valid_email_request_v3: SimpleMessage,
    #[future] valid_response_send_v3: ResponseTemplate,
) {
    let mut test_client = TestApp::spawn_app()
        .await
        .expect("Failed to build a mock test client");

    Mock::given(path("/v3/send"))
        .and(method("POST"))
        .respond_with(valid_response_send_v3.await)
        .mount(
            test_client
                .email_server
                .as_deref()
                .expect("Failed to get a reference to the mock server"),
        )
        .await;

    let result = test_client
        .send_email_v3(&valid_email_request_v3.await)
        .await;

    debug!("Response: {:#?}", result);
    assert!(result.is_ok());
}
