// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;

use crate::analysis::item_counts::ItemCounts;
use crate::reporting::module_stats::ModuleStats;
use crate::reporting::scores::Scores;

pub trait Scorer {
    fn score_counts(&self, counts: &ItemCounts) -> Scores;
    fn agg_modules(
        &self,
        files: Vec<(String, ItemCounts)>,
    ) -> (ItemCounts, BTreeMap<String, ItemCounts>);
    fn module_stats(&self, modules: BTreeMap<String, ItemCounts>) -> Vec<ModuleStats>;
}
