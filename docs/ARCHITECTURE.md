# Architecture

How a `cargo grip4rust` invocation actually flows through the code. This
is a map of what exists today, not a decision record — see `docs/ADRs/`
for the "why" behind the shapes described here, and `docs/FORMULA.md` for
how the scores themselves are computed.

---

## Pipeline

```
Args (clap)
  → Config
    → App
        walker:   Box<dyn Walk>      — finds .rs files
        scorer:   Box<dyn Scorer>    — aggregates + scores
        reporter: Box<dyn Reporter>  — renders output
        cache:    Box<dyn CacheStore> — skips unchanged files
      → App::run()
          1. collect_files(): walk finds files; Collector parses each one
             not present in cache into (ItemCounts, Vec<FunctionInfo>);
             cache is populated for files it missed
          2. compute_report(): scorer aggregates every file's ItemCounts
             into OverallStats + per-module ModuleStats, assembles a
             GripReport
          3. handle_output(): if --threshold is set, compare and exit;
             otherwise hand the GripReport to the reporter
```

`App` (`app.rs`) is the only place that wires concrete types to trait
objects — everywhere else in the codebase depends on the `traits/`
interfaces, never on `FsWalk`/`DefaultScorer`/`StdoutReporter`/`Cache`
directly. See `docs/ADRs/ADR-DynDispatchAppOverGenerics.md` for why these
are `Box<dyn Trait>` fields rather than generic parameters.

## The four injected seams (`traits/`)

| Trait | Concrete impl | Responsibility |
|---|---|---|
| `Walk` | `FsWalk` | Recursively find `.rs` files under the target path, excluding `target/`, `tests/`, `examples/`, `benches/` by path component. |
| `Scorer` | `DefaultScorer` | Turn `ItemCounts` into ratios and a `grip_score`; aggregate per-file counts into per-module and overall stats. |
| `Reporter` | `StdoutReporter` | Render a `GripReport` as human-readable text or JSON. |
| `CacheStore` | `Cache` (real) / `NoOpCacheStore` (always-miss fake) | Skip re-parsing a file whose mtime+size haven't changed since the last run. |

Every fixture crate under `tests/fixtures/` exercises `App::with_deps()`
with a mix of real and fake implementations of these four traits — that's
the seam the whole test suite is built around.

## Per-file analysis: `Collector`

`Collector::collect(source, path)` parses one file's source with
`syn::parse_file` and walks the resulting AST (`syn::visit::Visit`),
producing an `ItemCounts` (aggregate counts for that file) and a
`Vec<FunctionInfo>` (one entry per function/method found). It delegates to
four smaller, single-purpose visitors as it walks:

| Helper | Question it answers |
|---|---|
| `ContributionSchedule` | Given a function's purity, trait-seam status, and hidden-dep weight, what's its absolute contribution `[0.0, 1.0]`? |
| `HiddenDepFinder` | Which calls in this function body are hidden dependencies, and how severe is each one? |
| `IoCallFinder` | Does this method body perform I/O (used to decide whether a method lacking `&mut self` is genuinely pure)? |
| `UnsafeFinder` | Does this function body contain an `unsafe` block? |

`Collector` itself decides trait-boundary classification (`is_foreign_trait`,
against `KNOWN_FOREIGN_TRAITS`) and purity heuristics; the four helpers above
answer narrower, independently-testable questions that `Collector` combines.

## Data model

| Type | Scope | Carries |
|---|---|---|
| `ItemCounts` | per file, merged to per-module/overall | raw counts (public items, pure functions, trait methods, `total_contribution`, …) that `Scorer` turns into ratios |
| `FunctionInfo` | per function | `grip_absolute`/`grip_normalized` (this function's own score), `is_pure`, `has_trait_seam`, `dep_weight`, `hidden_dep_labels` |
| `OverallStats` / `ModuleStats` | repo / module | `grip_score: Option<u32>`, the four scoring ratios, `grip_absolute_total` (sum of every in-scope function's `grip_absolute`) |
| `GripReport` | one CLI invocation | `overall`, `modules: Vec<ModuleStats>`, `functions: Vec<FunctionInfo>`, `offenders` (modules below `--threshold`) |
| `Offender` | modules below threshold | just enough to list them |

`GripReport` is what `Reporter::render` turns into either the
human-readable summary or the `--json` output — it's the single shape
both output modes are projections of.

## CLI layer

`Args` (`args.rs`, `clap::Parser`) parses argv into flags; `Config`
(`config.rs`) is the plain-data form `App` actually consumes, built once
via `Config::from_args(args)`. `main.rs` and `lib.rs::run()`/`run_from_args()`
are thin entry points — all real logic lives in `App` and below.

## Related

- `docs/FORMULA.md` — how `grip_score`, `grip_absolute`, and
  `grip_absolute_total` are actually computed.
- `docs/ADRs/` — why `App` is shaped this way, and why classification is
  name/structure-based rather than type-resolved.
- `ROADMAP.md` — what's shipped and what's planned next.
