// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::analysis::function_info::FunctionInfo;
use crate::analysis::item_classifier::ItemClassifier;
use crate::analysis::item_counts::ItemCounts;
use crate::analysis::method_purity_registry::MethodPurityRegistry;
use crate::analysis::struct_registry::{
    StructRegistry, field_type_head, is_trait_object_type, self_ty_name,
};
use crate::analysis::visibility_level::VisibilityLevel;
use crate::detection::contribution_schedule::ContributionSchedule;
use crate::detection::hidden_dep_finder::HiddenDepFinder;
use std::collections::HashMap;
use std::path::Path;
use syn::visit::Visit;
use syn::{
    Block, ImplItem, ImplItemFn, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait,
    parse_file,
};

#[derive(Debug)]
pub struct Collector<'a> {
    counts: ItemCounts,
    functions: Vec<FunctionInfo>,
    current_file: String,
    struct_concrete_fields: HashMap<String, HashMap<String, String>>,
    contribution_schedule: ContributionSchedule,
    registry: &'a StructRegistry,
    method_purity: &'a MethodPurityRegistry,
}

impl<'a> Collector<'a> {
    fn new(
        file: String,
        registry: &'a StructRegistry,
        method_purity: &'a MethodPurityRegistry,
    ) -> Self {
        Self {
            counts: ItemCounts::default(),
            functions: Vec::new(),
            current_file: file,
            struct_concrete_fields: HashMap::new(),
            contribution_schedule: ContributionSchedule::new(),
            registry,
            method_purity,
        }
    }

    pub fn collect(
        source: &str,
        path: &Path,
        registry: &'a StructRegistry,
        method_purity: &'a MethodPurityRegistry,
    ) -> (ItemCounts, Vec<FunctionInfo>) {
        let file = path.to_string_lossy().replace('\\', "/");
        let syntax = match parse_file(source) {
            Ok(s) => s,
            Err(_) => return (ItemCounts::default(), Vec::new()),
        };

        let mut collector = Self::new(file, registry, method_purity);
        for item in &syntax.items {
            collector.visit_item(item);
        }
        (collector.counts, collector.functions)
    }
}

impl<'ast, 'a> Visit<'ast> for Collector<'a> {
    fn visit_item(&mut self, item: &'ast Item) {
        match item {
            Item::Fn(item_fn) if !ItemClassifier::has_test_attr(&item_fn.attrs) => {
                self.visit_fn(item_fn)
            }
            Item::Struct(item_struct) if !ItemClassifier::has_test_attr(&item_struct.attrs) => {
                self.visit_struct(item_struct);
            }
            Item::Trait(item_trait) if !ItemClassifier::has_test_attr(&item_trait.attrs) => {
                self.visit_trait(item_trait);
            }
            Item::Enum(item_enum) if !ItemClassifier::has_test_attr(&item_enum.attrs) => {
                self.visit_enum(item_enum);
            }
            Item::Mod(item_mod) if !ItemClassifier::has_test_attr(&item_mod.attrs) => {
                self.visit_mod(item_mod)
            }
            Item::Impl(item_impl) if !ItemClassifier::has_test_attr(&item_impl.attrs) => {
                self.visit_impl(item_impl);
            }
            _ => {}
        }
    }
}

impl<'a> Collector<'a> {
    fn visit_fn(&mut self, item_fn: &ItemFn) {
        let name = item_fn.sig.ident.to_string();
        let is_pub = matches!(
            ItemClassifier::classify_visibility(&item_fn.vis),
            VisibilityLevel::Pub | VisibilityLevel::PubCrate
        );
        let is_pure = ItemClassifier::is_probably_pure(item_fn);
        let finder = self.count_hidden_deps_in_block(&item_fn.block);
        let has_trait_seam = false;
        let contr = self
            .contribution_schedule
            .contribution(is_pure, has_trait_seam, finder.weight);

        self.counts.total_functions += 1;
        self.counts.total_items += 1;
        self.counts.total_contribution += contr;
        if contr == 1.0 {
            self.counts.clean_functions += 1;
        }
        if is_pub {
            self.counts.public_items += 1;
        }
        if is_pure {
            self.counts.pure_functions += 1;
        }

        self.functions.push(FunctionInfo {
            name,
            file: self.current_file.clone(),
            is_pure,
            is_public: is_pub,
            hidden_deps: finder.count,
            has_trait_seam,
            dep_weight: finder.weight,
            hidden_dep_labels: finder.labels,
            grip_absolute: contr,
            grip_normalized: (contr * 100.0).round() as u32,
        });
    }

