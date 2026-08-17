// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemCounts {
    pub total_functions: usize,
    pub pure_functions: usize,
    pub total_items: usize,
    pub public_items: usize,
    pub inherent_methods: usize,
    pub inherent_impure: usize,
    pub local_trait_methods: usize,
    pub local_trait_impure: usize,
    pub total_contribution: f64,
    pub clean_functions: usize,
}

impl ItemCounts {
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
}
