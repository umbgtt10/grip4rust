# ADR-DynDispatchAppOverGenerics

- **Status:** Accepted
- **Date:** 2026-07-25

## Context

`App`, the top-level orchestrator that wires `Walk`, `Scorer`, `Reporter`,
and `CacheStore` together and drives `run()`, originally carried a generic
parameter per dependency: `App<W: Walk, S: Scorer, R: Reporter, C:
CacheStore>`. This is the idiomatic zero-cost Rust shape — the compiler
monomorphizes a distinct `App` type per concrete `(W, S, R, C)` combination,
so dispatch is static and there is no vtable indirection.

In practice `App` is constructed exactly twice per run — once in
`App::new()` with the real `FsWalk`/`DefaultScorer`/`StdoutReporter`/`Cache`,
and once per test in `App::with_deps()` with fakes — and every call site
that needed to *name* the type (test helper signatures, anywhere `App`
might be stored or returned) had to carry all four type parameters or fall
back to `impl Trait`, which cannot itself be named either.

## Decision

`App` is a non-generic struct holding `Box<dyn Walk>`, `Box<dyn Scorer>`,
`Box<dyn Reporter>`, `Box<dyn CacheStore>` fields. `with_deps()` takes those
four boxes directly as parameters, rather than generic or `impl Trait`
parameters that get boxed internally — construction is explicit at the call
site.

## Forcing constraints / Evidence

N/A — this was a deliberate simplification, not a response to an external
constraint. The generic version compiled and worked correctly; it was
replaced because nothing in `grip`'s actual usage exercised more than two
concrete instantiations, and the CLI's own per-invocation cost (a handful
of file-system walks and AST parses) makes vtable dispatch overhead
immaterial.

## Rejected alternatives

**Keep the generics, accept the signature noise.** Rejected: every place
that needed to name `App<W, S, R, C>` — test helpers especially — either
had to spell out all four parameters or fall back to duplicating
construction logic inline, for a static-dispatch benefit that never
mattered at this call frequency.

**`impl Trait` parameters on `with_deps()`, boxing internally.** This was
the intermediate step actually taken before landing on the final shape:
`with_deps(walker: impl Walk + 'static, ...)` still hid the boxing from the
caller. Reverted in favor of `with_deps(walker: Box<dyn Walk>, ...)` so the
box is visible and explicit at every construction site, not implicit
inside the function.

## Consequences

`App` is a single, nameable, ordinary type — usable in any signature
without generic parameters or turbofish. Two things fell out of dropping
the generics, both accepted:

`#[derive(Debug)]` was removed from `App`. `Box<dyn Trait>` is not `Debug`
unless the trait requires it as a supertrait, and none of `Walk`/`Scorer`/
`Reporter`/`CacheStore` do — `App` was never `{:?}`-printed anywhere, so
this cost nothing real.

`App::reporter()` (a test-only accessor that reached back into `App` to
read a `CaptureReporter`'s captured output after `run()`) became
impossible — there is no way to downcast `Box<dyn Reporter>` back to a
concrete type without adding `Any` to the trait bound. Every fixture test
that used this pattern moved to holding its own `Rc<RefCell<String>>`
clone of the capture buffer *before* moving the reporter into `App`, then
reading from that kept clone after `run()` — the standard pattern for
inspecting a moved object's side effects through a shared handle.

## Enforcement

N/A — structural; enforced only by the type signature of `App` itself
(there is no generic parameter left to reintroduce accidentally).

## Related

- `braintax`'s own `ADR-DynDispatchAppOverGenerics.md` — the identical
  decision, made independently for `braintax`'s own `App<W, S, R>`, which
  had the same three-dependency shape (`Walk`/`Scorer`/`Reporter`, no
  `CacheStore` — `braintax` has no cache).
