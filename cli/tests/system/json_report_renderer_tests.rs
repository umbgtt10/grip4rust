// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::function_info::FunctionInfo;
use grip::reporting::grip_report::GripReport;
use grip::reporting::module_stats::ModuleStats;
use grip::reporting::offender::Offender;
use grip::reporting::overall_stats::OverallStats;
use grip::traits::report_renderer::ReportRenderer;
use grip_cli::system::json_report_renderer::JsonReportRenderer;
use serde_json::Value;
use serde_json::from_str;

fn dummy_report() -> GripReport {
    report_with(
        OverallStats {
            grip_score: Some(71),
            public_items: 10,
            total_functions: 20,
            pure_functions: 12,
            pure_ratio: 0.6,
            public_ratio: 0.5,
            grip_absolute_total: 12.0,
            ..Default::default()
        },
        vec![
            ModuleStats {
                path: "alpha".to_string(),
                grip_score: Some(80),
                pure_ratio: 0.8,
                public_items: 5,
                total_functions: 10,
                pure_functions: 8,
                public_ratio: 0.5,
                grip_absolute_total: 7.0,
                ..Default::default()
            },
            ModuleStats {
                path: "beta".to_string(),
                grip_score: Some(50),
                pure_ratio: 0.4,
                public_items: 5,
                total_functions: 10,
                pure_functions: 4,
                public_ratio: 0.5,
                grip_absolute_total: 5.0,
                ..Default::default()
            },
        ],
        vec![],
        vec![],
    )
}

fn parsed(report: &GripReport) -> Value {
    from_str(&rendered(report)).unwrap()
}

fn rendered(report: &GripReport) -> String {
    JsonReportRenderer::new().render(report).unwrap()
}

fn report_with(
    overall: OverallStats,
    modules: Vec<ModuleStats>,
    offenders: Vec<Offender>,
    functions: Vec<FunctionInfo>,
) -> GripReport {
    GripReport {
        version: "0.1.0".to_string(),
        target: "test".to_string(),
        overall,
        modules,
        offenders,
        offender_threshold: 50,
        functions,
    }
}

fn sample() -> GripReport {
    report_with(
        OverallStats {
            grip_score: Some(50),
            public_items: 1,
            total_functions: 2,
            pure_functions: 1,
            pure_ratio: 0.5,
            public_ratio: 0.5,
            grip_absolute_total: 1.5,
            ..Default::default()
        },
        vec![],
        vec![],
        vec![],
    )
}

#[test]
fn render_carries_a_functions_name_and_hidden_dep_labels() {
    // Arrange
    let report = report_with(
        OverallStats::default(),
        vec![],
        vec![],
        vec![FunctionInfo {
            name: "process".to_string(),
            file: "src/lib.rs".to_string(),
            is_pure: true,
            is_public: true,
            hidden_deps: 1,
            dep_weight: 0.2,
            hidden_dep_labels: vec!["println".to_string()],
            grip_absolute: 0.8,
            grip_normalized: 80,
            ..Default::default()
        }],
    );

    // Act
    let json = rendered(&report);

    // Assert
    assert!(json.contains("process"));
    assert!(json.contains("println"));
}

#[test]
fn render_carries_a_modules_path_and_score() {
    // Arrange
    let report = report_with(
        OverallStats::default(),
        vec![ModuleStats {
            path: "test".to_string(),
            grip_score: Some(42),
            pure_ratio: 0.5,
            public_items: 3,
            total_functions: 10,
            pure_functions: 5,
            public_ratio: 0.3,
            grip_absolute_total: 7.5,
            ..Default::default()
        }],
        vec![],
        vec![],
    );

    // Act
    let json = rendered(&report);

    // Assert
    assert!(json.contains("test"));
    assert!(json.contains("42"));
}

#[test]
fn render_carries_an_offenders_path_and_score() {
    // Arrange
    let report = report_with(
        OverallStats::default(),
        vec![],
        vec![Offender {
            path: "bad_module".to_string(),
            grip_score: 30,
        }],
        vec![],
    );

    // Act
    let json = rendered(&report);

    // Assert
    assert!(json.contains("bad_module"));
    assert!(json.contains("30"));
}

