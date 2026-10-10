// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::sloppy_calc_analysis::SloppyCalcAnalysis;

#[test]
fn analyze_a_sloppy_calculator_scores_below_fifty() {
    // Arrange & Act
    let parsed = SloppyCalcAnalysis::report();

    // Assert
    let grip_score = parsed["overall"]["grip_score"].as_u64().unwrap();
    assert!(
        grip_score < 50,
        "expected bad score < 50, got {}",
        grip_score
    );
}

#[test]
fn has_few_public_items() {
    // Arrange & Act
    let parsed = SloppyCalcAnalysis::report();

    // Assert
    let public_items = parsed["overall"]["public_items"].as_u64().unwrap();
    assert!(
        public_items < 6,
        "expected few public items, got {}",
        public_items
    );
}

#[test]
fn low_pure_ratio() {
    // Arrange & Act
    let parsed = SloppyCalcAnalysis::report();

    // Assert
    let pure_ratio = parsed["overall"]["pure_ratio"].as_f64().unwrap();
    assert!(
        pure_ratio < 0.7,
        "expected low pure ratio, got {}",
        pure_ratio
    );
}
