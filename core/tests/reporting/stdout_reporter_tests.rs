// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::function_info::FunctionInfo;
use grip::reporting::grip_report::GripReport;
use grip::reporting::module_stats::ModuleStats;
use grip::reporting::offender::Offender;
use grip::reporting::overall_stats::OverallStats;
use grip::reporting::stdout_reporter::StdoutReporter;
use grip::traits::reporter::Reporter;
use serde_json::Value;
use serde_json::from_str;

fn dummy_modules() -> Vec<ModuleStats> {
    vec![
        ModuleStats::default()
            .with_path("alpha".to_string())
            .with_grip_score(Some(80))
            .with_pure_ratio(0.8)
            .with_public_items(5)
            .with_total_functions(10)
            .with_pure_functions(8)
            .with_public_ratio(0.5)
            .with_grip_absolute_total(7.0),
        ModuleStats::default()
            .with_path("beta".to_string())
            .with_grip_score(Some(50))
            .with_pure_ratio(0.4)
            .with_public_items(5)
            .with_total_functions(10)
            .with_pure_functions(4)
            .with_public_ratio(0.5)
            .with_grip_absolute_total(5.0),
    ]
}

fn dummy_report() -> GripReport {
    report_with(dummy_modules(), vec![], vec![])
}

fn report_with(
    modules: Vec<ModuleStats>,
    offenders: Vec<Offender>,
    functions: Vec<FunctionInfo>,
) -> GripReport {
    GripReport::new(
        "0.1.0".to_string(),
        "my-crate".to_string(),
        OverallStats::default()
            .with_grip_score(Some(71))
            .with_public_items(10)
            .with_total_functions(20)
            .with_pure_functions(12)
            .with_pure_ratio(0.6)
            .with_public_ratio(0.5)
            .with_grip_absolute_total(12.0),
        modules,
        offenders,
        50,
        functions,
    )
}

fn reporter(json: bool) -> StdoutReporter {
    StdoutReporter::new(json, false)
}

#[test]
fn human_output_contains_module_lines() {
    // Arrange
    let report = dummy_report();
    let reporter = reporter(false);

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    assert!(out.contains("alpha"));
    assert!(out.contains("beta"));
}

#[test]
fn human_output_contains_score() {
    // Arrange
    let report = dummy_report();
    let reporter = reporter(false);

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    assert!(out.contains("71 / 100"));
}

#[test]
fn human_output_non_verbose_hides_per_function_detail() {
    // Arrange
    let report = report_with(
        dummy_modules(),
        vec![],
        vec![
            FunctionInfo::default()
                .with_name("compute".to_string())
                .with_file("src/lib.rs".to_string())
                .with_is_pure(true)
                .with_is_public(true)
                .with_grip_absolute(0.95)
                .with_grip_normalized(95),
        ],
    );
    let reporter = reporter(false);

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    assert!(!out.contains("Per-function detail"));
}

#[test]
fn human_output_shows_offenders_section() {
    // Arrange
    let report = report_with(
        vec![
            ModuleStats::default()
                .with_path("bad_mod".to_string())
                .with_grip_score(Some(30))
                .with_pure_ratio(0.3)
                .with_public_items(1)
                .with_total_functions(5)
                .with_pure_functions(1)
                .with_public_ratio(0.2)
                .with_grip_absolute_total(2.0),
        ],
        vec![Offender::new("bad_mod".to_string(), 30)],
        vec![],
    );
    let reporter = reporter(false);

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    assert!(out.contains("Offenders"));
    assert!(out.contains("bad_mod"));
}

#[test]
fn human_output_verbose_shows_per_function_detail() {
    // Arrange
    let report = report_with(
        dummy_modules(),
        vec![],
        vec![
            FunctionInfo::default()
                .with_name("compute".to_string())
                .with_file("src/lib.rs".to_string())
                .with_is_pure(true)
                .with_is_public(true)
                .with_grip_absolute(0.95)
                .with_grip_normalized(95),
        ],
    );
    let reporter = StdoutReporter::new(false, true);

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    assert!(out.contains("Per-function detail"));
    assert!(out.contains("compute"));
}

#[test]
fn json_output_has_version() {
    // Arrange
    let report = dummy_report();
    let reporter = reporter(true);

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    let parsed: Value = from_str(&out).unwrap();
    assert_eq!(parsed["version"], "0.1.0");
}

#[test]
fn json_output_is_valid() {
    // Arrange
    let report = dummy_report();
    let reporter = reporter(true);

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    let parsed: Value = from_str(&out).unwrap();
    assert_eq!(parsed["overall"]["grip_score"], 71);
    assert!(parsed.get("modules").is_some());
}
