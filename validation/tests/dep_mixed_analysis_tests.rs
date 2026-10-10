// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::dep_mixed_analysis::DepMixedAnalysis;

#[test]
fn mixed_has_all_eight_cases() {
    // Arrange & Act
    let report = DepMixedAnalysis::report();
    let functions = report["functions"].as_array().unwrap();

    // Assert
    assert_eq!(functions.len(), 8, "should have 8 functions (1 per case)");
}

#[test]
fn mixed_has_clean_and_dirty_functions() {
    // Arrange & Act
    let report = DepMixedAnalysis::report();
    let overall = &report["overall"];
    let avg = overall["avg_contribution"].as_f64().unwrap();

    // Assert
    assert!(avg > 0.0, "avg contribution should be > 0");
    assert!(
        avg < 1.0,
        "avg contribution should be < 1.0 (mix of clean and dirty)"
    );
}

#[test]
fn mixed_hidden_deps_count_correct() {
    // Arrange & Act
    let report = DepMixedAnalysis::report();
    let functions = report["functions"].as_array().unwrap();

    // Assert
    let mut total_deps = 0u64;
    for f in functions {
        total_deps += f["hidden_deps"].as_u64().unwrap();
    }
    assert!(
        total_deps >= 4,
        "should have at least 4 hidden deps across all functions"
    );
}
