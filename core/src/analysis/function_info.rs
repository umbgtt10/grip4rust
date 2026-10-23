// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FunctionInfo {
    name: String,
    file: String,
    is_pure: bool,
    is_public: bool,
    hidden_deps: usize,
    has_trait_seam: bool,
    dep_weight: f64,
    hidden_dep_labels: Vec<String>,
    grip_absolute: f64,
    grip_normalized: u32,
}

impl FunctionInfo {
    #[must_use]
    pub fn dep_weight(&self) -> f64 {
        self.dep_weight
    }

    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    #[must_use]
    pub fn has_trait_seam(&self) -> bool {
        self.has_trait_seam
    }

    #[must_use]
    pub fn hidden_dep_labels(&self) -> &[String] {
        &self.hidden_dep_labels
    }

    #[must_use]
    pub fn hidden_deps(&self) -> usize {
        self.hidden_deps
    }

    #[must_use]
    pub fn is_pure(&self) -> bool {
        self.is_pure
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn with_dep_weight(self, dep_weight: f64) -> Self {
        Self { dep_weight, ..self }
    }

    #[must_use]
    pub fn with_file(self, file: String) -> Self {
        Self { file, ..self }
    }

    #[must_use]
    pub fn with_grip_absolute(self, grip_absolute: f64) -> Self {
        Self {
            grip_absolute,
            ..self
        }
    }

    #[must_use]
    pub fn with_grip_normalized(self, grip_normalized: u32) -> Self {
        Self {
            grip_normalized,
            ..self
        }
    }

    #[must_use]
    pub fn with_has_trait_seam(self, has_trait_seam: bool) -> Self {
        Self {
            has_trait_seam,
            ..self
        }
    }

    #[must_use]
    pub fn with_hidden_dep_labels(self, hidden_dep_labels: Vec<String>) -> Self {
        Self {
            hidden_dep_labels,
            ..self
        }
    }

    #[must_use]
    pub fn with_hidden_deps(self, hidden_deps: usize) -> Self {
        Self {
            hidden_deps,
            ..self
        }
    }

    #[must_use]
    pub fn with_is_public(self, is_public: bool) -> Self {
        Self { is_public, ..self }
    }

    #[must_use]
    pub fn with_is_pure(self, is_pure: bool) -> Self {
        Self { is_pure, ..self }
    }

    #[must_use]
    pub fn with_name(self, name: String) -> Self {
        Self { name, ..self }
    }
}
