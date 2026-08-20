// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::detection::io_call_finder::IoCallFinder;
use syn::visit::Visit;
use syn::{ItemFn, parse_str};

fn performs_io(body: &str) -> bool {
    let item: ItemFn = parse_str(body).expect("test fixture must be a valid fn");
    let mut finder = IoCallFinder::new();
    finder.visit_block(&item.block);
    finder.found
}

#[test]
fn a_body_with_no_calls_performs_no_io() {
    // Arrange & Act & Assert -- the baseline. Pure arithmetic must stay clean,
    // or the distinction the finder exists to draw is worthless.
    assert!(!performs_io("fn f(a: u64, b: u64) -> u64 { a + b }"));
}

#[test]
fn a_free_function_whose_name_is_an_io_method_counts() {
    // Arrange -- the call arm checks the last path segment too, so `open(..)`
    // reached through a `use` counts even without its module qualifier.

    // Act & Assert
    assert!(performs_io("fn f() { open(); }"));
}

#[test]
fn a_single_segment_path_is_not_treated_as_a_flagged_root() {
    // Arrange -- the root rule needs at least two segments. A bare local call
    // named `fs()` is not the `fs` module, and flagging it would make every
    // short helper name a false positive.

    // Act & Assert
    assert!(!performs_io("fn f() { helper(); }"));
}

#[test]
fn default_starts_in_the_same_state_as_new() {
    // Arrange & Act -- both constructors are public, so either can begin a
    // scan. One starting in the found state would condemn a clean function.
    let from_default = IoCallFinder::default();
    let from_new = IoCallFinder::new();

    // Assert
    assert_eq!(from_default.found, from_new.found);
    assert!(!from_default.found);
}

#[test]
fn each_flagged_path_root_is_recognised() {
    // Arrange -- a call qualified by one of these roots is I/O regardless of
    // the method name, which is what catches constructors like File::open.
    let roots = [
        "fs",
        "net",
        "io",
        "TcpStream",
        "UdpSocket",
        "File",
        "OpenOptions",
    ];

    // Act & Assert
    for root in roots {
        let body = format!("fn f() {{ {root}::anything(); }}");
        assert!(performs_io(&body), "path root {root} must count as I/O");
    }
}

#[test]
fn each_io_method_name_is_recognised() {
    // Arrange -- these ten names are the finder's entire method vocabulary.
    // One dropped from the list is a whole class of I/O silently reclassified
    // as pure, and nothing else in the tool would notice.
    let methods = [
        "connect",
        "send_to",
        "recv_from",
        "write_all",
        "read_to_string",
        "flush",
        "open",
        "create",
        "bind",
        "accept",
    ];

    // Act & Assert
    for method in methods {
        let body = format!("fn f(x: T) {{ x.{method}(); }}");
        assert!(performs_io(&body), "method {method} must count as I/O");
    }
}

#[test]
fn finding_io_once_does_not_stop_a_later_body_being_judged_afresh() {
    // Arrange -- each finder judges one function. Sharing state across them
    // would mark everything after the first impure function impure too.

    // Act & Assert
    assert!(performs_io("fn f(s: S) { s.connect(); }"));
    assert!(!performs_io("fn g(a: u64) -> u64 { a * 2 }"));
}

#[test]
fn io_nested_deep_inside_a_body_is_still_found() {
    // Arrange -- real I/O rarely sits at the top of a function. If the visitor
    // stopped descending into blocks, branches or loops, the common case would
    // be the one it missed.
    let body = r#"
fn f(items: Vec<T>, socket: S) {
    for item in items {
        if item.is_ready() {
            match item.kind() {
                Kind::A => { socket.send_to(item); }
                _ => {}
            }
        }
    }
}
"#;

    // Act & Assert
    assert!(performs_io(body));
}

#[test]
fn new_starts_having_found_nothing() {
    // Arrange & Act -- a finder that began in the found state would mark every
    // function impure before inspecting a single expression.
    let finder = IoCallFinder::new();

    // Assert
    assert!(!finder.found);
}

#[test]
fn print_macros_are_not_io_here() {
    // Arrange -- printing is a hidden dependency, scored separately and more
    // leniently by HiddenDepFinder. Counting it here too would charge one
    // println twice under two different headings.

    // Act & Assert
    assert!(!performs_io(r#"fn f() { println!("x"); }"#));
    assert!(!performs_io(r#"fn f() { eprintln!("x"); }"#));
}

#[test]
fn write_macros_count_as_io() {
    // Arrange -- write! and writeln! reach a sink the signature never mentions.

    // Act & Assert
    assert!(performs_io(r#"fn f(w: W) { write!(w, "x"); }"#));
    assert!(performs_io(r#"fn f(w: W) { writeln!(w, "x"); }"#));
}
