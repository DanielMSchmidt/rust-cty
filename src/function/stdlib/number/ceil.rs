//! `ceil` (go-cty: `cty/function/stdlib/number.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`ceil`] (go-cty: `stdlib.CeilFunc`).
pub fn ceil_func() -> Function {
    todo!()
}

/// The smallest integer greater than or equal to the number
/// (go-cty: `stdlib.Ceil`).
pub fn ceil(num: &Value) -> Result<Value, CtyError> {
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

    use crate::Value;
    use crate::function::stdlib;

    // Ported from TestCeil:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L799
    #[test]
    #[ignore = "not yet implemented"]
    fn ceil() {
        // NOTE(deviation): cannot pass. Numbers are `f64` (deviation 1 in the
        // workbench's docs/deviations.md); upstream numbers are arbitrary-precision.
        // Out of range here: the 53- and 56-digit values below.
        // Kept as transcribed: the assertions state upstream behavior, which is the
        // point. Do not trim the table or relax them to make this green.
        let tests: Vec<(Value, Value, bool)> = vec![
            (Value::number(-1.8), Value::number(-1.0), false),
            (Value::number(1.2), Value::number(2.0), false),
            (
                Value::number(f64::INFINITY),
                Value::number(f64::INFINITY),
                false,
            ),
            (
                Value::number(f64::NEG_INFINITY),
                Value::number(f64::NEG_INFINITY),
                false,
            ),
            (
                Value::parse_number("99999999999999999999999999999999999999999999999999998.123"),
                Value::parse_number("99999999999999999999999999999999999999999999999999999"),
                false,
            ),
            (
                Value::parse_number("-99999999999999999999999999999999999999999999999999998.123"),
                Value::parse_number("-99999999999999999999999999999999999999999999999999998"),
                false,
            ),
        ];

        for (i, (num, want, want_err)) in tests.iter().enumerate() {
            let result = stdlib::ceil(num);
            if *want_err {
                assert!(result.is_err(), "case {i}: succeeded; want error");
                continue;
            }
            let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
