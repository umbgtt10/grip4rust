# Grip4Rust

## Meaning

`grip` is a cargo subcommand that measures the testability of Rust code — a composite
score across purity, public surface, trait seams, and hidden dependencies.

It is self-contained.

## Boundary Rule

This repository is **SELF-CONTAINED**.

The LLM **SHALL NOT cross its boundaries without asking**.

That means:
- do not inspect, edit, or rely on files outside `grip/` unless the user explicitly asks
- do not pull assumptions from sibling repositories or crates
- do not propose cross-repository changes by default

## Quality Gates

### Mandatory after every change to `src/` or `tests/` of any crate in the workspace

Run gates:

`just stage1`
`just stage2`

If either gate is not green, the work is not complete.

Both run identically on Windows, Linux and macOS, and the same two commands run
in CI -- there is no second definition of the gates to drift out of step.

Stage 1 is `sync`, then formatting, clippy and tests -- cargo built-ins only, so
it works on a fresh checkout with none of the house tools installed, though
`sync` needs the network. `sync` is `cargo update`: while the tools are in
development, every dependency moves to its newest compatible version, in the
lock file only -- a requirement in a manifest never moves, so a published
crate's users can still resolve what they could. What it moves is committed on
its own. Breaking versions are a separate decision, and not taken here. Stage 2 is nine gates,
run in this order:

| gate | asks |
|---|---|
| Technology-free core | does `core/` still touch no technology |
| `cargo stern4rust` | do the house coding rules hold |
| `cargo shear` | is every declared dependency used, in the section that uses it |
| `cargo dry4rust` | did this change add duplicated code |
| `cargo grip4rust` | does this tool still score itself above its floor |
| `cargo crap4rust` | is any function complex and untested |
| `cargo twin4rust` | does every source file have a mirrored test file |
| `cargo iceberg4rust` | is any file's private implementation risk too high |
| `cargo deny` | do the dependencies carry an advisory, a refused licence or a foreign source |

The technology-free gate runs **first**. It needs no installed tool -- it reads
`cargo metadata` and parses with the xtask's own `syn` -- and fails when
`grip4rust-core`'s manifest names a crate outside `libm`, `quote` and `syn` in
any section, when `core/src/lib.rs` lacks `#![no_std]` or
`#![forbid(unsafe_code)]`, or when a file under `core/src` or `core/tests`
reaches `std::{env,fs,io,net,os,process}`, a printing macro, a file-system
method or `extern crate std`. Its corrections move code and dependencies from
core into cli, which changes what every gate behind it measures, stern4rust's
included, and it costs a second. It matches calls by name, without type
information: a use inside a macro's arguments is not seen, and a method of one's
own named like a file-system one is reported.

stern4rust runs **second** because its corrections are renames, file moves and
directory splits: a layout it is about to reject is a layout the others would
have measured for nothing. Its findings are also the cheapest to act on.

All twenty-four of its rules are enforced on every member -- `grip4rust-core`,
`cargo-grip4rust`, `validation` and `xtask` -- with nothing skipped, selected,
excluded or unconfigured, and no baseline. `docs/header.txt` holds the
three-line header every `.rs` file carries and `stern4rust.toml` names it -- in
the config rather than the gate script, so a hand-run of `cargo stern4rust`
checks exactly what the gate checks. The gate names all four members for the
same reason: while it named two of three, `validation` broke `imported-paths`
nine times with stage 2 green.

cargo-shear runs **third**: its corrections edit manifests only -- a
dependency removed, or moved to the section that uses it -- so they change
nothing the gates behind it measure, and it takes a second. It runs `--locked`,
reading the lock file and never writing it, and `--deny-warnings`, so an
optional dependency no feature's code uses fails as an unused one does. A
dependency it cannot see used -- linked only for what it provides, declared
only to switch on a feature, used only by generated code -- goes in
`[package.metadata.cargo-shear] ignored`, and the gate refuses an entry without
its reason in the same table's `reasons`; there are none today. Pinned at
1.14.0, as CI installs it.

dry4rust runs **fourth**, for the reason stern4rust runs early: removing a
duplicate moves code between files, which changes what every gate behind it
measures -- the grip score included. It runs once from the workspace root over
both published crates' sources, `core/src` and `cli/src`, so a copy from one
into the other is seen; `xtask`, `validation`, `fixture` and both crates'
`tests` are excluded, each by its absolute path -- dry4rust matches an exclusion
as a substring of a file's path, and tests repeat their arrangement by design.
It checks with zero ceilings against `dry4rust-baseline.json`, the
duplication already there when the gate arrived. So it fails on what a change
adds, not on what it inherited; a duplicate removed is admitted, a copy added
to a recorded group is not.

