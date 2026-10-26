# A technology-free core and a cli crate

Status: **in progress** -- one commit per slice, in the order below.

## Why

Mutation testing pays for every mutant with a build and a test run. A mutant in
code that walks a disk, reads a clock or prints is slow to kill, and a test
that needs a directory to run in is slow to run: today forty-eight of the
collector's tests write a file only to have a path to hand it, the walker,
cache, app and entry-point tests build temporary trees, and two run the
binary. So the analysis -- the collector, the two registries, the detectors,
the scorer, the offenders, the text report -- moves into a crate that touches
no technology at all, and mutation testing is scoped to that crate alone.

## The shape

The workspace grows a member:

| member | package | library | what it holds |
|---|---|---|---|
| `core/` | `grip4rust-core` | `grip` | `#![no_std]`, `#![forbid(unsafe_code)]`; the analysis, the scores, the offenders and the text report. Depends on `libm`, `quote` and `syn` only |
| `cli/` | `cargo-grip4rust` | `grip_cli` | the binary, `Args` (clap), the composition root `EntryPoint`, and every adapter: the file system, the cache on disk, the JSON report |
| `validation/` | `validation` | `validation` | the end-to-end scenarios over `fixture/`, now on both crates |
| `xtask/` | `xtask` | `xtask` | the gates |

`cli` depends on `core`, never the reverse. Both published crates share one
workspace version. `fixture/` stays what it is: bare source trees, no member.

### Ports

Core declares a trait for everything technological it needs, implemented in
`cli/src/system/` and, for core's own tests, by in-memory fakes under
`core/tests/support/`:

| port (`core/src/traits/`) | what it answers | cli adapter |
|---|---|---|
| `Walk` | every `.rs` file under the analysed root: its path, its path beneath the root, its text | `FsWalk` |
| `ReportRenderer` | a report as text | `JsonReportRenderer`; `TextReportRenderer` lives in core |
| `CacheStore` | the counts remembered for a file, by its path | `Cache` (`.grip_cache/cache.json`) |
| `Scorer` | unchanged | `DefaultScorer`, in core |

Core holds paths as text only: it replaces `\` with `/` on every platform, as
the tool always has, and the one component-wise step -- a path beneath the
root -- is the adapter's. It fails with typed errors (`WalkFailure`,
`RenderFailure`, `RunFailure`) and returns its output as a string with a
verdict; writing it and turning the verdict into an exit code is
`EntryPoint`'s.

The binary's behaviour does not change: stdout, stderr, exit codes and the
cache file are compared byte for byte against 0.9.1 after every slice that
touches them.

### The gate

A stage-2 gate, first in the run, keeps core technology-free: its manifest may
name only `libm`, `quote` and `syn` in any section, its crate root must carry
`#![no_std]` and `#![forbid(unsafe_code)]`, and no file under `core/src` or
`core/tests` may reach `std::{env,fs,io,net,os,process}`, the printing macros,
the file-system methods or `extern crate std`.

### Mutation testing

`.cargo/mutants.toml` scopes cargo-mutants to `core/src`; each mutant runs
core's tests only.

## Slices

| # | slice | status |
|---|---|---|
| S0 | this record | done |
| S1 | `DryGate`: excludes by full path, so one run can cover the workspace | done |
| S2 | the split: `cli` takes `main.rs`, `Args` and `EntryPoint`; core is renamed; one workspace version; `[workspace.lints]` forbids unsafe code; `Config` becomes a record built by `Args::config` | done |
| S3 | the `Walk` port on `SourceFile` records; the walker becomes cli's `FsWalk` | done |
| S4 | the `CacheStore` port on text paths; the cache becomes cli's `Cache` | done |
| S5 | the `ReportRenderer` port and `RunOutcome`: the text report in core, JSON, writing and the exit code in cli | done |
| S6 | `#![no_std]`: alloc imports, ordered maps, `libm`, typed errors; anyhow leaves core | done |
| S7 | the technology-free gate, first in stage 2 | done |
| S8 | `.cargo/mutants.toml` | |
| S9 | docs, and this record moves to `docs/history/` | |

## Tests that move

No test is deleted. Tests whose subject moves go with it:

| from (`core/tests/`) | to |
|---|---|
| `main_tests.rs` (2) | `cli/tests/main_tests.rs` |
| `invocation/args_tests.rs` (13) | `cli/tests/invocation/args_tests.rs` |
| `invocation/entry_point_tests.rs` (9) | `cli/tests/invocation/entry_point_tests.rs`, same names |
| `invocation/config_tests.rs` (4, `from_args_preserves_*`) | `cli/tests/invocation/args_tests.rs`, `config_preserves_*` |
| `analysis/fs_walk_tests.rs` (4) | `cli/tests/system/fs_walk_tests.rs` |
| `invocation/cache_tests.rs` (2) | `cli/tests/system/cache_tests.rs` |
| `reporting/stdout_reporter_tests.rs` (7) | the five `human_output_*` to `reporting/text_report_renderer_tests.rs`; the two `json_output_*` to `cli/tests/system/json_report_renderer_tests.rs` |
| the records' ten serde tests | `cli/tests/system/json_report_renderer_tests.rs` |

The collector's, registries' and finder's tests keep their names and
assertions and arrange text paths and `SourceFile` records instead of files on
disk.

Rewritten, because what they called leaves the library:
`from_args_preserves_json` and `from_args_preserves_verbose` become
`compose_with_json_renders_the_report_as_json` and
`compose_with_verbose_renders_the_per_function_detail` once the two flags
choose the renderer rather than travel in `Config`; the records' five
`*_deserializes_from_json` become `render_writes_*_under_their_names`, which
assert the same field names and values on the JSON the cli renders; their five
`*_serializes_to_json` become `render_carries_*`, the same `contains` checks.
`json_output_has_version` and `json_output_is_valid` become
`render_writes_the_version` and
`render_produces_valid_json_with_the_overall_score_and_the_modules`.
`validation`'s `write_a_report_captures_it_as_pretty_json` becomes
`render_a_report_captures_it_as_pretty_json`: the port renders, and writing is
cli's. `new_builds_an_app_that_runs_on_the_injected_dependencies` and
`compose_on_a_valid_dir_builds_an_app_whose_run_succeeds` assert the output
they get instead of `ExitCode::SUCCESS`.
