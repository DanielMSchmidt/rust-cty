//! `equal` (go-cty: `cty/function/stdlib/general.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`equal`] (go-cty: `stdlib.EqualFunc`).
pub fn equal_func() -> Function {
    todo!()
}

/// Whether the two values are equal (go-cty: `stdlib.Equal`).
pub fn equal(a: &Value, b: &Value) -> Result<Value, CtyError> {
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

    // Ported from TestEqual:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/general_test.go#L10
    #[test]
    #[ignore = "not yet implemented"]
    fn equal() {
        let tests: Vec<(Value, Value, Value)> = vec![
            (Value::number(1), Value::number(2), Value::bool(false)),
            (Value::number(2), Value::number(2), Value::bool(true)),
            (
                Value::null(Type::number()),
                Value::null(Type::number()),
                Value::bool(true),
            ),
            (
                Value::number(2),
                Value::null(Type::number()),
                Value::bool(false),
            ),
            (
                Value::number(1),
                Value::unknown(Type::number()),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::unknown(Type::number()),
                Value::unknown(Type::number()),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::number(1),
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
            let got = stdlib::equal(a, b)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
