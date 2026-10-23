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
    version: String,
    target: String,
    overall: OverallStats,
    modules: Vec<ModuleStats>,
    offenders: Vec<Offender>,
    offender_threshold: u32,
    functions: Vec<FunctionInfo>,
}

impl GripReport {
    #[must_use]
    pub fn new(
        version: String,
        target: String,
        overall: OverallStats,
        modules: Vec<ModuleStats>,
        offenders: Vec<Offender>,
        offender_threshold: u32,
        functions: Vec<FunctionInfo>,
    ) -> Self {
        Self {
            version,
            target,
            overall,
            modules,
            offenders,
            offender_threshold,
            functions,
        }
    }

    #[must_use]
    pub fn functions(&self) -> &[FunctionInfo] {
        &self.functions
    }

    #[must_use]
    pub fn modules(&self) -> &[ModuleStats] {
        &self.modules
    }

    #[must_use]
    pub fn offender_threshold(&self) -> u32 {
        self.offender_threshold
    }

    #[must_use]
    pub fn offenders(&self) -> &[Offender] {
        &self.offenders
    }

    #[must_use]
    pub fn overall(&self) -> &OverallStats {
        &self.overall
    }

    #[must_use]
    pub fn target(&self) -> &str {
        &self.target
    }

    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
}
