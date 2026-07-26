// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use syn::visit::Visit;

pub const KNOWN_STD_VALUE_TYPES: &[&str] = &[
    "Vec", "HashMap", "HashSet", "BTreeMap", "BTreeSet", "VecDeque", "String", "Option", "Cell",
    "RefCell",
];

pub fn field_type_head(ty: &syn::Type) -> String {
    if let syn::Type::Path(type_path) = ty {
        type_path
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default()
    } else {
        String::new()
    }
}

#[derive(Debug, Default)]
pub struct StructRegistry {
    fields_by_struct: HashMap<String, Vec<String>>,
}

impl StructRegistry {
    #[must_use]
    pub fn build(files: &[(PathBuf, String)]) -> Self {
        let mut registry = Self::default();
        for (_, source) in files {
            if let Ok(syntax) = syn::parse_file(source) {
                for item in &syntax.items {
                    registry.visit_item(item);
                }
            }
        }
        registry
    }

    #[must_use]
    pub fn is_transitive_value_type(&self, type_name: &str) -> bool {
        self.resolve(type_name, &mut HashSet::new())
    }

    fn resolve(&self, type_name: &str, visiting: &mut HashSet<String>) -> bool {
        if KNOWN_STD_VALUE_TYPES.contains(&type_name) {
            return true;
        }
        if !visiting.insert(type_name.to_string()) {
            return false;
        }
        let result = self
            .fields_by_struct
            .get(type_name)
            .is_some_and(|fields| fields.iter().all(|field| self.resolve(field, visiting)));
        visiting.remove(type_name);
        result
    }
}

impl<'ast> Visit<'ast> for StructRegistry {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        match item {
            syn::Item::Struct(item_struct) => self.record_struct(item_struct),
            syn::Item::Mod(item_mod) => self.visit_inline_mod(item_mod),
            _ => {}
        }
    }
}

impl StructRegistry {
    fn record_struct(&mut self, item_struct: &syn::ItemStruct) {
        let name = item_struct.ident.to_string();
        let fields = item_struct
            .fields
            .iter()
            .map(|f| field_type_head(&f.ty))
            .collect();
        self.fields_by_struct.insert(name, fields);
    }

    fn visit_inline_mod(&mut self, item_mod: &syn::ItemMod) {
        if let Some((_, items)) = &item_mod.content {
            for inner in items {
                self.visit_item(inner);
            }
        }
    }
}
