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

pub(crate) fn self_ty_name(ty: &syn::Type) -> String {
    if let syn::Type::Path(type_path) = ty {
        type_path
            .path
            .segments
            .first()
            .map(|s| s.ident.to_string())
            .unwrap_or_default()
    } else {
        String::new()
    }
}

pub(crate) fn is_trait_object_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::TraitObject(_) => true,
        syn::Type::Reference(r) => is_trait_object_type(&r.elem),
        syn::Type::Path(p) => p.path.segments.iter().any(|seg| match &seg.arguments {
            syn::PathArguments::AngleBracketed(args) => args.args.iter().any(|arg| match arg {
                syn::GenericArgument::Type(t) => is_trait_object_type(t),
                _ => false,
            }),
            _ => false,
        }),
        _ => false,
    }
}

#[derive(Debug, Default)]
pub struct StructRegistry {
    fields_by_struct: HashMap<String, Vec<String>>,
    concrete_fields_by_struct: HashMap<String, HashMap<String, String>>,
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

    #[must_use]
    pub fn concrete_fields_of(&self, type_name: &str) -> HashMap<String, String> {
        self.concrete_fields_by_struct
            .get(type_name)
            .cloned()
            .unwrap_or_default()
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
        let mut all_field_types = Vec::new();
        let mut concrete_fields = HashMap::new();
        for f in &item_struct.fields {
            let type_head = field_type_head(&f.ty);
            all_field_types.push(type_head.clone());
            if !is_trait_object_type(&f.ty) {
                if let Some(field_name) = f.ident.as_ref().map(ToString::to_string) {
                    concrete_fields.insert(field_name, type_head);
                }
            }
        }
        self.fields_by_struct.insert(name.clone(), all_field_types);
        self.concrete_fields_by_struct.insert(name, concrete_fields);
    }

    fn visit_inline_mod(&mut self, item_mod: &syn::ItemMod) {
        if let Some((_, items)) = &item_mod.content {
            for inner in items {
                self.visit_item(inner);
            }
        }
    }
}
