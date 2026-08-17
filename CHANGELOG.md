# Changelog

All notable changes to `cargo-grip4rust` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

## [0.8.0] — 2026-08-17

Two scoring defects, both surfaced by the first tests these files ever had.
Minor rather than patch: `IoCallFinder` becomes public API, and crates using
`print!` or a bare `write!` statement will see their scores move.

### Fixed

- `print!` and `eprint!` are scored at 0.2 like `println!`/`eprintln!`, instead
  of falling through to the 0.6 unknown-dependency weight. `dep_weight` matched
  on the prefixes `print!` and `eprint!` — with a `!` — but labels come from
  `path_label`, which joins macro **path segments** and so yields the bare name
  `print`. Both arms were unreachable and every `print!` was scored as if it
  were an unrecognised reach-out. `docs/FORMULA.md` documented the intended 0.2
  all along; the code never implemented it.
- `IoCallFinder` detects `write!`/`writeln!` in statement position. It only
  overrode `visit_expr`, and a macro used as a statement is a `Stmt::Macro`
  that the default visitor never routes through `visit_expr`. A bare
  `write!(w, "x");` discarding its `Result` went unseen, while the same call
  written `write!(w, "x")?` was caught, because the `?` makes it an expression.
  `HiddenDepFinder` already reached print macros via `visit_stmt`; this brings
  `IoCallFinder` into line, and both macro paths now share one predicate so
  they cannot drift apart again.

Neither defect changes this crate's own score: its sources contain no `print!`
or `eprint!` calls and no bare `write!` statements. Self-analysis stays at 59.

### Added

- Mirrored test files for `hidden_dep_finder`, `io_call_finder` and
  `no_op_cache_store`, the three sources that had none. 24 tests covering the
  whole weight ladder and its ordering, all ten I/O method names, all seven
  flagged path roots, macros in both statement and expression position, and the
  concrete-field purity paths. Both defects above were found by these tests.

### Changed

- `IoCallFinder` is `pub` rather than `pub(crate)`, so it is reachable from the
  integration tests this crate requires, with a `Default` impl alongside `new`.
- `OPEN_POINTS.md` and `ROADMAP.md` moved to `docs/`, alongside the other
  long-form documentation.

## [0.7.0] — 2026-07-26

### Fixed
- `has_mut_param` treated `mut self` (by-value, locally-mutable, no observable
  side effect — the standard consuming-builder idiom) identically to `&mut self`
  (a real mutable reference), because `syn::Receiver::mutability` is set for
  both. A method like `fn with_x(mut self, x: T) -> Self` was misclassified
  impure purely from the `mut` keyword. Now checks `reference.is_some() &&
  mutability.is_some()`, matching what `docs/FORMULA.md` already documented as
  the intended rule. Found via empirical analysis of Faction's commit history —
  verified against the actual flagged source before fixing, not assumed.

- `handle_method_call_expr` flagged `self.concrete_field.method(...)` as a hidden
  dependency whenever `concrete_field` wasn't behind `Box`/`Arc`/`&dyn Trait`,
  regardless of what `method` actually was — so `self.plain_vec.clone()`/`.get(i)`/
  `.len()` cost exactly as much as `self.db.query(...)`. This was the single most
  common driver of false-positive hidden-dep flags found in the same analysis
  (roughly half of the persisting ratio drops after the two fixes above).
  `Collector::visit_struct` now records each concrete field's type head alongside
  its name, and `handle_method_call_expr` exempts calls where the field's type is
  a known value-type constructor (`Vec`, `HashMap`, `HashSet`, `BTreeMap`,
  `BTreeSet`, `VecDeque`, `String`, `Option`, `Cell`, `RefCell`) *and* the method
  is a known-pure value-type method (`clone`, `len`, `get`, `is_empty`, `contains`,
  `iter`). A project's own data-only wrapper structs were intentionally still
  flagged at this point — see the next entry.

- A project's *own* data-only wrapper structs (a `Members`-style type wrapping
  a `Vec` internally, possibly nested) are now also exempted, but only for
  `.clone()`. New `StructRegistry` (`src/struct_registry.rs`) does a
  project-wide, struct-only AST pass ahead of the normal per-file
  `Collector::collect` pass — something the tool had never done before, since
  every prior pass scored one file in isolation — and
  `is_transitive_value_type` recursively proves a type is plain data: every
  field must resolve down to a known std value type, with a cycle guard for
  mutually-recursive structs. `Collector::collect` and `HiddenDepFinder::new`
  now take the registry through their constructors. Deliberately scoped to
  `clone` only: proving a type's *fields* are plain data does not prove its
  *methods* are pure — a `DiskCache { path: String }` clears the field-shape
  check but `DiskCache::get()` can still hit disk, so `get`/`len`/`is_empty`/
  `contains`/`iter` stay std-types-only. See `OPEN_POINTS.md` for what's still
  open.

