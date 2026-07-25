// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::overall_stats::OverallStats;
use grip::overall_summary_renderer::OverallSummaryRenderer;

fn stats() -> OverallStats {
    OverallStats {
        grip_score: Some(71),
        public_items: 10,
        total_functions: 20,
        pure_functions: 12,
        pure_ratio: 0.6,
        public_ratio: 0.5,
        inherent_methods: 3,
        local_trait_methods: 2,
        trait_ratio: 0.4,
        avg_contribution: 0.7,
        clean_fn_ratio: 0.65,
        grip_absolute_total: 12.34,
    }
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
fn render_always_shows_absolute_total() {
    // Arrange
    let renderer = OverallSummaryRenderer::new();

    // Act
    let lines = renderer.render(&stats());

    // Assert
    assert!(lines.iter().any(|l| l.contains("12.34")));
}

#[test]
fn render_with_zero_impl_methods_shows_na_trait_line() {
    // Arrange
    let renderer = OverallSummaryRenderer::new();
    let overall = OverallStats {
        inherent_methods: 0,
        local_trait_methods: 0,
        ..stats()
    };

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
    let overall = OverallStats {
        inherent_methods: 5,
        local_trait_methods: 0,
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
