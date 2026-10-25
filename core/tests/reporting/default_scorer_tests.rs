// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::item_counts::ItemCounts;
use grip::reporting::default_scorer::DefaultScorer;
use grip::traits::scorer::Scorer;
use std::collections::BTreeMap;

fn scorer() -> DefaultScorer {
    DefaultScorer::new()
}

#[test]
fn agg_modules_over_two_files_sums_their_counts_and_keeps_both_modules() {
    // Arrange
    let files = vec![
        (
            "a".to_string(),
            ItemCounts::default()
                .with_total_functions(1)
                .with_pure_functions(1)
                .with_public_items(1)
                .with_total_items(1),
        ),
        (
            "b".to_string(),
            ItemCounts::default()
                .with_total_functions(1)
                .with_pure_functions(0)
                .with_public_items(0)
                .with_total_items(1),
        ),
    ];

    // Act
    let (overall, modules) = scorer().agg_modules(files);

    // Assert
    assert_eq!(overall.total_functions(), 2);
    assert_eq!(overall.pure_functions(), 1);
    assert_eq!(modules.len(), 2);
}

#[test]
fn empty_module_gives_no_score() {
    // Arrange
    let counts = ItemCounts::default();

    // Act
    let scores = scorer().score_counts(&counts);

    // Assert
    assert_eq!(scores.grip_score, None);
    assert_eq!(scores.pure_ratio, 0.0);
    assert_eq!(scores.public_ratio, 0.0);
}

#[test]
fn module_stats_sorted() {
    // Arrange
    let mut map = BTreeMap::new();
    map.insert(
        "alpha".to_string(),
        ItemCounts::default()
            .with_total_functions(1)
            .with_pure_functions(1)
            .with_public_items(1)
            .with_total_items(1),
    );

    // Act
    let stats = scorer().module_stats(map);

    // Assert
    assert_eq!(stats.len(), 1);
    assert_eq!(stats[0].path, "alpha");
    assert_eq!(stats[0].grip_score, Some(75));
    assert_eq!(stats[0].trait_ratio, 1.0);
    assert_eq!(stats[0].avg_contribution, 0.0);
    assert_eq!(stats[0].clean_fn_ratio, 0.0);
}

#[test]
fn score_counts_with_everything_pure_and_public_returns_one_hundred() {
    // Arrange
    let counts = ItemCounts::default()
        .with_total_functions(2)
        .with_pure_functions(2)
        .with_total_items(3)
        .with_public_items(3)
        .with_inherent_methods(1)
        .with_local_trait_methods(1)
        .with_local_trait_impure(1)
        .with_total_contribution(2.0)
        .with_clean_functions(2);

    // Act
    let scores = scorer().score_counts(&counts);

    // Assert
    assert_eq!(scores.grip_score, Some(100));
    assert_eq!(scores.pure_ratio, 1.0);
    assert_eq!(scores.public_ratio, 1.0);
}

#[test]
fn score_counts_with_nothing_pure_or_public_returns_zero() {
    // Arrange
    let counts = ItemCounts::default()
        .with_total_functions(2)
        .with_pure_functions(0)
        .with_total_items(3)
        .with_public_items(0)
        .with_inherent_impure(1);

    // Act
    let scores = scorer().score_counts(&counts);

    // Assert
    assert_eq!(scores.grip_score, Some(0));
    assert_eq!(scores.pure_ratio, 0.0);
    assert_eq!(scores.public_ratio, 0.0);
}
