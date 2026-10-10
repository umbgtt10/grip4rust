// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::dep_injected_analysis::DepInjectedAnalysis;

#[test]
fn injected_module_has_high_score() {
    // Arrange & Act
    let report = DepInjectedAnalysis::report();
    let score = report["overall"]["grip_score"].as_u64().unwrap();

    // Assert
    assert!(score >= 70, "injected code should score >= 70, got {score}");
}

#[test]
fn injected_module_zero_hidden_deps() {
    // Arrange & Act
    let report = DepInjectedAnalysis::report();
    let functions = report["functions"].as_array().unwrap();

    // Assert
    for f in functions {
        let deps = f["hidden_deps"].as_u64().unwrap();
        assert_eq!(deps, 0, "function {} should have 0 hidden deps", f["name"]);
    }
}
