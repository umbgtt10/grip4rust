// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::{Item, ItemMod};

pub struct InlineModule<'m> {
    item_mod: &'m ItemMod,
}

impl<'m> InlineModule<'m> {
    #[must_use]
    pub fn new(item_mod: &'m ItemMod) -> Self {
        Self { item_mod }
    }

    #[must_use]
    pub fn items(&self) -> &'m [Item] {
        self.item_mod
            .content
            .as_ref()
            .map_or(&[], |(_, items)| items.as_slice())
    }
}
