// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use crate::args::Args;

#[derive(Debug, Clone)]
pub struct Config {
    pub path: PathBuf,
    pub json: bool,
    pub min_score: Option<u32>,
}

impl Config {
    #[must_use]
    pub fn from_args(args: Args) -> Self {
        Self {
            path: args.path,
            json: args.json,
            min_score: args.min_score,
        }
    }
}
