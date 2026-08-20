// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::analysis::function_info::FunctionInfo;
use crate::reporting::module_stats::ModuleStats;
use crate::reporting::offender::Offender;
use crate::reporting::overall_stats::OverallStats;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GripReport {
    pub version: String,
    pub target: String,
    pub overall: OverallStats,
    pub modules: Vec<ModuleStats>,
    pub offenders: Vec<Offender>,
    pub offender_threshold: u32,
    pub functions: Vec<FunctionInfo>,
}
