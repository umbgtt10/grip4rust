// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

#[derive(Debug, Clone)]
pub struct Offender {
    pub path: String,
    pub grip_score: u32,
}
