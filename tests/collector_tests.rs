// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs::File;
use std::io::Write;

use grip::collector::Collector;
use tempfile::TempDir;

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
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.pure_functions, 1);
    assert_eq!(counts.public_functions, 1);
    assert_eq!(counts.total_items, 1);
    assert_eq!(counts.public_items, 1);
}

#[test]
fn impure_function_is_not_counted_as_pure() {
    // Arrange
    let source = "pub fn impure(x: &mut i32) { *x += 1; }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.pure_functions, 0);
}

#[test]
fn unit_return_is_not_pure() {
    // Arrange
    let source = "pub fn side_effect() { println!(\"hello\"); }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.pure_functions, 0);
}

#[test]
fn unsafe_function_is_not_pure() {
    // Arrange
    let source = "pub fn raw() -> i32 { unsafe { 42 } }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.pure_functions, 0);
}

#[test]
fn private_function_is_not_public() {
    // Arrange
    let source = "fn private() -> i32 { 42 }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.public_functions, 0);
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
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 3);
    assert_eq!(counts.public_functions, 2);
    assert_eq!(counts.pure_functions, 2);
    assert_eq!(counts.public_structs, 1);
    assert_eq!(counts.public_enums, 1);
    assert_eq!(counts.public_traits, 1);
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
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 0);
    assert_eq!(counts.public_items, 0);
}

#[test]
fn pubcrate_is_public_item() {
    // Arrange
    let source = "pub(crate) fn internal() -> i32 { 42 }\n";
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.total_functions, 1);
    assert_eq!(counts.pubcrate_functions, 1);
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
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

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
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

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
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.local_trait_methods, 1);
    assert_eq!(counts.inherent_methods, 0);
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
    let dir = tempfile::tempdir().unwrap();
    let _file = write_file(&dir, "lib.rs", source);

    // Act
    let (counts, _fns) = Collector::collect(source, &_file);

    // Assert
    assert_eq!(counts.local_trait_methods, 0);
    assert_eq!(counts.inherent_methods, 0);
}
