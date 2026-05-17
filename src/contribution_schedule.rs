// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[must_use]
pub fn contribution(is_pure: bool, has_trait_seam: bool, hidden_deps: usize) -> f64 {
    if hidden_deps >= 2 {
        return 0.0;
    }
    match (is_pure, has_trait_seam, hidden_deps) {
        (true, true, 0) => 1.0,
        (true, false, 0) => 0.95,
        (false, true, 0) => 0.85,
        (true, true, 1) => 0.60,
        (true, false, 1) => 0.40,
        (false, true, 1) => 0.25,
        (false, false, 0) => 0.15,
        (false, false, 1) => 0.0,
        (_, _, _) => 0.0,
    }
}
