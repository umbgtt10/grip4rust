// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::reporting::grip_report::GripReport;
use crate::reporting::render_failure::RenderFailure;

pub trait ReportRenderer {
    fn render(&self, report: &GripReport) -> Result<String, RenderFailure>;
}
