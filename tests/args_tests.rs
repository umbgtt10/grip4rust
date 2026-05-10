// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::args::Args;

#[test]
fn default_path_is_dot() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip"]);

    // Assert
    assert_eq!(args.path.to_string_lossy(), ".");
}

#[test]
fn json_flag_is_false_by_default() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip"]);

    // Assert
    assert_eq!(args.json, false);
}

#[test]
fn min_score_is_none_by_default() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip"]);

    // Assert
    assert_eq!(args.min_score, None);
}

#[test]
fn path_arg_is_parsed() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip", "some/path"]);

    // Assert
    assert_eq!(args.path.to_string_lossy(), "some/path");
}

#[test]
fn json_flag_is_parsed() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip", "--json"]);

    // Assert
    assert_eq!(args.json, true);
}

#[test]
fn min_score_is_parsed() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip", "--min-score", "50"]);

    // Assert
    assert_eq!(args.min_score, Some(50));
}
