// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::offender::Offender;
use grip::offenders_renderer::OffendersRenderer;

#[test]
fn render_with_empty_offenders_returns_empty() {
    // Arrange
    let renderer = OffendersRenderer::new();

    // Act
    let lines = renderer.render(&[], 50);

    // Assert
    assert!(lines.is_empty());
}

#[test]
fn render_with_offenders_shows_threshold_in_header() {
    // Arrange
    let renderer = OffendersRenderer::new();
    let offenders = vec![Offender {
        path: "bad_mod".to_string(),
        grip_score: 30,
    }];

    // Act
    let lines = renderer.render(&offenders, 50);

    // Assert
    assert!(
        lines
            .iter()
            .any(|l| l.contains("Offenders") && l.contains("50"))
    );
}

#[test]
fn render_with_offenders_shows_each_path_and_score() {
    // Arrange
    let renderer = OffendersRenderer::new();
    let offenders = vec![
        Offender {
            path: "bad_mod".to_string(),
            grip_score: 30,
        },
        Offender {
            path: "worse_mod".to_string(),
            grip_score: 12,
        },
    ];

    // Act
    let lines = renderer.render(&offenders, 50);

    // Assert
    assert!(
        lines
            .iter()
            .any(|l| l.contains("bad_mod") && l.contains("30"))
    );
    assert!(
        lines
            .iter()
            .any(|l| l.contains("worse_mod") && l.contains("12"))
    );
}
