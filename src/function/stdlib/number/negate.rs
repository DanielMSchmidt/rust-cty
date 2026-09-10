//! `negate` (go-cty: `cty/function/stdlib/number.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`negate`] (go-cty: `stdlib.NegateFunc`).
pub fn negate_func() -> Function {
    todo!()
}

/// The additive inverse of a number (go-cty: `stdlib.Negate`).
pub fn negate(num: &Value) -> Result<Value, CtyError> {
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

    // Ported from TestNegate:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L342
    #[test]
    #[ignore = "not yet implemented"]
    fn negate() {
        let tests: Vec<(Value, Value)> = vec![
            (Value::number(15), Value::number(-15)),
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
            let got = stdlib::negate(input)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
