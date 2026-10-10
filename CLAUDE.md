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

Stage 1 is formatting, clippy and tests -- cargo built-ins only, so it works on
a fresh checkout with none of the house tools installed. Stage 2 is six gates,
run in this order:

| gate | asks |
|---|---|
| `cargo stern4rust` | do the house coding rules hold |
| `cargo dry4rust` | did this change add duplicated code |
| `cargo grip4rust` | does this tool still score itself above its floor |
| `cargo crap4rust` | is any function complex and untested |
| `cargo twin4rust` | does every source file have a mirrored test file |
| `cargo iceberg4rust` | is any file's private implementation risk too high |

stern4rust runs **first** because its corrections are renames, file moves and
directory splits: a layout it is about to reject is a layout the others would
have measured for nothing. Its findings are also the cheapest to act on.

All twenty-two of its rules are enforced on every member -- `cargo-grip4rust`,
`validation` and `xtask` -- with nothing skipped, selected, excluded or
unconfigured, and no baseline. `docs/header.txt` holds the three-line header
every `.rs` file carries and `stern4rust.toml` names it -- in the config rather
than the gate script, so a hand-run of `cargo stern4rust` checks exactly what
the gate checks. The gate names all three members for the same reason: while it
named two, `validation` broke `imported-paths` nine times with stage 2 green.

dry4rust runs **second**, for the reason stern4rust runs first: removing a
duplicate moves code between files, which changes what every gate behind it
measures -- the grip score included. It scans `core/src` only -- tests repeat their arrangement by
design -- and checks with zero ceilings against `dry4rust-baseline.json`, the
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
`cargo dry4rust --path core/src --min-nodes 25 --baseline "$PWD/dry4rust-baseline.json" baseline`.

`cargo install just`
`cargo install cargo-llvm-cov`
`cargo install cargo-stern4rust`
`cargo install cargo-dry4rust`
`cargo install cargo-crap4rust`
`cargo install cargo-twin4rust`
`cargo install cargo-iceberg4rust`

cargo-grip4rust is deliberately not in that list. The self-analysis gate builds
it from this checkout, so the score it reports is the score of the tree being
changed rather than of whatever version happens to be installed.

Stage 2 is driven by `cargo xtask stage2` -- a real crate under `xtask/`, gated
like any other code, rather than a script. Each gate is a `Gate` implementation
constructed against a `CommandRunner` trait, so the argument lists and the
failure messages are covered by `xtask`'s own integration tests.

Every measuring gate in stage 2 is scoped to `cargo-grip4rust`, which is what
keeps the rest of the repository out of them. The house-rules gate is the one
exception: it takes every member.

- `fixture/` holds bare source trees, deliberately written to score badly. They
  carry no manifest and sit beside the members rather than inside one, so they
  are not packages and no gate reaches them -- stern4rust included, which is
  why `stern4rust.toml` needs no exclusion for them.
- `validation/` holds the end-to-end tests that point the analyser at those
  trees. It **is** a workspace member, so the root `cargo test` runs all of it.
  Its `src/` is the harness every scenario shares (`FixtureAnalysis`,
  `CaptureReporter`) and one `<fixture>_analysis.rs` per scenario, the single
  place naming the tree it analyses; each `<fixture>_analysis_tests.rs` pairs
  with one, so stern4rust holds it to every rule, `paired-test-file` included.
  The measuring gates leave it alone: its subject is a fixture scenario, not
  code this crate ships.
- `xtask/` is the gate runner itself. Stage 1 and the house rules cover it in
  full; the measuring gates would otherwise be measuring the thing that invoked
  them.

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

- one struct per file
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
