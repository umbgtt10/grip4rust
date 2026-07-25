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
