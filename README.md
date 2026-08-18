# cty

A Rust implementation of [go-cty](https://github.com/zclconf/go-cty) — the dynamic value and
type system that underpins HCL and Terraform.

**Status: early. `src/` is empty by design** — see below.

Not published to crates.io. Depend on it by git URL:

```toml
cty = { git = "https://github.com/DanielMSchmidt/rust-cty", tag = "v0.1.0" }
```

## Why this exists

A learning project: a from-scratch Rust port of go-cty, working toward an alternative
Terraform implementation. Every line of the implementation is written by hand, deliberately.
AI assistance here is limited to research, test porting, and debugging help — it does not
write the implementation.

## Conformance

go-cty has no written specification; its Go test tables are the spec. Tests under
`tests/conformance/` are transcribed from those tables, with expected values taken as
literals from upstream so they specify behavior rather than mirror this implementation.
Each carries the upstream path and pinned commit SHA.

They fail until the corresponding behavior exists. That is the backlog, not a defect.

Tests that need the pinned go-cty checkout read `$REFERENCE_DIR` and skip when it is unset.

## Development

```sh
cargo test
```

Toolchain is pinned in `rust-toolchain.toml`.
