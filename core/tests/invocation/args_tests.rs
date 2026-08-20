// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::invocation::args::Args;
use std::ffi::OsString;

#[test]
fn default_path_is_dot() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip4rust"]);

    // Assert
    assert_eq!(args.path.to_string_lossy(), ".");
}

#[test]
fn json_flag_is_false_by_default() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip4rust"]);

    // Assert
    assert_eq!(args.json, false);
}

#[test]
fn json_flag_is_parsed() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip4rust", "--json"]);

    // Assert
    assert_eq!(args.json, true);
}

#[test]
fn min_score_alias_still_works() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip4rust", "--min-score", "30"]);

    // Assert
    assert_eq!(args.threshold, Some(30));
}

#[test]
fn path_arg_is_parsed() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip4rust", "some/path"]);

    // Assert
    assert_eq!(args.path.to_string_lossy(), "some/path");
}

#[test]
fn threshold_is_none_by_default() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip4rust"]);

    // Assert
    assert_eq!(args.threshold, None);
}

#[test]
fn threshold_is_parsed() {
    // Arrange & Act
    let args = Args::parse_from_args(vec!["cargo-grip4rust", "--threshold", "50"]);

    // Assert
    assert_eq!(args.threshold, Some(50));
}

#[test]
fn without_cargo_subcommand_drops_the_name_cargo_repeats() {
    // Arrange
    let raw = ["cargo-grip4rust", "grip4rust", "--json"]
        .map(OsString::from)
        .to_vec();

    // Act
    let forwarded = Args::without_cargo_subcommand(raw);

    // Assert
    assert_eq!(forwarded, ["cargo-grip4rust", "--json"].map(OsString::from));
}

#[test]
fn without_cargo_subcommand_keeps_a_path_named_after_the_tool() {
    // Arrange
    let raw = ["cargo-grip4rust", "grip4rust", "grip4rust"]
        .map(OsString::from)
        .to_vec();

    // Act
    let forwarded = Args::without_cargo_subcommand(raw);

    // Assert
    assert_eq!(
        forwarded,
        ["cargo-grip4rust", "grip4rust"].map(OsString::from)
    );
}

#[test]
fn without_cargo_subcommand_leaves_a_direct_invocation_untouched() {
    // Arrange
    let raw = ["cargo-grip4rust", "--json"].map(OsString::from).to_vec();

    // Act
    let forwarded = Args::without_cargo_subcommand(raw);

    // Assert
    assert_eq!(forwarded, ["cargo-grip4rust", "--json"].map(OsString::from));
}

#[test]
fn without_cargo_subcommand_on_a_bare_binary_name_returns_it_unchanged() {
    // Arrange
    let raw = vec![OsString::from("cargo-grip4rust")];

    // Act
    let forwarded = Args::without_cargo_subcommand(raw);

    // Assert
    assert_eq!(forwarded, [OsString::from("cargo-grip4rust")]);
}
