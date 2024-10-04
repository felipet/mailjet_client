// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

use crate::helper::TestApp;
use async_std::fs::read_to_string;
use mailjet_client::{
    data_objects::{ContactQuery, MessageBuilder, NameAndEmail, SendEmailParams, SimpleMessage},
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

    serde_json::from_str(&valid_mail).expect("Failed to parse valid email JSON")
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

#[rstest]
async fn test_send_email_v3(#[future] valid_email_request_v3: SimpleMessage) {
    let mut test_client = TestApp::new().expect("Failed to build a test client");
    let result = test_client
        .send_email_v3(&valid_email_request_v3.await)
        .await;

    assert!(result.is_ok());
    debug!("Response: {:#?}", result.unwrap());
}

#[rstest]
async fn test_add_contact() {
    let test_client = TestApp::new().expect("Failed to build a test client");
    let request = ContactQuery {
        is_excluded_from_campaigns: Some(true),
        email: "demo@mailjet.com".to_string(),
        name: Some("John Doe".into()),
    };
    let result = test_client.post_contact(&request).await;

    info!("{:#?}", result);
}
