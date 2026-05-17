// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::ffi::OsString;
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(name = "cargo-grip4rust", version = "0.2.0")]
#[command(about = "Measure Rust testability")]
pub struct Args {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(long)]
    pub json: bool,

    #[arg(long, alias = "min-score")]
    pub threshold: Option<u32>,

    #[arg(long)]
    pub verbose: bool,
}

impl Args {
    pub fn parse_from_args<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::parse_from(args)
    }

    pub fn parse_cargo() -> Self {
        let raw: Vec<OsString> = std::env::args_os().collect();
        if raw.len() > 1 && raw[1].to_string_lossy() == "grip4rust" {
            let mut filtered = vec![raw[0].clone()];
            filtered.extend(raw.into_iter().skip(2));
            Self::parse_from(filtered)
        } else {
            Self::parse()
        }
    }
}
