// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use xtask::grip::grip_report_parser::GripReportParser;

fn report_scoring(score: i64) -> xtask::grip::grip_report::GripReport {
    let json = format!(r#"{{"overall": {{"grip_score": {score}}}}}"#);
    GripReportParser::new().parse(&json).expect("parses")
}

// grip is a floor, not a ceiling: a higher score is better, so the gate fails
// below the line rather than above it. Every other threshold in stage 2 runs
// the other way, which is exactly why this is worth asserting.
#[test]
fn meets_a_score_above_the_floor_returns_true() {
    // Arrange
    let report = report_scoring(61);

    // Act & Assert
    assert!(report.meets(59));
}

#[test]
fn meets_a_score_below_the_floor_returns_false() {
    // Arrange
    let report = report_scoring(58);

    // Act & Assert
    assert!(!report.meets(59));
}

#[test]
fn meets_a_score_exactly_on_the_floor_returns_true() {
    // Arrange
    let report = report_scoring(59);

    // Act & Assert
    assert!(report.meets(59));
}

#[test]
fn score_returns_the_overall_grip_score() {
    // Arrange
    let report = report_scoring(61);

    // Act
    let score = report.score();

    // Assert
    assert_eq!(score, 61);
}

#[test]
fn summary_states_the_score_against_the_floor() {
    // Arrange
    let report = report_scoring(61);

    // Act
    let summary = report.summary(59);

    // Assert
    assert_eq!(summary, "grip score: 61 / 100  (floor: 59)");
}
