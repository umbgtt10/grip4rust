// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::invocation::app::App;
use grip::invocation::args::Args;
use std::env::args_os;
use std::process::ExitCode;

fn main() -> ExitCode {
    match App::run_from_args(Args::without_cargo_subcommand(args_os().collect())) {
        Ok(ec) => ec,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
