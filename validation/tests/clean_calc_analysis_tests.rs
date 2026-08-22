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
        .join("clean_calc");
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

fn analyze_at(fixture_path: &PathBuf) -> serde_json::Value {
    let config = Config {
        path: fixture_path.clone(),
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
fn analyze_a_clean_calculator_scores_between_forty_and_seventy() {
    // Arrange & Act
    let parsed = analyze();

    // Assert
    let grip_score = parsed["overall"]["grip_score"].as_u64().unwrap();
    assert!(
        grip_score >= 40,
        "expected decent score >= 40, got {}",
        grip_score
    );
    assert!(
        grip_score < 70,
        "expected imperfect score < 70, got {}",
        grip_score
    );
}

#[test]
fn has_many_public_items() {
    // Arrange & Act
    let parsed = analyze();

    // Assert
    let public_items = parsed["overall"]["public_items"].as_u64().unwrap();
    assert!(
        public_items >= 10,
        "expected many public items, got {}",
        public_items
    );
}

#[test]
fn high_pure_ratio() {
    // Arrange & Act
    let parsed = analyze();

    // Assert
    let pure_ratio = parsed["overall"]["pure_ratio"].as_f64().unwrap();
    assert!(
        pure_ratio >= 0.5,
        "expected reasonable pure ratio, got {}",
        pure_ratio
    );
}

#[test]
fn scores_higher_than_sloppy() {
    // Arrange & Act
    let clean_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("fixture")
        .join("clean_calc");
    let sloppy_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("fixture")
        .join("sloppy_calc");
    let clean = analyze_at(&clean_path);
    let sloppy = analyze_at(&sloppy_path);

    // Assert
    let clean_score = clean["overall"]["grip_score"].as_u64().unwrap();
    let sloppy_score = sloppy["overall"]["grip_score"].as_u64().unwrap();
    assert!(
        clean_score > sloppy_score + 10,
        "expected clean ({}) to be > sloppy ({}) + 10",
        clean_score,
        sloppy_score,
    );
}
