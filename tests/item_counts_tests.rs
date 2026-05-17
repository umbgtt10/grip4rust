// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::item_counts::ItemCounts;

fn full_counts() -> ItemCounts {
    ItemCounts {
        total_functions: 5,
        pure_functions: 3,
        public_functions: 2,
        pubcrate_functions: 1,
        public_structs: 1,
        public_traits: 1,
        public_enums: 1,
        total_items: 9,
        public_items: 6,
        inherent_methods: 3,
        inherent_impure: 1,
        local_trait_methods: 2,
        local_trait_impure: 2,
        total_contribution: 4.0,
        clean_functions: 2,
    }
}

#[test]
fn merge_adds_all_fields() {
    // Arrange
    let a = full_counts();
    let b = full_counts();

    // Act
    let merged = a.merged(&b);

    // Assert
    assert_eq!(merged.total_functions, 10);
    assert_eq!(merged.pure_functions, 6);
    assert_eq!(merged.public_functions, 4);
    assert_eq!(merged.pubcrate_functions, 2);
    assert_eq!(merged.public_structs, 2);
    assert_eq!(merged.public_traits, 2);
    assert_eq!(merged.public_enums, 2);
    assert_eq!(merged.total_items, 18);
    assert_eq!(merged.public_items, 12);
    assert_eq!(merged.inherent_methods, 6);
    assert_eq!(merged.inherent_impure, 2);
    assert_eq!(merged.local_trait_methods, 4);
    assert_eq!(merged.local_trait_impure, 4);
    assert_eq!(merged.total_contribution, 8.0);
    assert_eq!(merged.clean_functions, 4);
}

#[test]
fn merge_with_default_is_identity() {
    // Arrange
    let a = full_counts();
    let empty = ItemCounts::default();

    // Act
    let merged = a.merged(&empty);

    // Assert
    assert_eq!(merged.total_functions, 5);
    assert_eq!(merged.total_items, 9);
    assert_eq!(merged.inherent_methods, 3);
    assert_eq!(merged.local_trait_methods, 2);
}
