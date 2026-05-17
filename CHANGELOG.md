# Changelog

All notable changes to `cargo-grip4rust` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

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

[0.4.0]: https://github.com/umbgtt10/grip4rust/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/umbgtt10/grip4rust/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/umbgtt10/grip4rust/compare/v0.1.4...v0.2.0
[Unreleased]: https://github.com/umbgtt10/grip4rust/compare/v0.2.0...HEAD
[0.1.3]: https://github.com/umbgtt10/grip4rust/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/umbgtt10/grip4rust/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/umbgtt10/grip4rust/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/umbgtt10/grip4rust/releases/tag/v0.1.0
