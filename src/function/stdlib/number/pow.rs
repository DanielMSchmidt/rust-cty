//! `pow` (go-cty: `cty/function/stdlib/number.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`pow`] (go-cty: `stdlib.PowFunc`).
pub fn pow_func() -> Function {
    todo!()
}

/// `num` raised to the power `power` (go-cty: `stdlib.Pow`).
pub fn pow(num: &Value, power: &Value) -> Result<Value, CtyError> {
    let _ = (num, power);
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

    use crate::Value;
    use crate::function::stdlib;

    // Ported from TestPow:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L969
    #[test]
    #[ignore = "not yet implemented"]
    fn pow() {
        let tests: Vec<(Value, Value, Value, bool)> = vec![
            (
                Value::number(1.0),
                Value::number(0.0),
                Value::number(1.0),
                false,
            ),
            (
                Value::number(1.0),
                Value::number(1.0),
                Value::number(1.0),
                false,
            ),
            (
                Value::number(2.0),
                Value::number(0.0),
                Value::number(1.0),
                false,
            ),
            (
                Value::number(2.0),
                Value::number(1.0),
                Value::number(2.0),
                false,
            ),
            (
                Value::number(3.0),
                Value::number(2.0),
                Value::number(9.0),
                false,
            ),
            (
                Value::number(-3.0),
                Value::number(2.0),
                Value::number(9.0),
                false,
            ),
            (
                Value::number(2.0),
                Value::number(-2.0),
                Value::number(0.25),
                false,
            ),
            (
                Value::number(0.0),
                Value::number(2.0),
                Value::number(0.0),
                false,
            ),
        ];

        for (i, (num, power, want, want_err)) in tests.iter().enumerate() {
            let result = stdlib::pow(num, power);
            if *want_err {
                assert!(result.is_err(), "case {i}: succeeded; want error");
                continue;
            }
            let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
