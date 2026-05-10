// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;
use std::process::ExitCode;

use anyhow::Result;

use crate::collector::collect_file;
use crate::config::Config;
use crate::grip_report::GripReport;
use crate::overall_stats::OverallStats;
use crate::reporter::write;
use crate::scorer::{agg_modules, module_stats, score_counts};
use crate::walk::Walk;

pub fn run(config: Config) -> Result<ExitCode> {
    let walker = Walk::new(&config.path);
    let files = walker.rust_files()?;
    let mut indexed = Vec::with_capacity(files.len());
    for (path, source) in files {
        let module = module_from_path(&path, &config.path);
        let counts = collect_file(&source, &path);
        indexed.push((module, counts));
    }
    if indexed.is_empty() {
        return Err(anyhow::anyhow!(
            "no Rust source files found in {}",
            config.path.display()
        ));
    }
    let (overall_counts, modules) = agg_modules(indexed);
    let (grip_score, pure_ratio, public_ratio) = score_counts(&overall_counts);
    let overall = OverallStats {
        grip_score,
        public_items: overall_counts.public_items,
        total_functions: overall_counts.total_functions,
        pure_functions: overall_counts.pure_functions,
        pure_ratio,
        public_ratio,
    };
    let target = config
        .path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(".")
        .to_string();
    let report = GripReport {
        version: env!("CARGO_PKG_VERSION").to_string(),
        target,
        overall,
        modules: module_stats(modules),
    };
    if let Some(min) = config.min_score {
        return Ok(if grip_score >= min {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        });
    }
    write(&report, config.json)?;
    Ok(ExitCode::SUCCESS)
}

fn module_from_path(path: &Path, root: &Path) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let s = relative.to_string_lossy().replace('\\', "/");
    let without_src = s.strip_prefix("src/").map(|s| s.to_string()).unwrap_or(s);
    if let Some(pos) = without_src.rfind('/') {
        without_src[..pos].to_string()
    } else {
        ".".to_string()
    }
}
