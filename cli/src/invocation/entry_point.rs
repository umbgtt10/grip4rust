// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::invocation::args::Args;
use anyhow::Result;
use grip::analysis::fs_walk::FsWalk;
use grip::invocation::app::App;
use grip::invocation::cache::Cache;
use grip::invocation::config::Config;
use grip::reporting::default_scorer::DefaultScorer;
use grip::reporting::stdout_reporter::StdoutReporter;
use std::ffi::OsString;
use std::process::ExitCode;

pub struct EntryPoint;

impl EntryPoint {
    #[must_use]
    pub fn compose(config: Config) -> App {
        App::new(
            Box::new(FsWalk::new(&config.path)),
            Box::new(DefaultScorer::new()),
            Box::new(StdoutReporter::new(config.json, config.verbose)),
            Box::new(Cache::new(&config.path)),
            config,
        )
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
        Self::compose(Args::parse_from_args(args).config()).run()
    }
}
