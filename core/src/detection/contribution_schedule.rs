// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, Copy, Default)]
pub struct ContributionSchedule;

impl ContributionSchedule {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn contribution(&self, is_pure: bool, has_trait_seam: bool, dep_weight: f64) -> f64 {
        if dep_weight >= 1.0 {
            return 0.0;
        }
        let base = match (is_pure, has_trait_seam) {
            (true, true) => 1.00,
            (true, false) => 0.95,
            (false, true) => 0.85,
            (false, false) => 0.15,
        };
        base * (1.0 - dep_weight)
    }
}
