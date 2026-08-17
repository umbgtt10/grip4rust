// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;

use grip::item_counts::ItemCounts;
use grip::no_op_cache_store::NoOpCacheStore;
use grip::traits::cache_store::CacheStore;

fn counts(total_functions: usize) -> ItemCounts {
    ItemCounts {
        total_functions,
        ..ItemCounts::default()
    }
}

#[test]
fn get_on_a_fresh_store_returns_nothing() {
    // Arrange -- this store stands in where caching is switched off. Its whole
    // contract is to be a miss: a `get` that ever answered would serve counts
    // for a file the run never analysed.
    let store = NoOpCacheStore::new();

    // Act
    let result = store.get(Path::new("src/lib.rs"));

    // Assert
    assert!(result.is_none());
}

#[test]
fn get_after_set_still_returns_nothing() {
    // Arrange -- the failure mode worth pinning is a store that quietly starts
    // remembering. Callers treat a `Some` as authoritative and skip re-parsing,
    // so a single retained entry silently freezes that file's score.
    let store = NoOpCacheStore::new();

    // Act
    store.set(Path::new("src/lib.rs"), "pub fn f() {}", &counts(1));
    let result = store.get(Path::new("src/lib.rs"));

    // Assert
    assert!(result.is_none());
}

#[test]
fn get_returns_nothing_however_many_times_set_is_called() {
    // Arrange -- one call proving nothing was stored is weaker than it looks.
    // A store accumulating internally and answering only past some threshold
    // would pass a single-call test and fail on a real run.
    let store = NoOpCacheStore::new();

    // Act & Assert
    for i in 0..32 {
        store.set(Path::new("src/lib.rs"), "pub fn f() {}", &counts(i));
        assert!(store.get(Path::new("src/lib.rs")).is_none());
    }
}

#[test]
fn flush_is_inert_and_leaves_the_store_a_miss() {
    // Arrange -- flush is called once at the end of every run. It must neither
    // fail nor turn the store into something that answers.
    let store = NoOpCacheStore::new();
    store.set(Path::new("src/lib.rs"), "pub fn f() {}", &counts(1));

    // Act
    store.flush();

    // Assert
    assert!(store.get(Path::new("src/lib.rs")).is_none());
}

#[test]
fn distinct_paths_are_all_misses() {
    // Arrange -- a real run asks about every file it walks. None may hit.
    let store = NoOpCacheStore::new();

    // Act & Assert
    for path in ["src/lib.rs", "src/main.rs", "src/a/b/c.rs", ""] {
        store.set(Path::new(path), "pub fn f() {}", &counts(1));
        assert!(
            store.get(Path::new(path)).is_none(),
            "path {path} must not be cached"
        );
    }
}

#[test]
fn default_behaves_the_same_as_new() {
    // Arrange -- the type derives Default, so a caller can obtain one either
    // way. Both must be equally inert.
    let store = NoOpCacheStore::default();

    // Act
    store.set(Path::new("src/lib.rs"), "pub fn f() {}", &counts(1));

    // Assert
    assert!(store.get(Path::new("src/lib.rs")).is_none());
}
