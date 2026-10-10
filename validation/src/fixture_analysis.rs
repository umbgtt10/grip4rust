// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::capture_reporter::CaptureReporter;
use grip::analysis::fs_walk::FsWalk;
use grip::invocation::app::App;
use grip::invocation::config::Config;
use grip::invocation::no_op_cache_store::NoOpCacheStore;
use grip::reporting::default_scorer::DefaultScorer;
use serde_json::Value;
use serde_json::from_str;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub struct FixtureAnalysis {
    path: PathBuf,
}

impl FixtureAnalysis {
    #[must_use]
    pub fn new(fixture: &str) -> Self {
        Self {
            path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("fixture")
                .join(fixture),
        }
    }

    #[must_use]
    pub fn report(&self) -> Value {
        let config = Config {
            path: self.path.clone(),
            json: true,
            threshold: None,
            verbose: false,
        };
        let captured = Rc::new(RefCell::new(String::new()));
        let app = App::with_deps(
            Box::new(FsWalk::new(&config.path)),
            Box::new(DefaultScorer::new()),
            Box::new(CaptureReporter::new(Rc::clone(&captured))),
            Box::new(NoOpCacheStore::new()),
            config,
        );
        app.run().expect("app run failed");
        let json = captured.borrow().clone();
        from_str(&json).expect("valid JSON")
    }
}
