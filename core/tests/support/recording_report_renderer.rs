// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;
use grip::reporting::grip_report::GripReport;
use grip::traits::reporter::Reporter;
use std::cell::RefCell;
use std::rc::Rc;

// A reporter that writes nothing a reader would want: it answers with a fixed
// text and hands every report it was given to whoever holds the other end of
// `handed`.
pub struct RecordingReportRenderer {
    handed: Rc<RefCell<Vec<GripReport>>>,
}

impl RecordingReportRenderer {
    pub fn new(handed: Rc<RefCell<Vec<GripReport>>>) -> Self {
        Self { handed }
    }
}

impl Reporter for RecordingReportRenderer {
    fn render(&self, report: &GripReport) -> Result<String> {
        self.handed.borrow_mut().push(report.clone());
        Ok(String::from("rendered"))
    }

    fn write(&self, report: &GripReport) -> Result<()> {
        self.render(report).map(|_| ())
    }
}
