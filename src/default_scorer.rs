// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;

use crate::item_counts::ItemCounts;
use crate::module_stats::ModuleStats;
use crate::traits::scorer::Scorer;

#[derive(Debug, Clone, Default)]
pub struct DefaultScorer;

impl DefaultScorer {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Scorer for DefaultScorer {
    fn score_counts(&self, counts: &ItemCounts) -> (u32, f64, f64, f64, f64, f64) {
        let pure_ratio = if counts.total_functions > 0 {
            counts.pure_functions as f64 / counts.total_functions as f64
        } else {
            0.0
        };
        let public_ratio = if counts.total_items > 0 {
            counts.public_items as f64 / counts.total_items as f64
        } else {
            0.0
        };
        let total_impure = counts.inherent_impure + counts.local_trait_impure;
        let trait_ratio = if total_impure > 0 {
            counts.local_trait_impure as f64 / total_impure as f64
        } else {
            0.0
        };
        let avg_contribution = if counts.total_functions > 0 {
            counts.total_contribution / counts.total_functions as f64
        } else {
            0.0
        };
        let clean_fn_ratio = if counts.total_functions > 0 {
            counts.clean_functions as f64 / counts.total_functions as f64
        } else {
            0.0
        };
        let grip = ((pure_ratio * 0.30 + public_ratio * 0.20 + trait_ratio * 0.25 + avg_contribution * 0.25) * 100.0)
            .round() as u32;
        (grip, pure_ratio, public_ratio, trait_ratio, avg_contribution, clean_fn_ratio)
    }

    fn agg_modules(
        &self,
        files: Vec<(String, ItemCounts)>,
    ) -> (ItemCounts, BTreeMap<String, ItemCounts>) {
        let mut overall = ItemCounts::default();
        let mut modules: BTreeMap<String, ItemCounts> = BTreeMap::new();
        for (module, counts) in files {
            overall = overall.merged(&counts);
            modules
                .entry(module)
                .and_modify(|existing| *existing = existing.clone().merged(&counts))
                .or_insert(counts);
        }
        (overall, modules)
    }

    fn module_stats(&self, modules: BTreeMap<String, ItemCounts>) -> Vec<ModuleStats> {
        modules
            .into_iter()
            .map(|(path, counts)| {
                let (grip_score, pure_ratio, public_ratio, trait_ratio, avg_contribution, clean_fn_ratio) =
                    self.score_counts(&counts);
                ModuleStats {
                    path,
                    grip_score,
                    pure_ratio,
                    public_items: counts.public_items,
                    total_functions: counts.total_functions,
                    pure_functions: counts.pure_functions,
                    public_ratio,
                    inherent_methods: counts.inherent_methods,
                    local_trait_methods: counts.local_trait_methods,
                    trait_ratio,
                    avg_contribution,
                    clean_fn_ratio,
                }
            })
            .collect()
    }
}
