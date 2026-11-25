// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::reporting::overall_stats::OverallStats;
use grip::reporting::overall_summary_renderer::OverallSummaryRenderer;

fn stats() -> OverallStats {
    OverallStats {
        grip_score: Some(71),
        public_items: 10,
        total_functions: 20,
        pure_functions: 12,
        pure_ratio: 0.6,
        public_ratio: 0.5,
        inherent_methods: 6,
        inherent_impure: 3,
        local_trait_methods: 4,
        local_trait_impure: 2,
        trait_ratio: 0.4,
        avg_contribution: 0.7,
        avg_hidden_deps: 0.35,
        clean_fn_ratio: 0.65,
        grip_absolute_total: 12.34,
    }
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
fn render_with_hidden_deps_shows_their_average_per_function() {
    // Arrange
    let renderer = OverallSummaryRenderer::new();

    // Act
    let lines = renderer.render(&stats());

    // Assert
    assert!(lines.contains(
        &"Hidden deps:           avg 0.35  — 65.0% clean  (70.0% avg contribution)".to_string()
    ));
}

#[test]
fn render_with_impure_impl_methods_counts_the_trait_bound_ones_out_of_the_impure_ones() {
    // Arrange
    let renderer = OverallSummaryRenderer::new();

    // Act
    let lines = renderer.render(&stats());

    // Assert
    assert!(lines.contains(
        &"Trait methods:         2 / 5 impure impl methods are trait-bound  (40.0%)".to_string()
    ));
}

#[test]
fn render_with_none_grip_score_shows_na() {
    // Arrange
    let renderer = OverallSummaryRenderer::new();
    let overall = OverallStats {
        grip_score: None,
        ..stats()
    };

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
fn render_with_only_pure_impl_methods_shows_na_trait_line() {
    // Arrange
    let renderer = OverallSummaryRenderer::new();
    let overall = OverallStats {
        inherent_impure: 0,
        local_trait_impure: 0,
        trait_ratio: 1.0,
        ..stats()
    };

    // Act
    let lines = renderer.render(&overall);

    // Assert
    assert!(lines.contains(&"Trait methods:         N/A    (no impure impl methods)".to_string()));
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
    let overall = OverallStats {
        inherent_methods: 0,
        inherent_impure: 0,
        local_trait_methods: 0,
        local_trait_impure: 0,
        ..stats()
    };

    // Act
    let lines = renderer.render(&overall);

    // Assert
    assert!(lines.iter().any(|l| l.contains("Trait methods")
        && l.contains("N/A")
        && l.contains("no impure impl methods")));
}

#[test]
fn render_with_zero_trait_ratio_shows_zero_percent() {
    // Arrange
    let renderer = OverallSummaryRenderer::new();
    let overall = OverallStats {
        inherent_methods: 5,
        local_trait_methods: 0,
        local_trait_impure: 0,
        trait_ratio: 0.0,
        ..stats()
    };

    // Act
    let lines = renderer.render(&overall);

    // Assert
    assert!(
        lines
            .iter()
            .any(|l| l.contains("Trait methods") && l.contains("0.0%"))
    );
}
