//! A Rust implementation of [go-cty](https://github.com/zclconf/go-cty).
//!
//! **Everything here is a stub.** The module tree and public signatures define
//! the API surface that the conformance tests under `tests/conformance/`
//! compile against; every body is `todo!()`, to be written by hand — see
//! `README.md`. The Go→Rust naming and design decisions are recorded in
//! `docs/api-mapping.md`.
//!
//! Upstream layout and where it lives here:
//!
//! | go-cty package | here |
//! |---|---|
//! | `cty` | crate root: [`Type`], [`Value`], marks, paths, walk, refinements |
//! | `cty/convert` | [`convert`] |
//! | `cty/function` | [`function`] |
//! | `cty/function/stdlib` | [`function::stdlib`] |
//! | `cty/json` | [`json`] |
//! | `cty/msgpack` | [`msgpack`] |
//! | `cty/set` | [`set`] |
//! | `cty/ctystrings` | [`strings`] |
//! | `cty/gocty` | [`interop`] (trait-based instead of reflection) |
//!
//! Two decisions worth making deliberately before writing much:
//!
//! - **Numbers.** cty uses `math/big.Float` at 512-bit precision. Rust has no std
//!   equivalent, and the choice of crate determines whether spec compliance is even
//!   reachable.
//! - **Strings.** cty strings are NFC-normalized and grapheme-cluster oriented, which is a
//!   different unit than Rust's `char`.

pub mod capsule;
pub mod convert;
mod error;
pub mod function;
pub mod internals;
pub mod interop;
pub mod json;
mod marks;
pub mod msgpack;
mod path;
mod refinement;
pub mod set;
pub mod strings;
mod types;
mod value;
mod value_ops;
mod walk;

pub use capsule::CapsuleOps;
pub use error::Error;
pub use marks::{Mark, PathValueMarks, ValueMarks, WrangleAction, WrangleFunc};
pub use path::{Path, PathSet, PathStep};
pub use refinement::{RefinementBuilder, ValueRange};
pub use set::ValueSet;
pub use types::Type;
pub use value::Value;
pub use value_ops::ElementIterator;
pub use walk::{
    Transformer, deep_values, transform, transform_with_transformer, unknown_as_null, walk,
};
