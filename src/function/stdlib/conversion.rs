//! go-cty: `cty/function/stdlib/conversion.go`.

mod make_to;

pub use make_to::*;

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`assert_not_null`] (go-cty: `stdlib.AssertNotNullFunc`).
pub fn assert_not_null_func() -> Function {
    todo!()
}

/// Returns the value unchanged, or an error if it is null
/// (go-cty: `stdlib.AssertNotNull`).
pub fn assert_not_null(val: &Value) -> Result<Value, CtyError> {
    let _ = val;
    todo!()
}
