// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::reporting::scores::Scores;

#[test]
fn eq_with_the_pure_and_public_ratios_swapped_returns_false() {
    // Arrange
    let scores = Scores::new(Some(50), 0.25, 0.75, 1.0, 0.0, 0.0);
    let swapped = Scores::new(Some(50), 0.75, 0.25, 1.0, 0.0, 0.0);

    // Act
    let equal = scores == swapped;

    // Assert
    assert!(!equal);
}
