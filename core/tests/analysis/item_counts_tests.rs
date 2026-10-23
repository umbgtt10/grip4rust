// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::item_counts::ItemCounts;

fn full_counts() -> ItemCounts {
    ItemCounts::default()
        .with_total_functions(5)
        .with_pure_functions(3)
        .with_total_items(9)
        .with_public_items(6)
        .with_inherent_methods(3)
        .with_inherent_impure(1)
        .with_local_trait_methods(2)
        .with_local_trait_impure(2)
        .with_total_contribution(4.0)
        .with_clean_functions(2)
}

#[test]
fn merge_adds_all_fields() {
    // Arrange
    let a = full_counts();
    let b = full_counts();

    // Act
    let merged = a.merged(&b);

    // Assert
    assert_eq!(merged.total_functions(), 10);
    assert_eq!(merged.pure_functions(), 6);
    assert_eq!(merged.total_items(), 18);
    assert_eq!(merged.public_items(), 12);
    assert_eq!(merged.inherent_methods(), 6);
    assert_eq!(merged.inherent_impure(), 2);
    assert_eq!(merged.local_trait_methods(), 4);
    assert_eq!(merged.local_trait_impure(), 4);
    assert_eq!(merged.total_contribution(), 8.0);
    assert_eq!(merged.clean_functions(), 4);
}

#[test]
fn merge_with_default_is_identity() {
    // Arrange
    let a = full_counts();
    let empty = ItemCounts::default();

    // Act
    let merged = a.merged(&empty);

    // Assert
    assert_eq!(merged.total_functions(), 5);
    assert_eq!(merged.total_items(), 9);
    assert_eq!(merged.inherent_methods(), 3);
    assert_eq!(merged.local_trait_methods(), 2);
}

#[test]
fn record_function_for_a_clean_pure_function_counts_it_clean_and_pure() {
    // Arrange
    let mut counts = ItemCounts::default();

    // Act
    counts.record_function(1.0, true);

    // Assert
    assert_eq!(counts.total_functions(), 1);
    assert_eq!(counts.pure_functions(), 1);
    assert_eq!(counts.clean_functions(), 1);
    assert_eq!(counts.total_contribution(), 1.0);
    assert_eq!(counts.total_items(), 0);
}

#[test]
fn record_function_for_an_impure_function_counts_it_neither_clean_nor_pure() {
    // Arrange
    let mut counts = ItemCounts::default();

    // Act
    counts.record_function(0.25, false);

    // Assert
    assert_eq!(counts.total_functions(), 1);
    assert_eq!(counts.pure_functions(), 0);
    assert_eq!(counts.clean_functions(), 0);
    assert_eq!(counts.total_contribution(), 0.25);
}

#[test]
fn record_impl_method_for_a_pure_inherent_method_counts_it_without_marking_it_impure() {
    // Arrange
    let mut counts = ItemCounts::default();

    // Act
    counts.record_impl_method(false, true);

    // Assert
    assert_eq!(counts.inherent_methods(), 1);
    assert_eq!(counts.inherent_impure(), 0);
    assert_eq!(counts.local_trait_methods(), 0);
}

#[test]
fn record_impl_method_for_a_pure_trait_method_counts_it_without_marking_it_impure() {
    // Arrange
    let mut counts = ItemCounts::default();

    // Act
    counts.record_impl_method(true, true);

    // Assert
    assert_eq!(counts.local_trait_methods(), 1);
    assert_eq!(counts.local_trait_impure(), 0);
    assert_eq!(counts.inherent_methods(), 0);
}

#[test]
fn record_impl_method_for_an_impure_inherent_method_counts_it_as_impure() {
    // Arrange
    let mut counts = ItemCounts::default();

    // Act
    counts.record_impl_method(false, false);

    // Assert
    assert_eq!(counts.inherent_methods(), 1);
    assert_eq!(counts.inherent_impure(), 1);
}

#[test]
fn record_impl_method_for_an_impure_trait_method_counts_it_as_impure() {
    // Arrange
    let mut counts = ItemCounts::default();

    // Act
    counts.record_impl_method(true, false);

    // Assert
    assert_eq!(counts.local_trait_methods(), 1);
    assert_eq!(counts.local_trait_impure(), 1);
}

#[test]
fn record_item_for_a_private_item_counts_it_without_marking_it_public() {
    // Arrange
    let mut counts = ItemCounts::default();

    // Act
    counts.record_item(false);

    // Assert
    assert_eq!(counts.total_items(), 1);
    assert_eq!(counts.public_items(), 0);
}

#[test]
fn record_item_for_a_public_item_counts_it_as_public() {
    // Arrange
    let mut counts = ItemCounts::default();

    // Act
    counts.record_item(true);

    // Assert
    assert_eq!(counts.total_items(), 1);
    assert_eq!(counts.public_items(), 1);
}
