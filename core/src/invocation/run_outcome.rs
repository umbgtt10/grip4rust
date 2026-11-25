// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

// What a run ends with: the text to print -- the rendered report with its line
// ending -- and whether the score fell below the threshold the run was given.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunOutcome {
    pub output: String,
    pub below_threshold: bool,
}
