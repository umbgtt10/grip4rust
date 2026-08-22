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
        .join("trait_check");
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
    from_str(&captured).expect("valid JSON")
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
        print!("{json}");
        Ok(())
    }
}

#[test]
fn modules_have_trait_fields() {
    // Arrange & Act & Assert
    let report = analyze();
    let modules = report["modules"].as_array().unwrap();
    for module in modules {
        assert!(
            module.get("inherent_methods").is_some(),
            "module {} missing inherent_methods",
            module["path"]
        );
        assert!(
            module.get("local_trait_methods").is_some(),
            "module {} missing local_trait_methods",
            module["path"]
        );
        assert!(
            module.get("trait_ratio").is_some(),
            "module {} missing trait_ratio",
            module["path"]
        );
    }
}

#[test]
fn overall_has_trait_fields() {
    // Arrange & Act & Assert
    let report = analyze();
    let overall = &report["overall"];
    assert!(
        overall.get("inherent_methods").is_some(),
        "overall missing inherent_methods"
    );
    assert!(
        overall.get("local_trait_methods").is_some(),
        "overall missing local_trait_methods"
    );
}

#[test]
fn overall_has_trait_ratio() {
    // Arrange & Act & Assert
    let report = analyze();
    let overall = &report["overall"];
    assert!(
        overall.get("trait_ratio").is_some(),
        "overall must have trait_ratio"
    );
}

#[test]
fn overall_score_is_reasonable() {
    // Arrange & Act & Assert
    let report = analyze();
    let overall = &report["overall"];
    let score = overall["grip_score"].as_u64().unwrap();
    assert!(score > 0, "grip score should be positive, got {score}");
    assert!(
        score <= 100,
        "grip score should not exceed 100, got {score}"
    );
}

#[test]
fn overall_trait_ratio_is_below_one() {
    // Arrange & Act & Assert
    let report = analyze();
    let overall = &report["overall"];
    let ratio = overall["trait_ratio"].as_f64().unwrap();
    assert!(
        ratio < 1.0,
        "machinery's impure inherent methods should drag trait ratio below 1.0, got {ratio}"
    );
}

#[test]
fn total_impl_methods_are_counted() {
    // Arrange & Act & Assert
    let report = analyze();
    let overall = &report["overall"];
    let inherent = overall["inherent_methods"].as_u64().unwrap();
    let local_trait = overall["local_trait_methods"].as_u64().unwrap();
    assert!(
        inherent + local_trait > 0,
        "should find impl methods, got inherent={inherent}, local_trait={local_trait}"
    );
}
