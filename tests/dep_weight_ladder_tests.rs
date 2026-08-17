// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::dep_weight_ladder::DepWeightLadder;
use syn::Path;

fn path(source: &str) -> Path {
    syn::parse_str(source).expect("path should parse")
}

#[test]
fn weight_of_a_print_macro_returns_the_lightest_weight() {
    // Arrange & Act
    let weight = DepWeightLadder::weight_of("print");

    // Assert
    assert!((weight - 0.2).abs() < f64::EPSILON);
}

// `print` prefixes `println` and `eprint` prefixes `eprintln`, so two arms have
// to cover all four macros.
#[test]
fn weight_of_every_print_family_macro_returns_the_lightest_weight() {
    // Act & Assert
    for label in ["print", "println", "eprint", "eprintln"] {
        assert!((DepWeightLadder::weight_of(label) - 0.2).abs() < f64::EPSILON);
    }
}

#[test]
fn weight_of_a_clock_reference_returns_more_than_printing() {
    // Arrange & Act
    let clock = DepWeightLadder::weight_of("Instant::now");
    let printing = DepWeightLadder::weight_of("println");

    // Assert
    assert!((clock - 0.3).abs() < f64::EPSILON);
    assert!(clock > printing);
}

#[test]
fn weight_of_an_elapsed_call_is_treated_as_a_clock_reference() {
    // Arrange & Act
    let weight = DepWeightLadder::weight_of("start.elapsed");

    // Assert
    assert!((weight - 0.3).abs() < f64::EPSILON);
}

#[test]
fn weight_of_an_environment_call_returns_more_than_a_clock() {
    // Arrange & Act
    let environment = DepWeightLadder::weight_of("env::var");
    let clock = DepWeightLadder::weight_of("Instant::now");

    // Assert
    assert!((environment - 0.4).abs() < f64::EPSILON);
    assert!(environment > clock);
}

#[test]
fn weight_of_an_unsafe_block_returns_more_than_touching_the_environment() {
    // Arrange & Act
    let unsafe_weight = DepWeightLadder::weight_of("unsafe");
    let environment = DepWeightLadder::weight_of("env::var");

    // Assert
    assert!((unsafe_weight - 0.5).abs() < f64::EPSILON);
    assert!(unsafe_weight > environment);
}

#[test]
fn weight_of_an_unrecognised_dependency_returns_the_heaviest_weight() {
    // Arrange & Act
    let weight = DepWeightLadder::weight_of("StripeGateway::charge");

    // Assert
    assert!((weight - 0.6).abs() < f64::EPSILON);
}

#[test]
fn weight_of_the_ladder_is_strictly_ordered_from_print_to_unknown() {
    // Arrange & Act
    let ladder = [
        DepWeightLadder::weight_of("println"),
        DepWeightLadder::weight_of("Instant::now"),
        DepWeightLadder::weight_of("env::var"),
        DepWeightLadder::weight_of("unsafe"),
        DepWeightLadder::weight_of("Whatever::new"),
    ];

    // Assert
    assert!(ladder.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn label_of_a_single_segment_path_returns_the_bare_name() {
    // Arrange & Act
    let label = DepWeightLadder::label_of(&path("println"));

    // Assert
    assert_eq!(label, "println");
}

#[test]
fn label_of_a_multi_segment_path_joins_with_double_colons() {
    // Arrange & Act
    let label = DepWeightLadder::label_of(&path("std::env::var"));

    // Assert
    assert_eq!(label, "std::env::var");
}
