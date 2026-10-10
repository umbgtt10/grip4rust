// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::fixture_analysis::FixtureAnalysis;
use serde_json::Value;

pub struct SloppyCalcAnalysis;

impl SloppyCalcAnalysis {
    #[must_use]
    pub fn report() -> Value {
        FixtureAnalysis::new("sloppy_calc").report()
    }
}
