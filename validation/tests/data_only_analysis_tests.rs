// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::data_only_analysis::DataOnlyAnalysis;

#[test]
fn analyze_zero_function_module_has_null_score() {
    // Arrange & Act
    let report = DataOnlyAnalysis::report();

    // Assert
    assert!(report["overall"]["grip_score"].is_null());
}

#[test]
fn analyze_zero_function_module_is_not_an_offender() {
    // Arrange & Act
    let report = DataOnlyAnalysis::report();
    let offenders = report["offenders"].as_array().unwrap();

    // Assert
    assert!(
        offenders.is_empty(),
        "zero-function module should never be flagged as an offender, got {offenders:?}"
    );
}

#[test]
fn analyze_zero_function_module_still_reports_public_items() {
    // Arrange & Act
    let report = DataOnlyAnalysis::report();
    let public_items = report["overall"]["public_items"].as_u64().unwrap();

    // Assert
    assert!(
        public_items > 0,
        "public struct/enum items should still be counted even with no functions"
    );
}
