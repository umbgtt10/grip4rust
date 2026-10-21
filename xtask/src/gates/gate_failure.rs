// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt;

use crate::crap::report_failure::ReportFailure;
use crate::grip::grip_report_failure::GripReportFailure;
use crate::process::launch_failure::LaunchFailure;

// Why a gate failed, one variant per way it can: its tool is not installed or
// could not be started, it exited with a code nobody expected, it could not
// finish, it found what it exists to find, or it ran and reported nothing a
// reader could parse. Each says itself exactly as the run has always said it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateFailure {
    CouldNotRun(Option<i32>),
    Exited(Option<i32>),
    ExitedWith {
        exit_code: Option<i32>,
        stderr: String,
    },
    Found(String),
    GripUnreadable(GripReportFailure),
    NotInstalled(String),
    NotLaunched(LaunchFailure),
    Unreported {
        failure: ReportFailure,
        exit_code: Option<i32>,
        stderr: String,
        stdout_tail: String,
    },
}

impl From<GripReportFailure> for GateFailure {
    fn from(failure: GripReportFailure) -> Self {
        Self::GripUnreadable(failure)
    }
}

impl From<LaunchFailure> for GateFailure {
    fn from(failure: LaunchFailure) -> Self {
        Self::NotLaunched(failure)
    }
}

impl fmt::Display for GateFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CouldNotRun(Some(code)) => write!(formatter, "could not run, exit code {code}"),
            Self::CouldNotRun(None) => write!(formatter, "could not run, terminated by signal"),
            Self::Exited(code) => write!(formatter, "exit code {code:?}"),
            Self::ExitedWith { exit_code, stderr } => {
                write!(formatter, "exit code {exit_code:?}\nstderr: {stderr}")
            }
            Self::Found(finding) => write!(formatter, "{finding}"),
            Self::GripUnreadable(failure) => write!(formatter, "{failure}"),
            Self::NotInstalled(binary) => {
                write!(
                    formatter,
                    "{binary} is not installed -- run: cargo install {binary}"
                )
            }
            Self::NotLaunched(failure) => write!(formatter, "{failure}"),
            Self::Unreported {
                failure,
                exit_code,
                stderr,
                stdout_tail,
            } => write!(
                formatter,
                "{failure} (exit code {exit_code:?})\nstderr: {stderr}\nstdout tail:\n{stdout_tail}"
            ),
        }
    }
}
