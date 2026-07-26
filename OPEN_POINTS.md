# Open Points

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

## Structural hidden-dep detection can't verify a custom type's methods, only its shape

`HiddenDepFinder` (`src/hidden_dep_finder.rs`) exempts `self.field.clone()` from being
flagged as a hidden dependency when `StructRegistry::is_transitive_value_type`
(`src/struct_registry.rs`) can prove `field`'s type is plain data — every field resolves
down to a known std value type, recursively. This is deliberately narrower than it could
be:

- **Non-`clone` methods on custom types** (`self.members.len()`, `self.members.is_empty()`,
  etc.) always stay flagged, even when the wrapper is provably plain data. A type's
  *fields* being plain data doesn't prove its *methods* are pure — `DiskCache { path: String }`
  clears the field-shape check, but `DiskCache::get()` can still open a file at that path.
  Closing this needs verifying the *specific method's own body* is a pure, trivial
  delegation — recursively applying the same hidden-dep analysis to it — a materially
  bigger check than a field-shape proof, and deliberately out of scope for now.
  `tests/collector_tests.rs::hidden_dep_self_field_custom_wrapper_get_still_flagged_even_when_registry_resolves_it`
  is the regression guard for this boundary.
- **Enums** aren't registered by `StructRegistry` (structs only) — an enum field always
  resolves as unknown, so it's conservatively still flagged.
- **Generic fields** (`struct Wrapper<T> { inner: T }`) resolve the same way — `T` isn't a
  real type name, so the field never clears.
- **Cross-crate types** — the registry only sees whatever's inside the scanned path. A
  wrapper defined in a sibling crate stays unresolved.

All four fail *safe* (conservatively still flagged). Closing any of them needs either a
materially bigger check (method-body verification for the first) or real type resolution
(the other three) — neither is a small step from here. See `CHANGELOG.md` for what's
already fixed.
