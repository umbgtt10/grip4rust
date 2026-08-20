// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::method_purity_registry::MethodPurityRegistry;
use grip::analysis::struct_registry::StructRegistry;
use grip::detection::hidden_dep_finder::HiddenDepFinder;
use std::collections::HashMap;
use std::path::PathBuf;
use syn::visit::Visit;
use syn::{ItemFn, parse_str};

const EPSILON: f64 = 1e-9;

fn deps_of(body: &str) -> (usize, f64, Vec<String>) {
    let fixture = Fixture::empty();
    let item: ItemFn = parse_str(body).expect("test fixture must be a valid fn");
    let mut finder = fixture.finder();
    finder.visit_block(&item.block);
    (finder.count, finder.weight, finder.labels.clone())
}

struct Fixture {
    struct_registry: StructRegistry,
    method_purity: MethodPurityRegistry,
}

impl Fixture {
    fn new(sources: &[&str]) -> Self {
        let files: Vec<(PathBuf, String)> = sources
            .iter()
            .enumerate()
            .map(|(i, s)| (PathBuf::from(format!("f{i}.rs")), (*s).to_string()))
            .collect();
        let struct_registry = StructRegistry::build(&files);
        let method_purity = MethodPurityRegistry::build(&files, &struct_registry);
        Self {
            struct_registry,
            method_purity,
        }
    }

    fn empty() -> Self {
        Self::new(&[])
    }

    fn finder(&self) -> HiddenDepFinder<'_> {
        HiddenDepFinder::new(&self.struct_registry, &self.method_purity)
    }
}

fn weight_of(body: &str) -> f64 {
    deps_of(body).1
}

#[test]
fn a_dependency_nested_inside_control_flow_is_still_found() {
    // Arrange -- hidden dependencies hide inside branches and loops. A visitor
    // that stopped descending would report the tidy functions accurately and
    // the tangled ones as clean, which is exactly backwards.
    let body = r#"
fn f(items: Vec<u64>) {
    for item in items {
        if item > 0 {
            match item {
                1 => { env::var("HOME"); }
                _ => {}
            }
        }
    }
}
"#;

    // Act & Assert
    assert_eq!(deps_of(body).0, 1);
}

#[test]
fn a_method_call_on_bare_self_is_not_a_hidden_dependency() {
    // Arrange -- self.method() is an internal call. Counting it would make
    // every well-decomposed type look worse than one giant function.

    // Act & Assert
    assert_eq!(deps_of("fn f(&self) { self.helper(); }").0, 0);
}

#[test]
fn a_method_on_a_concrete_field_of_a_known_pure_type_is_not_a_dependency() {
    // Arrange -- calling `len` on a Vec field is a pure value read. Counting it
    // would penalise holding data by value.
    let fixture = Fixture::empty();
    let item: ItemFn = parse_str("fn f(&self) { self.items.len(); }").expect("valid fn");
    let mut finder = fixture.finder();
    finder.set_concrete_fields(HashMap::from([(
        String::from("items"),
        String::from("Vec"),
    )]));

    // Act
    finder.visit_block(&item.block);

    // Assert
    assert_eq!(finder.count, 0);
}

#[test]
fn a_method_on_an_unknown_field_is_not_a_dependency() {
    // Arrange -- without a concrete type the analyzer cannot tell a pure read
    // from a side effect, and guessing would produce findings it cannot defend.
    let fixture = Fixture::empty();
    let item: ItemFn = parse_str("fn f(&self) { self.whatever.poke(); }").expect("valid fn");
    let mut finder = fixture.finder();

    // Act
    finder.visit_block(&item.block);

    // Assert
    assert_eq!(finder.count, 0);
}

