// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::visit::Visit;

const IO_METHOD_NAMES: &[&str] = &[
    "connect",
    "send_to",
    "recv_from",
    "write_all",
    "read_to_string",
    "flush",
    "open",
    "create",
    "bind",
    "accept",
];

fn is_io_method(name: &str) -> bool {
    IO_METHOD_NAMES.contains(&name)
}

pub(crate) struct IoCallFinder {
    pub(crate) found: bool,
}

impl IoCallFinder {
    pub(crate) fn new() -> Self {
        Self { found: false }
    }
}

impl<'ast> Visit<'ast> for IoCallFinder {
    fn visit_expr(&mut self, expr: &'ast syn::Expr) {
        if self.handle_expr(expr) {
            syn::visit::visit_expr(self, expr);
        }
    }
}

impl IoCallFinder {
    /// Returns whether the visitor should still recurse into `expr`'s children.
    fn handle_expr(&mut self, expr: &syn::Expr) -> bool {
        match expr {
            syn::Expr::Call(expr_call) => self.handle_call_expr(expr_call),
            syn::Expr::MethodCall(expr_method) => self.handle_method_call_expr(expr_method),
            syn::Expr::Macro(expr_macro) => self.handle_macro_expr(expr_macro),
            _ => true,
        }
    }

    fn handle_call_expr(&mut self, expr_call: &syn::ExprCall) -> bool {
        let syn::Expr::Path(expr_path) = &*expr_call.func else {
            return true;
        };
        let segments: Vec<_> = expr_path
            .path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect();

        if Self::is_flagged_path(&segments) || segments.last().is_some_and(|n| is_io_method(n)) {
            self.found = true;
            return false;
        }
        true
    }

    fn is_flagged_path(segments: &[String]) -> bool {
        segments.len() >= 2
            && matches!(
                segments[0].as_str(),
                "fs" | "net" | "io" | "TcpStream" | "UdpSocket" | "File" | "OpenOptions"
            )
    }

    fn handle_method_call_expr(&mut self, expr_method: &syn::ExprMethodCall) -> bool {
        if is_io_method(&expr_method.method.to_string()) {
            self.found = true;
            return false;
        }
        true
    }

    fn handle_macro_expr(&mut self, expr_macro: &syn::ExprMacro) -> bool {
        let Some(name) = expr_macro.mac.path.get_ident() else {
            return true;
        };
        let n = name.to_string();
        if n == "writeln" || n == "write" {
            self.found = true;
            return false;
        }
        true
    }
}
