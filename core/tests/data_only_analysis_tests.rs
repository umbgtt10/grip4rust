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
        .join("data_only");
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
fn analyze_zero_function_module_has_null_score() {
    // Arrange & Act
    let report = analyze();

    // Assert
    assert!(report["overall"]["grip_score"].is_null());
}

#[test]
fn analyze_zero_function_module_is_not_an_offender() {
    // Arrange & Act
    let report = analyze();
    let offenders = report["offenders"].as_array().unwrap();

    // Assert
    assert!(
        offenders.is_empty(),
        "zero-function module should never be flagged as an offender, got {offenders:?}"
    );
}

#[test]
fn analyze_zero_function_module_still_reports_public_items() {
    // Arrange & Act
    let report = analyze();
    let public_items = report["overall"]["public_items"].as_u64().unwrap();

    // Assert
    assert!(
        public_items > 0,
        "public struct/enum items should still be counted even with no functions"
    );
}
