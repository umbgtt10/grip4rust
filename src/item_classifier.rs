// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::function_purity::FunctionPurity;
use crate::known_foreign_traits::KNOWN_FOREIGN_TRAITS;
use crate::visibility_level::VisibilityLevel;
use quote::ToTokens;
use syn::{Attribute, ImplItemFn, ItemFn, Path as SynPath, Visibility};

pub struct ItemClassifier;

impl ItemClassifier {
    #[must_use]
    pub fn is_impl_method_impure(method: &ImplItemFn) -> bool {
        if FunctionPurity::has_mut_param(&method.sig) {
            return true;
        }
        if FunctionPurity::is_unit_return(&method.sig) {
            return true;
        }
        if method.sig.unsafety.is_some() {
            return true;
        }
        FunctionPurity::has_unsafe_block(&method.block)
            || FunctionPurity::has_io_call(&method.block)
    }

    #[must_use]
    pub fn is_probably_pure(item_fn: &ItemFn) -> bool {
        if FunctionPurity::has_mut_param(&item_fn.sig) {
            return false;
        }
        if FunctionPurity::is_unit_return(&item_fn.sig) {
            return false;
        }
        if item_fn.sig.unsafety.is_some() {
            return false;
        }
        !FunctionPurity::has_unsafe_block(&item_fn.block)
    }

    // A single-segment path relies on the known-foreign list, because without
    // type resolution `impl Display for X` and a local `trait Display` are
    // indistinguishable. A multi-segment path rooted at std, core or alloc is
    // foreign whatever the trait is called.
    #[must_use]
    pub fn is_foreign_trait(path: &SynPath) -> bool {
        if let Some(last) = path.segments.last() {
            let name = last.ident.to_string();
            if KNOWN_FOREIGN_TRAITS.contains(&name.as_str()) {
                return true;
            }
        }
        if path.segments.len() > 1 {
            if let Some(first) = path.segments.first() {
                let name = first.ident.to_string();
                return name == "std" || name == "core" || name == "alloc";
            }
        }
        false
    }

    #[must_use]
    pub fn classify_visibility(vis: &Visibility) -> VisibilityLevel {
        match vis {
            Visibility::Public(_) => VisibilityLevel::Pub,
            Visibility::Restricted(_) => VisibilityLevel::PubCrate,
            _ => VisibilityLevel::Private,
        }
    }

    #[must_use]
    pub fn has_test_attr(attrs: &[Attribute]) -> bool {
        attrs.iter().any(|attr| {
            let tokens = attr.to_token_stream().to_string();
            let path = attr.path().get_ident().map(|i| i.to_string());
            matches!(path.as_deref(), Some("cfg")) && tokens.contains("test")
                || matches!(path.as_deref(), Some("test"))
                || matches!(path.as_deref(), Some("cfg_attr")) && tokens.contains("test")
        })
    }
}
