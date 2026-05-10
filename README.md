# cargo-grip4rust

**How much can tests grab onto your Rust codebase?**

`cargo-grip4rust` is a static analysis tool that measures **testability** — how many pure functions, public entry points, and trait seams a codebase exposes for testing. It produces a single grip score (0–100) with a per-module breakdown and a per-function offender list.

---

## The problem

Most engineering orgs measure test *coverage* — how much code was exercised. But coverage tells you nothing about *how hard it was to write those tests*. A codebase can have 95% coverage and still be a nightmare to test:

- Functions that smuggle hidden I/O (time, filesystem, randomness) as ambient dependencies
- Concrete types everywhere, no trait seams for test doubles
- Side effects mixed with computation, so you can't test logic without mocking the world
- Everything private — zero public surface for test entry points

`grip` measures the root cause, not the symptom.

---

## The formula (Phase 0)

```
pure_ratio = probably_pure_functions / total_functions

grip = pure_ratio × 100
```

**A function is classified as probably pure** when:
- No `&mut` parameters
- Returns a non-`()` value
- Contains no `unsafe` blocks

> `grip` uses a heuristic for purity. It will produce false positives and false negatives. The heuristic is useful *directionally*: a module scoring 80 has more testability grip than one scoring 20. The limitation is stated explicitly — `grip` does not claim to detect purity, it estimates it.

---

## Installation

```sh
cargo install cargo-grip4rust
```

## Usage

```sh
cargo grip4rust [PATH]
```

**Arguments:**

| Argument | Description |
|---|---|
| `[PATH]` | Path to Rust crate or workspace root (default: `.`) |

**Options:**

| Option | Description |
|---|---|
| `--json` | Emit structured JSON output |
| `--threshold N` | Exit non-zero if overall grip score < N (CI gate). Alias: `--min-score` |
| `-h`, `--help` | Print help |
| `-V`, `--version` | Print version |

---

## Output

```
cargo-grip4rust 0.1.3 -- my-crate
══════════════════════════════════════════════════════

Overall grip score:    71 / 100
Total functions:       20
Probably pure:         12 / 20  (60.0%)

Per module:
  consensus                      grip:  78   pure: 71.4%
  transport                      grip:  83   pure: 78.9%
  timer                          grip:  44   pure: 31.2%  ❌
  state                          grip:  91   pure: 88.3%

Offenders (score < 50):
  timer                          grip:  44  ❌
```

JSON output (`--json`) includes the full breakdown, `offenders` list, and a per-function `functions` array with each function's name, file path, purity, and visibility — suitable for CI pipelines, dashboards, and editor tooling.

---

## What the score means

| Range | Meaning |
|---|---|
| 80–100 | **High grip.** Tests can reach most behavior through pure functions. |
| 50–79 | **Moderate grip.** Some modules have impure functions that need refactoring. |
| 20–49 | **Low grip.** Most logic is mixed with side effects or hidden from tests. |
| 0–19 | **Minimal grip.** The codebase resists testing at every level. |

---

## Offender list

The `functions` array in JSON output marks every function with:
- `is_pure` — whether it passes the purity heuristic
- `is_public` — whether it's visible to test code

Run `cargo grip4rust --json | jq '.functions[] | select(.is_pure == false)'` to list all impure functions.

---

## Roadmap

`grip` is being built in phases:

| Phase | What it adds | Version |
|---|---|---|
| **0** ✅ | Pure function ratio | v0.1.3 |
| 1 | Trait boundary ratio (seams) | v0.2.0 |
| 2 | Hidden dependency detection | v0.3.0 |
| 3 | Testability Index (`grip / braintax`) | v0.4.0 |
| 4 | Git history tracking + Quality Index trend | v1.0.0 |

See [`ROADMAP.md`](ROADMAP.md) for the full plan.

---

## Limitations

- **Purity is a heuristic.** `grip` classifies functions by signature patterns, not by type inference. It will make mistakes at the margin.
- **No runtime analysis.** `grip` never executes code, runs tests, or instruments builds.
- **No coverage data.** `grip` measures *testability*, not *testing*. Use a coverage tool alongside it.

---

## License

MIT — see [`LICENSE`](LICENSE).
