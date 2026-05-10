// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::args::Args;
use grip::config::Config;

#[test]
fn from_args_preserves_path() {
    // Arrange
    let args = Args::parse_from_args(vec!["cargo-grip", "my-project"]);

    // Act
    let config = Config::from_args(args);

    // Assert
    assert_eq!(config.path.to_string_lossy(), "my-project");
}

#[test]
fn from_args_preserves_json() {
    // Arrange
    let args = Args::parse_from_args(vec!["cargo-grip", "--json"]);

    // Act
    let config = Config::from_args(args);

    // Assert
    assert_eq!(config.json, true);
}

#[test]
fn from_args_preserves_min_score() {
    // Arrange
    let args = Args::parse_from_args(vec!["cargo-grip", "--min-score", "42"]);

    // Act
    let config = Config::from_args(args);

    // Assert
    assert_eq!(config.min_score, Some(42));
}
