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
use serde_json::from_str;
use serde_json::to_string_pretty;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

fn analyze() -> serde_json::Value {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("fixture")
        .join("sloppy_calc");
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
    let _ = app.run().unwrap();
    let captured = captured.borrow().clone();
    from_str(&captured).unwrap()
}

struct CaptureReporter {
    captured: Rc<RefCell<String>>,
}

impl Reporter for CaptureReporter {
    fn render(&self, report: &GripReport) -> Result<String> {
        let json = to_string_pretty(report)?;
        *self.captured.borrow_mut() = json.clone();
        Ok(json)
    }

    fn write(&self, report: &GripReport) -> Result<()> {
        let json = self.render(report)?;
        print!("{}", json);
        Ok(())
    }
}

#[test]
fn analyze_a_sloppy_calculator_scores_below_fifty() {
    // Arrange & Act
    let parsed = analyze();

    // Assert
    let grip_score = parsed["overall"]["grip_score"].as_u64().unwrap();
    assert!(
        grip_score < 50,
        "expected bad score < 50, got {}",
        grip_score
    );
}

#[test]
fn has_few_public_items() {
    // Arrange & Act
    let parsed = analyze();

    // Assert
    let public_items = parsed["overall"]["public_items"].as_u64().unwrap();
    assert!(
        public_items < 6,
        "expected few public items, got {}",
        public_items
    );
}

#[test]
fn low_pure_ratio() {
    // Arrange & Act
    let parsed = analyze();

    // Assert
    let pure_ratio = parsed["overall"]["pure_ratio"].as_f64().unwrap();
    assert!(
        pure_ratio < 0.7,
        "expected low pure ratio, got {}",
        pure_ratio
    );
}
