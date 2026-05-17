// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::cell::RefCell;
use std::path::PathBuf;

use anyhow::Result;

use grip::app::App;
use grip::config::Config;
use grip::default_scorer::DefaultScorer;
use grip::fs_walk::FsWalk;
use grip::grip_report::GripReport;
use grip::traits::reporter::Reporter;

struct CaptureReporter {
    captured: RefCell<String>,
}

impl Reporter for CaptureReporter {
    fn render(&self, report: &GripReport) -> Result<String> {
        let json = serde_json::to_string_pretty(report)?;
        *self.captured.borrow_mut() = json.clone();
        Ok(json)
    }

    fn write(&self, report: &GripReport) -> Result<()> {
        let json = self.render(report)?;
        print!("{json}");
        Ok(())
    }
}

fn analyze() -> serde_json::Value {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("dep_mixed");
    let config = Config {
        path: fixture_path,
        json: true,
        threshold: None,
        verbose: false,
    };
    let reporter = CaptureReporter {
        captured: RefCell::new(String::new()),
    };
    let app: App<FsWalk, DefaultScorer, CaptureReporter> = App::with_deps(
        FsWalk::new(&config.path),
        DefaultScorer::new(),
        reporter,
        config,
    );
    app.run().expect("app run failed");
    let captured = app.reporter().captured.borrow();
    serde_json::from_str(&captured).expect("valid JSON")
}

#[test]
fn mixed_has_all_eight_cases() {
    // Arrange & Act
    let report = analyze();
    let functions = report["functions"].as_array().unwrap();

    // Assert
    assert_eq!(functions.len(), 8, "should have 8 functions (1 per case)");
}

#[test]
fn mixed_has_clean_and_dirty_functions() {
    // Arrange & Act
    let report = analyze();
    let overall = &report["overall"];
    let avg = overall["avg_contribution"].as_f64().unwrap();

    // Assert
    assert!(avg > 0.0, "avg contribution should be > 0");
    assert!(avg < 1.0, "avg contribution should be < 1.0 (mix of clean and dirty)");
}

#[test]
fn mixed_hidden_deps_count_correct() {
    // Arrange & Act
    let report = analyze();
    let functions = report["functions"].as_array().unwrap();

    // Assert
    let mut total_deps = 0u64;
    for f in functions {
        total_deps += f["hidden_deps"].as_u64().unwrap();
    }
    assert!(total_deps >= 4, "should have at least 4 hidden deps across all functions");
}
