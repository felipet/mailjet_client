// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

use crate::helper::TestApp;
use mailjet_client::{
    data_objects::{MessageBuilder, SendEmailParams, SimpleMessage},
    ClientError,
};
use rstest::*;
use std::mem::discriminant;
use tracing::{debug, info};

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
fn valid_email_request_v3_1() -> SendEmailParams {
    let message = MessageBuilder::default()
        .with_from("admin@nubecita.eu", None)
        .with_to("torresfelipex1@gmail.com", None)
        .with_text_body("A demo test message.")
        .build();

    SendEmailParams {
        sandbox_mode: Some(true),
        advance_error_handling: None,
        globals: None,
        messages: Vec::from([message]),
    }
}

#[fixture]
fn email_request_v3() -> SimpleMessage {
    SimpleMessage::default()
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
async fn test_send_valid_email_v3_1(valid_email_request_v3_1: SendEmailParams) {
    let mut test_client = TestApp::new().expect("Failed to build a test client");
    let result = test_client.send_email_v3_1(&valid_email_request_v3_1).await;

    debug!("Result: {:#?}", result);
    assert!(result.is_ok());
}

#[rstest]
async fn test_send_email_v3(email_request_v3: SimpleMessage) {
    let mut test_client = TestApp::new().expect("Failed to build a test client");
    let result = test_client.send_email_v3(&email_request_v3).await;

    assert!(result.is_err());
    let errors = result.err().unwrap();
    info!("Errors: {:#?}", errors);
    assert_eq!(
        discriminant(&errors),
        discriminant(&ClientError::BadRequest("".to_string()))
    );
}
