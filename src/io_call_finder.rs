// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::visit::Visit;

const IO_METHOD_NAMES: &[&str] = &[
    "connect", "send_to", "recv_from", "write_all", "read_to_string",
    "flush", "open", "create", "bind", "accept",
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
        match expr {
            syn::Expr::Call(expr_call) => {
                if let syn::Expr::Path(expr_path) = &*expr_call.func {
                    let segments: Vec<_> = expr_path
                        .path
                        .segments
                        .iter()
                        .map(|s| s.ident.to_string())
                        .collect();

                    let flagged = if segments.len() >= 2 {
                        matches!(
                            segments[0].as_str(),
                            "fs" | "net" | "io" | "TcpStream" | "UdpSocket" | "File"
                                | "OpenOptions"
                        )
                    } else {
                        false
                    };

                    if flagged || segments.last().is_some_and(|n| is_io_method(n)) {
                        self.found = true;
                        return;
                    }
                }
            }
            syn::Expr::MethodCall(expr_method) => {
                if is_io_method(&expr_method.method.to_string()) {
                    self.found = true;
                    return;
                }
            }
            syn::Expr::Macro(expr_macro) => {
                if let Some(name) = expr_macro.mac.path.get_ident() {
                    let n = name.to_string();
                    if n == "writeln" || n == "write" {
                        self.found = true;
                        return;
                    }
                }
            }
            _ => {}
        }
        syn::visit::visit_expr(self, expr);
    }
}
