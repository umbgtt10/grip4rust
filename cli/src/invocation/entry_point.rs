// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::invocation::args::Args;
use crate::system::fs_walk::FsWalk;
use crate::system::json_report_renderer::JsonReportRenderer;
use anyhow::Result;
use grip::invocation::app::App;
use grip::invocation::no_op_cache_store::NoOpCacheStore;
use grip::invocation::run_outcome::RunOutcome;
use grip::reporting::default_scorer::DefaultScorer;
use grip::reporting::text_report_renderer::TextReportRenderer;
use grip::traits::report_renderer::ReportRenderer;
use std::ffi::OsString;
use std::io::{Write, stdout};
use std::process::ExitCode;

// The composition root: the command line in, the report out on stdout, an exit
// code back. The one place the file system, the report's format and stdout are
// chosen; everything it composes is the core's, tested without them.
pub struct EntryPoint;

impl EntryPoint {
    #[must_use]
    pub fn compose(args: &Args) -> App {
        let renderer: Box<dyn ReportRenderer> = if args.json() {
            Box::new(JsonReportRenderer::new())
        } else {
            Box::new(TextReportRenderer::new(args.verbose()))
        };
        App::new(
            Box::new(FsWalk::new(args.path())),
            Box::new(DefaultScorer::new()),
            renderer,
            Box::new(NoOpCacheStore::new()),
            args.config(),
        )
    }

    pub fn finish(outcome: RunOutcome) -> Result<ExitCode> {
        stdout().write_all(outcome.output.as_bytes())?;
        Ok(if outcome.below_threshold {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        })
    }

    #[must_use]
    pub fn run(args: Vec<OsString>) -> ExitCode {
        match Self::run_from_args(Args::without_cargo_subcommand(args)) {
            Ok(exit_code) => exit_code,
            Err(error) => {
                eprintln!("error: {error}");
                ExitCode::FAILURE
            }
        }
    }

    pub fn run_from_args<I, T>(args: I) -> Result<ExitCode>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let outcome = Self::compose(&Args::parse_from_args(args)).run()?;
        Self::finish(outcome)
    }
}
