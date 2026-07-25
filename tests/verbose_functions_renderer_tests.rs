// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::function_info::FunctionInfo;
use grip::verbose_functions_renderer::VerboseFunctionsRenderer;

fn function(name: &str, file: &str, hidden_deps: usize, labels: Vec<&str>) -> FunctionInfo {
    FunctionInfo {
        name: name.to_string(),
        file: file.to_string(),
        is_pure: true,
        is_public: true,
        hidden_deps,
        has_trait_seam: false,
        dep_weight: 0.0,
        hidden_dep_labels: labels.into_iter().map(str::to_string).collect(),
        grip_absolute: 0.95,
        grip_normalized: 95,
    }
}

#[test]
fn render_with_verbose_false_returns_empty() {
    // Arrange
    let renderer = VerboseFunctionsRenderer::new(false);
    let functions = vec![function("compute", "src/lib.rs", 0, vec![])];

    // Act
    let lines = renderer.render(&functions);

    // Assert
    assert!(lines.is_empty());
}

#[test]
fn render_with_empty_functions_returns_empty() {
    // Arrange
    let renderer = VerboseFunctionsRenderer::new(true);

    // Act
    let lines = renderer.render(&[]);

    // Assert
    assert!(lines.is_empty());
}

#[test]
fn render_with_verbose_true_shows_function_name() {
    // Arrange
    let renderer = VerboseFunctionsRenderer::new(true);
    let functions = vec![function("compute", "src/lib.rs", 0, vec![])];

    // Act
    let lines = renderer.render(&functions);

    // Assert
    assert!(lines.iter().any(|l| l.contains("compute")));
}

#[test]
fn render_groups_functions_by_file() {
    // Arrange
    let renderer = VerboseFunctionsRenderer::new(true);
    let functions = vec![
        function("a_fn", "src/a.rs", 0, vec![]),
        function("b_fn", "src/b.rs", 0, vec![]),
    ];

    // Act
    let lines = renderer.render(&functions);

    // Assert
    assert!(lines.iter().any(|l| l.contains("src/a.rs:")));
    assert!(lines.iter().any(|l| l.contains("src/b.rs:")));
}

#[test]
fn render_with_no_hidden_deps_shows_check_marker() {
    // Arrange
    let renderer = VerboseFunctionsRenderer::new(true);
    let functions = vec![function("clean_fn", "src/lib.rs", 0, vec![])];

    // Act
    let lines = renderer.render(&functions);

    // Assert
    assert!(
        lines
            .iter()
            .any(|l| l.contains("clean_fn") && l.contains('✅'))
    );
}

#[test]
fn render_with_one_hidden_dep_shows_warning_marker() {
    // Arrange
    let renderer = VerboseFunctionsRenderer::new(true);
    let functions = vec![function("risky_fn", "src/lib.rs", 1, vec!["println"])];

    // Act
    let lines = renderer.render(&functions);

    // Assert
    assert!(
        lines
            .iter()
            .any(|l| l.contains("risky_fn") && l.contains("⚠️"))
    );
}

#[test]
fn render_with_multiple_hidden_deps_shows_cross_marker() {
    // Arrange
    let renderer = VerboseFunctionsRenderer::new(true);
    let functions = vec![function(
        "bad_fn",
        "src/lib.rs",
        2,
        vec!["println", "Instant::now"],
    )];

    // Act
    let lines = renderer.render(&functions);

    // Assert
    assert!(
        lines
            .iter()
            .any(|l| l.contains("bad_fn") && l.contains('❌'))
    );
}

#[test]
fn render_with_empty_labels_shows_dash() {
    // Arrange
    let renderer = VerboseFunctionsRenderer::new(true);
    let functions = vec![function("clean_fn", "src/lib.rs", 0, vec![])];

    // Act
    let lines = renderer.render(&functions);

    // Assert
    assert!(
        lines
            .iter()
            .any(|l| l.contains("clean_fn") && l.contains("[-]"))
    );
}

#[test]
fn render_with_labels_shows_them_in_brackets() {
    // Arrange
    let renderer = VerboseFunctionsRenderer::new(true);
    let functions = vec![function("risky_fn", "src/lib.rs", 1, vec!["println"])];

    // Act
    let lines = renderer.render(&functions);

    // Assert
    assert!(lines.iter().any(|l| l.contains("[println]")));
}