#[test]
fn render_carries_the_overall_score() {
    // Arrange
    let report = report_with(
        OverallStats {
            grip_score: Some(80),
            public_items: 10,
            total_functions: 20,
            pure_functions: 15,
            pure_ratio: 0.75,
            public_ratio: 0.5,
            grip_absolute_total: 15.0,
            ..Default::default()
        },
        vec![],
        vec![],
        vec![],
    );

    // Act
    let json = rendered(&report);

    // Assert
    assert!(json.contains("80"));
    assert!(json.contains("grip_score"));
}

#[test]
fn render_carries_the_target_and_the_score_field() {
    // Arrange
    let report = sample();

    // Act
    let json = rendered(&report);

    // Assert
    assert!(json.contains("grip_score"));
    assert!(json.contains("\"test\""));
}

#[test]
fn render_produces_valid_json_with_the_overall_score_and_the_modules() {
    // Arrange
    let report = dummy_report();

    // Act
    let parsed = parsed(&report);

    // Assert
    assert_eq!(parsed["overall"]["grip_score"], 71);
    assert!(parsed.get("modules").is_some());
}

#[test]
fn render_writes_a_functions_fields_under_their_names() {
    // Arrange
    let report = report_with(
        OverallStats::default(),
        vec![],
        vec![],
        vec![FunctionInfo {
            name: "compute".to_string(),
            file: "src/main.rs".to_string(),
            has_trait_seam: true,
            grip_absolute: 0.85,
            grip_normalized: 85,
            ..Default::default()
        }],
    );

    // Act
    let parsed = parsed(&report);

    // Assert
    assert_eq!(parsed["functions"][0]["name"], "compute");
    assert_eq!(parsed["functions"][0]["has_trait_seam"], true);
}

#[test]
fn render_writes_a_modules_fields_under_their_names() {
    // Arrange
    let report = report_with(
        OverallStats::default(),
        vec![ModuleStats {
            path: "mod".to_string(),
            grip_score: Some(75),
            pure_ratio: 0.8,
            public_items: 5,
            total_functions: 10,
            pure_functions: 8,
            public_ratio: 0.5,
            grip_absolute_total: 8.0,
            ..Default::default()
        }],
        vec![],
        vec![],
    );

    // Act
    let parsed = parsed(&report);

    // Assert
    assert_eq!(parsed["modules"][0]["path"], "mod");
    assert_eq!(parsed["modules"][0]["grip_score"], 75);
}

#[test]
fn render_writes_an_offenders_fields_under_their_names() {
    // Arrange
    let report = report_with(
        OverallStats::default(),
        vec![],
        vec![Offender {
            path: "worse_module".to_string(),
            grip_score: 10,
        }],
        vec![],
    );

    // Act
    let parsed = parsed(&report);

    // Assert
    assert_eq!(parsed["offenders"][0]["path"], "worse_module");
    assert_eq!(parsed["offenders"][0]["grip_score"], 10);
}

#[test]
fn render_writes_the_overall_fields_under_their_names() {
    // Arrange
    let report = report_with(
        OverallStats {
            grip_score: Some(60),
            public_items: 5,
            total_functions: 10,
            pure_functions: 5,
            pure_ratio: 0.5,
            public_ratio: 0.5,
            grip_absolute_total: 5.0,
            ..Default::default()
        },
        vec![],
        vec![],
        vec![],
    );

    // Act
    let parsed = parsed(&report);

    // Assert
    assert_eq!(parsed["overall"]["grip_score"], 60);
    assert_eq!(parsed["overall"]["total_functions"], 10);
}

#[test]
fn render_writes_the_version() {
    // Arrange
    let report = dummy_report();

    // Act
    let parsed = parsed(&report);

    // Assert
    assert_eq!(parsed["version"], "0.1.0");
}

#[test]
fn render_writes_the_version_and_the_overall_score_under_their_names() {
    // Arrange
    let report = sample();

    // Act
    let parsed = parsed(&report);

    // Assert
    assert_eq!(parsed["version"], "0.1.0");
    assert_eq!(parsed["overall"]["grip_score"], 50);
}
