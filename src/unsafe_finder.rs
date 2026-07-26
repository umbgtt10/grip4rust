// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::ExprUnsafe;
use syn::visit::Visit;

pub(crate) struct UnsafeFinder {
    pub(crate) found: bool,
}

impl UnsafeFinder {
    pub(crate) fn new() -> Self {
        Self { found: false }
    }
}

impl<'ast> Visit<'ast> for UnsafeFinder {
    fn visit_expr_unsafe(&mut self, _expr: &'ast ExprUnsafe) {
        self.found = true;
    }
}
