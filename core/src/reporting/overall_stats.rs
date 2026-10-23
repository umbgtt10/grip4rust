// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OverallStats {
    grip_score: Option<u32>,
    public_items: usize,
    total_functions: usize,
    pure_functions: usize,
    pure_ratio: f64,
    public_ratio: f64,
    inherent_methods: usize,
    local_trait_methods: usize,
    trait_ratio: f64,
    avg_contribution: f64,
    clean_fn_ratio: f64,
    grip_absolute_total: f64,
}

impl OverallStats {
    #[must_use]
    pub fn avg_contribution(&self) -> f64 {
        self.avg_contribution
    }

    #[must_use]
    pub fn clean_fn_ratio(&self) -> f64 {
        self.clean_fn_ratio
    }

    #[must_use]
    pub fn grip_absolute_total(&self) -> f64 {
        self.grip_absolute_total
    }

    #[must_use]
    pub fn grip_score(&self) -> Option<u32> {
        self.grip_score
    }

    #[must_use]
    pub fn inherent_methods(&self) -> usize {
        self.inherent_methods
    }

    #[must_use]
    pub fn local_trait_methods(&self) -> usize {
        self.local_trait_methods
    }

    #[must_use]
    pub fn public_items(&self) -> usize {
        self.public_items
    }

    #[must_use]
    pub fn public_ratio(&self) -> f64 {
        self.public_ratio
    }

    #[must_use]
    pub fn pure_functions(&self) -> usize {
        self.pure_functions
    }

    #[must_use]
    pub fn pure_ratio(&self) -> f64 {
        self.pure_ratio
    }

    #[must_use]
    pub fn total_functions(&self) -> usize {
        self.total_functions
    }

    #[must_use]
    pub fn trait_ratio(&self) -> f64 {
        self.trait_ratio
    }

    #[must_use]
    pub fn with_avg_contribution(self, avg_contribution: f64) -> Self {
        Self {
            avg_contribution,
            ..self
        }
    }

    #[must_use]
    pub fn with_clean_fn_ratio(self, clean_fn_ratio: f64) -> Self {
        Self {
            clean_fn_ratio,
            ..self
        }
    }

    #[must_use]
    pub fn with_grip_absolute_total(self, grip_absolute_total: f64) -> Self {
        Self {
            grip_absolute_total,
            ..self
        }
    }

    #[must_use]
    pub fn with_grip_score(self, grip_score: Option<u32>) -> Self {
        Self { grip_score, ..self }
    }

    #[must_use]
    pub fn with_inherent_methods(self, inherent_methods: usize) -> Self {
        Self {
            inherent_methods,
            ..self
        }
    }

    #[must_use]
    pub fn with_local_trait_methods(self, local_trait_methods: usize) -> Self {
        Self {
            local_trait_methods,
            ..self
        }
    }

    #[must_use]
    pub fn with_public_items(self, public_items: usize) -> Self {
        Self {
            public_items,
            ..self
        }
    }

    #[must_use]
    pub fn with_public_ratio(self, public_ratio: f64) -> Self {
        Self {
            public_ratio,
            ..self
        }
    }

    #[must_use]
    pub fn with_pure_functions(self, pure_functions: usize) -> Self {
        Self {
            pure_functions,
            ..self
        }
    }

    #[must_use]
    pub fn with_pure_ratio(self, pure_ratio: f64) -> Self {
        Self { pure_ratio, ..self }
    }

    #[must_use]
    pub fn with_total_functions(self, total_functions: usize) -> Self {
        Self {
            total_functions,
            ..self
        }
    }

    #[must_use]
    pub fn with_trait_ratio(self, trait_ratio: f64) -> Self {
        Self {
            trait_ratio,
            ..self
        }
    }
}
