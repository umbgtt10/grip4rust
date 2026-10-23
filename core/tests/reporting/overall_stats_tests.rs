// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::reporting::overall_stats::OverallStats;
use serde_json::from_str;
use serde_json::to_string;

#[test]
fn grip_absolute_total_returns_the_summed_contribution() {
    // Arrange
    let stats = OverallStats::default().with_grip_absolute_total(2.5);

    // Act
    let total = stats.grip_absolute_total();

    // Assert
    assert_eq!(total, 2.5);
}

#[test]
fn overall_stats_deserializes_from_json() {
    // Arrange
    let json = r#"{"grip_score":60,"public_items":5,"total_functions":10,"pure_functions":5,"pure_ratio":0.5,"public_ratio":0.5,"inherent_methods":0,"local_trait_methods":0,"trait_ratio":0.0,"avg_contribution":0.0,"clean_fn_ratio":0.0,"grip_absolute_total":5.0}"#;

    // Act
    let stats: OverallStats = from_str(json).unwrap();

    // Assert
    assert_eq!(stats.grip_score(), Some(60));
    assert_eq!(stats.total_functions(), 10);
}

#[test]
fn overall_stats_serializes_to_json() {
    // Arrange
    let stats = OverallStats::default()
        .with_grip_score(Some(80))
        .with_public_items(10)
        .with_total_functions(20)
        .with_pure_functions(15)
        .with_pure_ratio(0.75)
        .with_public_ratio(0.5)
        .with_grip_absolute_total(15.0);

    // Act
    let json = to_string(&stats).unwrap();

    // Assert
    assert!(json.contains("80"));
    assert!(json.contains("grip_score"));
}
