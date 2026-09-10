//! go-cty: `cty/function/stdlib/general.go`.

mod coalesce;
mod equal;

pub use coalesce::*;
pub use equal::*;

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`not_equal`] (go-cty: `stdlib.NotEqualFunc`).
pub fn not_equal_func() -> Function {
    todo!()
}

/// Whether the two values are not equal (go-cty: `stdlib.NotEqual`).
pub fn not_equal(a: &Value, b: &Value) -> Result<Value, CtyError> {
    let _ = (a, b);
    todo!()
}
