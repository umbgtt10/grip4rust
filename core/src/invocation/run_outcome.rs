// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// What a run ends with: the text to print -- the rendered report with its line
// ending, or nothing when the run was only asked whether the score reaches a
// threshold -- and whether it fell below that threshold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunOutcome {
    pub output: String,
    pub below_threshold: bool,
}
