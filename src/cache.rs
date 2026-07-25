// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::item_counts::ItemCounts;
use crate::traits::cache_store::CacheStore;

#[derive(Debug)]
pub struct Cache {
    cache_dir: PathBuf,
    store: RefCell<HashMap<PathBuf, CachedEntry>>,
    dirty: Cell<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedEntry {
    mtime_secs: u128,
    len: u64,
    counts: ItemCounts,
}

impl Cache {
    #[must_use]
    pub fn new(root: &Path) -> Self {
        let cache_dir = root.join(".grip_cache");
        let store = if cache_dir.exists() {
            Self::load(&cache_dir).unwrap_or_default()
        } else {
            HashMap::new()
        };
        Self {
            cache_dir,
            store: RefCell::new(store),
            dirty: Cell::new(false),
        }
    }

    fn mtime_secs(path: &Path) -> Option<u128> {
        let metadata = fs::metadata(path).ok()?;
        let mtime = metadata.modified().ok()?;
        Some(mtime.duration_since(SystemTime::UNIX_EPOCH).ok()?.as_secs() as u128)
    }

    fn load(cache_dir: &Path) -> Result<HashMap<PathBuf, CachedEntry>> {
        let path = cache_dir.join("cache.json");
        let json = fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&json)?)
    }
}

impl CacheStore for Cache {
    fn get(&self, path: &Path) -> Option<ItemCounts> {
        let mtime_secs = Self::mtime_secs(path)?;
        let len = fs::metadata(path).ok()?.len();
        let store = self.store.borrow();
        let entry = store.get(path)?;
        if entry.mtime_secs == mtime_secs && entry.len == len {
            Some(entry.counts.clone())
        } else {
            None
        }
    }

    fn set(&self, path: &Path, source: &str, counts: &ItemCounts) {
        let Some(mtime_secs) = Self::mtime_secs(path) else {
            return;
        };
        self.store.borrow_mut().insert(
            path.to_path_buf(),
            CachedEntry {
                mtime_secs,
                len: source.len() as u64,
                counts: counts.clone(),
            },
        );
        self.dirty.set(true);
    }

    fn flush(&self) {
        if !self.dirty.get() {
            return;
        }
        let _ = fs::create_dir_all(&self.cache_dir);
        if let Ok(json) = serde_json::to_string(&*self.store.borrow()) {
            let _ = fs::write(self.cache_dir.join("cache.json"), json);
        }
    }
}
