// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::args;
use std::path::Path;
use std::process::ExitCode;
use xtask::crap::crap_report_parser::CrapReportParser;
use xtask::gates::crap_gate::CrapGate;
use xtask::gates::dry_gate::DryGate;
use xtask::gates::gate::Gate;
use xtask::gates::grip_self_gate::GripSelfGate;
use xtask::gates::iceberg_gate::IcebergGate;
use xtask::gates::stage2::Stage2;
use xtask::gates::stern_gate::SternGate;
use xtask::gates::twin_gate::TwinGate;
use xtask::grip::grip_report_parser::GripReportParser;
use xtask::process::system_command_runner::SystemCommandRunner;

const PACKAGE: &str = "cargo-grip4rust";
const VALIDATION_PACKAGE: &str = "validation";
const XTASK_PACKAGE: &str = "xtask";
const CRAP_THRESHOLD: &str = "15";
const ICEBERG_THRESHOLD: &str = "20";
const GRIP_FLOOR: i64 = 59;
const DRY_PATH: &[&str] = &["core", "src"];
const DRY_BASELINE: &str = "dry4rust-baseline.json";
const DRY_MIN_NODES: &str = "25";

// Reading the real process argv and wiring the concrete runner are the two
// things no test can reach, so they are all this binary does.
fn main() -> ExitCode {
    match args().nth(1).as_deref() {
        Some("stage2") => run_stage2(),
        _ => {
            eprintln!("usage: cargo xtask stage2");
            ExitCode::FAILURE
        }
    }
}

fn run_stage2() -> ExitCode {
    let workspace_manifest = path_from_root(&["Cargo.toml"]);
    // The measuring gates are pointed at core/ rather than the workspace root,
    // which is what keeps them off the validation crate, whose subject is a
    // fixture scenario rather than code this crate ships.
    let core_manifest = path_from_root(&["core", "Cargo.toml"]);
    let core_dir = path_from_root(&["core"]);

    let runner = SystemCommandRunner::new();
    let crap_parser = CrapReportParser::new();
    let grip_parser = GripReportParser::new();
    let packages = vec![String::from(PACKAGE)];

    // Every member, so the gate checks what a hand-run of `cargo stern4rust`
    // checks. xtask is held to the rules because it is the crate that runs the
    // gates; validation because its test files now pair with the scenarios its
    // src/ names. Left out, validation broke imported-paths nine times with
    // the gate green. The measuring gates below cannot follow: they are scoped
    // to core/ to stay off validation, and xtask lives outside that manifest.
    let stern = SternGate::new(
        &runner,
        workspace_manifest,
        vec![
            String::from(PACKAGE),
            String::from(VALIDATION_PACKAGE),
            String::from(XTASK_PACKAGE),
        ],
    );
    // Second, for the same reason stern4rust is first: removing a duplicate
    // moves code between files, which changes what every gate behind it
    // measures. The published crate's source only -- tests repeat their
    // arrangement by design -- against a baseline of what was already
    // duplicated when the gate arrived, so it fails on what a change adds
    // rather than on what it inherited. Below the 25-node floor sit one-line
    // delegations whose sameness is a shared signature rather than a copy.
    let dry = DryGate::new(
        &runner,
        path_from_root(DRY_PATH),
        path_from_root(&[DRY_BASELINE]),
        String::from(DRY_MIN_NODES),
    );
    let grip = GripSelfGate::new(
        &runner,
        &grip_parser,
        String::from(PACKAGE),
        core_dir,
        GRIP_FLOOR,
    );
    let crap = CrapGate::new(
        &runner,
        &crap_parser,
        core_manifest.clone(),
        packages.clone(),
        String::from(CRAP_THRESHOLD),
    );
    let twin = TwinGate::new(&runner, core_manifest.clone(), packages.clone());
    let iceberg = IcebergGate::new(
        &runner,
        core_manifest,
        packages,
        String::from(ICEBERG_THRESHOLD),
    );

    let gates: Vec<&dyn Gate> = vec![&stern, &dry, &grip, &crap, &twin, &iceberg];

    match Stage2::new(gates).run() {
        Ok(()) => {
            println!("\ngrip4rust Stage 2 passed!");
            ExitCode::SUCCESS
        }
        Err(reason) => {
            eprintln!("\nFailed: {reason}");
            ExitCode::FAILURE
        }
    }
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives one directory below the workspace root")
}

fn path_from_root(segments: &[&str]) -> String {
    segments
        .iter()
        .fold(workspace_root().to_path_buf(), |path, segment| {
            path.join(segment)
        })
        .to_string_lossy()
        .into_owned()
}