#[test]
fn a_nonpure_method_on_a_concrete_field_is_a_dependency_labelled_by_the_field() {
    // Arrange -- the label names the field so a reader can find the call site.
    let fixture = Fixture::empty();
    let item: ItemFn = parse_str("fn f(&self) { self.store.write(); }").expect("valid fn");
    let mut finder = fixture.finder();
    finder.set_concrete_fields(HashMap::from([(
        String::from("store"),
        String::from("DiskStore"),
    )]));

    // Act
    finder.visit_block(&item.block);

    // Assert
    assert_eq!(finder.count, 1);
    assert_eq!(finder.labels, vec!["self.store.write"]);
}

#[test]
fn a_print_macro_is_found_in_statement_and_expression_position() {
    // Arrange -- a trailing `println!("x")` with no semicolon is an expression,
    // not a statement, and is handled by a different visitor arm. Covering only
    // one arm leaves half of all real print sites uncounted.

    // Act & Assert
    assert_eq!(deps_of(r#"fn f() { println!("x"); }"#).0, 1);
    assert_eq!(deps_of(r#"fn f() { println!("x") }"#).0, 1);
}

#[test]
fn a_print_macro_weighs_the_least_of_any_dependency() {
    // Arrange -- the ladder encodes a judgement: writing to stdout is the
    // mildest way to reach outside a function. These numbers feed the grip
    // score directly, so a shifted rung silently rescores every crate.

    // Act & Assert
    assert!((weight_of(r#"fn f() { println!("x"); }"#) - 0.2).abs() < EPSILON);
    assert!((weight_of(r#"fn f() { eprintln!("x"); }"#) - 0.2).abs() < EPSILON);
    assert!((weight_of(r#"fn f() { print!("x"); }"#) - 0.2).abs() < EPSILON);
    assert!((weight_of(r#"fn f() { eprint!("x"); }"#) - 0.2).abs() < EPSILON);
}

#[test]
fn a_provably_pure_accessor_on_a_local_type_is_not_a_dependency() {
    // Arrange -- MethodPurityRegistry proves `len` on this local type delegates
    // to a pure read, so the finder must trust it. Without that trust, every
    // wrapper type would be scored as if it were doing I/O.
    let fixture = Fixture::new(&[r#"
struct Members { ids: Vec<u64> }
impl Members {
    pub fn len(&self) -> usize { self.ids.len() }
}
"#]);
    let item: ItemFn = parse_str("fn f(&self) { self.members.len(); }").expect("valid fn");
    let mut finder = fixture.finder();
    finder.set_concrete_fields(HashMap::from([(
        String::from("members"),
        String::from("Members"),
    )]));

    // Act
    finder.visit_block(&item.block);

    // Assert
    assert_eq!(finder.count, 0);
}

#[test]
fn a_pure_body_has_no_dependencies_and_no_weight() {
    // Arrange & Act
    let (count, weight, labels) = deps_of("fn f(a: u64, b: u64) -> u64 { a + b }");

    // Assert
    assert_eq!(count, 0);
    assert!(weight.abs() < EPSILON);
    assert!(labels.is_empty());
}

#[test]
fn a_std_constructor_is_not_a_hidden_dependency() {
    // Arrange -- Vec::new and friends are uppercase-rooted paths, so without
    // the std-constructor allowlist every allocation in the codebase would
    // score as a hidden dependency and the metric would be noise.

    // Act & Assert
    for body in [
        "fn f() { Vec::new(); }",
        "fn f() { String::new(); }",
        "fn f() { Box::new(1); }",
        "fn f() { HashMap::new(); }",
    ] {
        assert_eq!(deps_of(body).0, 0, "{body} must not count");
    }
}

#[test]
fn a_std_module_call_is_recorded_under_its_two_segment_tail() {
    // Arrange -- the label is what a user reads in the offenders list, and the
    // tail is what makes fs::read and std::fs::read the same finding rather
    // than two.

    // Act & Assert
    assert_eq!(deps_of(r#"fn f() { fs::read("p"); }"#).2, vec!["fs::read"]);
    assert_eq!(
        deps_of(r#"fn f() { std::fs::read("p"); }"#).2,
        vec!["fs::read"]
    );
}

#[test]
fn an_associated_call_on_self_is_not_a_hidden_dependency() {
    // Arrange -- reaching into your own type is not reaching outside it.

    // Act & Assert
    assert_eq!(deps_of("fn f() { Self::helper(); }").0, 0);
    assert_eq!(deps_of("fn f() { self::helper(); }").0, 0);
}

#[test]
fn an_unrecognised_dependency_takes_the_heaviest_weight() {
    // Arrange -- the fallback is the top of the ladder, not the bottom. An
    // unknown reach-out is treated as the worst case rather than waved through,
    // which is the conservative direction for a score used as a gate.

    // Act & Assert
    assert!((weight_of("fn f() { Registry::global(); }") - 0.6).abs() < EPSILON);
}

#[test]
fn an_unsafe_block_weighs_more_than_touching_the_environment() {
    // Arrange -- unsafe suspends the guarantees the rest of the score assumes.

    // Act & Assert
    assert!((weight_of("fn f() { unsafe { do_it(); } }") - 0.5).abs() < EPSILON);
}

#[test]
fn count_weight_and_labels_accumulate_across_several_dependencies() {
    // Arrange -- the three outputs must stay in step. A label list that drifted
    // from the count would make the offenders report disagree with the score
    // printed beside it.
    let body = r#"
fn f() {
    println!("a");
    env::var("HOME");
    unsafe { raw(); }
}
"#;

    // Act
    let (count, weight, labels) = deps_of(body);

    // Assert
    assert_eq!(count, 3);
    assert_eq!(labels.len(), count);
    assert!((weight - (0.2 + 0.4 + 0.5)).abs() < EPSILON);
}

#[test]
fn reaching_the_environment_or_the_process_weighs_more_than_the_clock() {
    // Arrange -- env and process reach outside the program entirely.

    // Act & Assert
    assert!((weight_of(r#"fn f() { env::var("HOME"); }"#) - 0.4).abs() < EPSILON);
    assert!((weight_of("fn f() { process::exit(1); }") - 0.4).abs() < EPSILON);
}

#[test]
fn reading_the_clock_weighs_more_than_printing() {
    // Arrange -- time is a hidden input rather than an output: it makes a
    // function's result unreproducible, which printing does not.

    // Act & Assert
    assert!((weight_of("fn f() { Instant::now(); }") - 0.3).abs() < EPSILON);
    assert!((weight_of("fn f() { SystemTime::now(); }") - 0.3).abs() < EPSILON);
    assert!((weight_of("fn f() { Utc::now(); }") - 0.3).abs() < EPSILON);
    assert!((weight_of("fn f() { Local::now(); }") - 0.3).abs() < EPSILON);
}

#[test]
fn the_ladder_is_strictly_ordered_from_print_to_unknown() {
    // Arrange -- the individual rungs are asserted above; this pins their
    // relative order, which is the property the score actually rests on.
    let print = weight_of(r#"fn f() { println!("x"); }"#);
    let clock = weight_of("fn f() { Instant::now(); }");
    let env = weight_of(r#"fn f() { env::var("HOME"); }"#);
    let unsafe_block = weight_of("fn f() { unsafe { do_it(); } }");
    let unknown = weight_of("fn f() { Registry::global(); }");

    // Act & Assert
    assert!(print < clock);
    assert!(clock < env);
    assert!(env < unsafe_block);
    assert!(unsafe_block < unknown);
}

#[test]
fn the_same_dependency_twice_is_counted_twice() {
    // Arrange -- weight is a sum, not a set. Two clock reads are twice the
    // hidden input of one, and deduplicating would flatten that.

    // Act
    let (count, weight, _) = deps_of("fn f() { Instant::now(); Instant::now(); }");

    // Assert
    assert_eq!(count, 2);
    assert!((weight - 0.6).abs() < EPSILON);
}
