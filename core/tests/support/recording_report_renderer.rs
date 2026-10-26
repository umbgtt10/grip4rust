// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::not_read::NotRead;
use grip::reporting::grip_report::GripReport;
use grip::reporting::render_failure::RenderFailure;
use grip::traits::report_renderer::ReportRenderer;
use std::cell::RefCell;
use std::rc::Rc;

// A renderer that answers with a fixed text, or fails, and hands every report it
// was given to whoever holds the other end of `handed`.
pub struct RecordingReportRenderer {
    fails: bool,
    handed: Rc<RefCell<Vec<GripReport>>>,
}

impl RecordingReportRenderer {
    pub fn new(handed: Rc<RefCell<Vec<GripReport>>>) -> Self {
        Self {
            fails: false,
            handed,
        }
    }

    pub fn failing(handed: Rc<RefCell<Vec<GripReport>>>) -> Self {
        Self {
            fails: true,
            handed,
        }
    }
}

impl ReportRenderer for RecordingReportRenderer {
    fn render(&self, report: &GripReport) -> Result<String, RenderFailure> {
        self.handed.borrow_mut().push(report.clone());
        if self.fails {
            return Err(RenderFailure {
                cause: Box::new(NotRead),
            });
        }
        Ok(String::from("rendered"))
    }
}
