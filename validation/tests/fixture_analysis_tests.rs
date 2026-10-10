// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::fixture_analysis::FixtureAnalysis;

#[test]
fn report_of_a_fixture_targets_its_directory() {
    // Arrange
    let analysis = FixtureAnalysis::new("data_only");

    // Act
    let report = analysis.report();

    // Assert
    assert_eq!(report["target"], "data_only");
}

#[test]
#[should_panic(expected = "app run failed")]
fn report_of_a_missing_fixture_panics_naming_the_run() {
    // Arrange
    let analysis = FixtureAnalysis::new("no_such_fixture");

    // Act & Assert
    let _ = analysis.report();
}
