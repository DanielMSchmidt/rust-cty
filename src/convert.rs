//! Type conversion: the analogue of go-cty's `cty/convert` package.

use std::cmp::Ordering;

use crate::error::Error;
use crate::types::Type;
use crate::value::Value;

/// A prepared conversion from one type to another, obtained from
/// [`get_conversion`] or [`unify`] (go-cty: `convert.Conversion`).
pub struct Conversion {
    _priv: (),
}

impl std::fmt::Debug for Conversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Conversion").finish_non_exhaustive()
    }
}

impl Conversion {
    /// Applies the conversion to a value, which must be of the source type the
    /// conversion was created for.
    pub fn apply(&self, value: &Value) -> Result<Value, Error> {
        let _ = value;
        todo!()
    }
}

/// Converts a value to the given type, applying only safe (lossless and
/// unambiguous) conversions (go-cty: `convert.Convert`).
pub fn convert(value: &Value, want: &Type) -> Result<Value, Error> {
    let _ = (value, want);
    todo!()
}

/// A conversion between the given types using only safe conversions, or
/// `None` if no such conversion exists (go-cty: `convert.GetConversion`).
pub fn get_conversion(from: &Type, to: &Type) -> Option<Conversion> {
    let _ = (from, to);
    todo!()
}

/// Like [`get_conversion`], but also allowing lossy or ambiguous ("unsafe")
/// conversions (go-cty: `convert.GetConversionUnsafe`).
pub fn get_conversion_unsafe(from: &Type, to: &Type) -> Option<Conversion> {
    let _ = (from, to);
    todo!()
}

/// Finds a single type all of the given types can safely convert to, along
/// with the conversion for each input (`None` where the input already has the
/// unified type). Returns `None` when no unification is possible
/// (go-cty: `convert.Unify`, which returns `cty.NilType` on failure).
pub fn unify(types: &[Type]) -> Option<(Type, Vec<Option<Conversion>>)> {
    let _ = types;
    todo!()
}

/// Like [`unify`], but also allowing unsafe conversions
/// (go-cty: `convert.UnifyUnsafe`).
pub fn unify_unsafe(types: &[Type]) -> Option<(Type, Vec<Option<Conversion>>)> {
    let _ = types;
    todo!()
}

/// An English-language message describing why a value of type `got` is not
/// acceptable where `want` is required (go-cty: `convert.MismatchMessage`).
pub fn mismatch_message(got: &Type, want: &Type) -> String {
    let _ = (got, want);
    todo!()
}

/// Internal behaviors exposed only so the conformance suite can pin them; not
/// part of the supported API.
pub mod internals {
    use super::*;

    /// Preference ordering between two types for unification: `Less` when `a`
    /// is the preferred (more general) type (go-cty: the unexported
    /// `compareTypes` in `compare_types.go`, which returns a negative number
    /// in that case).
    pub fn compare_types(a: &Type, b: &Type) -> Ordering {
        let _ = (a, b);
        todo!()
    }

    /// Sorts types from the most general to the most specific, returning the
    /// indices of the input in sorted order (go-cty: the unexported
    /// `sortTypes` in `sort_types.go`).
    pub fn sort_types(types: &[Type]) -> Vec<usize> {
        let _ = types;
        todo!()
    }
}
