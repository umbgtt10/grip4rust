// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::fixture_analysis::FixtureAnalysis;
use serde_json::Value;

pub struct DataOnlyAnalysis;

impl DataOnlyAnalysis {
    #[must_use]
    pub fn report() -> Value {
        FixtureAnalysis::new("data_only").report()
    }
}