    fn visit_struct(&mut self, item_struct: &ItemStruct) {
        self.counts.total_items += 1;
        if matches!(
            ItemClassifier::classify_visibility(&item_struct.vis),
            VisibilityLevel::Pub | VisibilityLevel::PubCrate
        ) {
            self.counts.public_items += 1;
        }
        let name = item_struct.ident.to_string();
        let concrete: HashMap<String, String> = item_struct
            .fields
            .iter()
            .filter(|f| !is_trait_object_type(&f.ty))
            .filter_map(|f| {
                let field_name = f.ident.as_ref()?.to_string();
                Some((field_name, field_type_head(&f.ty)))
            })
            .collect();
        self.struct_concrete_fields.insert(name, concrete);
    }

    fn visit_trait(&mut self, item_trait: &ItemTrait) {
        self.counts.total_items += 1;
        if matches!(
            ItemClassifier::classify_visibility(&item_trait.vis),
            VisibilityLevel::Pub | VisibilityLevel::PubCrate
        ) {
            self.counts.public_items += 1;
        }
    }

    fn visit_enum(&mut self, item_enum: &ItemEnum) {
        self.counts.total_items += 1;
        if matches!(
            ItemClassifier::classify_visibility(&item_enum.vis),
            VisibilityLevel::Pub | VisibilityLevel::PubCrate
        ) {
            self.counts.public_items += 1;
        }
    }

    fn visit_mod(&mut self, item_mod: &ItemMod) {
        if let Some((_, items)) = &item_mod.content {
            for inner in items {
                self.visit_item(inner);
            }
        }
    }

    fn visit_impl(&mut self, item_impl: &ItemImpl) {
        if self.is_foreign_trait_impl(item_impl) {
            return;
        }
        let is_trait_impl = item_impl.trait_.is_some();
        let self_ty_name = self_ty_name(&item_impl.self_ty);

        for item in &item_impl.items {
            if let ImplItem::Fn(method) = item {
                if ItemClassifier::has_test_attr(&method.attrs) {
                    continue;
                }
                self.visit_impl_method(method, &self_ty_name, is_trait_impl);
            }
        }
    }

    fn is_foreign_trait_impl(&self, item_impl: &ItemImpl) -> bool {
        item_impl
            .trait_
            .as_ref()
            .is_some_and(|(_, p, _)| ItemClassifier::is_foreign_trait(p))
    }

    fn visit_impl_method(&mut self, method: &ImplItemFn, self_ty_name: &str, is_trait_impl: bool) {
        let is_pure = !ItemClassifier::is_impl_method_impure(method);
        let concrete_fields = self
            .struct_concrete_fields
            .get(self_ty_name)
            .cloned()
            .unwrap_or_default();
        let finder = self.count_hidden_deps_in_impl_method(method, concrete_fields);
        let contr = self
            .contribution_schedule
            .contribution(is_pure, is_trait_impl, finder.weight);

        self.counts.total_functions += 1;
        self.counts.total_contribution += contr;
        if contr == 1.0 {
            self.counts.clean_functions += 1;
        }
        if is_pure {
            self.counts.pure_functions += 1;
        }
        self.counts.record_impl_method(is_trait_impl, is_pure);

        let is_pub = matches!(
            ItemClassifier::classify_visibility(&method.vis),
            VisibilityLevel::Pub | VisibilityLevel::PubCrate
        );

        self.functions.push(FunctionInfo {
            name: method.sig.ident.to_string(),
            file: self.current_file.clone(),
            is_pure,
            is_public: is_pub,
            hidden_deps: finder.count,
            has_trait_seam: is_trait_impl,
            dep_weight: finder.weight,
            hidden_dep_labels: finder.labels,
            grip_absolute: contr,
            grip_normalized: (contr * 100.0).round() as u32,
        });
    }

    fn count_hidden_deps_in_block(&self, block: &Block) -> HiddenDepFinder<'a> {
        let mut finder = HiddenDepFinder::new(self.registry, self.method_purity);
        finder.visit_block(block);
        finder
    }

    fn count_hidden_deps_in_impl_method(
        &self,
        method: &ImplItemFn,
        concrete_fields: HashMap<String, String>,
    ) -> HiddenDepFinder<'a> {
        let mut finder = HiddenDepFinder::new(self.registry, self.method_purity);
        finder.set_concrete_fields(concrete_fields);
        finder.visit_block(&method.block);
        finder
    }
}
