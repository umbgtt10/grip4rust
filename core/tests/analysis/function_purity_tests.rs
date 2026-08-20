// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::function_purity::FunctionPurity;
use syn::{ItemFn, parse_str};

fn parse_fn(source: &str) -> ItemFn {
    parse_str(source).expect("valid fn source")
}

#[test]
fn has_io_call_with_io_call_returns_true() {
    // Arrange
    let item_fn = parse_fn(r#"fn f() { fs::read_to_string("x").unwrap(); }"#);

    // Act
    let result = FunctionPurity::has_io_call(&item_fn.block);

    // Assert
    assert!(result);
}

#[test]
fn has_io_call_with_no_io_call_returns_false() {
    // Arrange
    let item_fn = parse_fn("fn f() { let x = 1; }");

    // Act
    let result = FunctionPurity::has_io_call(&item_fn.block);

    // Assert
    assert!(!result);
}

#[test]
fn has_mut_param_with_by_value_mut_self_returns_false() {
    // Arrange
    let item_fn = parse_fn("fn f(mut self) {}");

    // Act
    let result = FunctionPurity::has_mut_param(&item_fn.sig);

    // Assert
    assert!(
        !result,
        "mut self is by-value, locally-mutable, no observable side effect"
    );
}

#[test]
fn has_mut_param_with_mut_reference_in_parens_returns_true() {
    // Arrange
    let item_fn = parse_fn("fn f(x: (&mut i32)) {}");

    // Act
    let result = FunctionPurity::has_mut_param(&item_fn.sig);

    // Assert
    assert!(result, "a parenthesized &mut type must still be recognized");
}

#[test]
fn has_mut_param_with_mut_reference_typed_arg_returns_true() {
    // Arrange
    let item_fn = parse_fn("fn f(x: &mut i32) {}");

    // Act
    let result = FunctionPurity::has_mut_param(&item_fn.sig);

    // Assert
    assert!(result, "a &mut typed parameter is a real mutable reference");
}

#[test]
fn has_mut_param_with_mut_self_reference_returns_true() {
    // Arrange
    let item_fn = parse_fn("fn f(&mut self) {}");

    // Act
    let result = FunctionPurity::has_mut_param(&item_fn.sig);

    // Assert
    assert!(result, "&mut self is a real mutable reference");
}

#[test]
fn has_mut_param_with_no_params_returns_false() {
    // Arrange
    let item_fn = parse_fn("fn f() {}");

    // Act
    let result = FunctionPurity::has_mut_param(&item_fn.sig);

    // Assert
    assert!(!result);
}

#[test]
fn has_mut_param_with_plain_self_returns_false() {
    // Arrange
    let item_fn = parse_fn("fn f(&self) {}");

    // Act
    let result = FunctionPurity::has_mut_param(&item_fn.sig);

    // Assert
    assert!(!result);
}

#[test]
fn has_unsafe_block_with_no_unsafe_returns_false() {
    // Arrange
    let item_fn = parse_fn("fn f() { let x = 1; }");

    // Act
    let result = FunctionPurity::has_unsafe_block(&item_fn.block);

    // Assert
    assert!(!result);
}

#[test]
fn has_unsafe_block_with_unsafe_block_returns_true() {
    // Arrange
    let item_fn = parse_fn("fn f() { unsafe {} }");

    // Act
    let result = FunctionPurity::has_unsafe_block(&item_fn.block);

    // Assert
    assert!(result);
}

#[test]
fn is_unit_return_with_explicit_unit_tuple_returns_true() {
    // Arrange
    let item_fn = parse_fn("fn f() -> () {}");

    // Act
    let result = FunctionPurity::is_unit_return(&item_fn.sig);

    // Assert
    assert!(result);
}

#[test]
fn is_unit_return_with_no_return_type_returns_true() {
    // Arrange
    let item_fn = parse_fn("fn f() {}");

    // Act
    let result = FunctionPurity::is_unit_return(&item_fn.sig);

    // Assert
    assert!(result);
}

#[test]
fn is_unit_return_with_non_empty_tuple_returns_false() {
    // Arrange
    let item_fn = parse_fn("fn f() -> (i32, i32) { (0, 0) }");

    // Act
    let result = FunctionPurity::is_unit_return(&item_fn.sig);

    // Assert
    assert!(!result, "a non-empty tuple return is not the same as unit");
}

#[test]
fn is_unit_return_with_non_unit_type_returns_false() {
    // Arrange
    let item_fn = parse_fn("fn f() -> i32 { 0 }");

    // Act
    let result = FunctionPurity::is_unit_return(&item_fn.sig);

    // Assert
    assert!(!result);
}
