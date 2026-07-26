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

## Structural hidden-dep detection can't distinguish a value type from a live collaborator

`HiddenDepFinder::check_path` (`src/hidden_dep_finder.rs`) flags
`self.concrete_field.method(...)` as a hidden dependency whenever `concrete_field`
isn't behind `Box`/`Arc`/`&dyn Trait` — correct for a live collaborator
(`self.db.query(...)`), but the same rule fires identically for
`self.plain_vec.clone()`, `self.plain_vec.get(i)`, `self.plain_vec.len()` — plain,
deterministic, side-effect-free methods on owned value types (`Vec`, `HashSet`,
`String`, a project's own data-only structs). Nothing about cloning or
bounds-checked-indexing a value type has the non-determinism or hidden-I/O
character this dimension exists to catch, yet it costs exactly the same as
reaching into a database.

Confirmed empirically, not just in theory: analyzing Faction's commit history
(10 sampled ratio-drop pairs), this single mechanism was the most common driver
of regressions. The clearest case: a commit that made indexing defensive
(`self.flags.get(i)` instead of panicking `self.flags[i]`) scored *worse* for it,
purely because `.get()` is structurally indistinguishable from a call to a live
collaborator.

Not started. Likely shape: an allowlist of known-pure value-type methods (`clone`,
`len`, `get`, `is_empty`, `contains`, `iter`, ...) on known value-type constructors
(`Vec`, `HashMap`, `HashSet`, `String`, `Option`, `Cell`, `RefCell`, ...), mirroring
the existing `STD_CONSTRUCTORS` allowlist that already excludes `Vec::new()`/
`Box::new()` from `check_path`. The harder open question is a project's *own*
data-only structs (a `ConfirmedSet`-style wrapper) — nothing structural
distinguishes those from a genuine collaborator without type resolution.
