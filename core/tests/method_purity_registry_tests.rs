// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::method_purity_registry::MethodPurityRegistry;
use grip::struct_registry::StructRegistry;
use std::path::PathBuf;

#[test]
fn is_known_pure_method_body_with_hidden_dep_returns_false() {
    // Arrange: pure signature, but the body reaches out to an unresolved
    // third-party constructor - the zero-hidden-deps requirement must
    // actually be enforced, not just signature shape.
    let source = r#"
struct Members {
    ids: Vec<i32>,
}
impl Members {
    pub fn len(&self) -> usize {
        ExternalHelper::compute(&self.ids)
    }
}
"#;
    let files = vec![(PathBuf::from("members.rs"), source.to_string())];
    let struct_registry = StructRegistry::build(&files);
    let method_purity = MethodPurityRegistry::build(&files, &struct_registry);

    // Act
    let result = method_purity.is_known_pure_method("Members", "len");

    // Assert
    assert!(
        !result,
        "a real hidden dependency in the body must block registration even with a pure signature"
    );
}

#[test]
fn is_known_pure_method_cross_source_impl_returns_true() {
    // Arrange: Members is declared in one file, its impl block lives in
    // another - the whole reason this is a project-wide pass rather than
    // something Collector could do per-file on its own.
    let struct_source = "struct Members { ids: Vec<i32> }";
    let impl_source = r#"
impl Members {
    pub fn len(&self) -> usize { self.ids.len() }
}
"#;
    let files = vec![
        (PathBuf::from("members.rs"), struct_source.to_string()),
        (PathBuf::from("members_impl.rs"), impl_source.to_string()),
    ];
    let struct_registry = StructRegistry::build(&files);
    let method_purity = MethodPurityRegistry::build(&files, &struct_registry);

    // Act
    let result = method_purity.is_known_pure_method("Members", "len");

    // Assert
    assert!(
        result,
        "the registry must resolve an inherent impl defined in a different source entry"
    );
}

#[test]
fn is_known_pure_method_io_call_body_returns_false() {
    // Arrange
    let source = r#"
struct DiskCache {
    path: String,
}
impl DiskCache {
    pub fn get(&self, key: &str) -> String {
        let contents = std::fs::read_to_string(&self.path).unwrap_or_default();
        format!("{key}:{contents}")
    }
}
"#;
    let files = vec![(PathBuf::from("disk_cache.rs"), source.to_string())];
    let struct_registry = StructRegistry::build(&files);
    let method_purity = MethodPurityRegistry::build(&files, &struct_registry);

    // Act
    let result = method_purity.is_known_pure_method("DiskCache", "get");

    // Assert
    assert!(
        !result,
        "a method whose body genuinely performs I/O must never be trusted, regardless of name"
    );
}

#[test]
fn is_known_pure_method_local_trait_impl_is_ignored() {
    // Arrange: the method body is genuinely pure, but it's reached through a
    // local trait impl rather than an inherent impl - deliberately out of
    // scope for this registry.
    let source = r#"
trait Lengthy {
    fn len(&self) -> usize;
}

struct Members {
    ids: Vec<i32>,
}

impl Lengthy for Members {
    fn len(&self) -> usize { self.ids.len() }
}
"#;
    let files = vec![(PathBuf::from("members.rs"), source.to_string())];
    let struct_registry = StructRegistry::build(&files);
    let method_purity = MethodPurityRegistry::build(&files, &struct_registry);

    // Act
    let result = method_purity.is_known_pure_method("Members", "len");

    // Assert
    assert!(
        !result,
        "trait-impl methods are out of scope, inherent impls only"
    );
}

#[test]
fn is_known_pure_method_mut_self_returns_false() {
    // Arrange
    let source = r#"
struct Counter {
    values: Vec<i32>,
}
impl Counter {
    pub fn get(&mut self, index: usize) -> i32 {
        self.values[index]
    }
}
"#;
    let files = vec![(PathBuf::from("counter.rs"), source.to_string())];
    let struct_registry = StructRegistry::build(&files);
    let method_purity = MethodPurityRegistry::build(&files, &struct_registry);

    // Act
    let result = method_purity.is_known_pure_method("Counter", "get");

    // Assert
    assert!(
        !result,
        "a &mut self method is never trusted as a pure accessor"
    );
}

#[test]
fn is_known_pure_method_pure_inherent_accessor_returns_true() {
    // Arrange
    let source = r#"
struct Members {
    ids: Vec<i32>,
}
impl Members {
    pub fn len(&self) -> usize { self.ids.len() }
}
"#;
    let files = vec![(PathBuf::from("members.rs"), source.to_string())];
    let struct_registry = StructRegistry::build(&files);
    let method_purity = MethodPurityRegistry::build(&files, &struct_registry);

    // Act
    let result = method_purity.is_known_pure_method("Members", "len");

    // Assert
    assert!(
        result,
        "a genuinely pure delegating accessor must be provable"
    );
}

#[test]
fn is_known_pure_method_unknown_type_returns_false() {
    // Arrange
    let method_purity = MethodPurityRegistry::default();

    // Act
    let result = method_purity.is_known_pure_method("NeverSeen", "len");

    // Assert
    assert!(
        !result,
        "a type with no registered methods is never trusted"
    );
}
