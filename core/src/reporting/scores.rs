// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scores {
    pub grip_score: Option<u32>,
    pub pure_ratio: f64,
    pub public_ratio: f64,
    pub trait_ratio: f64,
    pub avg_contribution: f64,
    pub clean_fn_ratio: f64,
}