- `len`/`get`/`is_empty`/`contains`/`iter` on a project's own custom types are
  now also exempted, when provably safe. New `MethodPurityRegistry`
  (`src/method_purity_registry.rs`) does a project-wide pass over every
  inherent (non-trait) `&self` method, re-running the same signature-purity
  and hidden-dep checks `Collector` already uses for scoring, and records
  which `(type, method)` pairs come back pure with zero hidden deps.
  `HiddenDepFinder` now consults it for any non-`clone` value method on a
  type that isn't a known std type. This is deliberately narrower than full
  resolution: only inherent methods are considered (a method reached through
  a local trait impl is still flagged), and the registry build is a single
  non-recursive pass, so a custom accessor whose own body calls *another*
  custom accessor won't have that inner call trusted yet. Both are documented
  scope calls, not oversights — see `OPEN_POINTS.md`.
  Extracted `FunctionPurity` (`src/function_purity.rs`) out of `Collector` so
  the signature/unsafe/io-call purity checks aren't duplicated between
  `Collector` and the new registry; `Collector`'s own behavior is unchanged.

### Added
- Six tests added before the previous fix, verified red against the unfixed
  code with a clean build: `Vec::get`/`Vec::clone` now correctly pure; a
  second collection type (`HashMap::len`) confirmed to work, not just `Vec`;
  `Vec::push` (not on the pure-methods list) still correctly flagged, proving
  the fix isn't a blanket exemption; and a custom type's own `.get()` method —
  the riskiest name to get wrong — still correctly flagged, since nothing
  distinguishes it from `Vec::get()` without type resolution.
- `tests/struct_registry_tests.rs` (8 tests) for the new registry's recursive
  resolution directly: known std types, an unknown type, a plain wrapper, a
  wrapper of a wrapper, a struct with one non-value field (must poison the
  whole struct), a mutual cycle (must terminate, not overflow the stack), and
  cross-source resolution (the registry must see a struct defined in a
  different source entry than the one being queried — the entire reason it
  exists as a project-wide pass rather than living inside `Collector`).
- Two new `collector_tests.rs` cases proving the registry is actually wired
  in end to end: `self.members.clone()` clears when the registry resolves
  `Members` as transitively a value type, and `self.cache.get(...)` stays
  flagged even when the registry resolves `DiskCache` — the regression guard
  for the clone-only scoping decision above.
- `tests/method_purity_registry_tests.rs` (7 tests) for the new registry
  directly: a genuinely pure delegating accessor registers; a body that
  actually performs I/O does not, regardless of name; a `&mut self` method
  does not; a method reached only through a local trait impl does not
  (inherent-only scope); an unknown type/method pair does not; a struct
  declared in one source with its impl in another still resolves (the reason
  this is a project-wide pass and not something `Collector` could do alone);
  and a pure-signature method whose body calls an unresolved third-party
  constructor does not register, proving the zero-hidden-deps check is real
  and not just a signature check in disguise.
