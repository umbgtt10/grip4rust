# Changelog

All notable changes to `cargo-grip` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

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
- CLI: `cargo grip [PATH]`, `--json`, `--min-score N`
- `#[cfg(test)]` and `#[cfg_attr(..., test)]` detection — test code excluded from analysis
- Purity heuristic: no `&mut` params, non-`()` return, no `unsafe` blocks
- JSON output with `--json` flag
- `--module PATH` filter (planned, not yet wired)
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

[Unreleased]: https://github.com/umbgtt10/grip/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/umbgtt10/grip/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/umbgtt10/grip/releases/tag/v0.1.0