The baseline is empty. When the gate arrived it found three exact groups at
25 nodes and every one could be shared without changing a result: the
last-segment and first-segment type names now share `TypePathSegments`, the
two inline-module walks share `InlineModule`, and the item counting `Collector`
repeated for structs, traits and enums is one method. Nothing was left to
record, so any group at all is one a change added.

It counts only code units of 25 AST nodes or more: below that sit one-line
delegations -- `field_type_head` handing its type to `TypePathSegments`, a
`visit_trait` handing its visibility to the counter -- whose sameness is a
shared signature rather than a copy. Re-record the baseline only to drop groups
that are gone, never to admit new ones, and at the same floor -- a baseline
matches only at the floor it was recorded at:
`cargo dry4rust --path "$PWD" --exclude "$PWD/xtask" --exclude "$PWD/validation" --exclude "$PWD/fixture" --exclude "$PWD/core/tests" --exclude "$PWD/cli/tests" --min-nodes 25 --baseline "$PWD/dry4rust-baseline.json" baseline`.

`.dry4rust-ignore.toml`, at the root the one run reads it from, holds one
fingerprint, with its reason: `JsonReportRenderer`'s `module` and `overall`
copy the same twelve figures of `ModuleStats` and `OverallStats` into the JSON
document's two objects, each in the field order the document has always had. A
mirror repeats its record's fields by definition, and sharing one shape would
reorder the document. (The bridge it replaces, for `App`'s and `Config`'s plain
constructors, went with the split: `Config` is a record, built by literal.)

cargo-deny runs **last**, and is the one gate that can turn red with nothing
changed: its advisories come from the RustSec database, so a newly published
one fails an untouched checkout, and it fetches that database over the network.
That is the point of it, and running last keeps it from hiding what the house
gates say about a change. `deny.toml` is the same file in every *4rust
repository and slotgate: licences MIT, Apache-2.0 and Unicode-3.0 -- the union
of what their trees carry -- crates.io as the only source, duplicate versions a
warning, and a wildcard requirement refused except a path dependency in an
unpublished crate. An accepted advisory goes in `[advisories] ignore` with its
id and its reason. The gate fetches the advisory database first, on its own and
up to three times, then checks `--frozen` -- locked and offline -- so a fetch
that keeps failing reports the gate as unable to run, never as an advisory
found. It decodes cargo-deny's exit code, one bit per failed check, into the
checks' names; a `deny.toml` it cannot read exits 1, as advisories does, and the
streamed output says which.

`cargo install just`
`cargo install cargo-llvm-cov`
`cargo install cargo-stern4rust`
`cargo install cargo-dry4rust`
`cargo install cargo-crap4rust`
`cargo install cargo-twin4rust`
`cargo install cargo-iceberg4rust`
`cargo install --locked cargo-shear --version 1.14.0`
`cargo install --locked cargo-deny --version 0.20.2`

cargo-grip4rust is deliberately not in that list. The self-analysis gate builds
it from this checkout, so the score it reports is the score of the tree being
changed rather than of whatever version happens to be installed. It scores
`core/` only: cli is the technology shell, scoring below the floor by its
nature, and the other gates still cover it.

Stage 2 is driven by `cargo xtask stage2` -- a real crate under `xtask/`, gated
like any other code, rather than a script. Each gate is a `Gate` implementation
constructed against a `CommandRunner` trait, so the argument lists and the
failure messages are covered by `xtask`'s own integration tests.

Every measuring gate in stage 2 is scoped to the two published crates,
`grip4rust-core` and `cargo-grip4rust`, which is what keeps the rest of the
repository out of them. The house-rules gate is the one exception: it takes
every member.

- `fixture/` holds bare source trees, deliberately written to score badly. They
  carry no manifest and sit beside the members rather than inside one, so they
  are not packages and no gate that takes packages reaches them -- stern4rust
  included, which is why `stern4rust.toml` needs no exclusion for them;
  dry4rust, which takes a directory, excludes them by path.
