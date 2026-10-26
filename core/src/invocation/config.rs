// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// What a run is asked to do: the analysed root as its caller names it, the name
// the report gives it, the score the run must reach, if any, and how the report
// is written.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Config {
    pub path: String,
    pub target: String,
    pub json: bool,
    pub threshold: Option<u32>,
    pub verbose: bool,
}
