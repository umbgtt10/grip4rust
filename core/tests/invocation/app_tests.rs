// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::fs_walk::FsWalk;
use grip::invocation::app::App;
use grip::invocation::config::Config;
use grip::invocation::no_op_cache_store::NoOpCacheStore;
use grip::reporting::default_scorer::DefaultScorer;
use grip::reporting::stdout_reporter::StdoutReporter;
use std::fs;
use std::process::ExitCode;
use tempfile::TempDir;

#[test]
fn run_on_empty_dir_errors() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let config = Config {
        path: dir.path().to_path_buf(),
        json: false,
        threshold: None,
        verbose: false,
    };
    let app = App::new(config);

    // Act
    let result = app.run();

    // Assert
    assert!(result.is_err());
}

#[test]
fn run_on_valid_dir_succeeds() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(
        src.join("lib.rs"),
        "pub fn greet() -> &'static str { \"hello\" }\n",
    )
    .unwrap();
    let config = Config {
        path: dir.path().to_path_buf(),
        json: false,
        threshold: None,
        verbose: false,
    };
    let app = App::new(config);

    // Act
    let exit_code = app.run().unwrap();

    // Assert
    assert_eq!(exit_code, ExitCode::SUCCESS);
}

#[test]
fn with_deps_builds_an_app_that_runs_on_the_injected_dependencies() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(
        src.join("lib.rs"),
        "pub fn greet() -> &'static str { \"hello\" }\n",
    )
    .unwrap();
    let config = Config {
        path: dir.path().to_path_buf(),
        json: true,
        threshold: None,
        verbose: false,
    };

    // Act
    let app = App::with_deps(
        Box::new(FsWalk::new(&config.path)),
        Box::new(DefaultScorer::new()),
        Box::new(StdoutReporter::new(true, false)),
        Box::new(NoOpCacheStore::new()),
        config,
    );

    // Assert
    assert_eq!(
        format!("{:?}", app.run().unwrap()),
        format!("{:?}", ExitCode::SUCCESS)
    );
}
