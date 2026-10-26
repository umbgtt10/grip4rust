# grip4rust-core

The analysis behind [`cargo-grip4rust`](https://crates.io/crates/cargo-grip4rust):
how testable Rust code is -- a composite score across purity, public surface,
trait seams and hidden dependencies. This crate is the collector, the
registries, the detectors, the scorer and the text report; the command,
`cargo grip4rust`, is the `cargo-grip4rust` crate.

What it measures, the formula and its limitations are in the
[main README](https://github.com/umbgtt10/grip4rust#readme) and
[`docs/FORMULA.md`](https://github.com/umbgtt10/grip4rust/blob/main/docs/FORMULA.md).

MIT -- see [`LICENSE`](https://github.com/umbgtt10/grip4rust/blob/main/LICENSE).
