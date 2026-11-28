// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::detection::io_call_finder::IoCallFinder;
use crate::detection::unsafe_finder::UnsafeFinder;
use syn::visit::Visit;
use syn::{Block, FnArg, ReceiverKind, ReturnType, Signature, Type};

pub struct FunctionPurity;

impl FunctionPurity {
    pub fn has_io_call(block: &Block) -> bool {
        let mut finder = IoCallFinder::new();
        finder.visit_block(block);
        finder.found()
    }

    pub fn has_mut_param(sig: &Signature) -> bool {
        sig.inputs.iter().any(|arg| match arg {
            FnArg::Receiver(recv) => Self::is_mut_receiver(&recv.kind),
            FnArg::Typed(pat_type) => Self::has_mut_in_type(&pat_type.ty),
        })
    }

    pub fn has_unsafe_block(block: &Block) -> bool {
        let mut finder = UnsafeFinder::new();
        finder.visit_block(block);
        finder.found()
    }

    pub fn is_unit_return(sig: &Signature) -> bool {
        match &sig.output {
            ReturnType::Default => true,
            ReturnType::Type(_, ty) => {
                if let Type::Tuple(tuple) = ty.as_ref() {
                    tuple.elems.is_empty()
                } else {
                    false
                }
            }
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn has_mut_in_type(ty: &Type) -> bool {
        match ty {
            Type::Reference(reference) => reference.mutability.is_some(),
            Type::Paren(inner) => Self::has_mut_in_type(&inner.elem),
            _ => false,
        }
    }

    fn is_mut_receiver(kind: &ReceiverKind) -> bool {
        match kind {
            ReceiverKind::Reference(_, _, mutability) => mutability.is_some(),
            ReceiverKind::Typed(_, ty) => Self::has_mut_in_type(ty),
            _ => false,
        }
    }
}
