//! `int` (go-cty: `cty/function/stdlib/number.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`int`] (go-cty: `stdlib.IntFunc`).
pub fn int_func() -> Function {
    todo!()
}

/// The integer part of a number, truncating toward zero (go-cty: `stdlib.Int`).
pub fn int(num: &Value) -> Result<Value, CtyError> {
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

    // Ported from TestInt:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L733
    #[test]
    #[ignore = "not yet implemented"]
    fn int() {
        // NOTE(deviation): cannot pass. Numbers are `f64` (deviation 1 in the
        // workbench's docs/deviations.md); upstream numbers are arbitrary-precision.
        // Out of range here: the 60- and 61-digit values below.
        // Kept as transcribed: the assertions state upstream behavior, which is the
        // point. Do not trim the table or relax them to make this green.
        // NOTE(port): upstream builds the two big-precision cases with
        // `cty.NumberVal(mustParseFloat("…"))`; the same values are expressed
        // here via `Value::parse_number`.
        let tests: Vec<(Value, Value)> = vec![
            (Value::number(0), Value::number(0)),
            (Value::number(1), Value::number(1)),
            (Value::number(-1), Value::number(-1)),
            (Value::number(1.3), Value::number(1)),
            (Value::number(-1.7), Value::number(-1)),
            (Value::number(-1.3), Value::number(-1)),
            (Value::number(-1.7), Value::number(-1)),
            (
                Value::parse_number(
                    "999999999999999999999999999999999999999999999999999999999999.7",
                ),
                Value::parse_number("999999999999999999999999999999999999999999999999999999999999"),
            ),
            (
                Value::parse_number(
                    "-999999999999999999999999999999999999999999999999999999999999.7",
                ),
                Value::parse_number(
                    "-999999999999999999999999999999999999999999999999999999999999",
                ),
            ),
        ];

        for (i, (input, want)) in tests.iter().enumerate() {
            let got = stdlib::int(input)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
