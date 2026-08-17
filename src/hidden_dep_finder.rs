// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::known_hidden_dep_names::{PURE_VALUE_METHODS, STD_CONSTRUCTORS, STD_MODULE_CALLS};
use crate::method_purity_registry::MethodPurityRegistry;
use crate::struct_registry::{KNOWN_STD_VALUE_TYPES, StructRegistry};
use std::collections::HashMap;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMacro, ExprMethodCall, ExprUnsafe, Member, Path, Stmt};

fn dep_weight(label: &str) -> f64 {
    // Labels come from `path_label`, which joins macro path segments, so a
    // print macro arrives as the bare name `print` with no `!`. Matching on
    // `print!` never fires and drops the call into the unknown-dependency
    // catch-all instead. `print` also prefixes `println`, and `eprint`
    // prefixes `eprintln`, so two arms cover all four macros.
    if label.starts_with("print") || label.starts_with("eprint") {
        0.2
    } else if label.starts_with("Instant")
        || label.starts_with("SystemTime")
        || label.starts_with("Utc")
        || label.starts_with("Local")
        || label.contains("elapsed")
    {
        0.3
    } else if label.starts_with("env::") || label.starts_with("process::") {
        0.4
    } else if label.starts_with("unsafe") {
        0.5
    } else {
        0.6
    }
}

fn path_label(path: &Path) -> String {
    path.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

pub struct HiddenDepFinder<'a> {
    pub count: usize,
    pub weight: f64,
    pub labels: Vec<String>,
    concrete_fields: HashMap<String, String>,
    registry: &'a StructRegistry,
    method_purity: &'a MethodPurityRegistry,
}

impl<'a> HiddenDepFinder<'a> {
    pub fn new(registry: &'a StructRegistry, method_purity: &'a MethodPurityRegistry) -> Self {
        Self {
            count: 0,
            weight: 0.0,
            labels: Vec::new(),
            concrete_fields: HashMap::new(),
            registry,
            method_purity,
        }
    }

    pub fn set_concrete_fields(&mut self, fields: HashMap<String, String>) {
        self.concrete_fields = fields;
    }

    fn add_dep(&mut self, label: &str) {
        self.count += 1;
        let w = dep_weight(label);
        self.weight += w;
        self.labels.push(label.to_string());
    }

    fn check_path(&mut self, path: &Path) {
        let segments: Vec<_> = path.segments.iter().map(|s| s.ident.to_string()).collect();
        if segments.is_empty() {
            return;
        }
        if segments[0] == "Self" || segments[0] == "self" {
            return;
        }

        let tail_start = segments.len().saturating_sub(2);
        let tail = segments[tail_start..].join("::");
        if STD_MODULE_CALLS.contains(&tail.as_str())
            && (segments.len() <= 2 || segments[0] == "std" || segments[0] == "core")
        {
            self.add_dep(&tail);
            return;
        }

        let first = &segments[0];
        if first.starts_with(|c: char| c.is_ascii_uppercase())
            && !STD_CONSTRUCTORS.contains(&first.as_str())
        {
            let label = segments.join("::");
            self.add_dep(&label);
        }
    }
}

fn is_print_macro(path: &Path) -> bool {
    if let Some(name) = path.get_ident() {
        let n = name.to_string();
        return n == "println" || n == "eprintln" || n == "print" || n == "eprint";
    }
    false
}

impl<'ast, 'a> Visit<'ast> for HiddenDepFinder<'a> {
    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        if let Stmt::Macro(stmt_macro) = stmt {
            if is_print_macro(&stmt_macro.mac.path) {
                self.add_dep(&path_label(&stmt_macro.mac.path));
                return;
            }
        }
        visit::visit_stmt(self, stmt);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        if self.handle_expr(expr) {
            visit::visit_expr(self, expr);
        }
    }

    fn visit_expr_unsafe(&mut self, _expr: &'ast ExprUnsafe) {
        self.add_dep("unsafe { ... }");
    }
}

impl<'a> HiddenDepFinder<'a> {
    /// Returns whether the visitor should still recurse into `expr`'s children.
    fn handle_expr(&mut self, expr: &Expr) -> bool {
        match expr {
            Expr::Call(expr_call) => {
                if let Expr::Path(expr_path) = &*expr_call.func {
                    self.check_path(&expr_path.path);
                }
                true
            }
            Expr::MethodCall(expr_method) => self.handle_method_call_expr(expr_method),
            Expr::Macro(expr_macro) => self.handle_macro_expr(expr_macro),
            _ => true,
        }
    }

    fn handle_method_call_expr(&mut self, expr_method: &ExprMethodCall) -> bool {
        if Self::is_bare_self(&expr_method.receiver) {
            return false;
        }
        if let Some(name) = Self::concrete_field_name(&expr_method.receiver) {
            if let Some(type_name) = self.concrete_fields.get(&name).cloned() {
                let method = expr_method.method.to_string();
                if !self.is_pure_value_method(&type_name, &method) {
                    let label = format!("self.{name}.{method}");
                    self.add_dep(&label);
                }
            }
        }
        true
    }

    fn is_pure_value_method(&self, type_name: &str, method: &str) -> bool {
        if !PURE_VALUE_METHODS.contains(&method) {
            return false;
        }
        if method == "clone" {
            return self.registry.is_transitive_value_type(type_name);
        }
        KNOWN_STD_VALUE_TYPES.contains(&type_name)
            || self.method_purity.is_known_pure_method(type_name, method)
    }

    fn handle_macro_expr(&mut self, expr_macro: &ExprMacro) -> bool {
        if is_print_macro(&expr_macro.mac.path) {
            self.add_dep(&path_label(&expr_macro.mac.path));
            return false;
        }
        true
    }

    fn is_bare_self(receiver: &Expr) -> bool {
        matches!(
            receiver,
            Expr::Path(expr_path)
                if expr_path.path.segments.len() == 1 && expr_path.path.segments[0].ident == "self"
        )
    }

    fn concrete_field_name(receiver: &Expr) -> Option<String> {
        let Expr::Field(expr_field) = receiver else {
            return None;
        };
        let Expr::Path(expr_path) = &*expr_field.base else {
            return None;
        };
        if expr_path.path.segments.len() != 1 || expr_path.path.segments[0].ident != "self" {
            return None;
        }
        let Member::Named(ident) = &expr_field.member else {
            return None;
        };
        Some(ident.to_string())
    }
}
