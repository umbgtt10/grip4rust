// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::ExprUnsafe;
use syn::visit::Visit;

pub struct UnsafeFinder {
    pub found: bool,
}

impl UnsafeFinder {
    pub fn new() -> Self {
        Self { found: false }
    }
}

impl Default for UnsafeFinder {
    fn default() -> Self {
        Self::new()
    }
}

impl<'ast> Visit<'ast> for UnsafeFinder {
    fn visit_expr_unsafe(&mut self, _expr: &'ast ExprUnsafe) {
        self.found = true;
    }
}
