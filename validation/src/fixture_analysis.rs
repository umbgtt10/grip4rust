// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::capture_reporter::CaptureReporter;
use grip::invocation::app::App;
use grip::invocation::no_op_cache_store::NoOpCacheStore;
use grip::reporting::default_scorer::DefaultScorer;
use grip_cli::invocation::args::Args;
use grip_cli::system::fs_walk::FsWalk;
use serde_json::Value;
use serde_json::from_str;
use std::cell::RefCell;
use std::ffi::OsString;
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
        let args = Args::parse_from_args([
            OsString::from("cargo-grip4rust"),
            self.path.clone().into_os_string(),
        ]);
        let captured = Rc::new(RefCell::new(String::new()));
        let app = App::new(
            Box::new(FsWalk::new(args.path())),
            Box::new(DefaultScorer::new()),
            Box::new(CaptureReporter::new(Rc::clone(&captured))),
            Box::new(NoOpCacheStore::new()),
            args.config(),
        );
        app.run().expect("app run failed");
        let json = captured.borrow().clone();
        from_str(&json).expect("valid JSON")
    }
}
