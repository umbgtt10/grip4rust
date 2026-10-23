// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemCounts {
    total_functions: usize,
    pure_functions: usize,
    total_items: usize,
    public_items: usize,
    inherent_methods: usize,
    inherent_impure: usize,
    local_trait_methods: usize,
    local_trait_impure: usize,
    total_contribution: f64,
    clean_functions: usize,
}

impl ItemCounts {
    #[must_use]
    pub fn clean_functions(&self) -> usize {
        self.clean_functions
    }

    #[must_use]
    pub fn inherent_impure(&self) -> usize {
        self.inherent_impure
    }

    #[must_use]
    pub fn inherent_methods(&self) -> usize {
        self.inherent_methods
    }

    #[must_use]
    pub fn local_trait_impure(&self) -> usize {
        self.local_trait_impure
    }

    #[must_use]
    pub fn local_trait_methods(&self) -> usize {
        self.local_trait_methods
    }

    #[must_use]
    pub fn merged(self, other: &ItemCounts) -> Self {
        Self {
            total_functions: self.total_functions + other.total_functions,
            pure_functions: self.pure_functions + other.pure_functions,
            total_items: self.total_items + other.total_items,
            public_items: self.public_items + other.public_items,
            inherent_methods: self.inherent_methods + other.inherent_methods,
            inherent_impure: self.inherent_impure + other.inherent_impure,
            local_trait_methods: self.local_trait_methods + other.local_trait_methods,
            local_trait_impure: self.local_trait_impure + other.local_trait_impure,
            total_contribution: self.total_contribution + other.total_contribution,
            clean_functions: self.clean_functions + other.clean_functions,
        }
    }

    #[must_use]
    pub fn public_items(&self) -> usize {
        self.public_items
    }

    #[must_use]
    pub fn pure_functions(&self) -> usize {
        self.pure_functions
    }

    pub fn record_function(&mut self, contribution: f64, is_pure: bool) {
        self.total_functions += 1;
        self.total_contribution += contribution;
        if contribution == 1.0 {
            self.clean_functions += 1;
        }
        if is_pure {
            self.pure_functions += 1;
        }
    }

    // The counts know how to record an impl method; the collector only decides
    // which kind it saw.
    pub fn record_impl_method(&mut self, is_trait_impl: bool, is_pure: bool) {
        if is_trait_impl {
            self.local_trait_methods += 1;
            if !is_pure {
                self.local_trait_impure += 1;
            }
        } else {
            self.inherent_methods += 1;
            if !is_pure {
                self.inherent_impure += 1;
            }
        }
    }

    pub fn record_item(&mut self, is_public: bool) {
        self.total_items += 1;
        if is_public {
            self.public_items += 1;
        }
    }

    #[must_use]
    pub fn total_contribution(&self) -> f64 {
        self.total_contribution
    }

    #[must_use]
    pub fn total_functions(&self) -> usize {
        self.total_functions
    }

    #[must_use]
    pub fn total_items(&self) -> usize {
        self.total_items
    }

    #[must_use]
    pub fn with_clean_functions(self, clean_functions: usize) -> Self {
        Self {
            clean_functions,
            ..self
        }
    }

    #[must_use]
    pub fn with_inherent_impure(self, inherent_impure: usize) -> Self {
        Self {
            inherent_impure,
            ..self
        }
    }

    #[must_use]
    pub fn with_inherent_methods(self, inherent_methods: usize) -> Self {
        Self {
            inherent_methods,
            ..self
        }
    }

    #[must_use]
    pub fn with_local_trait_impure(self, local_trait_impure: usize) -> Self {
        Self {
            local_trait_impure,
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
    pub fn with_pure_functions(self, pure_functions: usize) -> Self {
        Self {
            pure_functions,
            ..self
        }
    }

    #[must_use]
    pub fn with_total_contribution(self, total_contribution: f64) -> Self {
        Self {
            total_contribution,
            ..self
        }
    }

    #[must_use]
    pub fn with_total_functions(self, total_functions: usize) -> Self {
        Self {
            total_functions,
            ..self
        }
    }

    #[must_use]
    pub fn with_total_items(self, total_items: usize) -> Self {
        Self {
            total_items,
            ..self
        }
    }
}
