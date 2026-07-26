// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;

use grip::cache::Cache;
use grip::collector::Collector;
use grip::method_purity_registry::MethodPurityRegistry;
use grip::struct_registry::StructRegistry;
use grip::traits::cache_store::CacheStore;
use tempfile::TempDir;

#[test]
fn cache_hit_returns_same_counts() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    let file = src.join("lib.rs");
    fs::write(&file, "pub fn greet() -> i32 { 42 }").unwrap();

    let cache = Cache::new(dir.path());

    let source = fs::read_to_string(&file).unwrap();
    let (initial, _fns) = Collector::collect(
        &source,
        &file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );
    cache.set(&file, &source, &initial);

    // Act
    let cached = cache.get(&file);

    // Assert
    assert!(cached.is_some());
    assert_eq!(cached.unwrap().total_functions, 1);
}

#[test]
fn cache_miss_after_change() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    let file = src.join("lib.rs");
    fs::write(&file, "pub fn greet() -> i32 { 42 }").unwrap();

    let cache = Cache::new(dir.path());
    let source = fs::read_to_string(&file).unwrap();
    let (initial, _fns) = Collector::collect(
        &source,
        &file,
        &StructRegistry::default(),
        &MethodPurityRegistry::default(),
    );
    cache.set(&file, &source, &initial);

    fs::write(&file, "fn hidden() -> i32 { 0 }").unwrap();

    // Act
    let cached = cache.get(&file);

    // Assert
    assert!(cached.is_none());
}
