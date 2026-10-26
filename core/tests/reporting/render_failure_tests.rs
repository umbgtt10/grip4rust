// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::not_read::NotRead;
use grip::reporting::render_failure::RenderFailure;
use std::error::Error;

#[test]
fn source_returns_the_cause() {
    // Arrange
    let failure = RenderFailure {
        cause: Box::new(NotRead),
    };

    // Act
    let cause = failure.source().map(ToString::to_string);

    // Assert
    assert_eq!(cause, Some(String::from("not read")));
}

#[test]
fn to_string_says_the_report_could_not_be_rendered() {
    // Arrange
    let failure = RenderFailure {
        cause: Box::new(NotRead),
    };

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "failed to render the report");
}
