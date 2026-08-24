// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use xtask::grip::grip_report_parser::GripReportParser;

#[test]
fn grip_score_is_carried_through_from_the_report() {
    // Arrange
    let json = r#"{"overall": {"grip_score": 74}}"#;

    // Act
    let report = GripReportParser::new().parse(json).expect("parses");

    // Assert
    assert_eq!(report.overall.grip_score, 74);
}

#[test]
fn grip_score_of_zero_is_carried_rather_than_treated_as_absent() {
    // Arrange
    let json = r#"{"overall": {"grip_score": 0}}"#;

    // Act
    let report = GripReportParser::new().parse(json).expect("parses");

    // Assert
    assert_eq!(report.overall.grip_score, 0);
}
