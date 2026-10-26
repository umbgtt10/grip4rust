// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::not_read::NotRead;
use grip::analysis::walk_failure::WalkFailure;
use grip::invocation::run_failure::RunFailure;
use grip::reporting::render_failure::RenderFailure;
use std::error::Error;

fn walk_failure() -> WalkFailure {
    WalkFailure {
        path: String::from("src/lib.rs"),
        cause: Box::new(NotRead),
    }
}

#[test]
fn from_a_render_failure_wraps_it() {
    // Arrange
    let failure = RenderFailure {
        cause: Box::new(NotRead),
    };

    // Act
    let wrapped = RunFailure::from(failure);

    // Assert
    assert!(matches!(wrapped, RunFailure::Render(_)));
}

#[test]
fn from_a_walk_failure_wraps_it() {
    // Arrange & Act
    let wrapped = RunFailure::from(walk_failure());

    // Assert
    assert!(matches!(wrapped, RunFailure::Walk(_)));
}

#[test]
fn source_of_a_missing_source_tree_is_none() {
    // Arrange
    let failure = RunFailure::NoSourceFiles(String::from("some/root"));

    // Act
    let cause = failure.source();

    // Assert
    assert!(cause.is_none());
}

#[test]
fn source_of_a_render_failure_is_its_cause() {
    // Arrange
    let failure = RunFailure::Render(RenderFailure {
        cause: Box::new(NotRead),
    });

    // Act
    let cause = failure.source().map(ToString::to_string);

    // Assert
    assert_eq!(cause, Some(String::from("not read")));
}

#[test]
fn source_of_a_walk_failure_is_its_cause() {
    // Arrange
    let failure = RunFailure::Walk(walk_failure());

    // Act
    let cause = failure.source().map(ToString::to_string);

    // Assert
    assert_eq!(cause, Some(String::from("not read")));
}

#[test]
fn to_string_of_a_missing_source_tree_names_the_root() {
    // Arrange
    let failure = RunFailure::NoSourceFiles(String::from("some/root"));

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "no Rust source files found in some/root");
}

#[test]
fn to_string_of_a_render_failure_says_it() {
    // Arrange
    let failure = RunFailure::Render(RenderFailure {
        cause: Box::new(NotRead),
    });

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "failed to render the report");
}

#[test]
fn to_string_of_a_walk_failure_names_the_file() {
    // Arrange
    let failure = RunFailure::Walk(walk_failure());

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "failed to read src/lib.rs");
}
