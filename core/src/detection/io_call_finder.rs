// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::visit::{Visit, visit_expr, visit_stmt};
use syn::{Expr, ExprCall, ExprMacro, ExprMethodCall, Path, Stmt};

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

pub struct IoCallFinder {
    pub found: bool,
}

impl IoCallFinder {
    #[must_use]
    pub fn new() -> Self {
        Self { found: false }
    }
}

impl Default for IoCallFinder {
    fn default() -> Self {
        Self::new()
    }
}

fn is_write_macro(path: &Path) -> bool {
    match path.get_ident() {
        Some(name) => {
            let n = name.to_string();
            n == "write" || n == "writeln"
        }
        None => false,
    }
}

impl<'ast> Visit<'ast> for IoCallFinder {
    // A macro in statement position is a `Stmt::Macro`, which the default
    // visitor never routes through `visit_expr`. Without this arm a bare
    // `write!(w, "x");` that discards its Result goes unseen, while the same
    // call written `write!(w, "x")?` is caught, because the `?` makes it an
    // expression. `HiddenDepFinder` reaches print macros the same way.
    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        if let Stmt::Macro(stmt_macro) = stmt {
            if is_write_macro(&stmt_macro.mac.path) {
                self.found = true;
                return;
            }
        }
        visit_stmt(self, stmt);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        if self.handle_expr(expr) {
            visit_expr(self, expr);
        }
    }
}

impl IoCallFinder {
    /// Returns whether the visitor should still recurse into `expr`'s children.
    fn handle_expr(&mut self, expr: &Expr) -> bool {
        match expr {
            Expr::Call(expr_call) => self.handle_call_expr(expr_call),
            Expr::MethodCall(expr_method) => self.handle_method_call_expr(expr_method),
            Expr::Macro(expr_macro) => self.handle_macro_expr(expr_macro),
            _ => true,
        }
    }

    fn handle_call_expr(&mut self, expr_call: &ExprCall) -> bool {
        let Expr::Path(expr_path) = &*expr_call.func else {
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

    fn handle_method_call_expr(&mut self, expr_method: &ExprMethodCall) -> bool {
        if is_io_method(&expr_method.method.to_string()) {
            self.found = true;
            return false;
        }
        true
    }

    fn handle_macro_expr(&mut self, expr_macro: &ExprMacro) -> bool {
        if is_write_macro(&expr_macro.mac.path) {
            self.found = true;
            return false;
        }
        true
    }
}
