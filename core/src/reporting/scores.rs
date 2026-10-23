// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scores {
    grip_score: Option<u32>,
    pure_ratio: f64,
    public_ratio: f64,
    trait_ratio: f64,
    avg_contribution: f64,
    clean_fn_ratio: f64,
}

impl Scores {
    #[must_use]
    pub fn new(
        grip_score: Option<u32>,
        pure_ratio: f64,
        public_ratio: f64,
        trait_ratio: f64,
        avg_contribution: f64,
        clean_fn_ratio: f64,
    ) -> Self {
        Self {
            grip_score,
            pure_ratio,
            public_ratio,
            trait_ratio,
            avg_contribution,
            clean_fn_ratio,
        }
    }

    #[must_use]
    pub fn avg_contribution(&self) -> f64 {
        self.avg_contribution
    }

    #[must_use]
    pub fn clean_fn_ratio(&self) -> f64 {
        self.clean_fn_ratio
    }

    #[must_use]
    pub fn grip_score(&self) -> Option<u32> {
        self.grip_score
    }

    #[must_use]
    pub fn public_ratio(&self) -> f64 {
        self.public_ratio
    }

    #[must_use]
    pub fn pure_ratio(&self) -> f64 {
        self.pure_ratio
    }

    #[must_use]
    pub fn trait_ratio(&self) -> f64 {
        self.trait_ratio
    }
}
