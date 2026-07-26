// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::ffi::OsString;
use std::path::Path;
use std::process::ExitCode;

use anyhow::Result;

use crate::args::Args;
use crate::cache::Cache;
use crate::collector::Collector;
use crate::config::Config;
use crate::default_scorer::DefaultScorer;
use crate::fs_walk::FsWalk;
use crate::function_info::FunctionInfo;
use crate::grip_report::GripReport;
use crate::item_counts::ItemCounts;
use crate::offender::Offender;
use crate::overall_stats::OverallStats;
use crate::stdout_reporter::StdoutReporter;
use crate::struct_registry::StructRegistry;
use crate::traits::cache_store::CacheStore;
use crate::traits::reporter::Reporter;
use crate::traits::scorer::Scorer;
use crate::traits::walk::Walk;

type CollectedFiles = (Vec<(String, ItemCounts)>, Vec<FunctionInfo>);

pub struct App {
    walker: Box<dyn Walk>,
    scorer: Box<dyn Scorer>,
    reporter: Box<dyn Reporter>,
    cache: Box<dyn CacheStore>,
    config: Config,
}

impl App {
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self {
            walker: Box::new(FsWalk::new(&config.path)),
            scorer: Box::new(DefaultScorer::new()),
            reporter: Box::new(StdoutReporter::new(config.json, config.verbose)),
            cache: Box::new(Cache::new(&config.path)),
            config,
        }
    }

    #[must_use]
    pub fn with_deps(
        walker: Box<dyn Walk>,
        scorer: Box<dyn Scorer>,
        reporter: Box<dyn Reporter>,
        cache: Box<dyn CacheStore>,
        config: Config,
    ) -> Self {
        Self {
            walker,
            scorer,
            reporter,
            cache,
            config,
        }
    }

    pub fn run(&self) -> Result<ExitCode> {
        let (indexed, functions) = self.collect_files()?;
        self.cache.flush();

        if indexed.is_empty() {
            return Err(anyhow::anyhow!(
                "no Rust source files found in {}",
                self.config.path.display()
            ));
        }

        let report = self.compute_report(indexed, functions);
        self.handle_output(&report)
    }

    fn collect_files(&self) -> Result<CollectedFiles> {
        let files = self.walker.rust_files()?;
        let registry = StructRegistry::build(&files);
        let mut indexed = Vec::with_capacity(files.len());
        let mut all_functions = Vec::new();
        for (path, source) in files {
            let module = self.module_from_path(&path);
            let (counts, functions) = Collector::collect(&source, &path, &registry);
            all_functions.extend(functions);
            if self.cache.get(&path).is_none() {
                self.cache.set(&path, &source, &counts);
            }
            indexed.push((module, counts));
        }
        Ok((indexed, all_functions))
    }

    fn compute_report(
        &self,
        indexed: Vec<(String, ItemCounts)>,
        functions: Vec<FunctionInfo>,
    ) -> GripReport {
        let (overall_counts, modules) = self.scorer.agg_modules(indexed);
        let (grip_score, pure_ratio, public_ratio, trait_ratio, avg_contribution, clean_fn_ratio) =
            self.scorer.score_counts(&overall_counts);
        let overall = OverallStats {
            grip_score,
            public_items: overall_counts.public_items,
            total_functions: overall_counts.total_functions,
            pure_functions: overall_counts.pure_functions,
            pure_ratio,
            public_ratio,
            inherent_methods: overall_counts.inherent_methods,
            local_trait_methods: overall_counts.local_trait_methods,
            trait_ratio,
            avg_contribution,
            clean_fn_ratio,
            grip_absolute_total: overall_counts.total_contribution,
        };
        let target = self
            .config
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".")
            .to_string();
        let module_stats = self.scorer.module_stats(modules);
        let offender_threshold = self.config.threshold.unwrap_or(50);
        let offenders = module_stats
            .iter()
            .filter_map(|m| {
                let score = m.grip_score?;
                (score < offender_threshold).then(|| Offender {
                    path: m.path.clone(),
                    grip_score: score,
                })
            })
            .collect();
        GripReport {
            version: env!("CARGO_PKG_VERSION").to_string(),
            target,
            overall,
            modules: module_stats,
            offenders,
            offender_threshold,
            functions,
        }
    }

    fn handle_output(&self, report: &GripReport) -> Result<ExitCode> {
        if let Some(min) = self.config.threshold {
            return Ok(if report.overall.grip_score.is_none_or(|s| s >= min) {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            });
        }
        self.reporter.write(report)?;
        Ok(ExitCode::SUCCESS)
    }

    fn module_from_path(&self, path: &Path) -> String {
        let relative = path.strip_prefix(&self.config.path).unwrap_or(path);
        let s = relative.to_string_lossy().replace('\\', "/");
        let without_src = s.strip_prefix("src/").map(|s| s.to_string()).unwrap_or(s);
        if let Some(pos) = without_src.rfind('/') {
            without_src[..pos].to_string()
        } else {
            ".".to_string()
        }
    }
}

pub fn run() -> Result<ExitCode> {
    let args = Args::parse_cargo();
    let config = Config::from_args(args);
    App::new(config).run()
}

pub fn run_from_args<I, T>(args: I) -> Result<ExitCode>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args = Args::parse_from_args(args);
    let config = Config::from_args(args);
    App::new(config).run()
}
