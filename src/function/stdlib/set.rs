//! go-cty: `cty/function/stdlib/set.go`.

mod set_intersection;
mod set_subtract;
mod set_symmetric_difference;
mod set_union;

pub use set_intersection::*;
pub use set_subtract::*;
pub use set_symmetric_difference::*;
pub use set_union::*;

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`set_has_element`] (go-cty: `stdlib.SetHasElementFunc`).
pub fn set_has_element_func() -> Function {
    todo!()
}

/// Whether the set contains the given element
/// (go-cty: `stdlib.SetHasElement`).
pub fn set_has_element(set: &Value, elem: &Value) -> Result<Value, CtyError> {
    let _ = (set, elem);
    todo!()
}
