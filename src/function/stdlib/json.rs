//! go-cty: `cty/function/stdlib/json.go`.

mod json_encode;

pub use json_encode::*;

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`json_decode`] (go-cty: `stdlib.JSONDecodeFunc`).
pub fn json_decode_func() -> Function {
    todo!()
}

/// Decodes a JSON string into a value of an implied type
/// (go-cty: `stdlib.JSONDecode`).
pub fn json_decode(str_val: &Value) -> Result<Value, CtyError> {
    let _ = str_val;
    todo!()
}
