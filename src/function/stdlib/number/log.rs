//! `log` (go-cty: `cty/function/stdlib/number.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`log`] (go-cty: `stdlib.LogFunc`).
pub fn log_func() -> Function {
    todo!()
}

/// The logarithm of `num` in base `base` (go-cty: `stdlib.Log`).
pub fn log(num: &Value, base: &Value) -> Result<Value, CtyError> {
    let _ = (num, base);
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

    // Ported from TestLog:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L915
    #[test]
    #[ignore = "not yet implemented"]
    fn log() {
        let tests: Vec<(Value, Value, Value, bool)> = vec![
            (
                Value::number(1.0),
                Value::number(10.0),
                Value::number(0.0),
                false,
            ),
            (
                Value::number(10.0),
                Value::number(10.0),
                Value::number(1.0),
                false,
            ),
            (
                Value::number(0.0),
                Value::number(10.0),
                Value::negative_infinity(),
                false,
            ),
            (
                Value::number(10.0),
                Value::number(0.0),
                // NOTE(port): upstream writes `cty.NumberFloatVal(-0)`; Go's
                // untyped constant `-0` is exactly zero, so this is `0.0` rather
                // than Rust's distinct negative-zero literal `-0.0`.
                Value::number(0.0),
                false,
            ),
        ];

        for (i, (num, base, want, want_err)) in tests.iter().enumerate() {
            let result = stdlib::log(num, base);
            if *want_err {
                assert!(result.is_err(), "case {i}: succeeded; want error");
                continue;
            }
            let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
