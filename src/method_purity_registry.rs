// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use syn::visit::Visit;

use crate::function_purity::FunctionPurity;
use crate::hidden_dep_finder::HiddenDepFinder;
use crate::struct_registry::{StructRegistry, self_ty_name};

#[derive(Debug, Default)]
pub struct MethodPurityRegistry {
    pure_methods: HashSet<(String, String)>,
}

impl MethodPurityRegistry {
    #[must_use]
    pub fn build(files: &[(PathBuf, String)], struct_registry: &StructRegistry) -> Self {
        let mut registry = Self::default();
        let no_nested_trust = Self::default();
        for (_, source) in files {
            if let Ok(syntax) = syn::parse_file(source) {
                for item in &syntax.items {
                    registry.visit_top_level_item(item, struct_registry, &no_nested_trust);
                }
            }
        }
        registry
    }

    #[must_use]
    pub fn is_known_pure_method(&self, type_name: &str, method: &str) -> bool {
        self.pure_methods
            .contains(&(type_name.to_string(), method.to_string()))
    }

    fn visit_top_level_item(
        &mut self,
        item: &syn::Item,
        struct_registry: &StructRegistry,
        no_nested_trust: &MethodPurityRegistry,
    ) {
        match item {
            syn::Item::Impl(item_impl) => {
                self.record_impl(item_impl, struct_registry, no_nested_trust);
            }
            syn::Item::Mod(item_mod) => {
                if let Some((_, items)) = &item_mod.content {
                    for inner in items {
                        self.visit_top_level_item(inner, struct_registry, no_nested_trust);
                    }
                }
            }
            _ => {}
        }
    }

    fn record_impl(
        &mut self,
        item_impl: &syn::ItemImpl,
        struct_registry: &StructRegistry,
        no_nested_trust: &MethodPurityRegistry,
    ) {
        if item_impl.trait_.is_some() {
            return;
        }
        let type_name = self_ty_name(&item_impl.self_ty);
        let concrete_fields = struct_registry.concrete_fields_of(&type_name);
        for item in &item_impl.items {
            if let syn::ImplItem::Fn(method) = item {
                self.record_method(
                    &type_name,
                    method,
                    struct_registry,
                    no_nested_trust,
                    &concrete_fields,
                );
            }
        }
    }

    fn record_method(
        &mut self,
        type_name: &str,
        method: &syn::ImplItemFn,
        struct_registry: &StructRegistry,
        no_nested_trust: &MethodPurityRegistry,
        concrete_fields: &HashMap<String, String>,
    ) {
        if !Self::has_pure_signature(method) {
            return;
        }
        let mut finder = HiddenDepFinder::new(struct_registry, no_nested_trust);
        finder.set_concrete_fields(concrete_fields.clone());
        finder.visit_block(&method.block);
        if finder.count == 0 {
            self.pure_methods
                .insert((type_name.to_string(), method.sig.ident.to_string()));
        }
    }

    fn has_pure_signature(method: &syn::ImplItemFn) -> bool {
        !FunctionPurity::has_mut_param(&method.sig)
            && !FunctionPurity::is_unit_return(&method.sig)
            && method.sig.unsafety.is_none()
            && !FunctionPurity::has_unsafe_block(&method.block)
            && !FunctionPurity::has_io_call(&method.block)
    }
}
