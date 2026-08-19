// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::function_info::FunctionInfo;

fn sample() -> FunctionInfo {
    FunctionInfo {
        name: "process".to_string(),
        file: "src/lib.rs".to_string(),
        is_pure: true,
        is_public: true,
        hidden_deps: 1,
        has_trait_seam: false,
        dep_weight: 0.2,
        hidden_dep_labels: vec!["println".to_string()],
        grip_absolute: 0.8,
        grip_normalized: 80,
    }
}

#[test]
fn function_info_deserializes_from_json() {
    // Arrange
    let json = r#"{"name":"compute","file":"src/main.rs","is_pure":false,"is_public":false,"hidden_deps":0,"has_trait_seam":true,"dep_weight":0.0,"hidden_dep_labels":[],"grip_absolute":0.85,"grip_normalized":85}"#;

    // Act
    let info: FunctionInfo = serde_json::from_str(json).unwrap();

    // Assert
    assert_eq!(info.name, "compute");
    assert!(info.has_trait_seam);
}

#[test]
fn function_info_serializes_to_json() {
    // Arrange
    let info = sample();

    // Act
    let json = serde_json::to_string(&info).unwrap();

    // Assert
    assert!(json.contains("process"));
    assert!(json.contains("println"));
}
