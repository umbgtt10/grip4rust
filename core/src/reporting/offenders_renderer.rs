// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::reporting::offender::Offender;

#[derive(Debug, Clone, Copy, Default)]
pub struct OffendersRenderer;

impl OffendersRenderer {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn render(&self, offenders: &[Offender], threshold: u32) -> Vec<String> {
        if offenders.is_empty() {
            return Vec::new();
        }
        let mut lines = vec![format!("\nOffenders (score < {threshold}):")];
        for offender in offenders {
            lines.push(format!(
                "  {:<30}  grip: {:>3}  ❌",
                offender.path, offender.grip_score,
            ));
        }
        lines
    }
}
