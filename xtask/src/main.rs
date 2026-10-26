// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::args;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use xtask::crap::crap_report_parser::CrapReportParser;
use xtask::gates::crap_gate::CrapGate;
use xtask::gates::dry_gate::DryGate;
use xtask::gates::gate::Gate;
use xtask::gates::grip_self_gate::GripSelfGate;
use xtask::gates::iceberg_gate::IcebergGate;
use xtask::gates::shear_gate::ShearGate;
use xtask::gates::stage2::Stage2;
use xtask::gates::stern_gate::SternGate;
use xtask::gates::twin_gate::TwinGate;
use xtask::grip::grip_report_parser::GripReportParser;
use xtask::process::system_command_runner::SystemCommandRunner;

const CLI_PACKAGE: &str = "cargo-grip4rust";
const CORE_PACKAGE: &str = "grip4rust-core";
const VALIDATION_PACKAGE: &str = "validation";
const XTASK_PACKAGE: &str = "xtask";
const CRAP_THRESHOLD: &str = "15";
const ICEBERG_THRESHOLD: &str = "20";
const GRIP_FLOOR: i64 = 59;
const DRY_BASELINE: &str = "dry4rust-baseline.json";
const DRY_EXCLUDES: [&[&str]; 5] = [
    &["xtask"],
    &["validation"],
    &["fixture"],
    &["core", "tests"],
    &["cli", "tests"],
];
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
    let root = workspace_root();
    let manifest_path = below(&root, &["Cargo.toml"]).to_string_lossy().into_owned();

    let runner = SystemCommandRunner::new();
    let crap_parser = CrapReportParser::new();
    let grip_parser = GripReportParser::new();
    // The two published crates. validation's subject is a fixture scenario
    // rather than code this repository ships, and xtask is the crate that runs
    // the gates, so the measuring gates leave both alone.
    let measured = vec![String::from(CORE_PACKAGE), String::from(CLI_PACKAGE)];

    // Every member, so the gate checks what a hand-run of `cargo stern4rust`
    // checks. xtask is held to the rules because it is the crate that runs the
    // gates; validation because its test files pair with the scenarios its src/
    // names. Left out, validation broke imported-paths nine times with the gate
    // green.
    let stern = SternGate::new(
        &runner,
        manifest_path.clone(),
        vec![
            String::from(CORE_PACKAGE),
            String::from(CLI_PACKAGE),
            String::from(VALIDATION_PACKAGE),
            String::from(XTASK_PACKAGE),
        ],
    );
    // Every declared dependency used, in the section that uses it. Its
    // corrections edit manifests only, so they move nothing the gates behind it
    // measure, and it costs a second.
    let shear = ShearGate::new(&runner, manifest_path.clone());

    // For the same reason stern4rust runs early: removing a duplicate moves
    // code between files, which changes what every gate behind it measures. One
    // run over both published crates' sources, so a copy from one into the other
    // is seen -- tests repeat their arrangement by design, and xtask, validation
    // and the fixture trees are not shipped -- against a baseline of what was
    // already duplicated when the gate arrived, so it fails on what a change adds
    // rather than on what it inherited. Below the 25-node floor sit one-line
    // delegations whose sameness is a shared signature rather than a copy.
    let dry = DryGate::new(
        &runner,
        root.to_string_lossy().into_owned(),
        DRY_EXCLUDES
            .iter()
            .map(|exclude| below(&root, exclude).to_string_lossy().into_owned())
            .collect(),
        below(&root, &[DRY_BASELINE]).to_string_lossy().into_owned(),
        String::from(DRY_MIN_NODES),
    );
    let grip = GripSelfGate::new(
        &runner,
        &grip_parser,
        String::from(CLI_PACKAGE),
        below(&root, &["core"]).to_string_lossy().into_owned(),
        GRIP_FLOOR,
    );
    let crap = CrapGate::new(
        &runner,
        &crap_parser,
        manifest_path.clone(),
        measured.clone(),
        String::from(CRAP_THRESHOLD),
    );
    let twin = TwinGate::new(&runner, manifest_path.clone(), measured.clone());
    let iceberg = IcebergGate::new(
        &runner,
        manifest_path,
        measured,
        String::from(ICEBERG_THRESHOLD),
    );

    let gates: Vec<&dyn Gate> = vec![&stern, &shear, &dry, &grip, &crap, &twin, &iceberg];

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

// Joined a component at a time, so the path carries the platform's separator
// throughout -- dry4rust matches its exclusions as text.
fn below(root: &Path, components: &[&str]) -> PathBuf {
    components
        .iter()
        .fold(root.to_path_buf(), |path, component| path.join(component))
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives one directory below the workspace root")
        .to_path_buf()
}