- Rewrote the existing `collector_tests.rs` DiskCache regression case so its
  `get()` body actually performs I/O — previously it was pure by accident
  (`key.to_string()`, never touching `self.path`), so under the new
  body-verifying logic it would have wrongly cleared. Same assertion value
  (`hidden_deps == 1`), now testing the real remaining boundary (a method
  that's actually impure) instead of a coincidence of the old fixture.
  Added a matching positive/negative pair for the new logic itself: a custom
  wrapper's genuinely pure `len()` clears when called from another struct;
  the same shape with an impure `len()` body stays flagged.
- `tests/function_purity_tests.rs` (14 tests) — `FunctionPurity` had no
  dedicated test file despite carrying real recursive logic, an
  inconsistency with `struct_registry_tests.rs`/`method_purity_registry_tests.rs`
  from the same round of work. All pass unchanged against the existing,
  already-shipped logic; this is a coverage backfill, not a behavior change.

### Changed
- **Breaking (library surface only, not the CLI):** `Collector::collect`
  and `HiddenDepFinder::new` each gained two new required parameters,
  `&StructRegistry` and `&MethodPurityRegistry`. Both types implement
  `Default`, so `&StructRegistry::default()`/`&MethodPurityRegistry::default()`
  reproduce the exact pre-0.7.0 behavior for any caller that doesn't want
  project-wide resolution. CLI flags, human/JSON output shape, and
  `grip_score`/`grip_absolute_total` semantics are all unchanged.
- All of `src/` and `tests/` swept for inline fully-qualified paths
  (`grip::`, `crate::`, `syn::`, plus `tempfile::`/`anyhow::` call sites)
  in favor of `use` imports, per this repo's own "no fully qualified
  paths" coding standard. Deliberately left `serde_json::to_string`/
  `from_str`/`Value` qualified everywhere — importing those bare would
  shadow `ToString::to_string`/`FromStr::from_str`, a real collision
  hazard, not a style nit. Zero behavior change; verified by identical
  test counts and gate results at every step.

## [0.6.0] — 2026-07-25

### Added
- `docs/ADRs/` (index, `ADR-AstOnlyNoTypeResolution.md`,
  `ADR-DynDispatchAppOverGenerics.md`), `docs/ARCHITECTURE.md`, and
  `docs/FORMULA.md` — the authoritative formula and architecture
  reference, verified line-by-line against `src/` rather than
  transcribed from memory.
- `OverallSummaryRenderer`, `OffendersRenderer`, `VerboseFunctionsRenderer`
  — three structs extracted from `StdoutReporter::render_human`, each
  owning one report section and independently testable.
- Direct test coverage for the `--verbose` output path, previously
  reachable only end-to-end through `StdoutReporter` and never actually
  exercised by any test.
- `Invoke-Crap4RustGate` in `scripts/run_stage_2.ps1` — stage 2 now
  actually enforces CRAP score 0, matching the hard rule already stated
  in `CLAUDE.md`/`ROADMAP.md`. Previously it only ran the binary once
  and discarded the output, so the rule was asserted but never checked.

### Changed
- `README.md`: formula section trimmed to the headline equation plus a
  link to `docs/FORMULA.md`; the duplicate "Roadmap" section (already
  diverged from `ROADMAP.md`) removed in favor of a Documentation nav
  table; the stale "Output" example (missing the `Absolute grip total`
  line) replaced with real captured output.
- `ROADMAP.md` reconciled with shipped reality: removed the unbuilt
  "Testability Index" (old Phase 3) and "git history / QI trend / cost
  projection" (old Phase 4) phases, neither of which this repo has any
  current plan to build. The two versions that actually shipped in
  their place — v0.4.0 (weighted hidden dependencies) and v0.5.0
  (per-function absolute scores, the `CacheStore` seam, dyn dispatch) —
  were never recorded as roadmap phases before now.
- `docs/FORMULA.md`: the `grip / braintax` testability-index ratio,
  previously flagged as an open question, is now decided —
  `TI = grip_absolute_total / total_braintax` (raw sums, not the
  normalized `grip_score`/`braintax_normalized`). Still not implemented
  by any released code.
- `StdoutReporter::render_human`, `HiddenDepFinder::visit_expr`,
  `Collector::visit_impl`, and `IoCallFinder::visit_expr` decomposed
  into smaller, single-purpose methods — all four were CRAP violations
  (up to 42.9 against a threshold of 15) invisible until the stage 2
  gate above was fixed to actually check. All behavior-preserving,
  verified by the full pre-existing test suite passing unchanged
  throughout.
- The overall-summary trait-methods line's three-branch conditional
  collapsed to two: the `total_impl > 0 && trait_ratio == 0.0` branch
  produced output byte-identical to the general case, so it was
  incidental complexity rather than a real distinction.

## [0.5.0] — 2026-07-25

### Added
- Per-function `grip_absolute: f64` and `grip_normalized: u32` on `FunctionInfo` —
  every function reports its own contribution as both a raw value and a 0–100
  normalized score, not just the repo-level `grip_score`.
- `grip_absolute_total: f64` on `OverallStats` and `ModuleStats` — summed
  absolute contribution across all functions, so a repo's grip index can
  always be computed even where `grip_score` alone would need a non-empty
  function set.
- `CacheStore` trait (`traits/cache_store.rs`) — `Cache`'s `get`/`set`/`flush`
  is now a swappable seam; `NoOpCacheStore` is a real always-miss fake used
  by fixture tests.
- `data_only` fixture — zero-function module regression coverage.
- `contribution_schedule_tests.rs`, `offender_tests.rs`, `function_info_tests.rs`
  — closed three source files that had no dedicated test file.
- "Absolute grip total" line in human-readable stdout output.

### Changed
- **Breaking:** `App<W: Walk, S: Scorer, R: Reporter, C: CacheStore>` is now
  a non-generic `App` holding `Box<dyn Walk>` / `Box<dyn Scorer>` /
  `Box<dyn Reporter>` / `Box<dyn CacheStore>` fields. `with_deps()` takes
  those boxes directly instead of `impl Trait`. `App::reporter()` and
  `#[derive(Debug)]` removed — both were only reachable through the now-erased
  generics.
- `Cache` rewritten with interior mutability (`RefCell`/`Cell`) so every
  `CacheStore` method takes `&self`, matching how `Reporter::write` already
  does stateful I/O.
- `contribution_schedule::contribution()` is now a `ContributionSchedule`
  struct method instead of a free function.
- `grip_score` is `Option<u32>` — `None` for zero-function modules instead
  of a misleading deterministic 20.
- `trait_ratio`'s zero-impure guard now returns `1.0` (vacuously satisfied)
  instead of `0.0`.
- Internal `crate::` fully-qualified paths replaced with `use` imports
  throughout.
- Removed dead `ItemCounts` fields (`public_functions`, `pubcrate_functions`,
  `public_structs`, `public_traits`, `public_enums`) — collected but never
  consumed downstream.
- Removed unreachable `.max(0.0)` in `contribution_schedule.rs`.

### Fixed
- CLI `--version` was hardcoded instead of reading `CARGO_PKG_VERSION`.
- `#[cfg(test)]` structs/traits/enums/impls outside a `mod tests {}` block
  were counted as production code.
- `fs_walk.rs` excluded `/target/`, `tests/`, `examples/`, `benches/` via
  unanchored substring match, false-positiving on paths like
  `src/target/mod.rs`; now anchored to path components.
- 6 fixture crates (`trait_check`, `dep_clean`, `dep_mixed`, `dep_monolith`,
  `dep_injected`, `data_only`) carried a stray `Cargo.toml` that made
  `cargo package` treat them as nested packages and silently drop their
  `[[test]]` targets from the published crate's verification build.

## [0.4.0] — 2026-05-17

### Added
- Weighted hidden dependency contributions: each dep has a weight (0.2–0.6)
  instead of binary count. `dep_weight >= 1.0` → zero contribution.
- Dependency labels in `--verbose` output: shows each hidden dep source
  (e.g. `[Database::new, println]`) alongside per-function detail.
- Continuous contribution formula: `contr = base × (1.0 − dep_weight)`
  — smooth gradient instead of binary 0/1 dep tiers.
- `dep_injected` fixture: gold standard with all deps behind `Box<dyn Trait>`.
- Cross-fixture comparison test: verifies injected (≥70) ≫ monolith (<50).

### Changed
- `contribution_schedule::contribution()` takes `dep_weight: f64` not
  `hidden_deps: usize`. Same call signature, different semantics.
- `FunctionInfo` has `dep_weight: f64` and `hidden_dep_labels: Vec<String>`.
- Verbose output shows labels in `[brackets]` after contribution.

## [0.3.0] — 2026-05-17

### Added
- Phase 2: Hidden dependency detection
- `HiddenDepFinder` — scans function bodies for time, randomness, filesystem,
  environment, process, output, and network hidden dependencies
- 8-case contribution matrix: `contribution_schedule::contribution(pure, seam, deps)`
  maps every function to a 0.0–1.0 contribution value
- Per-function `hidden_deps: usize` and `has_trait_seam: bool` in JSON output
- `avg_contribution` and `clean_fn_ratio` in ModuleStats and OverallStats
- Updated grip formula: `(pure * 0.30 + public * 0.20 + trait * 0.25 + avg_contribution * 0.25) * 100`
- `--verbose` flag with per-function detail table (hidden deps, seam status, contribution)
- Four fixture crates: `dep_clean` (all clean), `dep_mixed` (all 8 cases),
  `dep_monolith` (5 case-8 functions), `dep_injected` (trait-injected, zero hidden deps)

### Changed
- `FunctionInfo` now tracks `hidden_deps` and `has_trait_seam`
- `StdoutReporter::new()` takes `(json, verbose)` instead of `(json)`
- `Config.verbose` and `Args.verbose` added
- `Scorer::score_counts` returns 6-tuple (added `avg_contribution`, `clean_fn_ratio`)
- Impl methods now include FunctionInfo entries in report
- `ItemCounts` tracks `total_contribution: f64` for per-function contribution aggregation

### Fixed
- `pure_functions` counter was not incremented for impl block methods
- Foreign trait impls no longer fall through to inherent counting
- HiddenDepFinder handles multi-segment paths (`std::env::var`, `std::process::exit`)

## [0.2.0] — 2026-05-17

### Added
- Phase 1: Trait boundary ratio metric
- Method-level collection: `inherent_methods`, `local_trait_methods`, `trait_ratio`
  in `ItemCounts`, `ModuleStats`, `OverallStats`
- `IoCallFinder` — scans method bodies for I/O calls (TcpStream::connect,
  fs::write, writeln!, etc.) to detect impure methods lacking `&mut self`
- Known foreign trait exclusion list — `Display`, `Clone`, `Debug`, `Serialize`
  and 40+ other std/crate traits excluded from counting
- `#[test]` attribute skipping in both inherent and trait impl methods
- N/A display for modules with zero impl methods (clarifies vs 0.0%)
- `trait_check` fixture with 6 integration tests covering pure-inherent,
  impure-inherent, well-seamed, foreign-only, and mixed modules

### Changed
- Grip formula: `(pure_ratio * 0.6 + public_ratio * 0.4) * 100` →
  `(pure_ratio * 0.4 + public_ratio * 0.3 + trait_ratio * 0.3) * 100`
- `Scorer::score_counts` returns `(u32, f64, f64, f64)` — includes trait_ratio
- Human-readable output adds `Trait methods:` line and `traits:` column
- ModuleStats and OverallStats serialization includes new trait fields

### Fixed
- `has_mut_param` now detects `&mut self` receiver (was only checking typed
  parameters)
- Foreign trait detection covers multi-segment paths like `serde::Serialize`
  via last-segment check against known list
- Pure-function heuristic expanded with I/O call detection

## [0.1.4] - 2026-05-08

### Added
- Per-function tracking: `FunctionInfo` struct with `name`, `file`, `is_pure`, `is_public`
- `Collector::collect` returns `(ItemCounts, Vec<FunctionInfo>)` — per-function data alongside aggregates
- `GripReport.functions` field — all functions with file paths in JSON output
- `type CollectedFiles` alias for cleaner return types
- Private functions visible in JSON output (`is_public` field on each function entry)

### Changed
- Grip score simplified to `grip = pure_ratio × 100` (no public_ratio weighting)

---

## [0.1.3] — 2026-05-08

### Added
- `FunctionInfo` first version with per-function tracking
- Grip score formula refined

---

## [0.1.2] — 2026-05-08

### Added
- `Walk`, `Scorer`, `Reporter` traits in `src/traits/` — protocol dependency seams
- `FsWalk`, `DefaultScorer`, `StdoutReporter` concrete implementations behind traits
- `App::with_deps()` constructor for dependency injection (testing, alternate impls)
- Incremental file cache in `.grip_cache/cache.json` — re-parses only changed files
- `Cache` struct with mtime+size-based cache keys
- `cache_tests.rs` — hit and miss coverage

### Changed
- `visit_item` decomposed: 50-line match → 7-line dispatcher dispatching to `visit_fn`,
  `visit_struct`, `visit_trait`, `visit_enum`, `visit_mod`
- `App::run` decomposed: `collect_files`, `compute_report`, `handle_output` extracted
- `ItemCounts` derives `Serialize`, `Deserialize` for cache serialization

---

## [0.1.1] — 2026-05-08

### Added
- Phase 0: public surface + pure function ratio analysis
- CLI: `cargo grip [PATH]`, `--json`, `--threshold N`
- `#[cfg(test)]` and `#[cfg_attr(..., test)]` detection — test code excluded from analysis
- Purity heuristic: no `&mut` params, non-`()` return, no `unsafe` blocks
- JSON output with `--json` flag
- 46 integration tests across 13 source files, AAA-compliant
- `scripts/run_stage_1.ps1` and `scripts/run_stage_2.ps1` CI gates
- README with formula, score table, roadmap, and limitations
- ROADMAP with 4-phase plan

### Changed
- Struct consolidation: `Scorer`, `Reporter`, `App` as structs with methods
- `UnsafeFinder` extracted to own file
- Shortened qualifying paths (`args::Args` → `Args`, etc.)
- Crate renamed from `grip` to `cargo-grip`

### Fixed
- `cfg_attr` false positive — now checks `test` in token payload
- AAA blank-line separation across all tests

---

## [0.1.0] — 2026-05-08

### Added
- Initial publish on crates.io as `cargo-grip`
- Hello-world binary with cargo subcommand support
- `Cargo.toml` metadata, MIT license, README placeholder

[0.6.0]: https://github.com/umbgtt10/grip4rust/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/umbgtt10/grip4rust/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/umbgtt10/grip4rust/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/umbgtt10/grip4rust/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/umbgtt10/grip4rust/compare/v0.1.4...v0.2.0
[0.1.3]: https://github.com/umbgtt10/grip4rust/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/umbgtt10/grip4rust/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/umbgtt10/grip4rust/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/umbgtt10/grip4rust/releases/tag/v0.1.0
