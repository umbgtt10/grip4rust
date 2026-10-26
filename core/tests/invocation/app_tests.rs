// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_walk::InMemoryWalk;
use crate::support::recording_cache_store::RecordingCacheStore;
use crate::support::recording_report_renderer::RecordingReportRenderer;
use grip::invocation::app::App;
use grip::invocation::config::Config;
use grip::invocation::no_op_cache_store::NoOpCacheStore;
use grip::invocation::run_outcome::RunOutcome;
use grip::reporting::default_scorer::DefaultScorer;
use grip::reporting::grip_report::GripReport;
use std::cell::RefCell;
use std::rc::Rc;

const GREET: &str = "pub fn greet() -> &'static str { \"hello\" }\n";
const HIDDEN: &str = "fn hidden() { println!(\"x\"); }\n";

fn app(walk: InMemoryWalk, handed: &Rc<RefCell<Vec<GripReport>>>, threshold: Option<u32>) -> App {
    App::new(
        Box::new(walk),
        Box::new(DefaultScorer::new()),
        Box::new(RecordingReportRenderer::new(Rc::clone(handed))),
        Box::new(NoOpCacheStore::new()),
        config(threshold),
    )
}

fn config(threshold: Option<u32>) -> Config {
    Config {
        path: String::from("some/root"),
        target: String::from("root"),
        threshold,
    }
}

#[test]
fn new_builds_an_app_that_runs_on_the_injected_dependencies() {
    // Arrange
    let handed = Rc::new(RefCell::new(Vec::new()));
    let walk = InMemoryWalk::new().with_file("src/lib.rs", GREET);

    // Act
    let app = app(walk, &handed, None);

    // Assert
    assert_eq!(
        app.run().unwrap(),
        RunOutcome {
            output: String::from("rendered\n"),
            below_threshold: false,
        }
    );
}

#[test]
fn run_asks_the_cache_for_each_file_and_stores_only_the_ones_it_misses() {
    // Arrange
    let calls = Rc::new(RefCell::new(Vec::new()));
    let handed = Rc::new(RefCell::new(Vec::new()));
    let app = App::new(
        Box::new(
            InMemoryWalk::new()
                .with_file("src/a.rs", GREET)
                .with_file("src/b.rs", GREET),
        ),
        Box::new(DefaultScorer::new()),
        Box::new(RecordingReportRenderer::new(Rc::clone(&handed))),
        Box::new(RecordingCacheStore::new(
            vec![String::from("root/src/a.rs")],
            Rc::clone(&calls),
        )),
        config(None),
    );

    // Act
    app.run().unwrap();

    // Assert
    assert_eq!(
        *calls.borrow(),
        [
            "get root/src/a.rs",
            "get root/src/b.rs",
            "set root/src/b.rs",
            "flush"
        ]
    );
}

#[test]
fn run_lists_the_modules_below_fifty_as_offenders_when_no_threshold_is_given() {
    // Arrange
    let handed = Rc::new(RefCell::new(Vec::new()));
    let walk = InMemoryWalk::new()
        .with_file("src/clean/lib.rs", GREET)
        .with_file("src/dirty/lib.rs", HIDDEN);

    // Act
    app(walk, &handed, None).run().unwrap();

    // Assert
    let report = &handed.borrow()[0];
    let offenders: Vec<(&str, u32)> = report
        .offenders
        .iter()
        .map(|offender| (offender.path.as_str(), offender.grip_score))
        .collect();
    assert_eq!(report.offender_threshold, 50);
    assert_eq!(offenders, [("dirty", 28)]);
}

#[test]
fn run_names_each_module_by_its_directory_beneath_src() {
    // Arrange
    let handed = Rc::new(RefCell::new(Vec::new()));
    let walk = InMemoryWalk::new()
        .with_file("src/lib.rs", GREET)
        .with_file("src/a/b/c.rs", GREET)
        .with_file("src\\d\\e.rs", GREET)
        .with_file("other/f.rs", GREET);

    // Act
    app(walk, &handed, None).run().unwrap();

    // Assert
    let report = &handed.borrow()[0];
    let modules: Vec<&str> = report.modules.iter().map(|m| m.path.as_str()).collect();
    assert_eq!(modules, [".", "a/b", "d", "other"]);
}

