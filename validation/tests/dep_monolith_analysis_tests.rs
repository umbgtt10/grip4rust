// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::dep_monolith_analysis::DepMonolithAnalysis;

#[test]
fn monolith_all_functions_have_hidden_deps() {
    // Arrange & Act
    let report = DepMonolithAnalysis::report();
    let functions = report["functions"].as_array().unwrap();

    // Assert
    for f in functions {
        let deps = f["hidden_deps"].as_u64().unwrap();
        assert!(
            deps >= 1,
            "function {} should have >=1 hidden dep",
            f["name"]
        );
    }
}

#[test]
fn monolith_has_low_score() {
    // Arrange & Act
    let report = DepMonolithAnalysis::report();
    let score = report["overall"]["grip_score"].as_u64().unwrap();

    // Assert
    assert!(
        score <= 35,
        "monolith should have low score <= 35, got {score}"
    );
}

#[test]
fn monolith_has_zero_clean_functions() {
    // Arrange & Act
    let report = DepMonolithAnalysis::report();
    let clean = report["overall"]["clean_fn_ratio"].as_f64().unwrap();

    // Assert
    assert_eq!(clean, 0.0, "monolith should have 0 clean functions");
}
