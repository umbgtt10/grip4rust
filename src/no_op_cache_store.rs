// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::item_counts::ItemCounts;
use crate::traits::cache_store::CacheStore;
use std::path::Path;

#[derive(Debug, Clone, Copy, Default)]
pub struct NoOpCacheStore;

impl NoOpCacheStore {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl CacheStore for NoOpCacheStore {
    fn get(&self, _path: &Path) -> Option<ItemCounts> {
        None
    }

    fn set(&self, _path: &Path, _source: &str, _counts: &ItemCounts) {}

    fn flush(&self) {}
}
