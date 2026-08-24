// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::grip::grip_report::GripReport;
use serde_json::from_str;

pub struct GripReportParser;

impl GripReportParser {
    pub fn new() -> Self {
        Self
    }

    // `cargo run --quiet` puts nothing of its own on stdout, so unlike
    // crap4rust's report this one needs no searching for where the JSON starts.
    pub fn parse(&self, stdout: &str) -> Result<GripReport, String> {
        from_str(stdout).map_err(|error| format!("could not parse grip4rust JSON: {error}"))
    }
}

impl Default for GripReportParser {
    fn default() -> Self {
        Self::new()
    }
}
