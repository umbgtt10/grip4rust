// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::function_info::FunctionInfo;
use grip::reporting::grip_report::GripReport;
use grip::reporting::module_stats::ModuleStats;
use grip::reporting::offender::Offender;
use grip::reporting::overall_stats::OverallStats;
use grip::reporting::render_failure::RenderFailure;
use grip::traits::report_renderer::ReportRenderer;
use serde::Serialize;
use serde_json::to_string_pretty;

// The machine surface. Its records are its own: the core's carry no serde, so
// the shape of the JSON is written down here, field for field in the order the
// document has always had.
#[derive(Serialize)]
struct JsonReport<'a> {
    pub version: &'a str,
    pub target: &'a str,
    pub overall: JsonOverall,
    pub modules: Vec<JsonModule<'a>>,
    pub offenders: Vec<JsonOffender<'a>>,
    pub offender_threshold: u32,
    pub functions: Vec<JsonFunction<'a>>,
}

#[derive(Serialize)]
struct JsonOverall {
    pub grip_score: Option<u32>,
    pub public_items: usize,
    pub total_functions: usize,
    pub pure_functions: usize,
    pub pure_ratio: f64,
    pub public_ratio: f64,
    pub inherent_methods: usize,
    pub local_trait_methods: usize,
    pub trait_ratio: f64,
    pub avg_contribution: f64,
    pub clean_fn_ratio: f64,
    pub grip_absolute_total: f64,
}

#[derive(Serialize)]
struct JsonModule<'a> {
    pub path: &'a str,
    pub grip_score: Option<u32>,
    pub pure_ratio: f64,
    pub public_items: usize,
    pub total_functions: usize,
    pub pure_functions: usize,
    pub public_ratio: f64,
    pub inherent_methods: usize,
    pub local_trait_methods: usize,
    pub trait_ratio: f64,
    pub avg_contribution: f64,
    pub clean_fn_ratio: f64,
    pub grip_absolute_total: f64,
}

#[derive(Serialize)]
struct JsonOffender<'a> {
    pub path: &'a str,
    pub grip_score: u32,
}

#[derive(Serialize)]
struct JsonFunction<'a> {
    pub name: &'a str,
    pub file: &'a str,
    pub is_pure: bool,
    pub is_public: bool,
    pub hidden_deps: usize,
    pub has_trait_seam: bool,
    pub dep_weight: f64,
    pub hidden_dep_labels: &'a [String],
    pub grip_absolute: f64,
    pub grip_normalized: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct JsonReportRenderer;

impl JsonReportRenderer {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn function(info: &FunctionInfo) -> JsonFunction<'_> {
        JsonFunction {
            name: &info.name,
            file: &info.file,
            is_pure: info.is_pure,
            is_public: info.is_public,
            hidden_deps: info.hidden_deps,
            has_trait_seam: info.has_trait_seam,
            dep_weight: info.dep_weight,
            hidden_dep_labels: &info.hidden_dep_labels,
            grip_absolute: info.grip_absolute,
            grip_normalized: info.grip_normalized,
        }
    }

    fn module(stats: &ModuleStats) -> JsonModule<'_> {
        JsonModule {
            path: &stats.path,
            grip_score: stats.grip_score,
            pure_ratio: stats.pure_ratio,
            public_items: stats.public_items,
            total_functions: stats.total_functions,
            pure_functions: stats.pure_functions,
            public_ratio: stats.public_ratio,
            inherent_methods: stats.inherent_methods,
            local_trait_methods: stats.local_trait_methods,
            trait_ratio: stats.trait_ratio,
            avg_contribution: stats.avg_contribution,
            clean_fn_ratio: stats.clean_fn_ratio,
            grip_absolute_total: stats.grip_absolute_total,
        }
    }

    fn offender(offender: &Offender) -> JsonOffender<'_> {
        JsonOffender {
            path: &offender.path,
            grip_score: offender.grip_score,
        }
    }

    fn overall(stats: &OverallStats) -> JsonOverall {
        JsonOverall {
            grip_score: stats.grip_score,
            public_items: stats.public_items,
            total_functions: stats.total_functions,
            pure_functions: stats.pure_functions,
            pure_ratio: stats.pure_ratio,
            public_ratio: stats.public_ratio,
            inherent_methods: stats.inherent_methods,
            local_trait_methods: stats.local_trait_methods,
            trait_ratio: stats.trait_ratio,
            avg_contribution: stats.avg_contribution,
            clean_fn_ratio: stats.clean_fn_ratio,
            grip_absolute_total: stats.grip_absolute_total,
        }
    }
}

impl ReportRenderer for JsonReportRenderer {
    fn render(&self, report: &GripReport) -> Result<String, RenderFailure> {
        let document = JsonReport {
            version: &report.version,
            target: &report.target,
            overall: Self::overall(&report.overall),
            modules: report.modules.iter().map(Self::module).collect(),
            offenders: report.offenders.iter().map(Self::offender).collect(),
            offender_threshold: report.offender_threshold,
            functions: report.functions.iter().map(Self::function).collect(),
        };
        to_string_pretty(&document).map_err(|cause| RenderFailure {
            cause: Box::new(cause),
        })
    }
}
