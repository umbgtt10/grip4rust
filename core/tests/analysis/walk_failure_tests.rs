// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::not_read::NotRead;
use grip::analysis::walk_failure::WalkFailure;
use std::error::Error;

#[test]
fn source_returns_the_cause() {
    // Arrange
    let failure = WalkFailure {
        path: String::from("src/lib.rs"),
        cause: Box::new(NotRead),
    };

    // Act
    let cause = failure.source().map(ToString::to_string);

    // Assert
    assert_eq!(cause, Some(String::from("not read")));
}

#[test]
fn to_string_names_the_file_it_could_not_read() {
    // Arrange
    let failure = WalkFailure {
        path: String::from("src/lib.rs"),
        cause: Box::new(NotRead),
    };

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "failed to read src/lib.rs");
}
