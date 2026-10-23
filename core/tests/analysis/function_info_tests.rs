// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::function_info::FunctionInfo;
use serde_json::from_str;
use serde_json::to_string;

fn sample() -> FunctionInfo {
    FunctionInfo::default()
        .with_name("process".to_string())
        .with_file("src/lib.rs".to_string())
        .with_is_pure(true)
        .with_is_public(true)
        .with_hidden_deps(1)
        .with_dep_weight(0.2)
        .with_hidden_dep_labels(vec!["println".to_string()])
        .with_grip_absolute(0.8)
        .with_grip_normalized(80)
}

#[test]
fn file_returns_the_file_the_function_is_in() {
    // Arrange
    let info = sample();

    // Act
    let file = info.file();

    // Assert
    assert_eq!(file, "src/lib.rs");
}

#[test]
fn function_info_deserializes_from_json() {
    // Arrange
    let json = r#"{"name":"compute","file":"src/main.rs","is_pure":false,"is_public":false,"hidden_deps":0,"has_trait_seam":true,"dep_weight":0.0,"hidden_dep_labels":[],"grip_absolute":0.85,"grip_normalized":85}"#;

    // Act
    let info: FunctionInfo = from_str(json).unwrap();

    // Assert
    assert_eq!(info.name(), "compute");
    assert!(info.has_trait_seam());
}

#[test]
fn function_info_serializes_to_json() {
    // Arrange
    let info = sample();

    // Act
    let json = to_string(&info).unwrap();

    // Assert
    assert!(json.contains("process"));
    assert!(json.contains("println"));
}

#[test]
fn with_has_trait_seam_marks_the_function_as_behind_a_trait() {
    // Arrange
    let info = sample();

    // Act
    let info = info.with_has_trait_seam(true);

    // Assert
    assert!(info.has_trait_seam());
}
