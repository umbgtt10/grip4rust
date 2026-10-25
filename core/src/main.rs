// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::invocation::entry_point::EntryPoint;
use std::env::args_os;
use std::process::ExitCode;

fn main() -> ExitCode {
    EntryPoint::run(args_os().collect())
}
