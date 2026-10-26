// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::invocation::run_outcome::RunOutcome;
use grip_cli::invocation::args::Args;
use grip_cli::invocation::entry_point::EntryPoint;
use serde_json::Value;
use serde_json::from_str;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use tempfile::TempDir;

fn args_for(dir: &TempDir, flags: &[&str]) -> Args {
    let mut argv = vec![
        OsString::from("cargo-grip4rust"),
        dir.path().as_os_str().to_os_string(),
    ];
    argv.extend(flags.iter().map(OsString::from));
    Args::parse_from_args(argv)
}

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
    let args = args_for(&dir, &[]);

    // Act
    let outcome = EntryPoint::compose(&args).run().unwrap();

    // Assert
    let name = dir.path().file_name().unwrap().to_string_lossy();
    let header = format!("cargo-grip4rust {} -- {name}\n", env!("CARGO_PKG_VERSION"));
    assert!(outcome.output.starts_with(&header));
}

#[test]
fn compose_on_an_empty_dir_builds_an_app_whose_run_errors() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let args = args_for(&dir, &[]);

    // Act
    let result = EntryPoint::compose(&args).run();

    // Assert
    assert!(result.is_err());
}

#[test]
fn compose_with_json_renders_the_report_as_json() {
    // Arrange
    let dir = TempDir::new().unwrap();
    write_project(&dir, "pub fn greet() -> &'static str { \"hello\" }\n");
    let args = args_for(&dir, &["--json"]);

    // Act
    let outcome = EntryPoint::compose(&args).run().unwrap();

    // Assert
    let parsed: Value = from_str(&outcome.output).unwrap();
    assert_eq!(parsed["overall"]["total_functions"], 1);
}

#[test]
fn compose_with_verbose_renders_the_per_function_detail() {
    // Arrange
    let dir = TempDir::new().unwrap();
    write_project(&dir, "pub fn greet() -> &'static str { \"hello\" }\n");
    let args = args_for(&dir, &["--verbose"]);

    // Act
    let outcome = EntryPoint::compose(&args).run().unwrap();

    // Assert
    assert!(outcome.output.contains("Per-function detail"));
}

#[test]
fn finish_at_or_above_the_threshold_returns_success() {
    // Arrange & Act
    let exit_code = EntryPoint::finish(RunOutcome {
        output: String::new(),
        below_threshold: false,
    })
    .unwrap();

    // Assert
    assert_eq!(exit_code, ExitCode::SUCCESS);
}

#[test]
fn finish_below_the_threshold_returns_failure() {
    // Arrange & Act
    let exit_code = EntryPoint::finish(RunOutcome {
        output: String::new(),
        below_threshold: true,
    })
    .unwrap();

    // Assert
    assert_eq!(exit_code, ExitCode::FAILURE);
}

#[test]
fn finish_with_a_report_returns_success() {
    // Arrange & Act
    let exit_code = EntryPoint::finish(RunOutcome {
        output: String::from("report\n"),
        below_threshold: false,
    })
    .unwrap();

    // Assert
    assert_eq!(exit_code, ExitCode::SUCCESS);
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
