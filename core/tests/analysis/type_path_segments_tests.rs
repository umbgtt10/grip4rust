// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::type_path_segments::TypePathSegments;
use syn::Type;
use syn::parse_str;

#[test]
fn first_ident_of_a_generic_path_returns_its_leading_segment() {
    // Arrange
    let ty: Type = parse_str("Holder<Inner>").expect("parse type");

    // Act
    let first = TypePathSegments::new(&ty).first_ident();

    // Assert
    assert_eq!(first, "Holder");
}

#[test]
fn first_ident_of_a_qualified_path_returns_the_qualifier() {
    // Arrange
    let ty: Type = parse_str("std::collections::HashMap").expect("parse type");

    // Act
    let first = TypePathSegments::new(&ty).first_ident();

    // Assert
    assert_eq!(first, "std");
}

#[test]
fn first_ident_of_a_type_that_is_not_a_path_returns_an_empty_string() {
    // Arrange
    let ty: Type = parse_str("&Holder").expect("parse type");

    // Act
    let first = TypePathSegments::new(&ty).first_ident();

    // Assert
    assert_eq!(first, "");
}

#[test]
fn last_ident_of_a_qualified_path_returns_the_type_name() {
    // Arrange
    let ty: Type = parse_str("std::collections::HashMap").expect("parse type");

    // Act
    let last = TypePathSegments::new(&ty).last_ident();

    // Assert
    assert_eq!(last, "HashMap");
}

#[test]
fn last_ident_of_a_single_segment_path_returns_that_segment() {
    // Arrange
    let ty: Type = parse_str("u32").expect("parse type");

    // Act
    let last = TypePathSegments::new(&ty).last_ident();

    // Assert
    assert_eq!(last, "u32");
}

#[test]
fn last_ident_of_a_type_that_is_not_a_path_returns_an_empty_string() {
    // Arrange
    let ty: Type = parse_str("(u8, u16)").expect("parse type");

    // Act
    let last = TypePathSegments::new(&ty).last_ident();

    // Assert
    assert_eq!(last, "");
}
