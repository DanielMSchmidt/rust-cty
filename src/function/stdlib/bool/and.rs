//! `and` (go-cty: `cty/function/stdlib/bool.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`and`] (go-cty: `stdlib.AndFunc`).
pub fn and_func() -> Function {
    todo!()
}

/// Logical AND of two bool values (go-cty: `stdlib.And`).
pub fn and(a: &Value, b: &Value) -> Result<Value, CtyError> {
    let _ = (a, b);
    todo!()
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/function/stdlib/bool_test.go
    //!   cty/function/stdlib/bytes_test.go
    //!   cty/function/stdlib/csv_test.go
    //!   cty/function/stdlib/conversion_test.go
    //!   cty/function/stdlib/datetime_test.go
    //!   cty/function/stdlib/general_test.go
    //!   cty/function/stdlib/json_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::function::stdlib;
    use crate::{Type, Value};

    // Ported from TestAnd:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/bool_test.go#L52
    #[test]
    #[ignore = "not yet implemented"]
    fn and() {
        let tests: Vec<(Value, Value, Value)> = vec![
            (Value::bool(false), Value::bool(false), Value::bool(false)),
            (Value::bool(false), Value::bool(true), Value::bool(false)),
            (Value::bool(true), Value::bool(false), Value::bool(false)),
            (Value::bool(true), Value::bool(true), Value::bool(true)),
            (
                Value::bool(true),
                Value::unknown(Type::bool()),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::unknown(Type::bool()),
                Value::unknown(Type::bool()),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::bool(true),
                Value::dynamic(),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::dynamic(),
                Value::dynamic(),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
        ];

        for (i, (a, b, want)) in tests.iter().enumerate() {
            let got =
                stdlib::and(a, b).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
