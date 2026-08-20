// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::detection::contribution_schedule::ContributionSchedule;

fn schedule() -> ContributionSchedule {
    ContributionSchedule::new()
}

#[test]
fn contribution_dep_weight_above_one_returns_zero() {
    // Arrange & Act
    let contr = schedule().contribution(true, true, 1.5);

    // Assert
    assert_eq!(contr, 0.0);
}

#[test]
fn contribution_dep_weight_at_one_returns_zero() {
    // Arrange & Act
    let contr = schedule().contribution(true, true, 1.0);

    // Assert
    assert_eq!(contr, 0.0);
}

#[test]
fn contribution_dep_weight_scales_base_linearly() {
    // Arrange & Act
    let contr = schedule().contribution(true, true, 0.5);

    // Assert
    assert_eq!(contr, 0.5);
}

#[test]
fn contribution_impure_and_seam_returns_0_85() {
    // Arrange & Act
    let contr = schedule().contribution(false, true, 0.0);

    // Assert
    assert_eq!(contr, 0.85);
}

#[test]
fn contribution_impure_no_seam_returns_0_15() {
    // Arrange & Act
    let contr = schedule().contribution(false, false, 0.0);

    // Assert
    assert_eq!(contr, 0.15);
}

#[test]
fn contribution_pure_and_seam_returns_1_00() {
    // Arrange & Act
    let contr = schedule().contribution(true, true, 0.0);

    // Assert
    assert_eq!(contr, 1.00);
}

#[test]
fn contribution_pure_no_seam_returns_0_95() {
    // Arrange & Act
    let contr = schedule().contribution(true, false, 0.0);

    // Assert
    assert_eq!(contr, 0.95);
}
