// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GripOverall {
    grip_score: i64,
}

impl GripOverall {
    pub fn grip_score(&self) -> i64 {
        self.grip_score
    }
}
