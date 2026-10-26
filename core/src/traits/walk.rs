// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::analysis::source_file::SourceFile;
use crate::analysis::walk_failure::WalkFailure;

pub trait Walk {
    fn rust_files(&self) -> Result<Vec<SourceFile>, WalkFailure>;
}
