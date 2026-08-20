// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::reporting::grip_report::GripReport;
use crate::reporting::module_stats::ModuleStats;
use crate::reporting::offenders_renderer::OffendersRenderer;
use crate::reporting::overall_summary_renderer::OverallSummaryRenderer;
use crate::reporting::verbose_functions_renderer::VerboseFunctionsRenderer;
use crate::traits::reporter::Reporter;
use anyhow::Result;
use serde_json::to_string_pretty;
use std::io::{Write, stdout};

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
            Ok(to_string_pretty(report)?)
        } else {
            Ok(self.render_human(report))
        }
    }

    fn write(&self, report: &GripReport) -> Result<()> {
        let out = self.render(report)?;
        stdout().write_all(out.as_bytes())?;
        stdout().write_all(b"\n")?;
        Ok(())
    }
}

impl StdoutReporter {
    fn render_human(&self, report: &GripReport) -> String {
        let mut lines = vec![self.render_header(report)];

        lines.extend(OverallSummaryRenderer::new().render(&report.overall));

        lines.push("\nPer module:".to_string());
        for module in &report.modules {
            lines.push(self.render_module_line(module));
        }

        lines.extend(OffendersRenderer::new().render(&report.offenders, report.offender_threshold));
        lines.extend(VerboseFunctionsRenderer::new(self.verbose).render(&report.functions));

        lines.join("\n")
    }

    fn render_header(&self, report: &GripReport) -> String {
        let target = &report.target;
        let version = &report.version;
        let header = if self.verbose {
            format!("grip {version} — {target} — verbose")
        } else {
            format!("cargo-grip4rust {version} -- {target}")
        };
        format!("{header}\n══════════════════════════════════════════════════════\n")
    }

    fn render_module_line(&self, module: &ModuleStats) -> String {
        let marker = self.module_marker(module.grip_score);
        let total_impl = module.inherent_methods + module.local_trait_methods;
        let traits_display = if total_impl == 0 {
            "   N/A".to_string()
        } else {
            format!("{:>5.1}%", module.trait_ratio * 100.0)
        };
        let grip_display = match module.grip_score {
            Some(score) => format!("{score:>3}"),
            None => "N/A".to_string(),
        };
        format!(
            "  {:<30}  grip: {}   pure: {:>5.1}%   pub: {:>3}   traits: {}   clean: {:>5.1}%  {}",
            module.path,
            grip_display,
            module.pure_ratio * 100.0,
            module.public_items,
            traits_display,
            module.clean_fn_ratio * 100.0,
            marker,
        )
    }

    fn module_marker(&self, score: Option<u32>) -> &'static str {
        match score {
            Some(score) if score < 40 => "❌",
            Some(score) if score < 70 => "⚠️",
            _ => "",
        }
    }
}
