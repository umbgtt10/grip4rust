// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::process::command_runner_tests::FakeCommandRunner;
use xtask::gates::gate::Gate;
use xtask::gates::grip_self_gate::GripSelfGate;
use xtask::grip::grip_report_parser::GripReportParser;

fn gate<'a>(runner: &'a FakeCommandRunner, parser: &'a GripReportParser) -> GripSelfGate<'a> {
    GripSelfGate::new(
        runner,
        parser,
        String::from("cargo-grip4rust"),
        String::from("core"),
        59,
    )
}

fn report_scoring(score: i64) -> String {
    format!(r#"{{"overall": {{"grip_score": {score}}}}}"#)
}

#[test]
fn label_names_the_self_analysis_gate() {
    // Arrange
    let runner = FakeCommandRunner::new();
    let parser = GripReportParser::new();

    // Act
    let label = gate(&runner, &parser).label();

    // Assert
    assert_eq!(label, "grip self-analysis");
}

// fixture/ holds deliberately sloppy sample code that is analysis input, never
// this tool's own source, so the gate must analyse core rather than the
// workspace root.
#[test]
fn run_analyses_the_configured_target_rather_than_the_workspace_root() {
    // Arrange
    let runner = FakeCommandRunner::new().with_stdout(&report_scoring(61));
    let parser = GripReportParser::new();

    // Act
    let _ = gate(&runner, &parser).run();

    // Assert
    let call = &runner.calls()[0];
    assert!(call.contains(&String::from("core")));
    assert!(call.contains(&String::from("--json")));
}

#[test]
fn run_builds_the_tool_from_this_checkout_rather_than_an_install() {
    // Arrange
    let runner = FakeCommandRunner::new().with_stdout(&report_scoring(61));
    let parser = GripReportParser::new();

    // Act
    let _ = gate(&runner, &parser).run();

    // Assert
    let call = &runner.calls()[0];
    assert_eq!(call[0], "run");
    assert!(call.contains(&String::from("cargo-grip4rust")));
}

// A build failure has no report to read, so the exit code and stderr are all
// there is to report with.
#[test]
fn run_with_a_failing_build_reports_the_exit_code_rather_than_parsing() {
    // Arrange
    let runner = FakeCommandRunner::new().with_streaming_code(Some(101));
    let parser = GripReportParser::new();

    // Act
    let result = gate(&runner, &parser).run();

    // Assert
    assert!(result.is_err_and(|error| error.contains("101")));
}

#[test]
fn run_with_a_score_above_the_floor_returns_ok() {
    // Arrange
    let runner = FakeCommandRunner::new().with_stdout(&report_scoring(61));
    let parser = GripReportParser::new();

    // Act
    let result = gate(&runner, &parser).run();

    // Assert
    assert!(result.is_ok());
}

#[test]
fn run_with_a_score_below_the_floor_names_both_numbers() {
    // Arrange
    let runner = FakeCommandRunner::new().with_stdout(&report_scoring(58));
    let parser = GripReportParser::new();

    // Act
    let result = gate(&runner, &parser).run();

    // Assert
    assert_eq!(
        result,
        Err(String::from("score 58 is below the floor of 59"))
    );
}

#[test]
fn run_with_a_score_exactly_on_the_floor_returns_ok() {
    // Arrange
    let runner = FakeCommandRunner::new().with_stdout(&report_scoring(59));
    let parser = GripReportParser::new();

    // Act
    let result = gate(&runner, &parser).run();

    // Assert
    assert!(result.is_ok());
}

#[test]
fn run_with_output_that_is_not_a_report_returns_a_parse_error() {
    // Arrange
    let runner = FakeCommandRunner::new().with_stdout("not json at all");
    let parser = GripReportParser::new();

    // Act
    let result = gate(&runner, &parser).run();

    // Assert
    assert!(result.is_err_and(|error| error.contains("could not parse grip4rust JSON")));
}
