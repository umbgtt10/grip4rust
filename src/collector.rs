// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;

use quote::ToTokens;
use syn::visit::Visit;
use syn::{Attribute, Item, ItemFn, Visibility};

use crate::item_counts::ItemCounts;

pub fn collect_file(source: &str, _path: &Path) -> ItemCounts {
    let syntax = match syn::parse_file(source) {
        Ok(s) => s,
        Err(_) => return ItemCounts::default(),
    };

    let mut collector = Collector::new();
    for item in &syntax.items {
        collector.visit_item(item);
    }
    collector.counts
}

#[derive(Debug)]
struct Collector {
    counts: ItemCounts,
}

impl Collector {
    fn new() -> Self {
        Self {
            counts: ItemCounts::default(),
        }
    }
}

impl<'ast> Visit<'ast> for Collector {
    fn visit_item(&mut self, item: &'ast Item) {
        match item {
            Item::Fn(item_fn) if !has_test_attr(&item_fn.attrs) => {
                self.counts.total_functions += 1;
                self.counts.total_items += 1;
                match classify_visibility(&item_fn.vis) {
                    VisibilityLevel::Pub => {
                        self.counts.public_functions += 1;
                        self.counts.public_items += 1;
                    }
                    VisibilityLevel::PubCrate => {
                        self.counts.pubcrate_functions += 1;
                        self.counts.public_items += 1;
                    }
                    _ => {}
                }
                if is_probably_pure(item_fn) {
                    self.counts.pure_functions += 1;
                }
            }
            Item::Struct(item_struct) => {
                self.counts.total_items += 1;
                if matches!(
                    classify_visibility(&item_struct.vis),
                    VisibilityLevel::Pub | VisibilityLevel::PubCrate
                ) {
                    self.counts.public_structs += 1;
                    self.counts.public_items += 1;
                }
            }
            Item::Trait(item_trait) => {
                self.counts.total_items += 1;
                if matches!(
                    classify_visibility(&item_trait.vis),
                    VisibilityLevel::Pub | VisibilityLevel::PubCrate
                ) {
                    self.counts.public_traits += 1;
                    self.counts.public_items += 1;
                }
            }
            Item::Enum(item_enum) => {
                self.counts.total_items += 1;
                if matches!(
                    classify_visibility(&item_enum.vis),
                    VisibilityLevel::Pub | VisibilityLevel::PubCrate
                ) {
                    self.counts.public_enums += 1;
                    self.counts.public_items += 1;
                }
            }
            Item::Mod(item_mod) if !has_test_attr(&item_mod.attrs) => {
                if let Some((_, items)) = &item_mod.content {
                    for inner in items {
                        self.visit_item(inner);
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisibilityLevel {
    Private,
    PubCrate,
    Pub,
}

fn classify_visibility(vis: &Visibility) -> VisibilityLevel {
    match vis {
        Visibility::Public(_) => VisibilityLevel::Pub,
        Visibility::Restricted(_) => VisibilityLevel::PubCrate,
        _ => VisibilityLevel::Private,
    }
}

fn has_test_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        let tokens = attr.to_token_stream().to_string();
        let path = attr.path().get_ident().map(|i| i.to_string());
        matches!(path.as_deref(), Some("cfg")) && tokens.contains("test")
            || matches!(path.as_deref(), Some("test"))
            || matches!(path.as_deref(), Some("cfg_attr"))
    })
}

fn is_probably_pure(item_fn: &ItemFn) -> bool {
    if has_mut_param(&item_fn.sig) {
        return false;
    }
    if is_unit_return(&item_fn.sig) {
        return false;
    }
    if item_fn.sig.unsafety.is_some() {
        return false;
    }
    !has_unsafe_block(&item_fn.block)
}

fn has_mut_param(sig: &syn::Signature) -> bool {
    sig.inputs.iter().any(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            has_mut_in_type(&pat_type.ty)
        } else {
            false
        }
    })
}

fn has_mut_in_type(ty: &syn::Type) -> bool {
    use syn::Type;
    match ty {
        Type::Reference(reference) => reference.mutability.is_some(),
        Type::Paren(inner) => has_mut_in_type(&inner.elem),
        _ => false,
    }
}

fn is_unit_return(sig: &syn::Signature) -> bool {
    match &sig.output {
        syn::ReturnType::Default => true,
        syn::ReturnType::Type(_, ty) => {
            if let syn::Type::Tuple(tuple) = ty.as_ref() {
                tuple.elems.is_empty()
            } else {
                false
            }
        }
    }
}

fn has_unsafe_block(block: &syn::Block) -> bool {
    use syn::visit::Visit;
    let mut finder = UnsafeFinder { found: false };
    finder.visit_block(block);
    finder.found
}

struct UnsafeFinder {
    found: bool,
}

impl<'ast> Visit<'ast> for UnsafeFinder {
    fn visit_expr_unsafe(&mut self, _expr: &'ast syn::ExprUnsafe) {
        self.found = true;
    }
}
