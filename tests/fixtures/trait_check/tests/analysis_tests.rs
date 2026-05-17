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
        .join("trait_check");
    let config = Config {
        path: fixture_path,
        json: true,
        threshold: None,
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
fn overall_score_is_reasonable() {
    let report = analyze();
    let overall = &report["overall"];
    let score = overall["grip_score"].as_u64().unwrap();
    assert!(score > 0, "grip score should be positive, got {score}");
    assert!(score <= 100, "grip score should not exceed 100, got {score}");
}

#[test]
fn overall_has_trait_ratio() {
    let report = analyze();
    let overall = &report["overall"];
    assert!(
        overall.get("trait_ratio").is_some(),
        "overall must have trait_ratio"
    );
}

#[test]
fn overall_trait_ratio_is_below_one() {
    let report = analyze();
    let overall = &report["overall"];
    let ratio = overall["trait_ratio"].as_f64().unwrap();
    assert!(ratio < 1.0, "machinery's impure inherent methods should drag trait ratio below 1.0, got {ratio}");
}

#[test]
fn overall_has_trait_fields() {
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
fn total_impl_methods_are_counted() {
    let report = analyze();
    let overall = &report["overall"];
    let inherent = overall["inherent_methods"].as_u64().unwrap();
    let local_trait = overall["local_trait_methods"].as_u64().unwrap();
    assert!(
        inherent + local_trait > 0,
        "should find impl methods, got inherent={inherent}, local_trait={local_trait}"
    );
}

#[test]
fn modules_have_trait_fields() {
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
