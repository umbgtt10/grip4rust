// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemCounts {
    pub total_functions: usize,
    pub pure_functions: usize,
    pub public_functions: usize,
    pub pubcrate_functions: usize,
    pub public_structs: usize,
    pub public_traits: usize,
    pub public_enums: usize,
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
    #[must_use]
    pub fn merged(self, other: &ItemCounts) -> Self {
        Self {
            total_functions: self.total_functions + other.total_functions,
            pure_functions: self.pure_functions + other.pure_functions,
            public_functions: self.public_functions + other.public_functions,
            pubcrate_functions: self.pubcrate_functions + other.pubcrate_functions,
            public_structs: self.public_structs + other.public_structs,
            public_traits: self.public_traits + other.public_traits,
            public_enums: self.public_enums + other.public_enums,
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
