// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::reporting::grip_report::GripReport;
use grip::reporting::overall_stats::OverallStats;
use grip::traits::reporter::Reporter;
use std::cell::RefCell;
use std::rc::Rc;
use validation::capture_reporter::CaptureReporter;

fn sample() -> GripReport {
    GripReport {
        version: "0.1.0".to_string(),
        target: "sample".to_string(),
        overall: OverallStats {
            grip_score: Some(50),
            public_items: 1,
            total_functions: 2,
            pure_functions: 1,
            pure_ratio: 0.5,
            public_ratio: 0.5,
            inherent_methods: 0,
            local_trait_methods: 0,
            trait_ratio: 0.0,
            avg_contribution: 0.0,
            clean_fn_ratio: 0.0,
            grip_absolute_total: 1.5,
        },
        modules: vec![],
        offenders: vec![],
        offender_threshold: 50,
        functions: vec![],
    }
}

#[test]
fn render_a_report_captures_the_json_it_returns() {
    // Arrange
    let captured = Rc::new(RefCell::new(String::new()));
    let reporter = CaptureReporter::new(Rc::clone(&captured));

    // Act
    let json = reporter.render(&sample()).unwrap();

    // Assert
    assert_eq!(*captured.borrow(), json);
}

#[test]
fn write_a_report_captures_it_as_pretty_json() {
    // Arrange
    let captured = Rc::new(RefCell::new(String::new()));
    let reporter = CaptureReporter::new(Rc::clone(&captured));

    // Act
    reporter.write(&sample()).unwrap();

    // Assert
    assert!(captured.borrow().contains("\"target\": \"sample\""));
}
