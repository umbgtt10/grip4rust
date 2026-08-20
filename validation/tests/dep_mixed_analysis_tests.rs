// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;
use grip::analysis::fs_walk::FsWalk;
use grip::invocation::app::App;
use grip::invocation::config::Config;
use grip::invocation::no_op_cache_store::NoOpCacheStore;
use grip::reporting::default_scorer::DefaultScorer;
use grip::reporting::grip_report::GripReport;
use grip::traits::reporter::Reporter;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

fn analyze() -> serde_json::Value {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("fixture")
        .join("dep_mixed");
    let config = Config {
        path: fixture_path,
        json: true,
        threshold: None,
        verbose: false,
    };
    let captured = Rc::new(RefCell::new(String::new()));
    let reporter = CaptureReporter {
        captured: Rc::clone(&captured),
    };
    let app = App::with_deps(
        Box::new(FsWalk::new(&config.path)),
        Box::new(DefaultScorer::new()),
        Box::new(reporter),
        Box::new(NoOpCacheStore::new()),
        config,
    );
    app.run().expect("app run failed");
    let captured = captured.borrow();
    serde_json::from_str(&captured).expect("valid JSON")
}

struct CaptureReporter {
    captured: Rc<RefCell<String>>,
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
    assert!(
        avg < 1.0,
        "avg contribution should be < 1.0 (mix of clean and dirty)"
    );
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
    assert!(
        total_deps >= 4,
        "should have at least 4 hidden deps across all functions"
    );
}
