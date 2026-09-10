//! go-cty: `cty/function/stdlib/datetime.go`.

mod format_date;

pub use format_date::*;

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`time_add`] (go-cty: `stdlib.TimeAddFunc`).
pub fn time_add_func() -> Function {
    todo!()
}

/// Adds a duration to an RFC 3339 timestamp (go-cty: `stdlib.TimeAdd`).
pub fn time_add(timestamp: &Value, duration: &Value) -> Result<Value, CtyError> {
    let _ = (timestamp, duration);
    todo!()
}
