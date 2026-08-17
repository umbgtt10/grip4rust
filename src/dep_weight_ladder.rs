// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::Path;

pub struct DepWeightLadder;

impl DepWeightLadder {
    // Labels come from `path_label`, which joins macro path segments, so a
    // print macro arrives as the bare name `print` with no `!`. Matching on
    // `print!` never fires and drops the call into the unknown-dependency
    // catch-all instead. `print` also prefixes `println`, and `eprint`
    // prefixes `eprintln`, so two arms cover all four macros.
    #[must_use]
    pub fn weight_of(label: &str) -> f64 {
        if Self::is_print(label) {
            0.2
        } else if Self::is_clock(label) {
            0.3
        } else if Self::is_environment(label) {
            0.4
        } else if label.starts_with("unsafe") {
            0.5
        } else {
            0.6
        }
    }

    #[must_use]
    pub fn label_of(path: &Path) -> String {
        path.segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join("::")
    }

    fn is_print(label: &str) -> bool {
        label.starts_with("print") || label.starts_with("eprint")
    }

    fn is_clock(label: &str) -> bool {
        label.starts_with("Instant")
            || label.starts_with("SystemTime")
            || label.starts_with("Utc")
            || label.starts_with("Local")
            || label.contains("elapsed")
    }

    fn is_environment(label: &str) -> bool {
        label.starts_with("env::") || label.starts_with("process::")
    }
}
