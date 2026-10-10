// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;
use grip::reporting::grip_report::GripReport;
use grip::traits::reporter::Reporter;
use serde_json::to_string_pretty;
use std::cell::RefCell;
use std::rc::Rc;

pub struct CaptureReporter {
    captured: Rc<RefCell<String>>,
}

impl CaptureReporter {
    #[must_use]
    pub fn new(captured: Rc<RefCell<String>>) -> Self {
        Self { captured }
    }
}

impl Reporter for CaptureReporter {
    fn render(&self, report: &GripReport) -> Result<String> {
        let json = to_string_pretty(report)?;
        *self.captured.borrow_mut() = json.clone();
        Ok(json)
    }

    fn write(&self, report: &GripReport) -> Result<()> {
        let json = self.render(report)?;
        print!("{json}");
        Ok(())
    }
}
