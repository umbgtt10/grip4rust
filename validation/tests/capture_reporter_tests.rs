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
    GripReport::new(
        "0.1.0".to_string(),
        "sample".to_string(),
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
