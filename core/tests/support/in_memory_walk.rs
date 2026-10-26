// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::not_read::NotRead;
use grip::analysis::source_file::SourceFile;
use grip::analysis::walk_failure::WalkFailure;
use grip::traits::walk::Walk;

// A walk over files held in memory, in the order they were given, or one that
// fails naming the file it could not read.
pub struct InMemoryWalk {
    files: Vec<SourceFile>,
    unreadable: Option<String>,
}

impl InMemoryWalk {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            unreadable: None,
        }
    }

    pub fn with_file(self, relative: &str, source: &str) -> Self {
        let mut files = self.files;
        files.push(SourceFile {
            path: format!("root/{relative}"),
            relative: String::from(relative),
            source: String::from(source),
        });
        Self { files, ..self }
    }

    pub fn with_unreadable_file(self, path: &str) -> Self {
        Self {
            unreadable: Some(String::from(path)),
            ..self
        }
    }
}

impl Default for InMemoryWalk {
    fn default() -> Self {
        Self::new()
    }
}

impl Walk for InMemoryWalk {
    fn rust_files(&self) -> Result<Vec<SourceFile>, WalkFailure> {
        if let Some(path) = &self.unreadable {
            return Err(WalkFailure {
                path: path.clone(),
                cause: Box::new(NotRead),
            });
        }
        Ok(self.files.clone())
    }
}
