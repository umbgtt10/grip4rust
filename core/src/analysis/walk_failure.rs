// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use core::error::Error;
use core::fmt;

// A source file the walk found but could not read, and the platform's reason.
#[derive(Debug)]
pub struct WalkFailure {
    pub path: String,
    pub cause: Box<dyn Error + Send + Sync>,
}

impl fmt::Display for WalkFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "failed to read {}", self.path)
    }
}

impl Error for WalkFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.cause.as_ref())
    }
}
