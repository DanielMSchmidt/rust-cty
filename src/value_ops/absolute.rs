//! `Value::absolute` (go-cty: `cty/value_ops.go`).

use crate::value::Value;

impl Value {
    /// Absolute value of a number (go-cty: `Value.Absolute`).
    pub fn absolute(&self) -> Value {
        todo!()
    }
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/value_ops_test.go
    //!     TestValueAdd
    //!     TestValueSubtract
    //!     TestValueNegate
    //!     TestValueMultiply
    //!     TestValueDivide
    //!     TestValueModulo
    //!     TestValueAbsolute
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::{Type, Value};

    // Ported from TestValueAbsolute:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2392
    #[test]
    #[ignore = "not yet implemented"]
    fn value_absolute() {
        let tests: Vec<(Value, Value)> = vec![
            (Value::number(1), Value::number(1)),
            (Value::number(-1), Value::number(1)),
            (Value::number(0.5), Value::number(0.5)),
            (Value::number(-0.5), Value::number(0.5)),
            (Value::positive_infinity(), Value::positive_infinity()),
            (Value::negative_infinity(), Value::positive_infinity()),
            (
                Value::unknown(Type::number()),
                Value::unknown(Type::number())
                    .refine()
                    .not_null()
                    .number_range_inclusive(Value::zero(), Value::unknown(Type::number()))
                    .new_value(),
            ),
            (
                Value::dynamic(),
                Value::unknown(Type::number())
                    .refine()
                    .not_null()
                    .number_range_inclusive(Value::zero(), Value::unknown(Type::number()))
                    .new_value(),
            ),
            (Value::number(-1).mark(1), Value::number(1).mark(1)),
        ];

        for (i, (receiver, expected)) in tests.iter().enumerate() {
            let got = receiver.absolute();
            assert!(
                got.raw_equals(expected),
                "case {i}: {receiver:?}.absolute(): wrong result\ngot:  {got:?}\nwant: {expected:?}"
            );
        }
    }
}
