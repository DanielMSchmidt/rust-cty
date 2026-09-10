//! `absolute` (go-cty: `cty/function/stdlib/number.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`absolute`] (go-cty: `stdlib.AbsoluteFunc`).
pub fn absolute_func() -> Function {
    todo!()
}

/// The absolute value of a number (go-cty: `stdlib.Absolute`).
pub fn absolute(num: &Value) -> Result<Value, CtyError> {
    let _ = num;
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

    // Ported from TestAbsolute:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L12
    #[test]
    #[ignore = "not yet implemented"]
    fn absolute() {
        let tests: Vec<(Value, Value)> = vec![
            (Value::number(15), Value::number(15)),
            (Value::number(-15), Value::number(15)),
            (Value::number(0), Value::number(0)),
            (Value::positive_infinity(), Value::positive_infinity()),
            (Value::negative_infinity(), Value::positive_infinity()),
            (
                Value::unknown(Type::number()),
                Value::unknown(Type::number()).refine_not_null(),
            ),
            (
                Value::dynamic(),
                Value::unknown(Type::number()).refine_not_null(),
            ),
        ];

        for (i, (input, want)) in tests.iter().enumerate() {
            let got = stdlib::absolute(input)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
