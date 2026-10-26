// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::reporting::grip_report::GripReport;
use grip::reporting::render_failure::RenderFailure;
use grip::traits::report_renderer::ReportRenderer;
use grip_cli::system::json_report_renderer::JsonReportRenderer;
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

impl ReportRenderer for CaptureReporter {
    fn render(&self, report: &GripReport) -> Result<String, RenderFailure> {
        let json = JsonReportRenderer::new().render(report)?;
        *self.captured.borrow_mut() = json.clone();
        Ok(json)
    }
}
