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

## Structural hidden-dep detection still can't clear a project's own value-type wrappers

`HiddenDepFinder::handle_method_call_expr` (`src/hidden_dep_finder.rs`) used to flag
`self.concrete_field.method(...)` as a hidden dependency whenever `concrete_field`
wasn't behind `Box`/`Arc`/`&dyn Trait`, regardless of what `method` was — so
`self.plain_vec.clone()`/`.get(i)`/`.len()` cost exactly as much as
`self.db.query(...)`. Fixed for known std value types: `Collector::visit_struct`
now records each concrete field's type head alongside its name, and
`handle_method_call_expr` exempts calls where both the field's type is a known
value-type constructor (`Vec`, `HashMap`, `HashSet`, `BTreeMap`, `BTreeSet`,
`VecDeque`, `String`, `Option`, `Cell`, `RefCell`) *and* the method is a known-pure
value-type method (`clone`, `len`, `get`, `is_empty`, `contains`, `iter`).

**What's still open:** a project's *own* data-only wrapper structs (a
`ConfirmedSet`-style type wrapping a `Vec` internally) aren't in the value-type
allowlist and still get flagged on every method call, including `.clone()`.
Nothing short of real type resolution can distinguish "clone of a data-only
wrapper" from "clone of a live collaborator" purely structurally — nor can it
tell a custom type's own `.get()` (which could easily be a real disk- or
network-backed lookup) from `Vec::get()`. Confirmed via Faction's commit
history: `self.members.clone()` (where `members: Members`, a custom peer-set
wrapper) is still flagged after this fix, correctly.

Not started, and may not be fixable without a real type-resolution pass — this
is a structurally harder problem than the fixed case, not just a bigger
allowlist.
