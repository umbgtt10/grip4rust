// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::clean_calc_analysis::CleanCalcAnalysis;
use validation::sloppy_calc_analysis::SloppyCalcAnalysis;

#[test]
fn analyze_a_clean_calculator_scores_between_forty_and_seventy() {
    // Arrange & Act
    let parsed = CleanCalcAnalysis::report();

    // Assert
    let grip_score = parsed["overall"]["grip_score"].as_u64().unwrap();
    assert!(
        grip_score >= 40,
        "expected decent score >= 40, got {}",
        grip_score
    );
    assert!(
        grip_score < 70,
        "expected imperfect score < 70, got {}",
        grip_score
    );
}

#[test]
fn has_many_public_items() {
    // Arrange & Act
    let parsed = CleanCalcAnalysis::report();

    // Assert
    let public_items = parsed["overall"]["public_items"].as_u64().unwrap();
    assert!(
        public_items >= 10,
        "expected many public items, got {}",
        public_items
    );
}

#[test]
fn high_pure_ratio() {
    // Arrange & Act
    let parsed = CleanCalcAnalysis::report();

    // Assert
    let pure_ratio = parsed["overall"]["pure_ratio"].as_f64().unwrap();
    assert!(
        pure_ratio >= 0.5,
        "expected reasonable pure ratio, got {}",
        pure_ratio
    );
}

#[test]
fn scores_higher_than_sloppy() {
    // Arrange & Act
    let clean = CleanCalcAnalysis::report();
    let sloppy = SloppyCalcAnalysis::report();

    // Assert
    let clean_score = clean["overall"]["grip_score"].as_u64().unwrap();
    let sloppy_score = sloppy["overall"]["grip_score"].as_u64().unwrap();
    assert!(
        clean_score > sloppy_score + 10,
        "expected clean ({}) to be > sloppy ({}) + 10",
        clean_score,
        sloppy_score,
    );
}
