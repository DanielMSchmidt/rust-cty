//! Internal behaviors exposed only so the conformance suite can pin them.
//!
//! These mirror unexported go-cty functions whose behavior is nevertheless
//! observable (set element ordering, hash formats) and covered by upstream
//! tests. Not part of the supported API.

use std::rc::Rc;

use crate::marks::ValueMarks;
use crate::set::Rules;
use crate::types::Type;
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

/// The element rules used by set values whose element type is the given type,
/// exposing their ordering (`less`) and compatibility (`same_rules`) behavior
/// (go-cty: the unexported `setRules` in `set_internals.go`).
///
/// These rules always define an ordering, so their `less` never returns
/// `None`.
pub fn set_rules(element_type: Type) -> Rc<dyn Rules<Value>> {
    let _ = element_type;
    todo!()
}
