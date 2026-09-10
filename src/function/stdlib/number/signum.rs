//! `signum` (go-cty: `cty/function/stdlib/number.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`signum`] (go-cty: `stdlib.SignumFunc`).
pub fn signum_func() -> Function {
    todo!()
}

/// The sign of the number as -1, 0, or 1 (go-cty: `stdlib.Signum`).
pub fn signum(num: &Value) -> Result<Value, CtyError> {
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

    // Ported from TestSignum:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L1047
    #[test]
    #[ignore = "not yet implemented"]
    #[allow(clippy::approx_constant)] // upstream's literal really is 3.14
    fn signum() {
        // NOTE(deviation): cannot pass. Numbers are `f64` (deviation 1 in the
        // workbench's docs/deviations.md); upstream numbers are arbitrary-precision.
        // Out of range here: ±2e+500 below, which overflows f64 to infinity.
        // Kept as transcribed: the assertions state upstream behavior, which is the
        // point. Do not trim the table or relax them to make this green.
        let tests: Vec<(Value, Value, bool)> = vec![
            (Value::number(0.0), Value::number(0.0), false),
            (Value::number(12.0), Value::number(1.0), false),
            (Value::number(-29.0), Value::number(-1.0), false),
            (Value::number(-9.2), Value::number(-1.0), false),
            (Value::number(3.14), Value::number(1.0), false),
            (Value::number(0.25), Value::number(1.0), false),
            (Value::number(f64::INFINITY), Value::number(1.0), false),
            (Value::number(f64::NEG_INFINITY), Value::number(-1.0), false),
            (Value::parse_number("2e+500"), Value::number(1.0), false),
            (Value::parse_number("-2e+500"), Value::number(-1.0), false),
        ];

        for (i, (num, want, want_err)) in tests.iter().enumerate() {
            let result = stdlib::signum(num);
            if *want_err {
                assert!(result.is_err(), "case {i}: succeeded; want error");
                continue;
            }
            let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
