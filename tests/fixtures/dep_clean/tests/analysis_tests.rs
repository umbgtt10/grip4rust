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
        .join("dep_clean");
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
fn clean_module_has_high_score() {
    // Arrange & Act
    let report = analyze();
    let score = report["overall"]["grip_score"].as_u64().unwrap();

    // Assert
    assert!(score >= 60, "expected decent score >= 60, got {score}");
}

#[test]
fn clean_module_has_high_contribution() {
    // Arrange & Act
    let report = analyze();
    let avg = report["overall"]["avg_contribution"].as_f64().unwrap();

    // Assert
    assert!(avg >= 0.75, "expected high avg contribution >= 0.75, got {avg}");
}

#[test]
fn clean_module_has_no_zero_functions() {
    // Arrange & Act
    let report = analyze();
    let functions = report["functions"].as_array().unwrap();

    // Assert
    for f in functions {
        let deps = f["hidden_deps"].as_u64().unwrap();
        assert_eq!(deps, 0, "function {} should have 0 hidden deps", f["name"]);
    }
}
