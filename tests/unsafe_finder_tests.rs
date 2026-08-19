// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::unsafe_finder::UnsafeFinder;
use syn::visit::Visit;

fn finds_unsafe_in(source: &str) -> bool {
    let file = syn::parse_file(source).expect("the source should parse");
    let mut finder = UnsafeFinder::new();
    finder.visit_file(&file);
    finder.found
}

#[test]
fn new_starts_having_found_nothing() {
    // Arrange & Act
    let finder = UnsafeFinder::new();

    // Assert
    assert!(!finder.found);
}

#[test]
fn visit_file_with_an_unsafe_block_finds_it() {
    // Arrange
    let source = "fn risky() {\n    unsafe {\n        let _ = 1;\n    }\n}\n";

    // Act
    let found = finds_unsafe_in(source);

    // Assert
    assert!(found);
}

// Nested inside another expression rather than at the top of the body, which is
// where a visitor that only looked one level deep would miss it.
#[test]
fn visit_file_with_an_unsafe_block_nested_in_an_expression_finds_it() {
    // Arrange
    let source = "fn wrapped() {\n    let _ = if true {\n        unsafe { 1 }\n    } else {\n        0\n    };\n}\n";

    // Act
    let found = finds_unsafe_in(source);

    // Assert
    assert!(found);
}

// An `unsafe fn` declares that calling it is unsafe; it does not contain an
// unsafe *expression*. This visitor looks for the block, so the distinction
// matters and is easy to get wrong by reading for the keyword instead.
#[test]
fn visit_file_with_an_unsafe_fn_but_no_unsafe_block_finds_nothing() {
    // Arrange
    let source = "unsafe fn declared() {\n    let _ = 1;\n}\n";

    // Act
    let found = finds_unsafe_in(source);

    // Assert
    assert!(!found);
}

// The visitor answers "is there an unsafe block", not "is anything named
// unsafe", so a function whose body is entirely safe stays false however much
// else it contains.
#[test]
fn visit_file_with_only_safe_code_finds_nothing() {
    // Arrange
    let source = "fn calm() {\n    let total = 1 + 2;\n    let _ = total;\n}\n";

    // Act
    let found = finds_unsafe_in(source);

    // Assert
    assert!(!found);
}

// Once found it stays found: a later safe function must not clear the flag.
#[test]
fn visit_file_with_unsafe_before_safe_code_stays_found() {
    // Arrange
    let source = "fn risky() {\n    unsafe { let _ = 1; }\n}\n\nfn calm() {\n    let _ = 2;\n}\n";

    // Act
    let found = finds_unsafe_in(source);

    // Assert
    assert!(found);
}
