// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::contribution_schedule::ContributionSchedule;
use crate::function_info::FunctionInfo;

#[derive(Debug, Clone, Copy)]
pub struct VerboseFunctionsRenderer {
    verbose: bool,
}

impl VerboseFunctionsRenderer {
    #[must_use]
    pub const fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    #[must_use]
    pub fn render(&self, functions: &[FunctionInfo]) -> Vec<String> {
        if !self.verbose || functions.is_empty() {
            return Vec::new();
        }
        let mut lines = vec!["\nPer-function detail:".to_string()];
        let mut sorted = functions.to_vec();
        sorted.sort_by(|a, b| a.file.cmp(&b.file).then(a.name.cmp(&b.name)));
        let mut current_file = String::new();
        for f in &sorted {
            if f.file != current_file {
                current_file.clone_from(&f.file);
                lines.push(format!("\n  {current_file}:"));
            }
            lines.push(Self::function_line(f));
        }
        lines
    }

    fn function_line(f: &FunctionInfo) -> String {
        let contr =
            ContributionSchedule::new().contribution(f.is_pure, f.has_trait_seam, f.dep_weight);
        let labels = f.hidden_dep_labels.join(", ");
        format!(
            "    {:<35}  pure: {:>5}  seam: {:>5}  hidden: {:>2}  contr: {:>5.0}%  [{}]  {}",
            f.name,
            if f.is_pure { "yes" } else { "no" },
            if f.has_trait_seam { "yes" } else { "no " },
            f.hidden_deps,
            contr * 100.0,
            if labels.is_empty() { "-" } else { &labels },
            Self::contribution_marker(f.hidden_deps),
        )
    }

    fn contribution_marker(hidden_deps: usize) -> &'static str {
        if hidden_deps == 0 {
            "✅"
        } else if hidden_deps == 1 {
            "⚠️"
        } else {
            "❌"
        }
    }
}
