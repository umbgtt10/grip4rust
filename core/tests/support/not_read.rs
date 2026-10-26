// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::error::Error;
use std::fmt;

// The cause a fake gives for a failure it was told to report.
#[derive(Debug)]
pub struct NotRead;

impl fmt::Display for NotRead {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("not read")
    }
}

impl Error for NotRead {}
