# The grip formula

Full reference for how `grip` computes its scores, at every level: per
function, per module, and per repo. `README.md` keeps a short summary and
points here for the complete picture — this is the document that stays in
sync with `src/`, not the other way around.

Every number here is read directly from the source that computes it
(`contribution_schedule.rs`, `default_scorer.rs`, `hidden_dep_finder.rs`),
not transcribed from memory — if this document and the source ever
disagree, the source is right and this file is stale.

---

## Per-function: `contribution`

Every function gets an absolute contribution in `[0.0, 1.0]`, computed by
`ContributionSchedule::contribution(is_pure, has_trait_seam, dep_weight)`:

```
contribution(pure, seam, dep_weight) =
    0.0                              if dep_weight >= 1.0
    base(pure, seam) × (1.0 − dep_weight)   otherwise
```

### `base(pure, seam)`

| Pure | Trait seam | `base` | Meaning |
|---|---|---|---|
| yes | yes | 1.00 | Ideal — substitutable *and* side-effect-free |
| yes | no | 0.95 | Pure but concretely coupled — minor penalty |
| no | yes | 0.85 | Has side effects but substitutable |
| no | no | 0.15 | Both impure *and* concretely coupled — heavy penalty |

### `dep_weight`

Each hidden dependency found in the function body (via `HiddenDepFinder`)
adds a weight; `dep_weight` is the sum across every hidden dependency in
that function. Weight is looked up per call by `dep_weight(label)` in
`hidden_dep_finder.rs`, keyed on the label's text prefix:

Labels come from `path_label`, which joins a macro's **path segments**, so a
print macro arrives as the bare name `print` — never `print!`. A prefix written
with a `!` can therefore never match. Two arms cover all four macros, since
`print` prefixes `println` and `eprint` prefixes `eprintln`.

| Label prefix | Weight | Examples |
|---|---|---|
| `print`, `eprint` (so also `println`, `eprintln`) | 0.2 | `println!("...")`, `print!("...")` |
| `Instant`, `SystemTime`, `Utc`, `Local`, or contains `elapsed` | 0.3 | `Instant::now()`, `.elapsed()` |
| `env::`, `process::` | 0.4 | `env::var(...)`, `process::exit(...)` |
| `unsafe` | 0.5 | `unsafe { ... }` |
| anything else recognized as hidden | 0.6 | `Database::new(...)`, `self.db.query(...)` |

`dep_weight >= 1.0` (roughly two or more non-trivial hidden dependencies)
forces `contribution = 0.0` outright — a floor the code enforces directly,
not a coincidence of the multiplication.

This same `contribution` value is exposed per function in JSON output as
two fields on `FunctionInfo`:

- **`grip_absolute: f64`** — the raw `contribution(...)` value, `[0.0,
  1.0]`.
- **`grip_normalized: u32`** — `round(grip_absolute × 100)`, `[0, 100]`,
  the same scale `grip_score` uses at the repo level.

`FunctionInfo.hidden_dep_labels: Vec<String>` names which calls
contributed to `dep_weight`, in the order they were found — this is what
`--verbose` output shows in `[brackets]` after each function's
contribution percentage.

---

## Per-module and per-repo: `grip_score`

`OverallStats`/`ModuleStats` aggregate every function in scope into four
ratios, then combine them:

```
grip_score = round(100 × (
    0.30 × pure_ratio +
    0.20 × public_ratio +
    0.25 × trait_ratio +
    0.25 × avg_contribution
))
```

