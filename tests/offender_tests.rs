// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::offender::Offender;

#[test]
fn offender_serializes_to_json() {
    // Arrange
    let offender = Offender {
        path: "bad_module".to_string(),
        grip_score: 30,
    };

    // Act
    let json = serde_json::to_string(&offender).unwrap();

    // Assert
    assert!(json.contains("bad_module"));
    assert!(json.contains("30"));
}

#[test]
fn offender_deserializes_from_json() {
    // Arrange
    let json = r#"{"path":"worse_module","grip_score":10}"#;

    // Act
    let offender: Offender = serde_json::from_str(json).unwrap();

    // Assert
    assert_eq!(offender.path, "worse_module");
    assert_eq!(offender.grip_score, 10);
}
