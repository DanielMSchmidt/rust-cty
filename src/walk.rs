//! Traversal of nested values: [`walk`], [`transform`], and
//! [`transform_with_transformer`]. Mirrors go-cty's `walk.go`.

use crate::error::Error;
use crate::path::Path;
use crate::value::Value;

/// Visits the given value and every nested value in depth-first order
/// (go-cty: `cty.Walk`).
///
/// The callback returns `Ok(true)` to descend into a value's children or
/// `Ok(false)` to skip them; any error aborts the walk.
pub fn walk(
    value: &Value,
    callback: impl FnMut(&Path, &Value) -> Result<bool, Error>,
) -> Result<(), Error> {
    let _ = (value, &callback);
    todo!()
}

/// Yields the given value and every nested value with its path, in the same
/// order as [`walk`] (go-cty: `cty.DeepValues`).
pub fn deep_values(value: &Value) -> Vec<(Path, Value)> {
    let _ = value;
    todo!()
}

/// Rewrites a value by applying the callback to every leaf and constructed
/// value, depth-first (go-cty: `cty.Transform`).
pub fn transform(
    value: &Value,
    callback: impl FnMut(&Path, &Value) -> Result<Value, Error>,
) -> Result<Value, Error> {
    let _ = (value, &callback);
    todo!()
}

/// A pair of callbacks for [`transform_with_transformer`]
/// (go-cty: `cty.Transformer`).
pub trait Transformer {
    /// Called before descending into a value (go-cty: `Transformer.Enter`).
    fn enter(&mut self, path: &Path, value: &Value) -> Result<Value, Error>;

    /// Called after a value's children have been transformed
    /// (go-cty: `Transformer.Exit`).
    fn exit(&mut self, path: &Path, value: &Value) -> Result<Value, Error>;
}

/// Rewrites a value using enter/exit callbacks, allowing transformation both
/// on the way down and on the way up (go-cty: `cty.TransformWithTransformer`).
pub fn transform_with_transformer(
    value: &Value,
    transformer: &mut dyn Transformer,
) -> Result<Value, Error> {
    let _ = (value, transformer);
    todo!()
}

/// Replaces every unknown value, deeply, with the null value of the same type
/// (go-cty: `cty.UnknownAsNull`).
pub fn unknown_as_null(value: &Value) -> Value {
    let _ = value;
    todo!()
}
