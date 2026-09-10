//! `less_than_or_equal_to` (go-cty: `cty/function/stdlib/number.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`less_than_or_equal_to`]
/// (go-cty: `stdlib.LessThanOrEqualToFunc`).
pub fn less_than_or_equal_to_func() -> Function {
    todo!()
}

/// Whether `a` is less than or equal to `b`
/// (go-cty: `stdlib.LessThanOrEqualTo`).
pub fn less_than_or_equal_to(a: &Value, b: &Value) -> Result<Value, CtyError> {
    let _ = (a, b);
    todo!()
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/function/stdlib/number_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::function::stdlib;
    use crate::{Type, Value};

    // Ported from TestLessThanOrEqualTo:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L439
    #[test]
    #[ignore = "not yet implemented"]
    fn less_than_or_equal_to() {
        let tests: Vec<(Value, Value, Value)> = vec![
            (Value::number(1), Value::number(2), Value::bool(true)),
            (Value::number(2), Value::number(1), Value::bool(false)),
            (Value::number(2), Value::number(2), Value::bool(true)),
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
            let got = stdlib::less_than_or_equal_to(a, b)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