- `validation/` holds the end-to-end tests that point the analyser at those
  trees. It **is** a workspace member, so the root `cargo test` runs all of it.
  Its `src/` is the harness every scenario shares (`FixtureAnalysis`,
  `CaptureReporter`) and one `<fixture>_analysis.rs` per scenario, the single
  place naming the tree it analyses; each `<fixture>_analysis_tests.rs` pairs
  with one, so stern4rust holds it to every rule, `paired-test-file` included.
  It runs the analysis end to end, on both crates: `App` and the scorer from
  core, `Args`, `FsWalk` and the JSON renderer from cli. The measuring gates
  leave it alone: its subject is a fixture scenario, not code these crates ship.
- `xtask/` is the gate runner itself. Stage 1 and the house rules cover it in
  full; the measuring gates would otherwise be measuring the thing that invoked
  them.

## Layout

| member | package | library | what it holds |
|---|---|---|---|
| `core/` | `grip4rust-core` | `grip` | the analysis: collector, registries, detectors, scorer, offenders, the text report. `#![no_std]`, `#![forbid(unsafe_code)]`; depends on `libm`, `quote` and `syn` only |
| `cli/` | `cargo-grip4rust` | `grip_cli` | the binary, `Args` (clap), the composition root `EntryPoint`, and the adapters in `system/`: `FsWalk`, `JsonReportRenderer` |
| `validation/` | `validation` | `validation` | the fixture scenarios, end to end (not published) |
| `xtask/` | `xtask` | `xtask` | the gates (not published) |

`cli` depends on `core`, never the reverse. Both published crates share the
workspace's version.

## Technology-free core

Core declares a trait for every piece of technology it needs and never touches
one itself, so mutation testing it needs nothing but its own tests. A new need
is a trait in `core/src/traits/`, an in-memory fake in `core/tests/support/`
and an adapter in `cli/src/system/`, composed in `EntryPoint`. Core works on
its own records -- a `SourceFile` holds a path as text, and core replaces `\`
with `/` on every platform, as the tool always has -- fails with typed errors
(`WalkFailure`, `RenderFailure`, `RunFailure`), and returns its output as a
string with a verdict (`RunOutcome`); writing it and the exit code are
`EntryPoint`'s. The technology-free gate holds the line.

`.cargo/mutants.toml` scopes cargo-mutants to `core/src`, each mutant tested by
core's tests only (`test_workspace = false`).

## Publishing

A release publishes both crates, `grip4rust-core` first: `cargo publish
--workspace` orders them and skips `validation` and `xtask` (`publish =
false`); `cargo publish --workspace --dry-run` is the check. The workspace
dependency names core's version, because a published cli must name a
registry version of core; the two move together.

## Orthogonality, trait surface and cognitive complexity

**When changing productive code, always maximize orthogonality and testable surface through traits, and minimize cognitive complexity.**

Specifically:
- prefer extracting behavior behind traits so individual pieces can be tested and swapped independently
- prefer small, focused methods with a single responsibility over large methods with many branches
- prefer named structs with methods over free functions operating on external state
- when `crap4rust` or a reviewer flags a function as too complex, reduce it by extracting internal structs with methods and adding integration coverage — not by extracting standalone helper functions
- never increase cognitive complexity to pass a test; find the root cause and fix it there
- when introducing a new protocol dependency seam, place the contract in `traits/`, place the protocol-facing state/data model parallel to the protocol, and place the concrete implementation in its own dedicated implementation area
- make constructors depend on traits, not directly on concrete implementations
- ALL dependencies are injected through the SINGLE constructor and stored in the struct
- apply the same split recursively to nested dependencies: trait first, state/data model second, concrete implementation third

## User coding standards

- one object per file; records where record-home puts them
- no unnecessary comments in code
- unit tests are not allowed. Only integration tests are
- consolidate scattered functions inside structs as appropriate
- no `&mut` input parameters; prefer return values
- only use `pub mod` in `mod.rs` and `lib.rs`
- split test files so there is one test file per source file, named `<source file name>_tests.rs`
- in `all_tests.rs`, reference test files one by one without `#[path = ...]`
- apply AAA (`Arrange`, `Act`, `Assert`) structure to tests with blank-line separation between the three sections
- use `// Arrange & Act` if there is no separate `Arrange`
- use `// Act & Assert` if there is no separate `Act`
- add the repository copyright and license header to every Rust source file
- tests should be named as follows `<method under test>_<test description>_<result>`
- do not use fully qualified paths; use `use` imports instead