#[test]
fn run_records_each_function_under_its_path_with_forward_slashes() {
    // Arrange
    let handed = Rc::new(RefCell::new(Vec::new()));
    let walk = InMemoryWalk::new().with_file("src\\lib.rs", GREET);

    // Act
    app(walk, &handed, None).run().unwrap();

    // Assert
    assert_eq!(handed.borrow()[0].functions[0].file, "root/src/lib.rs");
}

#[test]
fn run_stamps_the_report_with_the_configured_target_and_the_version() {
    // Arrange
    let handed = Rc::new(RefCell::new(Vec::new()));
    let walk = InMemoryWalk::new().with_file("src/lib.rs", GREET);

    // Act
    app(walk, &handed, None).run().unwrap();

    // Assert
    let report = &handed.borrow()[0];
    assert_eq!(report.target, "root");
    assert_eq!(report.version, env!("CARGO_PKG_VERSION"));
}

#[test]
fn run_with_a_failing_renderer_fails_with_its_failure() {
    // Arrange
    let handed = Rc::new(RefCell::new(Vec::new()));
    let app = App::new(
        Box::new(InMemoryWalk::new().with_file("src/lib.rs", GREET)),
        Box::new(DefaultScorer::new()),
        Box::new(RecordingReportRenderer::failing(Rc::clone(&handed))),
        Box::new(NoOpCacheStore::new()),
        config(None),
    );

    // Act
    let failure = app.run().unwrap_err();

    // Assert
    assert_eq!(failure.to_string(), "failed to render the report");
}

#[test]
fn run_with_a_threshold_above_the_score_returns_a_failing_verdict() {
    // Arrange
    let handed = Rc::new(RefCell::new(Vec::new()));
    let walk = InMemoryWalk::new().with_file("src/lib.rs", HIDDEN);

    // Act
    let outcome = app(walk, &handed, Some(29)).run().unwrap();

    // Assert
    assert_eq!(
        outcome,
        RunOutcome {
            output: String::new(),
            below_threshold: true,
        }
    );
}

#[test]
fn run_with_a_threshold_and_no_functions_returns_a_passing_verdict() {
    // Arrange
    let handed = Rc::new(RefCell::new(Vec::new()));
    let walk = InMemoryWalk::new().with_file("src/lib.rs", "pub struct Data;\n");

    // Act
    let outcome = app(walk, &handed, Some(100)).run().unwrap();

    // Assert
    assert_eq!(
        outcome,
        RunOutcome {
            output: String::new(),
            below_threshold: false,
        }
    );
}

#[test]
fn run_with_a_threshold_equal_to_the_score_returns_a_passing_verdict_without_rendering() {
    // Arrange
    let handed = Rc::new(RefCell::new(Vec::new()));
    let walk = InMemoryWalk::new().with_file("src/lib.rs", HIDDEN);

    // Act
    let outcome = app(walk, &handed, Some(28)).run().unwrap();

    // Assert
    assert_eq!(
        outcome,
        RunOutcome {
            output: String::new(),
            below_threshold: false,
        }
    );
    assert!(handed.borrow().is_empty());
}

#[test]
fn run_with_an_unreadable_file_fails_naming_it() {
    // Arrange
    let handed = Rc::new(RefCell::new(Vec::new()));
    let walk = InMemoryWalk::new().with_unreadable_file("root/src/lib.rs");

    // Act
    let failure = app(walk, &handed, None).run().unwrap_err();

    // Assert
    assert_eq!(failure.to_string(), "failed to read root/src/lib.rs");
}

#[test]
fn run_with_no_source_files_fails_naming_the_root() {
    // Arrange
    let handed = Rc::new(RefCell::new(Vec::new()));

    // Act
    let failure = app(InMemoryWalk::new(), &handed, None).run().unwrap_err();

    // Assert
    assert_eq!(
        failure.to_string(),
        "no Rust source files found in some/root"
    );
}
