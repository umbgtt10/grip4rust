// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

// What a run is asked to do: the analysed root as its caller names it, the name
// the report gives it, and the score the run must reach, if any. How the root is
// walked and the report rendered are the parts composed around it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Config {
    pub path: String,
    pub target: String,
    pub threshold: Option<u32>,
}
