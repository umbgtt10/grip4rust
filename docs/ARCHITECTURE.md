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
          1. collect_files():
             a. walk finds every file, all sources read into memory
             b. StructRegistry::build(&files) — project-wide, struct-only
                pass: which types are transitively plain data?
             c. MethodPurityRegistry::build(&files, &registry) — second
                project-wide pass, over every inherent impl this time:
                which (type, method) accessors are provably pure?
             d. Collector::collect(source, path, &registry, &method_purity)
                per file not present in cache, into
                (ItemCounts, Vec<FunctionInfo>); cache is populated for
                files it missed
          2. compute_report(): scorer aggregates every file's ItemCounts
             into OverallStats + per-module ModuleStats, assembles a
             GripReport
          3. handle_output(): if --threshold is set, compare and exit;
             otherwise hand the GripReport to the reporter
```

Steps (b) and (c) are why `collect_files` reads every file into memory
before scoring any of them, rather than streaming one file at a time: a
field's type, or a method's purity, can only be resolved once every file
in scope has been seen — a struct commonly gets its own file (`one struct
per file`) while its `impl` block sits in the same file, but nothing
requires that, and the two registries exist specifically to not depend on
it. See `docs/ADRs/ADR-TwoPassProjectWideRegistries.md` for why this is
two lightweight project-wide passes rather than one deeper one, and why
neither pass follows a call chain across type boundaries.

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

## Two project-wide pre-passes: `StructRegistry`, `MethodPurityRegistry`

Before any file is scored, two lightweight passes run over every source
file in scope — struct-only and impl-only AST walks, far cheaper than the
full `Collector` visit:

| Registry | Built from | Answers |
|---|---|---|
| `StructRegistry` | every `struct` field's type, recursively | Is `type_name` transitively plain data — every field resolves down to a known std value type (`Vec`, `HashMap`, `String`, …)? |
| `MethodPurityRegistry` | every inherent (non-trait) `&self` method, re-running `FunctionPurity` + `HiddenDepFinder` on each one ahead of time | Is `(type_name, method_name)` a provably pure, zero-hidden-dep accessor? |

Both exist to answer one question `HiddenDepFinder` can't answer from a
single file alone: is `self.field.clone()`/`.len()`/`.get()`/`.is_empty()`/
`.contains()`/`.iter()` on a project's *own* type safe to treat as pure, or
is it a live collaborator call in disguise? `.clone()` is trusted whenever
`StructRegistry` can prove the field's type is plain data; the other five
methods are trusted only when `MethodPurityRegistry` can prove that
*specific method's own body* is clean — proving a type's fields are plain
data does not prove its methods are pure. Both registries are built once
in `App::collect_files` and threaded into every `Collector`/
`HiddenDepFinder` through their constructors — see
`docs/ADRs/ADR-TwoPassProjectWideRegistries.md` for the scope boundaries
(inherent-only, non-recursive, no enums/generics/cross-crate) and why they
were drawn there.

## Per-file analysis: `Collector`

`Collector::collect(source, path, registry, method_purity)` parses one
file's source with `syn::parse_file` and walks the resulting AST
(`syn::visit::Visit`), producing an `ItemCounts` (aggregate counts for
that file) and a `Vec<FunctionInfo>` (one entry per function/method
found). It delegates to five smaller, single-purpose helpers as it walks:

| Helper | Question it answers |
|---|---|
| `ContributionSchedule` | Given a function's purity, trait-seam status, and hidden-dep weight, what's its absolute contribution `[0.0, 1.0]`? |
| `HiddenDepFinder` | Which calls in this function body are hidden dependencies, and how severe is each one? Consults `StructRegistry`/`MethodPurityRegistry` for `self.field.method()` calls. |
| `FunctionPurity` | Does this signature/body disqualify a function from being pure — a `&mut` param or receiver, a unit return, an `unsafe` fn or block, an I/O call? Shared by `Collector` and `MethodPurityRegistry`, so the same rule applies whether a function is being scored or being checked as a candidate accessor. |
| `IoCallFinder` | Does this method body perform I/O (the piece of `FunctionPurity`'s check that needs a real AST walk)? |
| `UnsafeFinder` | Does this function body contain an `unsafe` block (ditto)? |

`Collector` itself decides trait-boundary classification (`is_foreign_trait`,
against `KNOWN_FOREIGN_TRAITS`) and owns the per-file `struct_concrete_fields`
map that `HiddenDepFinder` uses to resolve a *receiver's* type — a separate,
narrower thing from what the two registries above resolve (a receiver's
*type name* is always this-file-local; whether that type is safe to trust is
project-wide).

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
- `docs/ADRs/` — why `App` is shaped this way, why classification is
  name/structure-based rather than type-resolved, and why the two
  registries are project-wide passes with the specific scope they have.
- `ROADMAP.md` — what's shipped and what's planned next.
