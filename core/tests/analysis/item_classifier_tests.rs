// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::item_classifier::ItemClassifier;
use grip::analysis::visibility_level::VisibilityLevel;
use syn::parse_str;
use syn::{Attribute, ImplItemFn, ItemFn, Path as SynPath};

fn attrs(source: &str) -> Vec<Attribute> {
    let parsed: ItemFn = parse_str(&format!("{source} fn placeholder() -> u32 {{ 1 }}"))
        .expect("attributed function should parse");
    parsed.attrs
}

fn impl_method(source: &str) -> ImplItemFn {
    parse_str(source).expect("method should parse")
}

fn item_fn(source: &str) -> ItemFn {
    parse_str(source).expect("function should parse")
}

fn path(source: &str) -> SynPath {
    parse_str(source).expect("path should parse")
}

#[test]
fn classify_visibility_for_inherited_returns_private() {
    // Arrange
    let parsed: ItemFn = parse_str("fn f() -> u32 { 1 }").expect("parses");

    // Act
    let level = ItemClassifier::classify_visibility(&parsed.vis);

    // Assert
    assert_eq!(level, VisibilityLevel::Private);
}

#[test]
fn classify_visibility_for_pub_crate_returns_pub_crate() {
    // Arrange
    let parsed: ItemFn = parse_str("pub(crate) fn f() -> u32 { 1 }").expect("parses");

    // Act
    let level = ItemClassifier::classify_visibility(&parsed.vis);

    // Assert
    assert_eq!(level, VisibilityLevel::PubCrate);
}

#[test]
fn classify_visibility_for_pub_returns_pub() {
    // Arrange
    let parsed: ItemFn = parse_str("pub fn f() -> u32 { 1 }").expect("parses");

    // Act
    let level = ItemClassifier::classify_visibility(&parsed.vis);

    // Assert
    assert_eq!(level, VisibilityLevel::Pub);
}

#[test]
fn has_test_attr_for_a_test_attribute_returns_true() {
    // Arrange & Act
    let found = ItemClassifier::has_test_attr(&attrs("#[test]"));

    // Assert
    assert!(found);
}

#[test]
fn has_test_attr_for_an_unrelated_attribute_returns_false() {
    // Arrange & Act
    let found = ItemClassifier::has_test_attr(&attrs("#[derive(Debug)]"));

    // Assert
    assert!(!found);
}

#[test]
fn has_test_attr_for_cfg_test_returns_true() {
    // Arrange & Act
    let found = ItemClassifier::has_test_attr(&attrs("#[cfg(test)]"));

    // Assert
    assert!(found);
}

#[test]
fn has_test_attr_for_no_attributes_returns_false() {
    // Arrange & Act
    let found = ItemClassifier::has_test_attr(&[]);

    // Assert
    assert!(!found);
}

#[test]
fn is_foreign_trait_for_a_known_std_trait_returns_true() {
    // Arrange & Act
    let foreign = ItemClassifier::is_foreign_trait(&path("Display"));

    // Assert
    assert!(foreign);
}

#[test]
fn is_foreign_trait_for_a_local_trait_returns_false() {
    // Arrange & Act
    let foreign = ItemClassifier::is_foreign_trait(&path("MyOwnTrait"));

    // Assert
    assert!(!foreign);
}

#[test]
fn is_foreign_trait_for_a_locally_rooted_path_returns_false() {
    // Arrange & Act
    let foreign = ItemClassifier::is_foreign_trait(&path("crate::traits::Anything"));

    // Assert
    assert!(!foreign);
}

// Without type resolution a single-segment path is ambiguous, so a multi-segment
// path rooted at std, core or alloc is the only unambiguous signal.
#[test]
fn is_foreign_trait_for_a_std_rooted_path_returns_true() {
    // Arrange & Act
    let foreign = ItemClassifier::is_foreign_trait(&path("std::fmt::Anything"));

    // Assert
    assert!(foreign);
}

#[test]
fn is_impl_method_impure_for_a_unit_returning_method_returns_true() {
    // Arrange & Act
    let impure = ItemClassifier::is_impl_method_impure(&impl_method("fn act(&self) {}"));

    // Assert
    assert!(impure);
}

#[test]
fn is_impl_method_impure_for_a_value_returning_method_returns_false() {
    // Arrange & Act
    let impure =
        ItemClassifier::is_impl_method_impure(&impl_method("fn value(&self) -> u32 { 1 }"));

    // Assert
    assert!(!impure);
}

#[test]
fn is_probably_pure_for_a_mutable_parameter_returns_false() {
    // Arrange & Act
    let pure =
        ItemClassifier::is_probably_pure(&item_fn("fn fill(buf: &mut Vec<u32>) -> u32 { 1 }"));

    // Assert
    assert!(!pure);
}

#[test]
fn is_probably_pure_for_a_unit_returning_function_returns_false() {
    // Arrange & Act
    let pure = ItemClassifier::is_probably_pure(&item_fn("fn act(a: u32) { let _ = a; }"));

    // Assert
    assert!(!pure);
}

#[test]
fn is_probably_pure_for_a_value_returning_function_returns_true() {
    // Arrange & Act
    let pure = ItemClassifier::is_probably_pure(&item_fn("fn compute(a: u32) -> u32 { a + 1 }"));

    // Assert
    assert!(pure);
}

#[test]
fn is_probably_pure_for_an_unsafe_function_returns_false() {
    // Arrange & Act
    let pure = ItemClassifier::is_probably_pure(&item_fn("unsafe fn raw() -> u32 { 1 }"));

    // Assert
    assert!(!pure);
}
