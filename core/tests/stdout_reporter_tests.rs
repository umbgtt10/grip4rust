// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::function_info::FunctionInfo;
use grip::grip_report::GripReport;
use grip::module_stats::ModuleStats;
use grip::offender::Offender;
use grip::overall_stats::OverallStats;
use grip::stdout_reporter::StdoutReporter;
use grip::traits::reporter::Reporter;

fn dummy_report() -> GripReport {
    GripReport {
        version: "0.1.0".to_string(),
        target: "my-crate".to_string(),
        overall: OverallStats {
            grip_score: Some(71),
            public_items: 10,
            total_functions: 20,
            pure_functions: 12,
            pure_ratio: 0.6,
            public_ratio: 0.5,
            inherent_methods: 0,
            local_trait_methods: 0,
            trait_ratio: 0.0,
            avg_contribution: 0.0,
            clean_fn_ratio: 0.0,
            grip_absolute_total: 12.0,
        },
        modules: vec![
            ModuleStats {
                path: "alpha".to_string(),
                grip_score: Some(80),
                pure_ratio: 0.8,
                public_items: 5,
                total_functions: 10,
                pure_functions: 8,
                public_ratio: 0.5,
                inherent_methods: 0,
                local_trait_methods: 0,
                trait_ratio: 0.0,
                avg_contribution: 0.0,
                clean_fn_ratio: 0.0,
                grip_absolute_total: 7.0,
            },
            ModuleStats {
                path: "beta".to_string(),
                grip_score: Some(50),
                pure_ratio: 0.4,
                public_items: 5,
                total_functions: 10,
                pure_functions: 4,
                public_ratio: 0.5,
                inherent_methods: 0,
                local_trait_methods: 0,
                trait_ratio: 0.0,
                avg_contribution: 0.0,
                clean_fn_ratio: 0.0,
                grip_absolute_total: 5.0,
            },
        ],
        offenders: vec![],
        offender_threshold: 50,
        functions: vec![],
    }
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
    let report = GripReport {
        functions: vec![FunctionInfo {
            name: "compute".to_string(),
            file: "src/lib.rs".to_string(),
            is_pure: true,
            is_public: true,
            hidden_deps: 0,
            has_trait_seam: false,
            dep_weight: 0.0,
            hidden_dep_labels: vec![],
            grip_absolute: 0.95,
            grip_normalized: 95,
        }],
        ..dummy_report()
    };
    let reporter = reporter(false);

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    assert!(!out.contains("Per-function detail"));
}

#[test]
fn human_output_shows_offenders_section() {
    // Arrange
    let report = GripReport {
        modules: vec![ModuleStats {
            path: "bad_mod".to_string(),
            grip_score: Some(30),
            pure_ratio: 0.3,
            public_items: 1,
            total_functions: 5,
            pure_functions: 1,
            public_ratio: 0.2,
            inherent_methods: 0,
            local_trait_methods: 0,
            trait_ratio: 0.0,
            avg_contribution: 0.0,
            clean_fn_ratio: 0.0,
            grip_absolute_total: 2.0,
        }],
        offenders: vec![Offender {
            path: "bad_mod".to_string(),
            grip_score: 30,
        }],
        offender_threshold: 50,
        functions: vec![],
        ..dummy_report()
    };
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
    let report = GripReport {
        functions: vec![FunctionInfo {
            name: "compute".to_string(),
            file: "src/lib.rs".to_string(),
            is_pure: true,
            is_public: true,
            hidden_deps: 0,
            has_trait_seam: false,
            dep_weight: 0.0,
            hidden_dep_labels: vec![],
            grip_absolute: 0.95,
            grip_normalized: 95,
        }],
        ..dummy_report()
    };
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
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
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
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(parsed["overall"]["grip_score"], 71);
    assert!(parsed.get("modules").is_some());
}
