// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::reporting::scores::Scores;

#[test]
fn eq_with_the_pure_and_public_ratios_swapped_returns_false() {
    // Arrange
    let scores = Scores {
        grip_score: Some(50),
        pure_ratio: 0.25,
        public_ratio: 0.75,
        trait_ratio: 1.0,
        avg_contribution: 0.0,
        clean_fn_ratio: 0.0,
    };
    let swapped = Scores {
        grip_score: Some(50),
        pure_ratio: 0.75,
        public_ratio: 0.25,
        trait_ratio: 1.0,
        avg_contribution: 0.0,
        clean_fn_ratio: 0.0,
    };

    // Act
    let equal = scores == swapped;

    // Assert
    assert!(!equal);
}
