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
| [`docs/ROADMAP.md`](docs/ROADMAP.md) | What's shipped, what's next. |
| [`docs/OPEN_POINTS.md`](docs/OPEN_POINTS.md) | Known gaps, deliberately deferred. |
| [`CHANGELOG.md`](CHANGELOG.md) | Release history. |

---

## Installation

```sh
cargo install cargo-grip4rust
```

## Development

```sh
just stage1
just stage2
```

Both must be green before a change is complete. Stage 1 is formatting, clippy
and tests — cargo built-ins only, so it works on a fresh checkout with none of
the tools below installed. Stage 2 is `cargo xtask stage2`, which runs, in
order: `cargo stern4rust` (house coding rules, all of them, on every workspace
member), `cargo dry4rust` (no duplication beyond `dry4rust-baseline.json`),
**grip self-analysis**, `cargo crap4rust` (complexity against coverage),
`cargo twin4rust` (every source file has a mirrored test file) and
`cargo iceberg4rust` (file risk).

The self-analysis gate is this repository's alone. It builds `cargo-grip4rust`
from the working tree, points it at `core/`, and fails if the score drops below
a floor — so a change that costs this codebase testability is caught by the very
measure the tool exists to report. It is a *floor*, unlike every other threshold
in stage 2: higher is better.

Everything the two stages need, none of which ships with cargo:

| Tool | Install | Needed by |
|---|---|---|
| [`just`](https://github.com/casey/just) | `cargo install just` | both stages |
| [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov) | `cargo install cargo-llvm-cov` | stage 2 |
| `llvm-tools` rustup component | `rustup component add llvm-tools` | stage 2 |
| `cargo-stern4rust` | `cargo install cargo-stern4rust` | stage 2 |
| `cargo-dry4rust` | `cargo install cargo-dry4rust` | stage 2 |
| `cargo-crap4rust` | `cargo install cargo-crap4rust` | stage 2 |
| `cargo-twin4rust` | `cargo install cargo-twin4rust` | stage 2 |
| `cargo-iceberg4rust` | `cargo install cargo-iceberg4rust` | stage 2 |

`cargo-llvm-cov` and `llvm-tools` are what the CRAP gate needs; without them it
fails with a bare exit code that says nothing about a missing install.

`cargo-grip4rust` itself is deliberately absent from that table — the gate
builds it from your checkout rather than taking an installed copy.

CI (`.github/workflows/ci.yml`) runs both stages on Ubuntu, Windows and macOS
for every pull request and every push to `main`.

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
cargo-grip4rust 0.9.1 -- core
══════════════════════════════════════════════════════

Overall grip score:    61 / 100
Absolute grip total:   75.68
Public surface:        40 items
Total functions:       130
Probably pure:         87 / 130  (66.9%)
Trait methods:         20 / 121 impl methods are trait-bound  (34.9%)
Hidden deps:           avg 54.32  — 3.1% clean  (58.2% avg contribution)

Per module:
  .                               grip:  74   pure: 100.0%   pub:   0   traits:    N/A   clean:   0.0%
  analysis                        grip:  55   pure:  65.4%   pub:  15   traits:  16.7%   clean:   0.0%  ⚠️
  detection                       grip:  54   pure:  51.5%   pub:   5   traits:  37.5%   clean:   0.0%  ⚠️
  invocation                      grip:  65   pure:  65.2%   pub:   7   traits:  62.5%   clean:   4.3%  ⚠️
  reporting                       grip:  94   pure:  95.2%   pub:   9   traits: 100.0%   clean:  14.3%
  traits                          grip: N/A   pure:   0.0%   pub:   4   traits:    N/A   clean:   0.0%
```

(`grip`'s own source, analyzed by itself — and the number `just stage2` holds
above a floor of 59. `N/A` on the `traits` module is the zero-function case:
`grip_score` is `Option<u32>`, `None` rather than a misleading default when
there's nothing to score.)

### Verbose output (`--verbose`)

```
grip 0.9.1 -- my-crate — verbose
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
or any other concrete dependency regardless of crate.

Two project-wide passes narrow the false positives this produces for a
project's own plain-data types: `StructRegistry` proves `self.field.clone()`
is safe when `field`'s type resolves — recursively, across every file in
scope — down to known std value types, and `MethodPurityRegistry` proves
`self.field.len()`/`.get()`/`.is_empty()`/`.contains()`/`.iter()` are safe
by re-checking the *specific method's own body* for hidden dependencies,
not just its receiver's shape. Full rule table in
[`docs/FORMULA.md`](docs/FORMULA.md#structural-hidden-dependency-detection).

---

## Limitations

- **Purity is a heuristic.** `grip` classifies functions by signature and body patterns, not by type inference. It makes mistakes at the margin.
- **No cross-crate analysis.** `StructRegistry`/`MethodPurityRegistry` resolve `self.field.clone()`/`.method()` project-wide — across files, as long as both the struct and its impl are inside the scanned path. A struct or impl defined outside it (a dependency, a sibling workspace member not included in the scan) is not resolved.
- **Method-purity resolution is inherent-only and non-recursive.** A pure accessor reached through a trait impl isn't trusted, and a custom accessor whose own body calls *another* custom type's accessor won't have that inner call trusted either — see `docs/OPEN_POINTS.md`.
- **No inter-procedural tracking.** A function that receives a constructed dependency from its caller appears clean.
- **No runtime or coverage data.** `grip` measures *testability*, not *testing*. Use a coverage tool alongside it.
- **Single-segment trait ambiguity.** `impl Display for X` with `use std::fmt::Display` is correctly excluded. `impl Display for X` without the import relies on the known-foreign list.

See [`docs/ADRs/ADR-AstOnlyNoTypeResolution.md`](docs/ADRs/ADR-AstOnlyNoTypeResolution.md)
for why these are accepted rather than fixed outright.

---

## License

MIT — see [`LICENSE`](LICENSE).
