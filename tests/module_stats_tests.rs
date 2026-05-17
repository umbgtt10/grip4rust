// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::module_stats::ModuleStats;

#[test]
fn module_stats_serializes_to_json() {
    // Arrange
    let stats = ModuleStats {
        path: "test".to_string(),
        grip_score: 42,
        pure_ratio: 0.5,
        public_items: 3,
        total_functions: 10,
        pure_functions: 5,
        public_ratio: 0.3,
        inherent_methods: 0,
        local_trait_methods: 0,
        trait_ratio: 0.0,
    };

    // Act
    let json = serde_json::to_string(&stats).unwrap();

    // Assert
    assert!(json.contains("test"));
    assert!(json.contains("42"));
}

#[test]
fn module_stats_deserializes_from_json() {
    // Arrange
    let json = r#"{"path":"mod","grip_score":75,"pure_ratio":0.8,"public_items":5,"total_functions":10,"pure_functions":8,"public_ratio":0.5,"inherent_methods":0,"local_trait_methods":0,"trait_ratio":0.0}"#;

    // Act
    let stats: ModuleStats = serde_json::from_str(json).unwrap();

    // Assert
    assert_eq!(stats.path, "mod");
    assert_eq!(stats.grip_score, 75);
}
