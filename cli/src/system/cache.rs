// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::item_counts::ItemCounts;
use grip::traits::cache_store::CacheStore;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_json::to_string;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug)]
pub struct Cache {
    cache_dir: PathBuf,
    store: RefCell<HashMap<PathBuf, CachedEntry>>,
    dirty: Cell<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedEntry {
    pub mtime_secs: u128,
    pub len: u64,
    pub counts: CachedCounts,
}

// The core's counts carry no serde, so the file's shape is written down here,
// field for field in the order the cache has always had.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedCounts {
    pub total_functions: usize,
    pub pure_functions: usize,
    pub total_items: usize,
    pub public_items: usize,
    pub inherent_methods: usize,
    pub inherent_impure: usize,
    pub local_trait_methods: usize,
    pub local_trait_impure: usize,
    pub total_contribution: f64,
    pub clean_functions: usize,
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

    fn cached_of(counts: &ItemCounts) -> CachedCounts {
        CachedCounts {
            total_functions: counts.total_functions(),
            pure_functions: counts.pure_functions(),
            total_items: counts.total_items(),
            public_items: counts.public_items(),
            inherent_methods: counts.inherent_methods(),
            inherent_impure: counts.inherent_impure(),
            local_trait_methods: counts.local_trait_methods(),
            local_trait_impure: counts.local_trait_impure(),
            total_contribution: counts.total_contribution(),
            clean_functions: counts.clean_functions(),
        }
    }

    fn counts_of(cached: &CachedCounts) -> ItemCounts {
        ItemCounts::default()
            .with_total_functions(cached.total_functions)
            .with_pure_functions(cached.pure_functions)
            .with_total_items(cached.total_items)
            .with_public_items(cached.public_items)
            .with_inherent_methods(cached.inherent_methods)
            .with_inherent_impure(cached.inherent_impure)
            .with_local_trait_methods(cached.local_trait_methods)
            .with_local_trait_impure(cached.local_trait_impure)
            .with_total_contribution(cached.total_contribution)
            .with_clean_functions(cached.clean_functions)
    }

    fn load(cache_dir: &Path) -> Option<HashMap<PathBuf, CachedEntry>> {
        let path = cache_dir.join("cache.json");
        let json = fs::read_to_string(path).ok()?;
        from_str(&json).ok()
    }

    fn mtime_secs(path: &Path) -> Option<u128> {
        let metadata = fs::metadata(path).ok()?;
        let mtime = metadata.modified().ok()?;
        Some(mtime.duration_since(SystemTime::UNIX_EPOCH).ok()?.as_secs() as u128)
    }
}

impl CacheStore for Cache {
    fn get(&self, path: &str) -> Option<ItemCounts> {
        let path = Path::new(path);
        let mtime_secs = Self::mtime_secs(path)?;
        let len = fs::metadata(path).ok()?.len();
        let store = self.store.borrow();
        let entry = store.get(path)?;
        if entry.mtime_secs == mtime_secs && entry.len == len {
            Some(Self::counts_of(&entry.counts))
        } else {
            None
        }
    }

    fn set(&self, path: &str, source: &str, counts: &ItemCounts) {
        let path = Path::new(path);
        let Some(mtime_secs) = Self::mtime_secs(path) else {
            return;
        };
        self.store.borrow_mut().insert(
            path.to_path_buf(),
            CachedEntry {
                mtime_secs,
                len: source.len() as u64,
                counts: Self::cached_of(counts),
            },
        );
        self.dirty.set(true);
    }

    fn flush(&self) {
        if !self.dirty.get() {
            return;
        }
        let _ = fs::create_dir_all(&self.cache_dir);
        if let Ok(json) = to_string(&*self.store.borrow()) {
            let _ = fs::write(self.cache_dir.join("cache.json"), json);
        }
    }
}
