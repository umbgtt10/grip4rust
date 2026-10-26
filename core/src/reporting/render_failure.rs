// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use core::error::Error;
use core::fmt;

// A report its renderer could not write, and the renderer's reason.
#[derive(Debug)]
pub struct RenderFailure {
    pub cause: Box<dyn Error + Send + Sync>,
}

impl fmt::Display for RenderFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("failed to render the report")
    }
}

impl Error for RenderFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.cause.as_ref())
    }
}
