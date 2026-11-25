// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::function_info::FunctionInfo;
use grip::reporting::grip_report::GripReport;
use grip::reporting::module_stats::ModuleStats;
use grip::reporting::offender::Offender;
use grip::reporting::overall_stats::OverallStats;
use grip::reporting::text_report_renderer::TextReportRenderer;
use grip::traits::report_renderer::ReportRenderer;

fn dummy_modules() -> Vec<ModuleStats> {
    vec![
        ModuleStats {
            path: "alpha".to_string(),
            grip_score: Some(80),
            pure_ratio: 0.8,
            public_items: 5,
            total_functions: 10,
            pure_functions: 8,
            public_ratio: 0.5,
            grip_absolute_total: 7.0,
            ..Default::default()
        },
        ModuleStats {
            path: "beta".to_string(),
            grip_score: Some(50),
            pure_ratio: 0.4,
            public_items: 5,
            total_functions: 10,
            pure_functions: 4,
            public_ratio: 0.5,
            grip_absolute_total: 5.0,
            ..Default::default()
        },
    ]
}

fn dummy_report() -> GripReport {
    report_with(dummy_modules(), vec![], vec![])
}

fn module_line<'a>(out: &'a str, path: &str) -> &'a str {
    let start = format!("  {path} ");
    out.lines()
        .find(|line| line.starts_with(&start))
        .expect("the module should have a line")
}

fn report_with(
    modules: Vec<ModuleStats>,
    offenders: Vec<Offender>,
    functions: Vec<FunctionInfo>,
) -> GripReport {
    GripReport {
        version: "0.1.0".to_string(),
        target: "my-crate".to_string(),
        overall: OverallStats {
            grip_score: Some(71),
            public_items: 10,
            total_functions: 20,
            pure_functions: 12,
            pure_ratio: 0.6,
            public_ratio: 0.5,
            grip_absolute_total: 12.0,
            ..Default::default()
        },
        modules,
        offenders,
        offender_threshold: 50,
        functions,
    }
}

fn reporter() -> TextReportRenderer {
    TextReportRenderer::new(false)
}

#[test]
fn human_output_contains_module_lines() {
    // Arrange
    let report = dummy_report();
    let reporter = reporter();

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    assert!(out.contains("alpha"));
    assert!(out.contains("beta"));
}

#[test]
fn human_output_contains_score() {
    // Arrange
    let report = dummy_report();
    let reporter = reporter();

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    assert!(out.contains("71 / 100"));
}

#[test]
fn human_output_non_verbose_hides_per_function_detail() {
    // Arrange
    let report = report_with(
        dummy_modules(),
        vec![],
        vec![FunctionInfo {
            name: "compute".to_string(),
            file: "src/lib.rs".to_string(),
            is_pure: true,
            is_public: true,
            grip_absolute: 0.95,
            grip_normalized: 95,
            ..Default::default()
        }],
    );
    let reporter = reporter();

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    assert!(!out.contains("Per-function detail"));
}

#[test]
fn human_output_shows_offenders_section() {
    // Arrange
    let report = report_with(
        vec![ModuleStats {
            path: "bad_mod".to_string(),
            grip_score: Some(30),
            pure_ratio: 0.3,
            public_items: 1,
            total_functions: 5,
            pure_functions: 1,
            public_ratio: 0.2,
            grip_absolute_total: 2.0,
            ..Default::default()
        }],
        vec![Offender {
            path: "bad_mod".to_string(),
            grip_score: 30,
        }],
        vec![],
    );
    let reporter = reporter();

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    assert!(out.contains("Offenders"));
    assert!(out.contains("bad_mod"));
}

#[test]
fn human_output_verbose_shows_per_function_detail() {
    // Arrange
    let report = report_with(
        dummy_modules(),
        vec![],
        vec![FunctionInfo {
            name: "compute".to_string(),
            file: "src/lib.rs".to_string(),
            is_pure: true,
            is_public: true,
            grip_absolute: 0.95,
            grip_normalized: 95,
            ..Default::default()
        }],
    );
    let reporter = TextReportRenderer::new(true);

    // Act
    let out = reporter.render(&report).unwrap();

    // Assert
    assert!(out.contains("Per-function detail"));
    assert!(out.contains("compute"));
}

#[test]
fn render_with_and_without_impl_methods_shows_the_trait_ratio_only_where_there_are_some() {
    // Arrange
    let report = report_with(
        vec![
            ModuleStats {
                path: "seamed".to_string(),
                grip_score: Some(90),
                local_trait_methods: 1,
                trait_ratio: 1.0,
                ..Default::default()
            },
            ModuleStats {
                path: "bare".to_string(),
                grip_score: Some(90),
                ..Default::default()
            },
        ],
        vec![],
        vec![],
    );

    // Act
    let out = reporter().render(&report).unwrap();

    // Assert
    assert!(module_line(&out, "seamed").contains("traits: 100.0%"));
    assert!(module_line(&out, "bare").contains("traits:    N/A"));
}

#[test]
fn render_with_modules_at_the_band_edges_marks_each_line_by_its_band() {
    // Arrange
    let modules = [39, 40, 69, 70]
        .map(|score| ModuleStats {
            path: format!("score_{score}"),
            grip_score: Some(score),
            ..Default::default()
        })
        .to_vec();
    let report = report_with(modules, vec![], vec![]);

    // Act
    let out = reporter().render(&report).unwrap();

    // Assert
    let markers: Vec<&str> = ["score_39", "score_40", "score_69", "score_70"]
        .map(|path| module_line(&out, path).rsplit("%  ").next().unwrap())
        .to_vec();
    assert_eq!(markers, ["❌", "⚠️", "⚠️", ""]);
}

#[test]
fn render_without_verbose_heads_the_output_with_the_version_and_target() {
    // Arrange
    let report = dummy_report();

    // Act
    let out = reporter().render(&report).unwrap();

    // Assert
    assert_eq!(
        out.lines().next(),
        Some("cargo-grip4rust 0.1.0 -- my-crate")
    );
}
