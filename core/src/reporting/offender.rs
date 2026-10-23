// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offender {
    path: String,
    grip_score: u32,
}

impl Offender {
    #[must_use]
    pub fn new(path: String, grip_score: u32) -> Self {
        Self { path, grip_score }
    }

    #[must_use]
    pub fn grip_score(&self) -> u32 {
        self.grip_score
    }

    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }
}
