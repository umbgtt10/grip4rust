# ADR-TwoPassProjectWideRegistries

- **Status:** Accepted
- **Date:** 2026-07-26

## Context

`Collector` scores one file at a time: `App::collect_files` walks every
`.rs` file under the target path, then calls `Collector::collect` once per
file, independently. Before this decision, that was true without
exception — `Collector`'s own `struct_concrete_fields` map, used to resolve
`self.field.method(...)` calls in `HiddenDepFinder`, was built from
scratch inside each `Collector::collect` call and discarded at the end of
it. A struct's field types were only known to the analysis if both the
struct's definition and the code calling methods on it lived in the same
file.

In practice this meant `self.field.clone()`/`.get()`/`.len()` on a
project's own plain-data wrapper type — a `Members`-style struct wrapping
a `Vec` internally, common enough to have its own dedicated file under
"one struct per file" conventions — was flagged as a hidden dependency
identical to a real collaborator call (`self.db.query(...)`), every time,
regardless of how simple the wrapper actually was. Empirically, analyzing
an unrelated real project's (`faction`) commit history to validate `grip`
and `braintax` against real code, this was the single largest driver of
suspicious ratio drops in that history — one commit alone added 35
occurrences of `self.members.clone()` (`members: Members`, a peer-set
wrapper defined in a different file than every place that cloned it), all
flagged identically to genuine I/O.

## Decision

Two lightweight, project-wide passes run once over every file in the
scanned path, before any file is scored, and are threaded into
`Collector`/`HiddenDepFinder` through their constructors:

- **`StructRegistry`** — a struct-only AST walk. `is_transitive_value_type`
  proves a type is plain data by recursing over its fields' types, down to
  a known std value-type base case, with a cycle guard for mutually
  recursive structs.
- **`MethodPurityRegistry`** — an inherent-impl-only walk, built using the
  finished `StructRegistry`. `is_known_pure_method` proves a *specific*
  `(type, method)` pair is safe by re-running the same signature-purity
  (`FunctionPurity`) and hidden-dependency (`HiddenDepFinder`) checks
  `Collector` already uses for scoring — just ahead of time, and once for
  the whole project rather than once per call site.

Both are pure structural recursion over the parsed AST — no type
resolution, no trait resolution, no cross-crate awareness. They widen the
net `ADR-AstOnlyNoTypeResolution` already accepted from "single file" to
"every file inside the scanned path," not from "syntax" to "semantics."

## Forcing constraints / Evidence

The `faction` commit-history validation (see grip's own analysis notes)
traced roughly half of the persisting ratio-drop anomalies in that
history directly to this gap, and confirmed the fix empirically: after
`StructRegistry` and `MethodPurityRegistry` landed, re-running the same
132-commit sweep showed the grip total rising in every commit where a
custom wrapper's `.clone()` or a custom accessor's `.len()`/`.get()`
newly resolved, never falling, and braintax's totals completely
unchanged — confirming the fix stayed scoped to grip's own hidden-dep
classification and didn't leak into unrelated scoring.

## Rejected alternatives

**Full type resolution (`rustc`/`rust-analyzer`).** Rejected for the same
reasons as `ADR-AstOnlyNoTypeResolution`: unstable API surface, a compile
requirement that breaks mid-refactor usability, and a much larger
dependency graph for a narrower gap than that ADR's original concern.

**An incrementally-built, mutable, cross-file cache** (extend `Collector`'s
existing per-file `struct_concrete_fields` into something shared and
updated as each file is visited, instead of two dedicated upfront passes).
Rejected: `WalkDir`'s file order isn't a contract callers can rely on, so
whether a struct's definition had already been seen when its first user
was scored would depend on incidental filesystem/iteration order —
non-deterministic, order-dependent results. Building each registry fully
before any file is scored removes the ordering dependency entirely: by
construction, every registry query happens after every file has been
seen.

**Recursive/fixpoint resolution in `MethodPurityRegistry`** (let a custom
accessor's purity depend on *other* custom accessors' purity, resolved to
a fixpoint, so nested wrapper chains fully resolve). Rejected for this
pass: a fixpoint or topological-sort build is real added complexity and a
new class of non-termination risk for uncertain marginal value against
`grip`'s actual usage — the dominant real-world shape validated against
`faction` is a single-level delegating accessor, which the current
single, non-recursive pass already proves correctly (each `HiddenDepFinder`
run inside the registry-build phase is handed an *empty*
`MethodPurityRegistry`, so nested custom-accessor calls inside a candidate
method's body are conservatively left unresolved during that pass — never
incorrectly trusted, just not yet proven). Left open in `OPEN_POINTS.md`,
not silently absent.

**Folding local trait-impl methods into `MethodPurityRegistry`** alongside
inherent ones. Rejected for this pass: `grip` already has a separate,
load-bearing local/foreign trait-impl distinction (`is_foreign_trait`,
feeding `trait_ratio` itself) — collapsing "is this method's body pure"
and "is this method reached through a trait boundary" into one registry
blurs an axis that's currently clean elsewhere in the codebase. Deferred,
not rejected outright; see `OPEN_POINTS.md`.

## Consequences

`App::collect_files` now performs two additional linear passes over the
in-memory file set — struct-only and impl-signature-only walks, both far
cheaper than a full `Collector` visit — before its existing per-file
scoring loop. `Collector::collect` and `HiddenDepFinder::new` each gained
two new required constructor parameters (`&StructRegistry`,
`&MethodPurityRegistry`), a breaking change to `grip`'s own library
surface; both types implement `Default` so a caller that doesn't need
project-wide resolution (an isolated single-file test, for instance) can
pass an empty registry and get the pre-registry behavior back exactly.
Both registries fail safe by construction — anything neither can prove
(an enum, a generic field, a type or impl outside the scanned path, a
trait-impl method, an unproven nested custom accessor) is flagged exactly
as if this ADR had never been adopted; nothing about this change can
produce a false negative beyond what was already possible before it.

## Enforcement

`tests/struct_registry_tests.rs` and `tests/method_purity_registry_tests.rs`
directly exercise the cycle guard, cross-source resolution (a struct in
one file, its impl or its consumer in another), and the "stays
conservatively unresolved" cases for both registries — not left as an
implication of `Collector`'s own test suite. `tests/collector_tests.rs`
carries the end-to-end proof that a custom wrapper's `.clone()`/pure
accessor clears through the full pipeline, and that a same-shaped but
genuinely impure method (real I/O in the body) does not.

## Related

- `ADR-AstOnlyNoTypeResolution.md` — this decision does not depart from
  that one; it widens the AST-only structural net from single-file to
  project-wide, never crossing into type or trait resolution.
- `docs/ARCHITECTURE.md` — where the two passes sit in `App`'s pipeline.
- `docs/FORMULA.md` — the exact trust rules `clone` vs. the other four
  value methods use, and why they differ.
- `OPEN_POINTS.md` — the four boundaries this ADR's rejected alternatives
  left open: trait-impl methods, recursive nested trust, enums, and
  generic/cross-crate types.
