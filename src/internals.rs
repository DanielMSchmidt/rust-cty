//! Internal behaviors exposed only so the conformance suite can pin them.
//!
//! These mirror unexported go-cty functions whose behavior is nevertheless
//! observable (set element ordering, hash formats) and covered by upstream
//! tests. Not part of the supported API.

use crate::marks::ValueMarks;
use crate::value::Value;

/// The byte string used to hash a value for set bucketing, along with any
/// marks found while producing it (go-cty: the unexported
/// `makeSetHashBytes` / `appendSetHashBytes` in `set_internals.go`).
///
/// The format (e.g. `string("hello");`) is pinned by upstream tests because it
/// determines set element ordering.
pub fn set_hash_bytes(value: &Value) -> (String, ValueMarks) {
    let _ = value;
    todo!()
}
