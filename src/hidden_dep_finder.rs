// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::visit::Visit;

pub(crate) struct HiddenDepFinder {
    pub(crate) count: usize,
}

impl HiddenDepFinder {
    pub(crate) fn new() -> Self {
        Self { count: 0 }
    }
}

impl<'ast> Visit<'ast> for HiddenDepFinder {
    fn visit_expr(&mut self, expr: &'ast syn::Expr) {
        match expr {
            // std::time::Instant::now()
            syn::Expr::Call(expr_call) => {
                if let syn::Expr::Path(expr_path) = &*expr_call.func {
                    let segments: Vec<_> = expr_path
                        .path
                        .segments
                        .iter()
                        .map(|s| s.ident.to_string())
                        .collect();
                    let path_str = segments.join("::");
                    let is_direct_match = matches!(
                        path_str.as_str(),
                        "Instant::now"
                            | "SystemTime::now"
                            | "Utc::now"
                            | "Local::now"
                            | "rand::random"
                            | "thread_rng"
                            | "random"
                            | "env::var"
                            | "env::args"
                            | "env::temp_dir"
                            | "env::current_dir"
                            | "process::exit"
                            | "process::abort"
                            | "fs::read"
                            | "fs::write"
                            | "fs::read_to_string"
                            | "fs::read_dir"
                            | "fs::create_dir"
                            | "fs::create_dir_all"
                            | "fs::remove_file"
                            | "fs::remove_dir"
                            | "fs::remove_dir_all"
                            | "fs::copy"
                            | "fs::rename"
                            | "fs::metadata"
                            | "File::open"
                            | "File::create"
                            | "OpenOptions::new"
                            | "TcpStream::connect"
                            | "TcpStream::bind"
                            | "UdpSocket::bind"
                    );
                    let tail_start = segments.len().saturating_sub(2);
                    let tail = segments[tail_start..].join("::");
                    let is_tail_match = matches!(
                        tail.as_str(),
                        "env::var"
                            | "env::args"
                            | "env::temp_dir"
                            | "env::current_dir"
                            | "process::exit"
                            | "process::abort"
                            | "fs::read"
                            | "fs::write"
                            | "fs::read_to_string"
                            | "fs::read_dir"
                            | "fs::create_dir"
                            | "fs::create_dir_all"
                            | "fs::remove_file"
                            | "fs::remove_dir"
                            | "fs::remove_dir_all"
                            | "fs::copy"
                            | "fs::rename"
                            | "fs::metadata"
                            | "File::open"
                            | "File::create"
                            | "OpenOptions::new"
                            | "TcpStream::connect"
                            | "TcpStream::bind"
                            | "UdpSocket::bind"
                            | "Instant::now"
                            | "SystemTime::now"
                    ) && (segments.len() <= 2 || segments[0] == "std" || segments[0] == "core");
                    if is_direct_match || is_tail_match {
                        self.count += 1;
                        return;
                    }
                    if segments.len() >= 2
                        && (segments[0] == "rand" && segments.last().map_or(false, |s| s.contains("rng") || s == "random"))
                    {
                        self.count += 1;
                        return;
                    }
                }
            }
            // Method calls: .now(), .thread_rng(), .gen(), .elapsed()
            syn::Expr::MethodCall(expr_method) => {
                let method = expr_method.method.to_string();
                if method == "now" || method == "elapsed" || method == "thread_rng" || method == "gen" {
                    self.count += 1;
                    return;
                }
            }
            // println!, eprintln!, print!, eprint!, writeln!, write!
            syn::Expr::Macro(expr_macro) => {
                if let Some(name) = expr_macro.mac.path.get_ident() {
                    let n = name.to_string();
                    if n == "println" || n == "eprintln" || n == "print" || n == "eprint" {
                        self.count += 1;
                        return;
                    }
                }
            }
            _ => {}
        }
        syn::visit::visit_expr(self, expr);
    }

    fn visit_expr_unsafe(&mut self, _expr: &'ast syn::ExprUnsafe) {
        self.count += 1;
    }
}
