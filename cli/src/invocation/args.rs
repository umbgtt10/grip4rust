// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use clap::Parser;
use grip::invocation::config::Config;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[command(name = "cargo-grip4rust")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Measure Rust testability")]
pub struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,

    #[arg(long)]
    json: bool,

    #[arg(long, alias = "min-score")]
    threshold: Option<u32>,

    #[arg(long)]
    verbose: bool,
}

impl Args {
    pub fn parse_from_args<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::parse_from(args)
    }

    #[must_use]
    pub fn config(&self) -> Config {
        Config {
            path: self.path.display().to_string(),
            target: self
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(".")
                .to_string(),
            json: self.json,
            threshold: self.threshold,
            verbose: self.verbose,
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

    // Cargo runs `cargo grip4rust ...` as `cargo-grip4rust grip4rust ...`, so
    // the name arrives twice. Running the binary directly does not repeat it,
    // which is why the strip is conditional; and it is positional, because
    // dropping every occurrence would swallow a path argument that happens to
    // be named after the tool.
    pub fn without_cargo_subcommand(raw: Vec<OsString>) -> Vec<OsString> {
        if raw.len() > 1 && raw[1].to_string_lossy() == "grip4rust" {
            let mut filtered = vec![raw[0].clone()];
            filtered.extend(raw.into_iter().skip(2));
            filtered
        } else {
            raw
        }
    }
}
