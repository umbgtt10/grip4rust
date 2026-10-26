// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::item_counts::ItemCounts;
use grip::traits::cache_store::CacheStore;
use std::cell::RefCell;
use std::rc::Rc;

// A cache that answers for the paths it was told it holds and records every call
// made to it, in order, as `get <path>`, `set <path>` or `flush`.
pub struct RecordingCacheStore {
    held: Vec<String>,
    calls: Rc<RefCell<Vec<String>>>,
}

impl RecordingCacheStore {
    pub fn new(held: Vec<String>, calls: Rc<RefCell<Vec<String>>>) -> Self {
        Self { held, calls }
    }
}

impl CacheStore for RecordingCacheStore {
    fn get(&self, path: &str) -> Option<ItemCounts> {
        self.calls.borrow_mut().push(format!("get {path}"));
        self.held
            .iter()
            .any(|held| held == path)
            .then(ItemCounts::default)
    }

    fn set(&self, path: &str, _source: &str, _counts: &ItemCounts) {
        self.calls.borrow_mut().push(format!("set {path}"));
    }

    fn flush(&self) {
        self.calls.borrow_mut().push(String::from("flush"));
    }
}
