# Open Points

## Fixture trees are un-excludable from convention linters, and manifests are not the answer

`stern4rust` reports ~100 offences against `tests/fixtures/*/`, because it walks
those trees as though this repository had written them: it demands a `mod.rs`
per folder, the repository header, and its test-file shape. The code is fine;
the tool is judging input data as source.

`stern4rust` skips any directory holding its own `Cargo.toml`, so adding one per
fixture makes the offences disappear -- verified, and `grip4rust`'s own 220
tests still pass. **Do not do it.** That is exactly what [0.5.0] removed: a
nested manifest makes `cargo package` treat the fixture as a separate package
and silently drop its `[[test]]` targets and files from the published crate.
Measured again while considering it: the published tarball goes from 28 fixture
files to 0.

The real answer is an exclude flag in the linter -- `crap4rust` already carries
`--exclude-path` for the same reason. Until `stern4rust` has one, this
repository's honest offence count is the non-fixture subset, not the total.

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
