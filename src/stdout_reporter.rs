// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io::{self, Write};

use anyhow::Result;

use crate::grip_report::GripReport;
use crate::module_stats::ModuleStats;
use crate::traits::reporter::Reporter;

#[derive(Debug, Clone)]
pub struct StdoutReporter {
    json: bool,
    verbose: bool,
}

impl StdoutReporter {
    #[must_use]
    pub fn new(json: bool, verbose: bool) -> Self {
        Self { json, verbose }
    }
}

impl Reporter for StdoutReporter {
    fn render(&self, report: &GripReport) -> Result<String> {
        if self.json {
            Ok(serde_json::to_string_pretty(report)?)
        } else {
            Ok(self.render_human(report))
        }
    }

    fn write(&self, report: &GripReport) -> Result<()> {
        let out = self.render(report)?;
        io::stdout().write_all(out.as_bytes())?;
        io::stdout().write_all(b"\n")?;
        Ok(())
    }
}

impl StdoutReporter {
    fn render_human(&self, report: &GripReport) -> String {
        let mut lines = Vec::new();
        let target = &report.target;
        let version = &report.version;
        let header = if self.verbose {
            format!("grip {version} — {target} — verbose")
        } else {
            format!("cargo-grip4rust {version} -- {target}")
        };
        lines.push(format!(
            "{header}\n══════════════════════════════════════════════════════\n"
        ));

        let overall = &report.overall;
        lines.push(format!(
            "Overall grip score:    {} / 100",
            overall.grip_score
        ));
        lines.push(format!(
            "Public surface:        {} items",
            overall.public_items
        ));
        lines.push(format!(
            "Total functions:       {}",
            overall.total_functions
        ));
        lines.push(format!(
            "Probably pure:         {} / {}  ({:.1}%)",
            overall.pure_functions,
            overall.total_functions,
            overall.pure_ratio * 100.0
        ));

        let total_impl = overall.inherent_methods + overall.local_trait_methods;
        if total_impl > 0 && overall.trait_ratio == 0.0 {
            lines.push(format!(
                "Trait methods:         {} / {} impl methods are trait-bound  (0.0%)",
                overall.local_trait_methods, total_impl,
            ));
        } else if total_impl == 0 {
            lines.push("Trait methods:         N/A    (no impl methods)".to_string());
        } else {
            lines.push(format!(
                "Trait methods:         {} / {} impl methods are trait-bound  ({:.1}%)",
                overall.local_trait_methods,
                total_impl,
                overall.trait_ratio * 100.0
            ));
        }

        lines.push(format!(
            "Hidden deps:           avg {:.2}  — {:.1}% clean  ({:.1}% avg contribution)",
            overall.total_functions as f64 - overall.avg_contribution * overall.total_functions as f64,
            overall.clean_fn_ratio * 100.0,
            overall.avg_contribution * 100.0,
        ));

        lines.push("\nPer module:".to_string());
        for module in &report.modules {
            lines.push(self.render_module_line(module));
        }

        if !report.offenders.is_empty() {
            lines.push(format!(
                "\nOffenders (score < {}):",
                report.offender_threshold
            ));
            for offender in &report.offenders {
                lines.push(format!(
                    "  {:<30}  grip: {:>3}  ❌",
                    offender.path, offender.grip_score,
                ));
            }
        }

        if self.verbose && !report.functions.is_empty() {
            lines.push("\nPer-function detail:".to_string());
            let mut sorted = report.functions.clone();
            sorted.sort_by(|a, b| a.file.cmp(&b.file).then(a.name.cmp(&b.name)));
            let mut current_file = String::new();
            for f in &sorted {
                if f.file != current_file {
                    current_file = f.file.clone();
                    lines.push(format!("\n  {}:", current_file));
                }
                let marker = contribution_marker(f.hidden_deps);
                let contr = crate::contribution_schedule::contribution(f.is_pure, f.has_trait_seam, f.dep_weight);
                let labels = f.hidden_dep_labels.join(", ");
                lines.push(format!(
                    "    {:<35}  pure: {:>5}  seam: {:>5}  hidden: {:>2}  contr: {:>5.0}%  [{}]  {}",
                    f.name,
                    if f.is_pure { "yes" } else { "no" },
                    if f.has_trait_seam { "yes" } else { "no " },
                    f.hidden_deps,
                    contr * 100.0,
                    if labels.is_empty() { "-" } else { &labels },
                    marker,
                ));
            }
        }

        lines.join("\n")
    }

    fn render_module_line(&self, module: &ModuleStats) -> String {
        let marker = self.module_marker(module.grip_score);
        let total_impl = module.inherent_methods + module.local_trait_methods;
        let traits_display = if total_impl == 0 {
            "   N/A".to_string()
        } else {
            format!("{:>5.1}%", module.trait_ratio * 100.0)
        };
        format!(
            "  {:<30}  grip: {:>3}   pure: {:>5.1}%   pub: {:>3}   traits: {}   clean: {:>5.1}%  {}",
            module.path,
            module.grip_score,
            module.pure_ratio * 100.0,
            module.public_items,
            traits_display,
            module.clean_fn_ratio * 100.0,
            marker,
        )
    }

    fn module_marker(&self, score: u32) -> &'static str {
        if score < 40 {
            "❌"
        } else if score < 70 {
            "⚠️"
        } else {
            ""
        }
    }
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
