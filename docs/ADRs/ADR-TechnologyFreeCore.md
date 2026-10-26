# ADR-TechnologyFreeCore

- **Status:** Accepted
- **Date:** 2026-10-26

## Context

The tool is meant to be mutation-tested, and mutation testing pays for every
mutant with a build and a test run. While one crate held everything, a mutant
anywhere rebuilt a crate that linked clap, serde, serde_json and walkdir, and
its tests wrote temporary trees -- forty-eight collector tests wrote a file only
to have a path to hand it -- and ran the binary. The analysis -- what a function
hides, how a module scores, which modules offend -- is what deserves mutating,
and it was mixed with code whose only job is to talk to the machine: walking a
directory, reading file times for the cache, rendering JSON, writing stdout.

## Decision

The analysis lives in a technology-free crate, `grip4rust-core` (library
`grip`): `#![no_std]` and `#![forbid(unsafe_code)]`, depending on `libm`,
`quote` and `syn` only, declaring a trait for every piece of technology it
needs -- `Walk`, `ReportRenderer`, `CacheStore` -- and returning its output as a
string and its verdict as a flag. A separate crate, `cargo-grip4rust`, implements
those traits with the real technology, parses the arguments, writes the output
and decides the exit code. The text report stays in core: it is pure string
building, and its layout is worth mutating. Core holds paths as text and
replaces `\` with `/` on every platform, as the tool always has; the one
component-wise step, a path beneath the root, is the walk adapter's, which
reads every file itself, so a path that is not UTF-8 still works under a lossy
name.

## Forcing constraints / Evidence

`cargo mutants --list` names 460 mutants, all in the core; the workspace held
635 before the split, 498 of them in the one crate, technology included, and
125 in `xtask`. The core's 240 tests touch no disk and start no process.
Measured on two fresh copies in one build slot, load about six, one constant
changed in `ContributionSchedule` as cargo-mutants changes it, medians of
three: a clean build of the crate's tests -- what each cargo-mutants worker
pays first -- went from 20.2 s to 7.7 s, a mutant's build from 0.70 s to
0.32 s, and its build and test run from 0.96 s to 0.63 s.

`f64::round` is not in `core` on the pinned compiler; `libm::round` agreed with
the standard library's to the bit on 28 million inputs, so no score moved.

The binary's behaviour did not move: stdout, stderr, exit codes and the cache
file were compared byte for byte against 0.9.1 on eighteen invocations after
every step.

## Rejected alternatives

- **One crate, with cargo-mutants' `exclude_globs` on the technology.** Scopes
  the mutants but not the build or the tests, and nothing would stop the next
  change reaching for `std::fs` inside the analysis.
- **serde in core, alloc-only.** Keeps the derives but adds a dependency and its
  build to every mutant; the JSON document's shape is written down in cli
  instead, field for field.
- **serde remote definitions for that shape.** Compiler-checked against the
  records, but named only inside attribute strings, which stern4rust's
  `record-home` cannot see; plain mirror records with one reasoned dry4rust
  ignore entry were chosen.
- **A path type of core's own, normalised by cli on Windows only.** Would change
  Unix output for a file name holding `\`, which the tool has always shown with
  `/`.

## Consequences

Two published crates, `grip4rust-core` first, sharing one version. A new
technology need is a trait in `core/src/traits/`, an in-memory fake in
`core/tests/support/` and an adapter in `cli/src/system/`, composed in
`EntryPoint`. The core's errors are typed; `EntryPoint` prints them as it
always did. `validation` runs the fixture scenarios on both crates and is not
mutated.

## Enforcement

The "Technology-free core" gate, first in stage 2: core's manifest names only
`libm`, `quote` and `syn` in any section; `core/src/lib.rs` carries
`#![no_std]` and `#![forbid(unsafe_code)]`; no file under `core/src` or
`core/tests` reaches `std::{env,fs,io,net,os,process}`, a printing macro, a
file-system method or `extern crate std`. `.cargo/mutants.toml` scopes
cargo-mutants to `core/src`, each mutant tested by core's tests only.

## Related

- [ADR-DynDispatchAppOverGenerics](ADR-DynDispatchAppOverGenerics.md)
- [`../ARCHITECTURE.md`](../ARCHITECTURE.md)
- [`../history/[IMPLEMENTED] CORE_CLI_SPLIT.md`](../history/%5BIMPLEMENTED%5D%20CORE_CLI_SPLIT.md)