| Ratio | Definition | Zero-denominator fallback |
|---|---|---|
| `pure_ratio` | `pure_functions / total_functions` | `0.0` |
| `public_ratio` | `public_items / total_items` | `0.0` |
| `trait_ratio` | `local_trait_impure / (inherent_impure + local_trait_impure)` | `1.0` — vacuously satisfied, not failed |
| `avg_contribution` | `total_contribution / total_functions` (mean of every function's `grip_absolute` in scope) | `0.0` |

`grip_score` itself is `Option<u32>` — `None` when `total_functions == 0`
for that scope, rather than a misleading deterministic value computed from
an empty set (this was a real bug once: a zero-function module produced a
deterministic `20` before `grip_score` became optional).

### `grip_absolute_total`

`OverallStats`/`ModuleStats` also expose **`grip_absolute_total: f64`** —
the sum (not average) of every in-scope function's `grip_absolute`. This
is the field designed to pair with `braintax`'s equivalent per-repo sum,
`total_braintax`, for a `grip / braintax` testability-index ratio:

```
TI = grip_absolute_total / total_braintax
```

**Decided in favor of the raw sums**, not `grip_score /
braintax_normalized`. Raw sums are the ground truth — a direct total of
real per-function measurements, with no re-weighting or clamping layered
on top. `grip_score` and `braintax_normalized` are each already a lossy
0–100 projection shaped by decisions specific to each tool's own
reporting needs: `grip_score` blends four independently-weighted ratios
(above), and `braintax_normalized` clamps `avg_braintax` — not
`total_braintax` — against a fixed ceiling, discarding total codebase
size in the process. Dividing two independently-shaped normalizations
would compound their distortions into `TI`; dividing the two raw sums
does not. Raw sums also only reach exactly zero when there are zero
functions — a case both tools already special-case — unlike a ratio of
two 0–100 scores, which can saturate to zero on merely bad code.

Not yet implemented — no released code computes `TI` today. The same
decision is recorded in `braintax`'s own `FORMULA.md`.

---

## Structural hidden-dependency detection

`HiddenDepFinder` does not use a hardcoded denylist of known function
names. Instead, it uses structural rules over the parsed call expression:

| Rule | Example | Flagged? |
|---|---|---|
| `Type::method(...)` where `Type` starts uppercase, not a std allocator | `StripeGateway::charge(...)`, `Database::query(...)` | ✅ |
| `self.concrete_field.method(...)` where field is not `Box\|Arc\|&dyn` | see "Value-type and method-purity exemptions" below | conditional |
| `self.trait_field.method(...)` where field is `Box\|Arc\|&dyn T` | `self.db.query(...)` where `db: Box<dyn Database>` | ❌ injected |
| `param.method(...)` where param is a function argument | `db.query(...)` where `db: &Database` | ❌ caller-provided |
| `Self::method(...)` or `self.method(...)` | `Self::new()`, `self.process()` | ❌ own type |
| `println!`, `eprintln!`, `print!`, `eprint!` | `println!("hello")` | ✅ |
| `unsafe { ... }` | `unsafe { ... }` | ✅ |
| `Box::new(...)`, `String::new()`, `Vec::new()` | — | ❌ std alloc-only (`STD_CONSTRUCTORS`) |
| known std module call, `module::fn` tail, `std`/`core`-prefixed or unqualified | `fs::read(...)`, `std::fs::read(...)` | ✅ |
| known std module call, third-party-qualified | `mycrate::fs::read(...)` | ❌ — tail matches but the crate prefix doesn't |

This catches any concrete dependency regardless of crate —
`StripeGateway`, `TcpStream`, `redis::Client`, `MyDatabase` — without
maintaining a denylist of third-party type names, at the cost of the
blind spots recorded in `docs/ADRs/ADR-AstOnlyNoTypeResolution.md`.

### Value-type and method-purity exemptions

`self.concrete_field.method(...)` is not a flat yes/no — `HiddenDepFinder`
consults `KNOWN_STD_VALUE_TYPES`/`PURE_VALUE_METHODS`
(`known_hidden_dep_names.rs`) plus the two project-wide registries built in
`App::collect_files` before deciding:

| `method` | Field's type is a known std value type (`Vec`, `HashMap`, `String`, …) | Field's type is a project-local type | Flagged? |
|---|---|---|---|
| `clone` | any of `PURE_VALUE_METHODS` | `StructRegistry::is_transitive_value_type` proves every field, recursively, resolves to a known std value type | ❌ — trusted either way |
| `clone` | — | not provable plain data (has a live-collaborator field, a cycle, or resolution fails) | ✅ |
| `len`, `get`, `is_empty`, `contains`, `iter` | trusted unconditionally | `MethodPurityRegistry::is_known_pure_method` proves *that exact method's own body* is pure and zero-hidden-dep | ❌ — trusted only if proven |
| `len`, `get`, `is_empty`, `contains`, `iter` | — | not proven (trait-impl method, body has a real hidden dep, or unresolved) | ✅ |
| anything else | — | — | ✅ — not in `PURE_VALUE_METHODS` at all |

The two rows for `clone` and for the other five methods use genuinely
different proofs, not the same one applied twice:

- **`clone` trusts the receiver's *shape*.** `StructRegistry` proves a
  type is plain data by recursion over field types alone — it never looks
  at a `Clone` impl's body (grip can't: `Clone` is in `KNOWN_FOREIGN_TRAITS`,
  so `impl Clone for X` blocks are never visited at all). Cloning a value
  made only of plain data is, by construction, just copying that data.
- **The other five trust the *specific method's body*.** Field shape alone
  isn't enough here: `DiskCache { path: String }` has an entirely
  value-typed field, so it would clear the same shape-only proof `clone`
  uses — but `DiskCache::get()` can still open a file at that path.
  `MethodPurityRegistry` closes this by re-running `FunctionPurity` and
  `HiddenDepFinder` on the method's actual body, ahead of time,
  project-wide, and only trusting it if that comes back clean.

Both registries fail *safe*: anything they can't prove — an enum, a
generic field, a type or impl outside the scanned path, a trait-impl
method, a custom accessor whose body itself calls another unproven custom
accessor — stays flagged exactly as if neither registry existed. See
`OPEN_POINTS.md` for the current boundary and
`docs/ADRs/ADR-TwoPassProjectWideRegistries.md` for why it's drawn there.

---

## Related

- `docs/ADRs/ADR-AstOnlyNoTypeResolution.md` — why classification is
  name/structure-based rather than type-resolved.
- `OPEN_POINTS.md` — the foreign-trait allowlist gap and configurable
  `grip_score` weights.
- `fixture/` — every fixture crate is a worked example of one dimension
  in isolation; `tests/fixtures/data_only` specifically demonstrates the
  zero-function `None` case.
