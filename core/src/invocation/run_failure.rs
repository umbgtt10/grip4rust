// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use core::error;
use core::fmt;

use crate::analysis::walk_failure::WalkFailure;
use crate::reporting::render_failure::RenderFailure;

// Why a run produced no outcome, one variant per way it can fail. A failure the
// run only passes on says itself and keeps its own cause.
#[derive(Debug)]
pub enum RunFailure {
    NoSourceFiles(String),
    Render(RenderFailure),
    Walk(WalkFailure),
}

impl From<RenderFailure> for RunFailure {
    fn from(failure: RenderFailure) -> Self {
        Self::Render(failure)
    }
}

impl From<WalkFailure> for RunFailure {
    fn from(failure: WalkFailure) -> Self {
        Self::Walk(failure)
    }
}

impl fmt::Display for RunFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSourceFiles(path) => write!(formatter, "no Rust source files found in {path}"),
            Self::Render(failure) => write!(formatter, "{failure}"),
            Self::Walk(failure) => write!(formatter, "{failure}"),
        }
    }
}

impl error::Error for RunFailure {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::NoSourceFiles(_) => None,
            Self::Render(failure) => failure.source(),
            Self::Walk(failure) => failure.source(),
        }
    }
}
