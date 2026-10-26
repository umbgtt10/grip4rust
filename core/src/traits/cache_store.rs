// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::analysis::item_counts::ItemCounts;

pub trait CacheStore {
    fn get(&self, path: &str) -> Option<ItemCounts>;
    fn set(&self, path: &str, source: &str, counts: &ItemCounts);
    fn flush(&self);
}
