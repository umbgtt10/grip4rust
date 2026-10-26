// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

// One Rust source file a walk found: its path as the walk wrote it, the same path
// beneath the analysed root, and what the file holds.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SourceFile {
    pub path: String,
    pub relative: String,
    pub source: String,
}
