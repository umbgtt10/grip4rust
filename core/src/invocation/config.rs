// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::invocation::args::Args;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    path: PathBuf,
    json: bool,
    threshold: Option<u32>,
    verbose: bool,
}

impl Config {
    #[must_use]
    pub fn from_args(args: Args) -> Self {
        Self {
            path: args.path().to_path_buf(),
            json: args.json(),
            threshold: args.threshold(),
            verbose: args.verbose(),
        }
    }

    #[must_use]
    pub fn new(path: PathBuf, json: bool, threshold: Option<u32>, verbose: bool) -> Self {
        Self {
            path,
            json,
            threshold,
            verbose,
        }
    }

    #[must_use]
    pub fn json(&self) -> bool {
        self.json
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[must_use]
    pub fn threshold(&self) -> Option<u32> {
        self.threshold
    }

    #[must_use]
    pub fn verbose(&self) -> bool {
        self.verbose
    }
}
