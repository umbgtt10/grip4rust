// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use xtask::grip::grip_report_parser::GripReportParser;

// The shape grip4rust actually emits: overall carries a dozen fields and the
// report a half-dozen more, of which the gate reads exactly one.
const REPORT: &str = r#"{
  "version": 1,
  "target": "core",
  "overall": {
    "grip_score": 61,
    "public_items": 120,
    "total_functions": 300,
    "pure_ratio": 0.5
  },
  "modules": [],
  "offenders": []
}"#;

#[test]
fn parse_a_report_ignores_the_fields_the_gate_does_not_read() {
    // Arrange
    let parser = GripReportParser::new();

    // Act
    let report = parser.parse(REPORT).expect("the report parses");

    // Assert
    assert_eq!(report.score(), 61);
}

#[test]
fn parse_a_report_missing_the_overall_section_returns_an_error() {
    // Arrange
    let parser = GripReportParser::new();

    // Act
    let result = parser.parse(r#"{"version": 1}"#);

    // Assert
    assert!(result.is_err_and(|error| error.contains("could not parse grip4rust JSON")));
}

#[test]
fn parse_empty_output_returns_an_error() {
    // Arrange
    let parser = GripReportParser::new();

    // Act
    let result = parser.parse("");

    // Assert
    assert!(result.is_err_and(|error| error.contains("could not parse grip4rust JSON")));
}

#[test]
fn parse_output_that_is_not_json_returns_an_error() {
    // Arrange
    let parser = GripReportParser::new();

    // Act
    let result = parser.parse("error: could not compile `cargo-grip4rust`");

    // Assert
    assert!(result.is_err_and(|error| error.contains("could not parse grip4rust JSON")));
}
