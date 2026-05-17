# cargo-grip4rust

**How much can tests grab onto your Rust codebase?**

`cargo-grip4rust` is a static analysis tool that measures **testability** — how many pure functions, public entry points, trait seams, and injected dependencies a codebase exposes for testing. It produces a single grip score (0–100) with a per-module breakdown and per-function detail.

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

## The formula

```
contr(i) = base(pureᵢ, seamᵢ) × max(0, 1 − dep_weightᵢ)

grip = 100 × (
    0.30 × (pure_fn / total_fn) +
    0.20 × (public_items / total_items) +
    0.25 × (trait_impure / total_impure) +
    0.25 × (Σ contr(i) / total_fn)
)
```

### base(pure, seam)

| Function type | base | Meaning |
|---|---|---|
| Pure + trait seam | 1.00 | Ideal — substitutable AND side-effect-free |
| Pure + inherent | 0.95 | Pure but concretely coupled — minor penalty |
| Impure + trait seam | 0.85 | Has side effects but substitutable |
| Impure + inherent | 0.15 | Both impure AND concretely coupled — heavy penalty |

### dep_weight(i)

Each hidden dependency adds weight:

| Source | Weight |
|---|---|
| `println!`, `eprintln!`, `print!`, `eprint!` | 0.2 |
| `Instant::now()`, `SystemTime::now()`, `.elapsed()` | 0.3 |
| `env::var()`, `env::args()`, `process::exit()` | 0.4 |
| `unsafe { ... }` | 0.5 |
| `Database::new(...)`, `StripeGateway::charge(...)`, `self.db.query(...)` | 0.6 |

Any `Type::method(...)` where `Type` starts with uppercase and is not a std allocator (`Box`, `Arc`, `String`, `Vec`, `HashMap`, etc.) is a hidden dependency. `self.concrete_field.method(...)` where the field is a concrete type (not `Box<dyn T>`, `&dyn T`, `Arc<dyn T>`) is also flagged.

Total weight ≥ 1.0 → contribution = 0.

### Ratios

| Ratio | Definition |
|---|---|
| `pure_fn / total_fn` | Functions passing purity heuristic (no `&mut`, non-`()` return, no `unsafe`) |
| `public_items / total_items` | `pub` and `pub(crate)` items |
| `trait_impure / total_impure` | Impure methods behind a local trait seam (excludes `Display`, `Clone`, `Debug`, etc.) |
| `Σ contr(i) / total_fn` | Average per-function contribution |

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
cargo-grip4rust 0.4.0 -- my-crate
══════════════════════════════════════════════════════

Overall grip score:    74 / 100

Public surface:        142 items
Total functions:       201
Probably pure:         127 / 201  (63.2%)
Trait methods:         6 / 9 impl methods are trait-bound  (66.7%)
Hidden deps:           avg 0.42  — 60.0% clean  (71.0% avg contribution)

Per module:
  consensus                      grip:  81   pure: 71.4%   pub: 18   traits: 66.7%   clean: 80.0%
  transport                      grip:  74   pure: 78.9%   pub: 22   traits: 18.2%   clean: 40.0%  ⚠️
  timer                          grip:  44   pure: 31.2%   pub:  6   traits: 50.0%   clean: 20.0%  ❌
  state                          grip:  91   pure: 88.3%   pub: 31   traits: 66.7%   clean: 90.0%
```

### Verbose output (`--verbose`)

```
grip 0.4.0 -- my-crate — verbose
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

## Hidden dependency detection (structural)

`grip` does NOT use a hardcoded denylist of known function names. Instead, it uses structural rules:

| Rule | Example | Flagged? |
|---|---|---|
| `Type::method(...)` where Type starts uppercase, not std allocator | `StripeGateway::charge(...)`, `Database::query(...)` | ✅ |
| `self.concrete_field.method(...)` where field is not `Box\|Arc\|& dyn` | `self.db.query(...)` where `db: Database` | ✅ |
| `self.trait_field.method(...)` where field is `Box\|Arc\|& dyn T` | `self.db.query(...)` where `db: Box<dyn Database>` | ❌ injected |
| `param.method(...)` where param is a function argument | `db.query(...)` where `db: &Database` | ❌ caller-provided |
| `Self::method(...)` or `self.method(...)` | `Self::new()`, `self.process()` | ❌ own type |
| `println!`, `eprintln!`, `print!`, `eprint!` | `println!("hello")` | ✅ |
| `unsafe { ... }` | `unsafe { ... }` | ✅ |
| `Box::new(...)`, `String::new()`, `Vec::new()` | — | ❌ std alloc-only |

This catches any concrete dependency regardless of crate — `StripeGateway`, `TcpStream`, `redis::Client`, `MyDatabase` — without maintaining a denylist.

---

## Roadmap

| Phase | What it adds | Version | Status |
|---|---|---|---|
| **0** | Pure function ratio, public surface | v0.1.x | ✅ Complete |
| **1** | Trait boundary ratio (seams), foreign trait exclusion | v0.2.0 | ✅ Complete |
| **2** | Hidden dependency detection, contribution matrix, `--verbose` | v0.3.0 | ✅ Complete |
| **3** | Testability Index (`grip / braintax`) | v0.4.0 | 🔜 Planned |
| **4** | Git history tracking + Quality Index trend | v1.0.0 | 🔜 Planned |

---

## Limitations

- **Purity is a heuristic.** `grip` classifies functions by signature and body patterns, not by type inference. It makes mistakes at the margin.
- **No cross-crate analysis.** Struct fields from external crates are not resolved — `self.field.method()` detection works only when both struct and impl are in the same file.
- **No inter-procedural tracking.** A function that receives a constructed dependency from its caller appears clean.
- **No runtime or coverage data.** `grip` measures *testability*, not *testing*. Use a coverage tool alongside it.
- **Single-segment trait ambiguity.** `impl Display for X` with `use std::fmt::Display` is correctly excluded. `impl Display for X` without the import relies on the known-foreign list.

---

## License

MIT — see [`LICENSE`](LICENSE).
