// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::analysis::collector::Collector;
use crate::analysis::function_info::FunctionInfo;
use crate::analysis::item_counts::ItemCounts;
use crate::analysis::method_purity_registry::MethodPurityRegistry;
use crate::analysis::struct_registry::StructRegistry;
use crate::invocation::config::Config;
use crate::reporting::grip_report::GripReport;
use crate::reporting::offender::Offender;
use crate::reporting::overall_stats::OverallStats;
use crate::traits::cache_store::CacheStore;
use crate::traits::reporter::Reporter;
use crate::traits::scorer::Scorer;
use crate::traits::walk::Walk;
use anyhow::{Result, anyhow};
use std::process::ExitCode;

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
    pub fn new(
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
            return Err(anyhow!(
                "no Rust source files found in {}",
                self.config.path
            ));
        }

        let report = self.compute_report(indexed, functions);
        self.handle_output(&report)
    }

    fn collect_files(&self) -> Result<CollectedFiles> {
        let files = self.walker.rust_files()?;
        let registry = StructRegistry::build(&files);
        let method_purity = MethodPurityRegistry::build(&files, &registry);
        let mut indexed = Vec::with_capacity(files.len());
        let mut all_functions = Vec::new();
        for file in files {
            let module = Self::module_of(&file.relative);
            let (counts, functions) =
                Collector::collect(&file.source, &file.path, &registry, &method_purity);
            all_functions.extend(functions);
            if self.cache.get(&file.path).is_none() {
                self.cache.set(&file.path, &file.source, &counts);
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
        let scores = self.scorer.score_counts(&overall_counts);
        let overall = OverallStats {
            grip_score: scores.grip_score,
            public_items: overall_counts.public_items(),
            total_functions: overall_counts.total_functions(),
            pure_functions: overall_counts.pure_functions(),
            pure_ratio: scores.pure_ratio,
            public_ratio: scores.public_ratio,
            inherent_methods: overall_counts.inherent_methods(),
            local_trait_methods: overall_counts.local_trait_methods(),
            trait_ratio: scores.trait_ratio,
            avg_contribution: scores.avg_contribution,
            clean_fn_ratio: scores.clean_fn_ratio,
            grip_absolute_total: overall_counts.total_contribution(),
        };
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
            target: self.config.target.clone(),
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

    fn module_of(relative: &str) -> String {
        let s = relative.replace('\\', "/");
        let without_src = s.strip_prefix("src/").map(|s| s.to_string()).unwrap_or(s);
        if let Some(pos) = without_src.rfind('/') {
            without_src[..pos].to_string()
        } else {
            ".".to_string()
        }
    }
}
