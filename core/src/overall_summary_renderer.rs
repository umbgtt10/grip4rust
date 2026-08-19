// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::overall_stats::OverallStats;

#[derive(Debug, Clone, Copy, Default)]
pub struct OverallSummaryRenderer;

impl OverallSummaryRenderer {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn render(&self, overall: &OverallStats) -> Vec<String> {
        vec![
            format!(
                "Overall grip score:    {}",
                Self::grip_score_display(overall.grip_score)
            ),
            format!("Absolute grip total:   {:.2}", overall.grip_absolute_total),
            format!("Public surface:        {} items", overall.public_items),
            format!("Total functions:       {}", overall.total_functions),
            format!(
                "Probably pure:         {} / {}  ({:.1}%)",
                overall.pure_functions,
                overall.total_functions,
                overall.pure_ratio * 100.0
            ),
            Self::trait_methods_line(overall),
            format!(
                "Hidden deps:           avg {:.2}  — {:.1}% clean  ({:.1}% avg contribution)",
                overall.total_functions as f64
                    - overall.avg_contribution * overall.total_functions as f64,
                overall.clean_fn_ratio * 100.0,
                overall.avg_contribution * 100.0,
            ),
        ]
    }

    fn grip_score_display(score: Option<u32>) -> String {
        match score {
            Some(score) => format!("{score} / 100"),
            None => "N/A    (no functions)".to_string(),
        }
    }

    fn trait_methods_line(overall: &OverallStats) -> String {
        let total_impl = overall.inherent_methods + overall.local_trait_methods;
        if total_impl == 0 {
            "Trait methods:         N/A    (no impl methods)".to_string()
        } else {
            format!(
                "Trait methods:         {} / {} impl methods are trait-bound  ({:.1}%)",
                overall.local_trait_methods,
                total_impl,
                overall.trait_ratio * 100.0
            )
        }
    }
}
