// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::invocation::config::Config;
use grip::invocation::entry_point::EntryPoint;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use tempfile::TempDir;

fn fixture_path(name: &str) -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("fixture")
        .join(name)
        .to_string_lossy()
        .to_string()
}

fn write_project(dir: &TempDir, source: &str) {
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), source).unwrap();
}

#[test]
fn compose_on_a_valid_dir_builds_an_app_whose_run_succeeds() {
    // Arrange
    let dir = TempDir::new().unwrap();
    write_project(&dir, "pub fn greet() -> &'static str { \"hello\" }\n");
    let config = Config::new(dir.path().to_path_buf(), false, None, false);

    // Act
    let exit_code = EntryPoint::compose(config).run().unwrap();

    // Assert
    assert_eq!(exit_code, ExitCode::SUCCESS);
}

#[test]
fn compose_on_an_empty_dir_builds_an_app_whose_run_errors() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let config = Config::new(dir.path().to_path_buf(), false, None, false);

    // Act
    let result = EntryPoint::compose(config).run();

    // Assert
    assert!(result.is_err());
}

#[test]
fn run_from_args_empty_dir_errors() {
    // Arrange
    let dir = TempDir::new().unwrap();

    // Act
    let result = EntryPoint::run_from_args(vec!["cargo-grip4rust", &dir.path().to_string_lossy()]);

    // Assert
    assert!(result.is_err());
}

#[test]
fn run_from_args_injected_outperforms_monolith() {
    // Arrange & Act
    let clean = EntryPoint::run_from_args(vec![
        "cargo-grip4rust",
        &fixture_path("dep_injected"),
        "--threshold",
        "70",
    ]);
    let mono = EntryPoint::run_from_args(vec![
        "cargo-grip4rust",
        &fixture_path("dep_monolith"),
        "--threshold",
        "50",
    ]);

    // Assert
    assert_eq!(
        clean.unwrap(),
        ExitCode::SUCCESS,
        "injected should score >= 70"
    );
    assert_ne!(
        mono.unwrap(),
        ExitCode::SUCCESS,
        "monolith should score < 50"
    );
}

#[test]
fn run_from_args_threshold_fails() {
    // Arrange
    let dir = TempDir::new().unwrap();
    write_project(&dir, "fn greet() -> &'static str { \"hello\" }\n");

    // Act
    let result = EntryPoint::run_from_args(vec![
        "cargo-grip4rust",
        &dir.path().to_string_lossy(),
        "--threshold",
        "100",
    ]);

    // Assert
    assert_eq!(result.unwrap(), ExitCode::FAILURE);
}

#[test]
fn run_from_args_threshold_passes() {
    // Arrange
    let dir = TempDir::new().unwrap();
    write_project(&dir, "pub fn greet() -> &'static str { \"hello\" }\n");

    // Act
    let result = EntryPoint::run_from_args(vec![
        "cargo-grip4rust",
        &dir.path().to_string_lossy(),
        "--threshold",
        "0",
    ]);

    // Assert
    assert_eq!(result.unwrap(), ExitCode::SUCCESS);
}

#[test]
fn run_from_args_valid_dir_succeeds() {
    // Arrange
    let dir = TempDir::new().unwrap();
    write_project(&dir, "pub fn greet() -> &'static str { \"hello\" }\n");

    // Act
    let result = EntryPoint::run_from_args(vec!["cargo-grip4rust", &dir.path().to_string_lossy()]);

    // Assert
    assert_eq!(result.unwrap(), ExitCode::SUCCESS);
}

#[test]
fn run_on_an_empty_dir_returns_failure() {
    // Arrange
    let dir = TempDir::new().unwrap();

    // Act
    let exit_code = EntryPoint::run(vec![
        OsString::from("cargo-grip4rust"),
        dir.path().as_os_str().to_os_string(),
    ]);

    // Assert
    assert_eq!(exit_code, ExitCode::FAILURE);
}

#[test]
fn run_with_the_cargo_subcommand_repeated_returns_success() {
    // Arrange
    let dir = TempDir::new().unwrap();
    write_project(&dir, "pub fn greet() -> &'static str { \"hello\" }\n");

    // Act
    let exit_code = EntryPoint::run(vec![
        OsString::from("cargo-grip4rust"),
        OsString::from("grip4rust"),
        dir.path().as_os_str().to_os_string(),
        OsString::from("--threshold"),
        OsString::from("0"),
    ]);

    // Assert
    assert_eq!(exit_code, ExitCode::SUCCESS);
}
