// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::visit::Visit;

use crate::io_call_finder::IoCallFinder;
use crate::unsafe_finder::UnsafeFinder;

pub struct FunctionPurity;

impl FunctionPurity {
    pub fn has_mut_param(sig: &syn::Signature) -> bool {
        sig.inputs.iter().any(|arg| match arg {
            syn::FnArg::Receiver(recv) => recv.reference.is_some() && recv.mutability.is_some(),
            syn::FnArg::Typed(pat_type) => Self::has_mut_in_type(&pat_type.ty),
        })
    }

    #[allow(clippy::only_used_in_recursion)]
    fn has_mut_in_type(ty: &syn::Type) -> bool {
        use syn::Type;
        match ty {
            Type::Reference(reference) => reference.mutability.is_some(),
            Type::Paren(inner) => Self::has_mut_in_type(&inner.elem),
            _ => false,
        }
    }

    pub fn is_unit_return(sig: &syn::Signature) -> bool {
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

    pub fn has_unsafe_block(block: &syn::Block) -> bool {
        let mut finder = UnsafeFinder::new();
        finder.visit_block(block);
        finder.found
    }

    pub fn has_io_call(block: &syn::Block) -> bool {
        let mut finder = IoCallFinder::new();
        finder.visit_block(block);
        finder.found
    }
}
