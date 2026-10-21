// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::gates::gate::Gate;
use crate::gates::gate_failure::GateFailure;
use crate::grip::grip_report_parser::GripReportParser;
use crate::process::command_runner::CommandRunner;

pub struct GripSelfGate<'a> {
    runner: &'a dyn CommandRunner,
    parser: &'a GripReportParser,
    package: String,
    target: String,
    floor: i64,
}

impl<'a> GripSelfGate<'a> {
    pub fn new(
        runner: &'a dyn CommandRunner,
        parser: &'a GripReportParser,
        package: String,
        target: String,
        floor: i64,
    ) -> Self {
        Self {
            runner,
            parser,
            package,
            target,
            floor,
        }
    }
}

impl Gate for GripSelfGate<'_> {
    fn label(&self) -> String {
        String::from("grip self-analysis")
    }

    fn run(&self) -> Result<(), GateFailure> {
        // Built from this checkout rather than taken from an install: a tool
        // that reports its own grip has to report the tree being changed.
        let args = vec![
            String::from("run"),
            String::from("--quiet"),
            String::from("-p"),
            self.package.clone(),
            String::from("--"),
            self.target.clone(),
            String::from("--json"),
        ];

        let outcome = self.runner.run_capturing("cargo", &args)?;

        if !outcome.is_success() {
            return Err(GateFailure::ExitedWith {
                exit_code: outcome.exit_code,
                stderr: outcome.stderr,
            });
        }

        let report = self.parser.parse(&outcome.stdout)?;

        println!("  {}", report.summary(self.floor));

        if report.meets(self.floor) {
            return Ok(());
        }

        Err(GateFailure::Found(format!(
            "score {} is below the floor of {}",
            report.score(),
            self.floor
        )))
    }
}
