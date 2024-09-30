// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

use crate::helper::TestApp;
use rstest::*;

#[rstest]
async fn test_send_email_v3() {
    let mut test_client = TestApp::new().expect("Failed to build a test client");

    assert!(test_client.send_email().is_ok());
}

#[rstest]
async fn test_send_email_v3_1() {
    let mut test_client = TestApp::new().expect("Failed to build a test client");

    assert!(test_client.send_email_v3_1().is_ok());
}
