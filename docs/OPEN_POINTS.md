# Open Points

## `pub` inside a private module is counted as public API

`ItemClassifier::classify_visibility` maps an item's own `Visibility` token to
`Pub` / `PubCrate` / `Private` and is given nothing else -- no module path, no
parent chain. So `pub struct Widget` inside `mod internals;` is scored as public
API even though nothing outside the crate can name it.

That idiom is ordinary Rust: `pub` within a private module is the normal way to
write internals that siblings may use freely. Any codebase leaning on it carries
an inflated `public_items` and a depressed `grip_score` for surface it does not
actually expose.

Measured on `fixture/sloppy_calc`, whose modules are private and whose types are
not: making the types `pub` while leaving the modules private takes
`public_items` from 4 to 6 and the score from 35 to 37 -- identical to making
them `pub(crate)`, because both readings stop at the item's own token.

Not fixable within [ADR-AstOnlyNoTypeResolution](ADRs/ADR-AstOnlyNoTypeResolution.md)
as it stands: effective visibility needs the module tree walked, which is the
resolution that ADR rules out. A narrower fix is available without abandoning
it -- a project-wide pre-pass recording which modules are private, then
demoting a `Pub` item found inside one, in the same shape as
[ADR-TwoPassProjectWideRegistries](ADRs/ADR-TwoPassProjectWideRegistries.md).

Related consequence worth recording: this is why `fixture/sloppy_calc`,
`fixture/dep_mixed` and `fixture/dep_monolith` do not compile and are not
workspace members. Their privacy and their undeclared types are the signal being
measured, and every spelling that satisfies the compiler moves the numbers their
own tests pin. Verified by trying it.

## Foreign-trait allowlist as configuration

`is_foreign_trait()` (`src/collector.rs`) checks a hardcoded ~40-name
`KNOWN_FOREIGN_TRAITS` list plus a `std`/`core`/`alloc` first-segment check
to decide whether a trait impl counts as a local architectural seam. Any
third-party framework trait not on that list — `tokio::io::AsyncRead`,
`axum::extract::FromRequest`, and similar — gets misclassified as local,
inflating `trait_ratio` for codebases built on such frameworks. Already
self-disclosed in the README's Limitations section as an inherent
AST-only, no-type-resolution constraint — not fixable in general, but a
per-project allowlist extension would narrow it for the common case.

Not started. Likely shape: an optional `--foreign-traits <file>` flag or a
`.grip.toml` listing additional names, merged with `KNOWN_FOREIGN_TRAITS`
at runtime.

## Configurable grip_score weights

`score_counts()` (`default_scorer.rs`) weights pure_ratio / public_ratio /
trait_ratio / avg_contribution at 0.30 / 0.20 / 0.25 / 0.25 — hardcoded,
summing to 1.0 by construction but not asserted anywhere. Every project
gets the same notion of "good" baked in by these four numbers.

Making them configurable (a `--weights` flag or config file, same shape
as the foreign-trait allowlist above) would let a team reflect its own
priorities — at the cost of cross-project comparability: once two
projects run different weight profiles, a `grip_score` of 70 no longer
means the same recipe in both. If built, the profile that produced a
given score should be recorded in JSON output, so any comparison stays
honest about whether it's apples-to-apples.

Not started.

## Structural hidden-dep detection still can't resolve enums, generics, or cross-crate types

`HiddenDepFinder` (`src/hidden_dep_finder.rs`) exempts `self.field.method(...)` from being
flagged as a hidden dependency in two ways: `.clone()` clears when
`StructRegistry::is_transitive_value_type` (`src/struct_registry.rs`) can prove the field's
type is plain data (every field resolves down to a known std value type, recursively);
`len`/`get`/`is_empty`/`contains`/`iter` clear when `MethodPurityRegistry`
(`src/method_purity_registry.rs`) can prove the *specific method being called* is a pure,
zero-hidden-dep inherent `&self` accessor — by re-running the same purity and hidden-dep
analysis `Collector` already uses for scoring, just ahead of time and project-wide. This is
still narrower than it could be:

- **Trait-impl methods aren't registered**, only inherent ones — a `len()` reached through a
  local trait impl stays flagged even if its body is genuinely pure. Foreign-trait impls (the
  same list `is_foreign_trait` already skips) are invisible either way, so nothing changes
  there.
- **Nested custom-method trust doesn't recurse.** `MethodPurityRegistry` is built in a single
  pass: while checking whether `Members::len()` is pure, any call inside its body to
  *another* custom type's non-`clone` accessor is conservatively treated as unresolved, since
  that other method's purity hasn't been established yet. A one-level accessor is proven; a
  chain of them isn't. Avoiding a fixpoint/topological-sort build was a deliberate scope
  call, not an oversight.
- **Enums** aren't registered by `StructRegistry` (structs only) — an enum field always
  resolves as unknown, so it's conservatively still flagged.
- **Generic fields** (`struct Wrapper<T> { inner: T }`) resolve the same way — `T` isn't a
  real type name, so the field never clears.
- **Cross-crate types** — both registries only see whatever's inside the scanned path. A
  wrapper (or its impl) defined in a sibling crate stays unresolved.

All fail *safe* (conservatively still flagged). See `CHANGELOG.md` for what's already
fixed.
