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

## Structural hidden-dep detection still can't verify a custom type's methods, only its shape

`HiddenDepFinder::handle_method_call_expr` (`src/hidden_dep_finder.rs`) used to flag
`self.concrete_field.method(...)` as a hidden dependency whenever `concrete_field`
wasn't behind `Box`/`Arc`/`&dyn Trait`, regardless of what `method` was — so
`self.plain_vec.clone()`/`.get(i)`/`.len()` cost exactly as much as
`self.db.query(...)`. Fixed in two stages, both in `CHANGELOG.md`:

1. Known std value types (`Vec`, `HashMap`, `HashSet`, `BTreeMap`, `BTreeSet`,
   `VecDeque`, `String`, `Option`, `Cell`, `RefCell`) paired with known-pure
   methods (`clone`, `len`, `get`, `is_empty`, `contains`, `iter`) are exempted.
2. A project's *own* data-only wrapper structs (a `Members`-style type wrapping
   a `Vec` internally, possibly nested) are now also exempted, but **only for
   `.clone()`**. `StructRegistry` (`src/struct_registry.rs`) does a
   project-wide, struct-only AST pass ahead of the normal per-file
   `Collector::collect` pass, and `is_transitive_value_type` recursively
   proves a type is plain data — every field must resolve down to a known std
   value type, with a cycle guard for mutually-recursive structs.
   `Collector` and `HiddenDepFinder` take the registry through their
   constructors.

**Why only `clone`, not `get`/`len`/etc., for custom types:** proving a type's
*fields* are all plain data does not prove its *methods* are pure. A
`DiskCache { path: String }` has an entirely value-typed field, so it clears
`is_transitive_value_type`, but `DiskCache::get()` can still open a file at
that path — the structural check can't see the method body, only the field
shapes. Cloning is different: cloning a struct made only of plain data is, by
definition, just copying that data, regardless of what any of its *other*
methods do. `tests/collector_tests.rs::hidden_dep_self_field_custom_wrapper_get_still_flagged_even_when_registry_resolves_it`
is the regression guard proving `.get()` stays flagged even when the registry
*can* prove the receiver is transitively a value type.

**What's still open:**
- **Non-`clone` methods on custom types** (`self.members.len()`,
  `self.members.is_empty()`, etc.) still get flagged even when the wrapper is
  provably plain data. Closing this needs verifying the *specific method's
  own body* is a pure, trivial delegation — recursively applying the same
  hidden-dep analysis to it — a materially bigger check than a field-shape
  proof, and deliberately out of scope for now.
- **Enums** aren't registered by `StructRegistry` (structs only) — an enum
  field always resolves as unknown, so it's conservatively still flagged.
- **Generic fields** (`struct Wrapper<T> { inner: T }`) resolve the same way —
  `T` isn't a real type name, so the field never clears.
- **Cross-crate types** — the registry only sees whatever's inside the
  scanned path. A wrapper defined in a sibling crate stays unresolved.

All four fail *safe* (conservatively still flagged), consistent with every
other allowlist in this file. Closing any of them needs either a materially
bigger check (method-body verification for the first) or real type resolution
(the other three) — neither is a small step from here.
