# Changelog

All notable changes to `cargo-grip4rust` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

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
  `iter`). A project's own data-only wrapper structs are intentionally still
  flagged — see `OPEN_POINTS.md`, that part needs real type resolution.

### Added
- Six tests added before the fix, verified red against the unfixed code with a
  clean build: `Vec::get`/`Vec::clone` now correctly pure; a second collection
  type (`HashMap::len`) confirmed to work, not just `Vec`; `Vec::push` (not on
  the pure-methods list) still correctly flagged, proving the fix isn't a
  blanket exemption; and a custom type's own `.get()` method — the riskiest
  name to get wrong — still correctly flagged, since nothing distinguishes it
  from `Vec::get()` without type resolution.

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
