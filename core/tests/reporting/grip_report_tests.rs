// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::function_info::FunctionInfo;
use grip::reporting::grip_report::GripReport;
use grip::reporting::module_stats::ModuleStats;
use grip::reporting::offender::Offender;
use grip::reporting::overall_stats::OverallStats;
use serde_json::from_str;
use serde_json::to_string;

fn sample() -> GripReport {
    GripReport::new(
        "0.1.0".to_string(),
        "test".to_string(),
        OverallStats::default()
            .with_grip_score(Some(50))
            .with_public_items(1)
            .with_total_functions(2)
            .with_pure_functions(1)
            .with_pure_ratio(0.5)
            .with_public_ratio(0.5)
            .with_grip_absolute_total(1.5),
        vec![],
        vec![],
        50,
        vec![],
    )
}

#[test]
fn functions_lists_every_function_in_order() {
    // Arrange
    let report = GripReport::new(
        "0.1.0".to_string(),
        "test".to_string(),
        OverallStats::default(),
        vec![],
        vec![],
        50,
        vec![
            FunctionInfo::default().with_name("compute".to_string()),
            FunctionInfo::default().with_name("render".to_string()),
        ],
    );

    // Act
    let functions = report.functions();

    // Assert
    let names: Vec<&str> = functions.iter().map(FunctionInfo::name).collect();
    assert_eq!(names, ["compute", "render"]);
}

#[test]
fn modules_lists_every_module_in_order() {
    // Arrange
    let report = GripReport::new(
        "0.1.0".to_string(),
        "test".to_string(),
        OverallStats::default(),
        vec![
            ModuleStats::default().with_path("alpha".to_string()),
            ModuleStats::default().with_path("beta".to_string()),
        ],
        vec![],
        50,
        vec![],
    );

    // Act
    let modules = report.modules();

    // Assert
    let paths: Vec<&str> = modules.iter().map(ModuleStats::path).collect();
    assert_eq!(paths, ["alpha", "beta"]);
}

#[test]
fn offender_threshold_returns_the_score_below_which_a_module_offends() {
    // Arrange
    let report = sample();

    // Act
    let offender_threshold = report.offender_threshold();

    // Assert
    assert_eq!(offender_threshold, 50);
}

#[test]
fn offenders_lists_every_offending_module() {
    // Arrange
    let report = GripReport::new(
        "0.1.0".to_string(),
        "test".to_string(),
        OverallStats::default(),
        vec![],
        vec![Offender::new("bad_mod".to_string(), 30)],
        50,
        vec![],
    );

    // Act
    let offenders = report.offenders();

    // Assert
    assert_eq!(offenders.len(), 1);
    assert_eq!(offenders[0].path(), "bad_mod");
    assert_eq!(offenders[0].grip_score(), 30);
}

#[test]
fn report_deserializes_from_json() {
    // Arrange
    let report = sample();
    let json = to_string(&report).unwrap();

    // Act
    let parsed: GripReport = from_str(&json).unwrap();

    // Assert
    assert_eq!(parsed.version(), "0.1.0");
    assert_eq!(parsed.overall().grip_score(), Some(50));
}

#[test]
fn report_serializes_to_json() {
    // Arrange
    let report = sample();

    // Act
    let json = to_string(&report).unwrap();

    // Assert
    assert!(json.contains("grip_score"));
    assert!(json.contains("\"test\""));
}

#[test]
fn target_returns_the_analysed_target() {
    // Arrange
    let report = sample();

    // Act
    let target = report.target();

    // Assert
    assert_eq!(target, "test");
}
