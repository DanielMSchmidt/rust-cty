//! `max` (go-cty: `cty/function/stdlib/number.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`max`] (go-cty: `stdlib.MaxFunc`).
pub fn max_func() -> Function {
    todo!()
}

/// The largest of the given numbers (go-cty: `stdlib.Max`).
pub fn max(numbers: &[Value]) -> Result<Value, CtyError> {
    let _ = numbers;
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

    // Ported from TestMax:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L675
    #[test]
    #[ignore = "not yet implemented"]
    fn max() {
        let tests: Vec<(Vec<Value>, Value)> = vec![
            (vec![Value::number(0)], Value::number(0)),
            (vec![Value::number(-12)], Value::number(-12)),
            (vec![Value::number(12)], Value::number(12)),
            (
                vec![Value::number(-12), Value::number(0), Value::number(2)],
                Value::number(2),
            ),
            (
                vec![Value::negative_infinity(), Value::number(0)],
                Value::number(0),
            ),
            (
                vec![Value::positive_infinity(), Value::number(0)],
                Value::positive_infinity(),
            ),
            (vec![Value::negative_infinity()], Value::negative_infinity()),
            (
                vec![Value::positive_infinity(), Value::unknown(Type::number())],
                Value::unknown(Type::number()).refine_not_null(),
            ),
            (
                vec![Value::positive_infinity(), Value::dynamic()],
                Value::unknown(Type::number()).refine_not_null(),
            ),
        ];

        for (i, (inputs, want)) in tests.iter().enumerate() {
            let got = stdlib::max(inputs)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
