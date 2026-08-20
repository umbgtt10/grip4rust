// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::invocation::app::run;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(ec) => ec,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
