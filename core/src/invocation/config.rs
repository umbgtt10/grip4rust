// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::invocation::args::Args;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub path: PathBuf,
    pub json: bool,
    pub threshold: Option<u32>,
    pub verbose: bool,
}

impl Config {
    #[must_use]
    pub fn from_args(args: Args) -> Self {
        Self {
            path: args.path,
            json: args.json,
            threshold: args.threshold,
            verbose: args.verbose,
        }
    }
}
