// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::reporting::module_stats::ModuleStats;
use serde_json::from_str;
use serde_json::to_string;

#[test]
fn grip_absolute_total_returns_the_summed_contribution() {
    // Arrange
    let stats = ModuleStats::default().with_grip_absolute_total(2.5);

    // Act
    let total = stats.grip_absolute_total();

    // Assert
    assert_eq!(total, 2.5);
}

#[test]
fn module_stats_deserializes_from_json() {
    // Arrange
    let json = r#"{"path":"mod","grip_score":75,"pure_ratio":0.8,"public_items":5,"total_functions":10,"pure_functions":8,"public_ratio":0.5,"inherent_methods":0,"local_trait_methods":0,"trait_ratio":0.0,"avg_contribution":0.0,"clean_fn_ratio":0.0,"grip_absolute_total":8.0}"#;

    // Act
    let stats: ModuleStats = from_str(json).unwrap();

    // Assert
    assert_eq!(stats.path(), "mod");
    assert_eq!(stats.grip_score(), Some(75));
}

#[test]
fn module_stats_serializes_to_json() {
    // Arrange
    let stats = ModuleStats::default()
        .with_path("test".to_string())
        .with_grip_score(Some(42))
        .with_pure_ratio(0.5)
        .with_public_items(3)
        .with_total_functions(10)
        .with_pure_functions(5)
        .with_public_ratio(0.3)
        .with_grip_absolute_total(7.5);

    // Act
    let json = to_string(&stats).unwrap();

    // Assert
    assert!(json.contains("test"));
    assert!(json.contains("42"));
}
