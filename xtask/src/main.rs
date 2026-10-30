// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::args;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use xtask::crap::crap_report_parser::CrapReportParser;
use xtask::gates::boundary::boundary_rule::BoundaryRule;
use xtask::gates::boundary::crate_root_attributes::CrateRootAttributes;
use xtask::gates::boundary::disk_source_files::DiskSourceFiles;
use xtask::gates::boundary::package_dependencies::PackageDependencies;
use xtask::gates::boundary_gate::BoundaryGate;
use xtask::gates::cargo_update_gate::CargoUpdateGate;
use xtask::gates::crap_gate::CrapGate;
use xtask::gates::deny_gate::DenyGate;
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
const ALLOWED_CRATES: [&str; 3] = ["libm", "quote", "syn"];
const CORE_DIRS: [[&str; 2]; 2] = [["core", "src"], ["core", "tests"]];
const CORE_ROOT: [&str; 3] = ["core", "src", "lib.rs"];
const FORBIDDEN_MODULES: [&str; 6] = [
    "std::env",
    "std::fs",
    "std::io",
    "std::net",
    "std::os",
    "std::process",
];
const FORBIDDEN_MACROS: [&str; 5] = ["dbg", "eprint", "eprintln", "print", "println"];
const REQUIRED_ATTRIBUTES: [&str; 2] = ["no_std", "forbid(unsafe_code)"];
const FORBIDDEN_METHODS: [&str; 10] = [
    "canonicalize",
    "exists",
    "is_dir",
    "is_file",
    "is_symlink",
    "metadata",
    "read_dir",
    "read_link",
    "symlink_metadata",
    "try_exists",
];
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
        Some("sync") => run_sync(),
        _ => {
            eprintln!("usage: cargo xtask <stage2|sync>");
            ExitCode::FAILURE
        }
    }
}

// Stage 1's first step: the lock file to the newest compatible versions.
fn run_sync() -> ExitCode {
    let root = workspace_root();
    let manifest_path = below(&root, &["Cargo.toml"]).to_string_lossy().into_owned();
    let runner = SystemCommandRunner::new();
    let update = CargoUpdateGate::new(&runner, manifest_path);
    match Stage2::new(vec![&update]).run() {
        Ok(()) => {
            println!("\ngrip4rust sync done");
            ExitCode::SUCCESS
        }
        Err(reason) => {
            eprintln!("\nFailed: {reason}");
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

    // First: core stays free of technology -- only its allowed crates in its
    // manifest, `no_std` and `forbid(unsafe_code)` on its root, and no file
    // system, process, environment or stdout in its tests -- so mutation testing
    // it needs nothing but its own tests. A breach is fixed by moving code into
    // cli, which changes what every later gate, stern4rust's included, measures;
    // and it costs a second.
    let sources = DiskSourceFiles::new();
    let dependencies = PackageDependencies::new();
    let attributes = CrateRootAttributes::new();
    let boundary = BoundaryGate::new(
        &runner,
        &sources,
        &dependencies,
        &attributes,
        manifest_path.clone(),
        BoundaryRule {
            package: String::from(CORE_PACKAGE),
            source_dirs: CORE_DIRS.iter().map(|dir| below(&root, dir)).collect(),
            allowed_crates: ALLOWED_CRATES.map(String::from).to_vec(),
            crate_root: below(&root, &CORE_ROOT),
            required_attributes: REQUIRED_ATTRIBUTES.map(String::from).to_vec(),
            forbidden_modules: FORBIDDEN_MODULES.map(String::from).to_vec(),
            forbidden_macros: FORBIDDEN_MACROS.map(String::from).to_vec(),
            forbidden_methods: FORBIDDEN_METHODS.map(String::from).to_vec(),
        },
    );

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
    // Last: its advisories change without the code changing (see DenyGate).
    let deny = DenyGate::new(&runner, manifest_path.clone());
    let iceberg = IcebergGate::new(
        &runner,
        manifest_path,
        measured,
        String::from(ICEBERG_THRESHOLD),
    );

    let gates: Vec<&dyn Gate> = vec![
        &boundary, &stern, &shear, &dry, &grip, &crap, &twin, &iceberg, &deny,
    ];

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
