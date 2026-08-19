// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;
use grip::app::App;
use grip::config::Config;
use grip::default_scorer::DefaultScorer;
use grip::fs_walk::FsWalk;
use grip::grip_report::GripReport;
use grip::no_op_cache_store::NoOpCacheStore;
use grip::traits::reporter::Reporter;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

fn analyze() -> serde_json::Value {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("fixture")
        .join("dep_monolith");
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
fn monolith_all_functions_have_hidden_deps() {
    // Arrange & Act
    let report = analyze();
    let functions = report["functions"].as_array().unwrap();

    // Assert
    for f in functions {
        let deps = f["hidden_deps"].as_u64().unwrap();
        assert!(
            deps >= 1,
            "function {} should have >=1 hidden dep",
            f["name"]
        );
    }
}

#[test]
fn monolith_has_low_score() {
    // Arrange & Act
    let report = analyze();
    let score = report["overall"]["grip_score"].as_u64().unwrap();

    // Assert
    assert!(
        score <= 35,
        "monolith should have low score <= 35, got {score}"
    );
}

#[test]
fn monolith_has_zero_clean_functions() {
    // Arrange & Act
    let report = analyze();
    let clean = report["overall"]["clean_fn_ratio"].as_f64().unwrap();

    // Assert
    assert_eq!(clean, 0.0, "monolith should have 0 clean functions");
}
