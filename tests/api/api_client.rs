// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

use crate::helper::TestApp;
use mailjet_client::{
    data_objects::{MessageBuilder, SendEmailParams},
    ClientError,
};
use rstest::*;
use std::mem::discriminant;
use tracing::{debug, info};

#[fixture]
fn def_email_request<'a>() -> SendEmailParams<'a> {
    let message = MessageBuilder::default().build();

    SendEmailParams {
        sandbox_mode: Some(true),
        advance_error_handling: None,
        globals: None,
        messages: Vec::from([message]),
    }
}

#[rstest]
async fn test_send_email_v3_1<'a>(def_email_request: SendEmailParams<'a>) {
    let mut test_client = TestApp::new().expect("Failed to build a test client");
    let result = test_client.send_email_v3_1(&def_email_request).await;

    assert!(result.is_err());
    let errors = result.err().unwrap();
    info!("Errors: {:#?}", errors);
    assert_eq!(
        discriminant(&errors),
        discriminant(&ClientError::BadRequest("".to_string()))
    );
}
