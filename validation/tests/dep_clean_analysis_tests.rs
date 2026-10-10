// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::dep_clean_analysis::DepCleanAnalysis;

#[test]
fn clean_module_has_high_contribution() {
    // Arrange & Act
    let report = DepCleanAnalysis::report();
    let avg = report["overall"]["avg_contribution"].as_f64().unwrap();

    // Assert
    assert!(
        avg >= 0.75,
        "expected high avg contribution >= 0.75, got {avg}"
    );
}

#[test]
fn clean_module_has_high_score() {
    // Arrange & Act
    let report = DepCleanAnalysis::report();
    let score = report["overall"]["grip_score"].as_u64().unwrap();

    // Assert
    assert!(score >= 60, "expected decent score >= 60, got {score}");
}

#[test]
fn clean_module_has_no_zero_functions() {
    // Arrange & Act
    let report = DepCleanAnalysis::report();
    let functions = report["functions"].as_array().unwrap();

    // Assert
    for f in functions {
        let deps = f["hidden_deps"].as_u64().unwrap();
        assert_eq!(deps, 0, "function {} should have 0 hidden deps", f["name"]);
    }
}
