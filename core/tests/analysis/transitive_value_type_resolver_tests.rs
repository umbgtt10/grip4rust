// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::transitive_value_type_resolver::TransitiveValueTypeResolver;
use std::collections::HashMap;

#[test]
fn resolve_known_std_type_returns_true() {
    // Arrange
    let fields_by_struct = HashMap::new();
    let mut resolver = TransitiveValueTypeResolver::new(&fields_by_struct);

    // Act
    let result = resolver.resolve("Vec");

    // Assert
    assert!(result, "Vec is a known std value type");
}

#[test]
fn resolve_mutual_cycle_returns_false() {
    // Arrange
    let mut fields_by_struct = HashMap::new();
    fields_by_struct.insert("A".to_string(), vec!["B".to_string()]);
    fields_by_struct.insert("B".to_string(), vec!["A".to_string()]);
    let mut resolver = TransitiveValueTypeResolver::new(&fields_by_struct);

    // Act
    let result = resolver.resolve("A");

    // Assert
    assert!(
        !result,
        "a mutual cycle must resolve false, not recurse forever"
    );
}

#[test]
fn resolve_struct_with_non_value_field_returns_false() {
    // Arrange
    let mut fields_by_struct = HashMap::new();
    fields_by_struct.insert(
        "Cache".to_string(),
        vec!["Vec".to_string(), "DbConnection".to_string()],
    );
    let mut resolver = TransitiveValueTypeResolver::new(&fields_by_struct);

    // Act
    let result = resolver.resolve("Cache");

    // Assert
    assert!(!result, "one non-value field poisons the whole struct");
}

#[test]
fn resolve_two_level_wrapper_chain_returns_true() {
    // Arrange
    let mut fields_by_struct = HashMap::new();
    fields_by_struct.insert("Members".to_string(), vec!["Vec".to_string()]);
    fields_by_struct.insert("Bootstrapped".to_string(), vec!["Members".to_string()]);
    let mut resolver = TransitiveValueTypeResolver::new(&fields_by_struct);

    // Act
    let result = resolver.resolve("Bootstrapped");

    // Assert
    assert!(result, "recursion must clear a two-level wrapper chain");
}

#[test]
fn resolve_unregistered_type_returns_false() {
    // Arrange
    let fields_by_struct = HashMap::new();
    let mut resolver = TransitiveValueTypeResolver::new(&fields_by_struct);

    // Act
    let result = resolver.resolve("DbConnection");

    // Assert
    assert!(
        !result,
        "an unregistered, unknown type is never a value type"
    );
}

#[test]
fn resolve_wrapper_of_known_std_field_returns_true() {
    // Arrange
    let mut fields_by_struct = HashMap::new();
    fields_by_struct.insert("Members".to_string(), vec!["Vec".to_string()]);
    let mut resolver = TransitiveValueTypeResolver::new(&fields_by_struct);

    // Act
    let result = resolver.resolve("Members");

    // Assert
    assert!(result, "a struct whose only field is a Vec resolves true");
}
