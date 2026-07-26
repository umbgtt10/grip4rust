// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use grip::collector::Collector;
use grip::contribution_schedule::ContributionSchedule;
use grip::method_purity_registry::MethodPurityRegistry;
use grip::struct_registry::StructRegistry;
use tempfile::{TempDir, tempdir};

fn write_file(dir: &TempDir, name: &str, contents: &str) -> std::path::PathBuf {
    let path = dir.path().join(name);
    let mut fh = File::create(&path).unwrap();
    fh.write_all(contents.as_bytes()).unwrap();
    path
}

#[test]
fn pure_function_is_counted() {
    // Arrange
    let source = "pub fn add(a: i32, b: i32) -> i32 { a + b }\n";
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.pure_functions, 1);
    assert_eq!(counts.total_items, 1);
    assert_eq!(counts.public_items, 1);
}

#[test]
fn impure_function_is_not_counted_as_pure() {
    // Arrange
    let source = "pub fn impure(x: &mut i32) { *x += 1; }\n";
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.pure_functions, 0);
}

#[test]
fn unit_return_is_not_pure() {
    // Arrange
    let source = "pub fn side_effect() { println!(\"hello\"); }\n";
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.pure_functions, 0);
}

#[test]
fn unsafe_function_is_not_pure() {
    // Arrange
    let source = "pub fn raw() -> i32 { unsafe { 42 } }\n";
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.pure_functions, 0);
}

#[test]
fn by_value_mut_self_builder_method_is_pure() {
    // Arrange
    let source = "pub struct Builder { x: i32 }\nimpl Builder {\n    pub fn with_x(mut self, x: i32) -> Self { self.x = x; self }\n}\n";
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(counts.pure_functions, 1);
    assert!(
        fns[0].is_pure,
        "mut self (by value) has no observable side effect and must not be treated the same as &mut self"
    );
}

#[test]
fn mut_self_reference_method_is_not_pure() {
    // Arrange
    let source = "pub struct Builder { x: i32 }\nimpl Builder {\n    pub fn set_x(&mut self, x: i32) { self.x = x; }\n}\n";
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(counts.pure_functions, 0);
    assert!(!fns[0].is_pure);
}

#[test]
fn private_function_is_not_public() {
    // Arrange
    let source = "fn private() -> i32 { 42 }\n";
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.public_items, 0);
}

#[test]
fn mixed_items_are_counted() {
    // Arrange
    let source = r#"
pub fn a() -> i32 { 1 }
pub fn b(x: &mut i32) { }
fn c() -> i32 { 2 }
pub struct S;
pub enum E {}
pub trait T {}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(counts.total_functions, 3);
    assert_eq!(counts.pure_functions, 2);
    assert_eq!(counts.total_items, 6);
    assert_eq!(counts.public_items, 5);
}

#[test]
fn test_attribute_is_skipped() {
    // Arrange
    let source = r#"
#[cfg(test)]
mod tests {
    pub fn test_helper() -> i32 { 42 }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(counts.total_functions, 0);
    assert_eq!(counts.public_items, 0);
}

#[test]
fn pubcrate_is_public_item() {
    // Arrange
    let source = "pub(crate) fn internal() -> i32 { 42 }\n";
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.public_items, 1);
}

