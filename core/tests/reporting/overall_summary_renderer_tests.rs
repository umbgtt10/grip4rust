// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::reporting::overall_stats::OverallStats;
use grip::reporting::overall_summary_renderer::OverallSummaryRenderer;

fn stats() -> OverallStats {
    OverallStats::default()
        .with_grip_score(Some(71))
        .with_public_items(10)
        .with_total_functions(20)
        .with_pure_functions(12)
        .with_pure_ratio(0.6)
        .with_public_ratio(0.5)
        .with_inherent_methods(3)
        .with_local_trait_methods(2)
        .with_trait_ratio(0.4)
        .with_avg_contribution(0.7)
        .with_clean_fn_ratio(0.65)
        .with_grip_absolute_total(12.34)
}

#[test]
fn render_always_shows_absolute_total() {
    // Arrange
    let renderer = OverallSummaryRenderer::new();

    // Act
    let lines = renderer.render(&stats());

    // Assert
    assert!(lines.iter().any(|l| l.contains("12.34")));
}

#[test]
fn render_with_none_grip_score_shows_na() {
    // Arrange
    let renderer = OverallSummaryRenderer::new();
    let overall = stats().with_grip_score(None);

    // Act
    let lines = renderer.render(&overall);

    // Assert
    assert!(
        lines
            .iter()
            .any(|l| l.contains("N/A") && l.contains("no functions"))
    );
}

#[test]
fn render_with_nonzero_trait_ratio_shows_percentage() {
    // Arrange
    let renderer = OverallSummaryRenderer::new();

    // Act
    let lines = renderer.render(&stats());

    // Assert
    assert!(
        lines
            .iter()
            .any(|l| l.contains("Trait methods") && l.contains("40.0%"))
    );
}

#[test]
fn render_with_some_grip_score_shows_score_line() {
    // Arrange
    let renderer = OverallSummaryRenderer::new();

    // Act
    let lines = renderer.render(&stats());

    // Assert
    assert!(lines.iter().any(|l| l.contains("71 / 100")));
}

#[test]
fn render_with_zero_impl_methods_shows_na_trait_line() {
    // Arrange
    let renderer = OverallSummaryRenderer::new();
    let overall = stats().with_inherent_methods(0).with_local_trait_methods(0);

    // Act
    let lines = renderer.render(&overall);

    // Assert
    assert!(lines.iter().any(|l| l.contains("Trait methods")
        && l.contains("N/A")
        && l.contains("no impl methods")));
}

#[test]
fn render_with_zero_trait_ratio_shows_zero_percent() {
    // Arrange
    let renderer = OverallSummaryRenderer::new();
    let overall = stats()
        .with_inherent_methods(5)
        .with_local_trait_methods(0)
        .with_trait_ratio(0.0);

    // Act
    let lines = renderer.render(&overall);

    // Assert
    assert!(
        lines
            .iter()
            .any(|l| l.contains("Trait methods") && l.contains("0.0%"))
    );
}
