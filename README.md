# cargo-grip4rust

**How much can tests grab onto your Rust codebase?**

`cargo-grip4rust` is a static analysis tool that measures **testability** — how many pure functions, public entry points, trait seams, and injected dependencies a codebase exposes for testing. It produces a single grip score (0–100) with a per-module and per-function breakdown.

---

## The problem

Coverage tells you *what* code was exercised, not *how hard* it was to write the tests. A codebase can have 95% coverage and be a nightmare to test:

- Functions that construct their own dependencies (`Database::new("prod")` instead of `self.db.query(...)`)
- Concrete types everywhere, no trait seams for test doubles
- Hidden I/O, time queries, randomness smuggled into function bodies
- Side effects mixed with computation — you can't test logic without mocking the world
- Everything private — zero public surface for test entry points

`grip` measures the root cause, not the symptom.

---

## The formula, briefly

```
grip = 100 × (0.30 × pure_ratio + 0.20 × public_ratio + 0.25 × trait_ratio + 0.25 × avg_contribution)
```

Every function also gets its own absolute contribution in `[0.0, 1.0]`
(`grip_absolute`/`grip_normalized` in JSON output), and every module/repo
gets `grip_absolute_total` — the sum across every function in scope.

Full derivation of every term, every weight, and the structural rules
`grip` uses to detect hidden dependencies without a denylist:
**[`docs/FORMULA.md`](docs/FORMULA.md)**.

---

## Documentation

| Doc | What's in it |
|---|---|
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | How a `grip` invocation flows through the code, module by module. |
| [`docs/FORMULA.md`](docs/FORMULA.md) | Every scoring term, in full, kept in sync with `src/`. |
| [`docs/ADRs/`](docs/ADRs/) | Why the codebase is shaped the way it is. |
| [`ROADMAP.md`](ROADMAP.md) | What's shipped, what's next. |
| [`OPEN_POINTS.md`](OPEN_POINTS.md) | Known gaps, deliberately deferred. |
| [`CHANGELOG.md`](CHANGELOG.md) | Release history. |

---

## Installation

```sh
cargo install cargo-grip4rust
```

## Usage

```sh
cargo grip4rust [OPTIONS] [PATH]
```

**Arguments:**

| Argument | Description |
|---|---|
| `[PATH]` | Path to Rust crate or workspace root (default: `.`) |

**Options:**

| Option | Description |
|---|---|
| `--json` | Emit structured JSON output |
| `--threshold N` | Exit non-zero if overall grip score < N. Alias: `--min-score` |
| `--verbose` | Per-function detail: purity, seam, hidden deps, contribution, labels |
| `-h`, `--help` | Print help |
| `-V`, `--version` | Print version |

---

## Output

```
cargo-grip4rust 0.5.0 -- .
══════════════════════════════════════════════════════

Overall grip score:    63 / 100
Absolute grip total:   48.95
Public surface:        25 items
Total functions:       78
Probably pure:         53 / 78  (67.9%)
Trait methods:         18 / 68 impl methods are trait-bound  (52.0%)
Hidden deps:           avg 29.05  — 5.1% clean  (62.8% avg contribution)

Per module:
  .                               grip:  63   pure:  67.9%   pub:  21   traits:  52.0%   clean:   5.1%  ⚠️
  traits                          grip: N/A   pure:   0.0%   pub:   4   traits:    N/A   clean:   0.0%
```

(`grip`'s own source, analyzed by itself — `N/A` on the `traits` module is
the zero-function case: `grip_score` is `Option<u32>`, `None` rather than
a misleading default when there's nothing to score.)

### Verbose output (`--verbose`)

```
grip 0.5.0 -- my-crate — verbose
══════════════════════════════════════════════════════

  timer.rs:
    schedule_round_timeout    pure:    no  seam:   no   hidden:  2  contr:   0%  [Instant::now, thread::sleep]  ❌
    compute_timeout_ms        pure:   yes  seam:   no   hidden:  0  contr:  95%  [-]                              ✅
    reset_timer               pure:    no  seam:   no   hidden:  1  contr:  12%  [Instant::now]                    ⚠️
```

---

## What the score means

| Range | Meaning |
|---|---|
| 80–100 | **High grip.** Tests can reach most behavior through pure, seam-bound, injection-friendly code. |
| 50–79 | **Moderate grip.** Some modules have concrete dependencies or missing seams. |
| 20–49 | **Low grip.** Most logic mixes side effects with computation, hardcodes dependencies. |
| 0–19 | **Minimal grip.** The codebase resists testing at every level — every function constructs its own world. |

---

## Hidden dependency detection, without a denylist

`grip` does not maintain a list of known third-party function/type names.
Instead it uses structural rules — `Type::method(...)` where `Type` starts
uppercase and isn't a std allocator, `self.concrete_field.method(...)`
where the field isn't a trait object, and a handful of known std/core
module calls. This catches `StripeGateway`, `TcpStream`, `redis::Client`,
or any other concrete dependency regardless of crate. Full rule table in
[`docs/FORMULA.md`](docs/FORMULA.md#structural-hidden-dependency-detection).

---

## Limitations

- **Purity is a heuristic.** `grip` classifies functions by signature and body patterns, not by type inference. It makes mistakes at the margin.
- **No cross-crate analysis.** Struct fields from external crates are not resolved — `self.field.method()` detection works only when both struct and impl are in the same file.
- **No inter-procedural tracking.** A function that receives a constructed dependency from its caller appears clean.
- **No runtime or coverage data.** `grip` measures *testability*, not *testing*. Use a coverage tool alongside it.
- **Single-segment trait ambiguity.** `impl Display for X` with `use std::fmt::Display` is correctly excluded. `impl Display for X` without the import relies on the known-foreign list.

See [`docs/ADRs/ADR-AstOnlyNoTypeResolution.md`](docs/ADRs/ADR-AstOnlyNoTypeResolution.md)
for why these are accepted rather than fixed outright.

---

## License

MIT — see [`LICENSE`](LICENSE).