#[test]
fn inherent_io_connect_is_impure() {
    // Arrange
    let source = r#"
use std::net::TcpStream;

pub struct Client;

impl Client {
    pub fn connect(&self) -> std::io::Result<()> {
        TcpStream::connect("127.0.0.1:8080")?;
        Ok(())
    }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(counts.inherent_methods, 1);
    assert_eq!(counts.inherent_impure, 1);
}

#[test]
fn inherent_io_writeln_is_impure() {
    // Arrange
    let source = r#"
use std::fs::File;

pub struct Logger;

impl Logger {
    pub fn log(&self) -> std::io::Result<()> {
        let mut f = File::create("/tmp/log.txt")?;
        writeln!(f, "hello")?;
        Ok(())
    }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(counts.inherent_methods, 1);
    assert_eq!(counts.inherent_impure, 1);
}

#[test]
fn local_trait_impl_is_not_foreign() {
    // Arrange
    let source = r#"
mod inner {
    pub trait MyTrait {
        fn do_thing(&self) -> i32;
    }
}

struct MyStruct;

impl inner::MyTrait for MyStruct {
    fn do_thing(&self) -> i32 { 42 }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(counts.local_trait_methods, 1);
    assert_eq!(counts.inherent_methods, 0);
}

#[test]
fn test_attr_is_skipped_in_local_trait_impl() {
    // Arrange
    let source = r#"
trait Helper {
    fn do_thing(&self) -> i32;
}

struct Impl;

impl Helper for Impl {
    fn do_thing(&self) -> i32 { 42 }

    #[test]
    fn test_helper(&self) -> i32 { 99 }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        counts.local_trait_methods, 1,
        "test_helper should be skipped"
    );
    assert_eq!(counts.total_functions, 1, "only do_thing should be counted");
}

#[test]
fn known_foreign_trait_is_excluded() {
    // Arrange
    let source = r#"
struct MyStruct;

impl std::fmt::Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyStruct")
    }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(counts.local_trait_methods, 0);
    assert_eq!(counts.inherent_methods, 0);
}

#[test]
fn hidden_dep_uppercase_type_constructor() {
    // Arrange
    let source = r#"
struct Handler;
impl Handler {
    pub fn handle() { TcpStream::connect("127.0.0.1:8080").unwrap(); }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 1,
        "TcpStream::connect should be a hidden dep"
    );
}

#[test]
fn hidden_dep_std_constructor_not_counted() {
    // Arrange
    let source = r#"
struct Builder;
impl Builder {
    pub fn build() -> String { String::new() }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 0,
        "String::new should not be a hidden dep"
    );
}

#[test]
fn hidden_dep_vec_new_not_counted() {
    // Arrange
    let source = r#"
struct Collector;
impl Collector {
    pub fn collect() -> Vec<i32> { Vec::new() }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(fns[0].hidden_deps, 0, "Vec::new should not be a hidden dep");
}

#[test]
fn hidden_dep_box_new_not_counted() {
    // Arrange
    let source = r#"
struct Wrapper;
impl Wrapper {
    pub fn wrap(x: i32) -> Box<i32> { Box::new(x) }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(fns[0].hidden_deps, 0, "Box::new should not be a hidden dep");
}

#[test]
fn hidden_dep_constructor_new_not_flagged() {
    // Arrange
    let source = r#"
struct Service;
impl Service {
    pub fn process() { MyDatabase::new("prod:5432"); }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 1,
        "MyDatabase::new constructs a concrete dependency"
    );
}

#[test]
fn hidden_dep_behavioral_method_is_flagged() {
    // Arrange
    let source = r#"
struct Service;
impl Service {
    pub fn process() { MyDatabase::query("SELECT 1"); }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 1,
        "MyDatabase::query should be a hidden dep"
    );
}

#[test]
fn hidden_dep_third_party_type_is_detected() {
    // Arrange
    let source = r#"
struct Service;
impl Service {
    pub fn charge() { StripeGateway::charge(100); }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 1,
        "StripeGateway::charge should be a hidden dep"
    );
}

#[test]
fn hidden_dep_self_call_not_counted() {
    // Arrange
    let source = r#"
struct Factory;
impl Factory {
    pub fn create() -> Self { Self::new() }
    pub fn new() -> Self { Self }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 0,
        "Self::new should not be a hidden dep"
    );
}

#[test]
fn hidden_dep_macro_println_is_detected() {
    // Arrange
    let source = r#"
struct Logger;
impl Logger {
    pub fn log() { println!("hello"); }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(fns[0].hidden_deps, 1, "println! should be a hidden dep");
}

#[test]
fn hidden_dep_multiple_calls_accumulate() {
    // Arrange
    let source = r#"
struct Service;
impl Service {
    pub fn run() {
        TcpStream::connect("127.0.0.1:8080").unwrap();
        MyDatabase::query("SELECT 1");
        File::create("/tmp/test.txt").unwrap();
    }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 3,
        "should detect all 3 concrete type calls"
    );
}

#[test]
fn hidden_dep_zero_deps_on_clean_function() {
    // Arrange
    let source = r#"
struct Calc;
impl Calc {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 0,
        "pure function should have 0 hidden deps"
    );
}

#[test]
fn hidden_dep_self_field_trait_object_not_counted() {
    // Arrange
    let source = r#"
struct Service {
    db: Box<dyn Database>,
}
impl Service {
    pub fn query(&self, sql: &str) { self.db.query(sql); }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 0,
        "self.db on trait object should not be a hidden dep"
    );
}

#[test]
fn hidden_dep_self_field_concrete_type_is_flagged() {
    // Arrange
    let source = r#"
struct DataStore {
    conn: String,
}
impl DataStore {
    pub fn query(&self, sql: &str) -> String { sql.to_string() }
}

struct Service {
    db: DataStore,
}
impl Service {
    pub fn run(&self) { self.db.query("SELECT 1"); }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[1].hidden_deps, 1,
        "self.db on concrete DataStore should be flagged"
    );
    assert_eq!(
        fns[0].hidden_deps, 0,
        "DataStore::query is a free impl method"
    );
}

#[test]
fn hidden_dep_self_field_vec_get_not_counted() {
    // Arrange
    let source = r#"
struct ConfirmedSet {
    flags: Vec<bool>,
}
impl ConfirmedSet {
    pub fn is_confirmed(&self, i: usize) -> bool { self.flags.get(i).copied().unwrap_or(false) }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 0,
        "Vec::get is a pure, bounds-checked read - not a hidden dependency"
    );
}

#[test]
fn hidden_dep_self_field_vec_clone_not_counted() {
    // Arrange
    let source = r#"
struct Tracker {
    peers: Vec<i32>,
}
impl Tracker {
    pub fn snapshot(&self) -> Vec<i32> { self.peers.clone() }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 0,
        "Vec::clone is deterministic and side-effect-free - not a hidden dependency"
    );
}

#[test]
fn hidden_dep_self_field_custom_struct_clone_is_still_flagged() {
    // Arrange: Members is a project's own type, not a known std value type -
    // nothing structural distinguishes cloning it from cloning a live
    // collaborator, so it must still be flagged.
    let source = r#"
struct Members {
    ids: Vec<i32>,
}

struct Bootstrapped {
    members: Members,
}
impl Bootstrapped {
    pub fn snapshot(&self) -> Members { self.members.clone() }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    let snapshot_fn = fns.iter().find(|f| f.name == "snapshot").unwrap();
    assert_eq!(
        snapshot_fn.hidden_deps, 1,
        "Members is not a known value type - still flagged, as designed"
    );
}

#[test]
fn hidden_dep_self_field_hashmap_len_not_counted() {
    // Arrange: confirms the allowlist isn't Vec-specific.
    let source = r#"
use std::collections::HashMap;

struct Registry {
    entries: HashMap<i32, i32>,
}
impl Registry {
    pub fn count(&self) -> usize { self.entries.len() }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 0,
        "HashMap::len is pure - not a hidden dependency"
    );
}

#[test]
fn hidden_dep_self_field_vec_push_is_still_flagged() {
    // Arrange: push isn't in the pure-methods allowlist - the fix must not
    // blanket-exempt every method on a known value type, only the listed ones.
    let source = r#"
struct Log {
    entries: Vec<i32>,
}
impl Log {
    pub fn record(&mut self, entry: i32) { self.entries.push(entry); }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 1,
        "Vec::push mutates shared state - still a hidden dependency"
    );
}

#[test]
fn hidden_dep_self_field_custom_struct_get_is_still_flagged() {
    // Arrange: `get` is the riskiest name to blanket-exempt - a project's own
    // type can name a real I/O method `get` (a disk-backed cache, a repository).
    // Nothing structural distinguishes it from Vec::get without type resolution,
    // so it must stay flagged.
    let source = r#"
struct DiskCache {
    path: String,
}
impl DiskCache {
    pub fn get(&self, key: &str) -> String { key.to_string() }
}

struct Service {
    cache: DiskCache,
}
impl Service {
    pub fn lookup(&self, key: &str) -> String { self.cache.get(key) }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    let lookup_fn = fns.iter().find(|f| f.name == "lookup").unwrap();
    assert_eq!(
        lookup_fn.hidden_deps, 1,
        "DiskCache is not a known value type - .get() still flagged"
    );
}

#[test]
fn hidden_dep_self_field_custom_wrapper_clone_not_counted_when_registry_resolves_it() {
    // Arrange: same shape as the "still flagged" case above, but this time
    // the registry has actually seen Members's definition (as it would after
    // a real multi-file project scan) - the clone must clear.
    let members_source = "struct Members { ids: Vec<i32> }";
    let registry =
        StructRegistry::build(&[(PathBuf::from("members.rs"), members_source.to_string())]);
    let method_purity = MethodPurityRegistry::default();

    let source = r#"
struct Members {
    ids: Vec<i32>,
}

struct Bootstrapped {
    members: Members,
}
impl Bootstrapped {
    pub fn snapshot(&self) -> Members { self.members.clone() }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(source, &_file, &registry, &method_purity);

    // Assert
    let snapshot_fn = fns.iter().find(|f| f.name == "snapshot").unwrap();
    assert_eq!(
        snapshot_fn.hidden_deps, 0,
        "the registry proves Members is transitively a value type - clone must clear"
    );
}

#[test]
fn hidden_dep_self_field_custom_wrapper_get_still_flagged_when_method_body_is_impure() {
    // Arrange: DiskCache's own field is transitively value-typed (just a
    // String), so a naive field-type check alone would wrongly clear .get()
    // too. get()'s body genuinely performs I/O here - the method-purity
    // registry must catch that and refuse to trust it, regardless of name.
    let source = r#"
struct DiskCache {
    path: String,
}
impl DiskCache {
    pub fn get(&self, key: &str) -> String {
        let contents = std::fs::read_to_string(&self.path).unwrap_or_default();
        format!("{key}:{contents}")
    }
}

struct Service {
    cache: DiskCache,
}
impl Service {
    pub fn lookup(&self, key: &str) -> String { self.cache.get(key) }
}
"#;
    let files = vec![(PathBuf::from("lib.rs"), source.to_string())];
    let struct_registry = StructRegistry::build(&files);
    let method_purity = MethodPurityRegistry::build(&files, &struct_registry);
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(source, &_file, &struct_registry, &method_purity);

    // Assert
    let lookup_fn = fns.iter().find(|f| f.name == "lookup").unwrap();
    assert_eq!(
        lookup_fn.hidden_deps, 1,
        "get() genuinely performs I/O - it must stay flagged regardless of its name"
    );
}

#[test]
fn hidden_dep_self_field_custom_wrapper_len_not_counted_when_body_is_provably_pure() {
    // Arrange: Members::len delegates purely to a std Vec field - the
    // method-purity registry should prove this and clear the call site,
    // even though `len` is not the clone-only structural exemption.
    let source = r#"
struct Members {
    ids: Vec<i32>,
}
impl Members {
    pub fn len(&self) -> usize { self.ids.len() }
}

struct Bootstrapped {
    members: Members,
}
impl Bootstrapped {
    pub fn member_count(&self) -> usize { self.members.len() }
}
"#;
    let files = vec![(PathBuf::from("lib.rs"), source.to_string())];
    let struct_registry = StructRegistry::build(&files);
    let method_purity = MethodPurityRegistry::build(&files, &struct_registry);
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(source, &_file, &struct_registry, &method_purity);

    // Assert
    let member_count_fn = fns.iter().find(|f| f.name == "member_count").unwrap();
    assert_eq!(
        member_count_fn.hidden_deps, 0,
        "Members::len is a provably pure delegation - the call site must clear"
    );
}

#[test]
fn hidden_dep_self_field_custom_wrapper_len_still_flagged_when_body_is_impure() {
    // Arrange: same shape as the positive case above, but Members::len
    // itself performs real I/O - the call site must stay flagged, proving
    // this is genuine body verification and not just name-based trust.
    let source = r#"
struct Members {
    ids: Vec<i32>,
}
impl Members {
    pub fn len(&self) -> usize {
        std::fs::read_to_string("audit.log").unwrap_or_default();
        self.ids.len()
    }
}

struct Bootstrapped {
    members: Members,
}
impl Bootstrapped {
    pub fn member_count(&self) -> usize { self.members.len() }
}
"#;
    let files = vec![(PathBuf::from("lib.rs"), source.to_string())];
    let struct_registry = StructRegistry::build(&files);
    let method_purity = MethodPurityRegistry::build(&files, &struct_registry);
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(source, &_file, &struct_registry, &method_purity);

    // Assert
    let member_count_fn = fns.iter().find(|f| f.name == "member_count").unwrap();
    assert_eq!(
        member_count_fn.hidden_deps, 1,
        "Members::len performs real I/O - the call site must stay flagged"
    );
}

#[test]
fn hidden_dep_self_field_ref_dyn_not_counted() {
    // Arrange
    let source = r#"
struct Service<'a> {
    handler: &'a dyn Handler,
}
impl Service<'_> {
    pub fn handle(&self) { self.handler.process(); }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 0,
        "self.handler on &dyn Handler should not be a hidden dep"
    );
}

#[test]
fn hidden_dep_free_function_is_detected() {
    // Arrange
    let source = "fn query_db() { Database::query(\"SELECT 1\"); }\n";
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 1,
        "free function with concrete call should be flagged"
    );
}

#[test]
fn hidden_dep_free_function_clean_not_flagged() {
    // Arrange
    let source = "fn add(a: i32, b: i32) -> i32 { a + b }\n";
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 0,
        "pure free function should have 0 hidden deps"
    );
}

#[test]
fn hidden_dep_eprintln_is_detected() {
    // Arrange
    let source = r#"
struct Logger;
impl Logger {
    pub fn log() { eprintln!("error"); }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(fns[0].hidden_deps, 1, "eprintln! should be a hidden dep");
}

#[test]
fn hidden_dep_arc_dyn_field_not_counted() {
    // Arrange
    let source = r#"
struct Service {
    db: Arc<dyn Database>,
}
impl Service {
    pub fn query(&self, sql: &str) { self.db.query(sql); }
}
"#;
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 0,
        "self.db on Arc<dyn Database> should not be a hidden dep"
    );
}

#[test]
fn hidden_dep_input_argument_not_counted() {
    // Arrange
    let source = "struct Service;\nimpl Service {\n    pub fn run(&self, db: &Database) { db.query(\"SELECT 1\"); }\n}\n";
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 0,
        "db.query on input argument should not be a hidden dep"
    );
}

#[test]
fn hidden_dep_print_macro_is_detected() {
    // Arrange
    let source = "struct Logger;\nimpl Logger {\n    pub fn log() { print!(\"hello\"); }\n}\n";
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(fns[0].hidden_deps, 1, "print! should be a hidden dep");
}

#[test]
fn hidden_dep_thread_sleep_is_detected() {
    // Arrange
    let source = "fn pause() { std::thread::sleep(std::time::Duration::from_secs(1)); }\n";
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert_eq!(
        fns[0].hidden_deps, 1,
        "thread::sleep should be a hidden dep"
    );
}

#[test]
fn hidden_dep_light_weight_vs_heavy() {
    // Arrange
    let source = "fn light() { println!(\"start\"); Instant::now(); }\n\
fn heavy() { Database::new(\"prod\"); StripeGateway::charge(100.0); }\n";
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    let schedule = ContributionSchedule::new();
    let light_contr =
        schedule.contribution(fns[0].is_pure, fns[0].has_trait_seam, fns[0].dep_weight);
    let heavy_contr =
        schedule.contribution(fns[1].is_pure, fns[1].has_trait_seam, fns[1].dep_weight);
    assert!(
        light_contr > 0.0,
        "light deps should have positive contribution, got {light_contr}"
    );
    assert_eq!(
        heavy_contr, 0.0,
        "heavy deps should have zero contribution, got {heavy_contr}"
    );
}

#[test]
fn hidden_dep_labels_are_recorded() {
    // Arrange
    let source = "fn run() { Database::new(\"prod\"); println!(\"done\"); }\n";
    let dir = tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (_counts, fns) = Collector::collect(
        source,
        &_file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );

    // Assert
    assert!(
        fns[0]
            .hidden_dep_labels
            .contains(&"Database::new".to_string()),
        "should contain Database::new label"
    );
    assert!(
        fns[0].hidden_dep_labels.contains(&"println".to_string()),
        "should contain println label"
    );
}
