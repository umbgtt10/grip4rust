// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt;

// Why grip4rust's output yielded no report: what it wrote did not parse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GripReportFailure {
    reason: String,
}

impl GripReportFailure {
    pub fn new(reason: String) -> Self {
        Self { reason }
    }
}

impl fmt::Display for GripReportFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "could not parse grip4rust JSON: {}", self.reason)
    }
}
