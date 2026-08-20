// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;

use crate::reporting::grip_report::GripReport;

pub trait Reporter {
    fn render(&self, report: &GripReport) -> Result<String>;
    fn write(&self, report: &GripReport) -> Result<()>;
}
