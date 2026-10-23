// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// Why grip4rust's output yielded no report: what it wrote did not parse.

use xtask::grip::grip_report_failure::GripReportFailure;

#[test]
fn to_string_carries_the_reason() {
    // Arrange
    let failure = GripReportFailure::new(String::from("EOF while parsing"));

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "could not parse grip4rust JSON: EOF while parsing");
}
