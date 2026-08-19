// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::struct_registry::StructRegistry;
use std::path::PathBuf;

#[test]
fn is_transitive_value_type_cross_source_field_returns_true() {
    // Arrange: Members and its user come from two different files - this is
    // the whole point of the registry existing, since a single-file Collector
    // pass can never see across this boundary on its own.
    let members_source = "struct Members { ids: Vec<i32> }";
    let user_source = "struct Bootstrapped { members: Members }";
    let files = vec![
        (PathBuf::from("members.rs"), members_source.to_string()),
        (PathBuf::from("bootstrapped.rs"), user_source.to_string()),
    ];
    let registry = StructRegistry::build(&files);

    // Act
    let result = registry.is_transitive_value_type("Bootstrapped");

    // Assert
    assert!(
        result,
        "the registry must resolve a field type defined in a different source entry"
    );
}

#[test]
fn is_transitive_value_type_field_declared_in_inline_mod_returns_true() {
    // Arrange
    let source = r#"
mod inner {
    pub struct Members {
        ids: Vec<i32>,
    }
}

struct Bootstrapped {
    members: Members,
}
"#;
    let files = vec![(PathBuf::from("lib.rs"), source.to_string())];
    let registry = StructRegistry::build(&files);

    // Act
    let result = registry.is_transitive_value_type("Bootstrapped");

    // Assert
    assert!(
        result,
        "a struct declared inside an inline mod block must still be registered"
    );
}

#[test]
fn is_transitive_value_type_known_std_type_returns_true() {
    // Arrange
    let registry = StructRegistry::default();

    // Act
    let result = registry.is_transitive_value_type("Vec");

    // Assert
    assert!(result, "Vec is a known std value type");
}

#[test]
fn is_transitive_value_type_mutual_cycle_returns_false() {
    // Arrange
    let source = r#"
struct A {
    b: B,
}

struct B {
    a: A,
}
"#;
    let files = vec![(PathBuf::from("cyclic.rs"), source.to_string())];
    let registry = StructRegistry::build(&files);

    // Act
    let result = registry.is_transitive_value_type("A");

    // Assert
    assert!(
        !result,
        "a mutual cycle must resolve false, not recurse forever"
    );
}

#[test]
fn is_transitive_value_type_plain_wrapper_struct_returns_true() {
    // Arrange
    let source = r#"
struct Members {
    ids: Vec<i32>,
}
"#;
    let files = vec![(PathBuf::from("members.rs"), source.to_string())];
    let registry = StructRegistry::build(&files);

    // Act
    let result = registry.is_transitive_value_type("Members");

    // Assert
    assert!(
        result,
        "a struct whose only field is a Vec is transitively a value type"
    );
}

#[test]
fn is_transitive_value_type_struct_with_non_value_field_returns_false() {
    // Arrange
    let source = r#"
struct Cache {
    data: Vec<u8>,
    conn: DbConnection,
}
"#;
    let files = vec![(PathBuf::from("cache.rs"), source.to_string())];
    let registry = StructRegistry::build(&files);

    // Act
    let result = registry.is_transitive_value_type("Cache");

    // Assert
    assert!(
        !result,
        "one non-value field must poison the whole struct, not just be ignored"
    );
}

#[test]
fn is_transitive_value_type_unknown_type_returns_false() {
    // Arrange
    let registry = StructRegistry::default();

    // Act
    let result = registry.is_transitive_value_type("DbConnection");

    // Assert
    assert!(
        !result,
        "an unregistered, unknown type is never a value type"
    );
}

#[test]
fn is_transitive_value_type_wrapper_of_wrapper_returns_true() {
    // Arrange
    let source = r#"
struct Members {
    ids: Vec<i32>,
}

struct Bootstrapped {
    members: Members,
}
"#;
    let files = vec![(PathBuf::from("lib.rs"), source.to_string())];
    let registry = StructRegistry::build(&files);

    // Act
    let result = registry.is_transitive_value_type("Bootstrapped");

    // Assert
    assert!(result, "recursion must clear a two-level wrapper chain");
}
