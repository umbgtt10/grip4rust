// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::grip::grip_overall::GripOverall;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GripReport {
    pub overall: GripOverall,
}

impl GripReport {
    pub fn score(&self) -> i64 {
        self.overall.grip_score
    }

    // A floor rather than a ceiling: grip measures how gripped the code is, so
    // a higher score is better and the gate fails below the line. The opposite
    // of every other threshold in stage 2.
    pub fn meets(&self, floor: i64) -> bool {
        self.score() >= floor
    }

    pub fn summary(&self, floor: i64) -> String {
        format!("grip score: {} / 100  (floor: {floor})", self.score())
    }
}
