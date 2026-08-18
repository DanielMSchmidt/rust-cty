//! A Rust implementation of [go-cty](https://github.com/zclconf/go-cty).
//!
//! Empty on purpose. Everything under `src/` is Daniel's to write — see `CLAUDE.md`.
//!
//! Upstream layout, as a rough map of what eventually lives here:
//!
//! | go-cty package | notes |
//! |---|---|
//! | `cty` | `Type` and `Value` core, unknowns, nulls, marks |
//! | `cty/convert` | conversion rules and type unification |
//! | `cty/function` | function definition machinery |
//! | `cty/function/stdlib` | the standard function library |
//! | `cty/json` | JSON encoding, type-directed |
//! | `cty/msgpack` | msgpack encoding |
//! | `cty/set` | set internals, element hashing |
//! | `cty/gocty` | Go interop — the Rust analogue is the stage-3 FFI layer |
//!
//! Two decisions worth making deliberately before writing much:
//!
//! - **Numbers.** cty uses `math/big.Float` at 512-bit precision. Rust has no std
//!   equivalent, and the choice of crate determines whether spec compliance is even
//!   reachable.
//! - **Strings.** cty strings are NFC-normalized and grapheme-cluster oriented, which is a
//!   different unit than Rust's `char`.
