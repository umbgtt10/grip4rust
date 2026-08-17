# Architecture Decision Records

Each ADR documents one load-bearing decision behind `cargo-grip4rust` —
succinct, self-contained, citable on its own. Unlike the larger `etheram`
ecosystem repos, these are not priority-tiered; `grip` is a single-crate CLI
tool with a small enough decision surface that a flat list is sufficient.

Further deferred/unstarted design decisions are tracked in
`../OPEN_POINTS.md`, not here — an ADR records a decision already made,
not one still being weighed.

## Index

| ADR | Decision |
|---|---|
| [ADR-AstOnlyNoTypeResolution](ADR-AstOnlyNoTypeResolution.md) | `grip` analyzes via `syn` AST parsing only, never type resolution — trait/call classification is name-based, with known, accepted blind spots for anything off the hardcoded known-lists. |
| [ADR-DynDispatchAppOverGenerics](ADR-DynDispatchAppOverGenerics.md) | `App` holds `Box<dyn Trait>` fields rather than generic type parameters — one nameable type instead of `App<W, S, R, C>` noise at every call site. |
| [ADR-TwoPassProjectWideRegistries](ADR-TwoPassProjectWideRegistries.md) | Two project-wide AST passes (`StructRegistry`, `MethodPurityRegistry`) run before per-file scoring, so `self.field.clone()`/`.method()` on a project's own plain-data type resolves across files — still zero type resolution, just a wider structural net. |

## Template

```markdown
# ADR-<Name>

- **Status:** Accepted | Proposed | Superseded by <ADR>
- **Date:** YYYY-MM-DD

## Context
The forces and tension this resolves.

## Decision
The choice, in one quotable sentence.

## Forcing constraints / Evidence
Why this was forced, not freely chosen — the real evidence. `N/A` if none.

## Rejected alternatives
What we did not do, and why.

## Consequences
What it commits us to; what it costs; obligations pushed onto consumers.

## Enforcement
The specific test, gate, or structural mechanism that keeps it true.
`N/A` if purely structural.

## Related
Links to other ADRs (this repo or `braintax`) and architecture docs.
```

Fields that do not apply are marked `N/A` rather than padded. Each ADR is a
snapshot of the decision as it stands today, not a changelog — state the
current shape as fact, don't narrate what an earlier version of this
document used to say.
