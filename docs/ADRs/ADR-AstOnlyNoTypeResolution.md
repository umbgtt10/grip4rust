# ADR-AstOnlyNoTypeResolution

- **Status:** Accepted
- **Date:** 2026-07-25

## Context

`grip` analyzes Rust source by parsing it with `syn::parse_file` into an AST
and walking it with `syn::visit::Visit`. It never performs type resolution,
borrow checking, or macro expansion — only syntactic pattern matching over
the parsed tree. Two consequences fall directly out of that choice:

Trait-boundary classification (`is_foreign_trait` in `collector.rs`) decides
whether an `impl` counts as a local architectural seam by checking the
trait's name against a hardcoded `KNOWN_FOREIGN_TRAITS` list (~40 entries:
`Display`, `Clone`, `Serialize`, …) plus a `std`/`core`/`alloc` first-segment
heuristic — not by resolving what crate the trait actually comes from.

Hidden-dependency detection (`HiddenDepFinder` in `hidden_dep_finder.rs`)
recognizes side-effecting calls by matching the last one or two path
segments against a known-call list (`STD_MODULE_CALLS`) or an
uppercase-identifier-not-a-safe-constructor heuristic — not by resolving
what function is actually being invoked.

Because analysis never requires type information, `grip` also never
requires the analyzed code to compile. `syn::parse_file` only needs valid
syntax, so `grip` runs unchanged against a branch with unresolved imports,
missing dependencies, or an incomplete refactor mid-flight.

## Decision

`grip` stays AST-only, built on `syn` alone. No dependency on `rustc`'s
internals (HIR/MIR/`rustc_middle::ty`) or on `rust-analyzer`'s semantic
layer is taken on, even though doing so would let both classification
mechanisms above resolve real types instead of matching names against a
list.

## Forcing constraints / Evidence

Both known-list mechanisms already have live, self-disclosed blind spots
that a type-resolving analyzer would not have: a third-party trait named
identically to nothing on the known list (`tokio::io::AsyncRead`,
`axum::extract::FromRequest`) is misclassified as local, inflating
`trait_ratio`. This is recorded as an open point (a configurable allowlist
extension), not treated as a bug to fix outright — extending the list is
compatible with this ADR; resolving the trait's real origin is not.

## Rejected alternatives

**Depend on `rustc_driver`/the compiler internals directly.** Rejected:
unstable API, tied to a specific toolchain version, and would require the
analyzed crate to actually compile — defeating the "runs on any
syntactically valid source, compiling or not" property that makes `grip`
usable mid-refactor and in editor-integration contexts.

**Depend on `rust-analyzer`'s IDE crates for semantic resolution.**
Rejected: a much larger dependency surface and a fundamentally different
architecture (incremental salsa-based query engine vs. a one-shot `syn`
walk) for a benefit — resolving foreign trait/call names correctly — that
is narrow and already has a cheaper mitigation (configurable allowlists).

## Consequences

`grip` has no compile requirement, no toolchain-version coupling, a small
dependency graph (`syn`, `quote`, `walkdir`), and runs uniformly across any
target the source happens to be written for. In exchange, both
classification mechanisms have a permanent, accepted ceiling: anything not
covered by their respective known-lists is misclassified, and no future
change within this architecture removes that ceiling — only narrows it via
list extension. `grip`'s own README documents this explicitly under
Limitations rather than presenting either heuristic as exact.

## Enforcement

N/A — this is a foundational dependency choice, not a runtime-checkable
property. The check is `Cargo.toml` itself: no dependency on `rustc_*`
crates or `ra_ap_*`/`rust-analyzer` crates should ever appear.

## Related

- `OPEN_POINTS.md` — "Foreign-trait allowlist as configuration" is the
  accepted mitigation for this ADR's classification ceiling, not a
  contradiction of it.
- `braintax`'s own `ADR-AstOnlyNoTypeResolution.md` — the same decision,
  independently applicable to `braintax`'s trait-factor and hidden-dep
  detection, which share the identical `syn`-only shape.
