// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::analysis::struct_registry::KNOWN_STD_VALUE_TYPES;
use std::collections::{HashMap, HashSet};

pub struct TransitiveValueTypeResolver<'fields> {
    fields_by_struct: &'fields HashMap<String, Vec<String>>,
    visiting: HashSet<String>,
}

impl<'fields> TransitiveValueTypeResolver<'fields> {
    #[must_use]
    pub fn new(fields_by_struct: &'fields HashMap<String, Vec<String>>) -> Self {
        Self {
            fields_by_struct,
            visiting: HashSet::new(),
        }
    }

    pub fn resolve(&mut self, type_name: &str) -> bool {
        if KNOWN_STD_VALUE_TYPES.contains(&type_name) {
            return true;
        }
        if !self.visiting.insert(type_name.to_string()) {
            return false;
        }
        let result = self
            .fields_by_struct
            .get(type_name)
            .is_some_and(|fields| fields.iter().all(|field| self.resolve(field)));
        self.visiting.remove(type_name);
        result
    }
}
