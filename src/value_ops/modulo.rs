//! `Value::modulo` (go-cty: `cty/value_ops.go`).

use crate::value::Value;

impl Value {
    /// Remainder of division, with the sign behavior of Go's
    /// `big.Float`-based implementation (go-cty: `Value.Modulo`).
    pub fn modulo(&self, other: &Value) -> Value {
        let _ = other;
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

    use crate::{Type, Value, ValueMarks};

    // Ported from TestValueModulo:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2289
    #[test]
    #[ignore = "not yet implemented"]
    fn value_modulo() {
        // NOTE(deviation): cannot pass. Numbers are `f64` (deviation 1 in the
        // workbench's docs/deviations.md); upstream numbers are arbitrary-precision.
        // Out of range here: the 48-digit operand below.
        // Kept as transcribed: the assertions state upstream behavior, which is the
        // point. Do not trim the table or relax them to make this green.
        let tests: Vec<(Value, Value, Value)> = vec![
            (Value::number(10), Value::number(2), Value::number(0)),
            (Value::number(-10), Value::number(2), Value::number(0)),
            (Value::number(11), Value::number(2), Value::number(1)),
            (Value::number(-11), Value::number(2), Value::number(-1)),
            (Value::number(1), Value::number(-2), Value::number(1.0)),
            (Value::number(5), Value::number(0.5), Value::number(0)),
            (Value::number(5), Value::number(1.5), Value::number(0.5)),
            (Value::number(5), Value::number(0), Value::number(5)),
            (Value::number(-5), Value::number(0), Value::number(-5)),
            (
                Value::number(1),
                Value::unknown(Type::number()),
                Value::unknown(Type::number()).refine_not_null(),
            ),
            (
                Value::unknown(Type::number()),
                Value::unknown(Type::number()),
                Value::unknown(Type::number()).refine_not_null(),
            ),
            (
                Value::number(1),
                Value::dynamic(),
                Value::unknown(Type::number()).refine_not_null(),
            ),
            (
                Value::dynamic(),
                Value::dynamic(),
                Value::unknown(Type::number()).refine_not_null(),
            ),
            (
                Value::number(10).mark(1),
                Value::number(10),
                Value::zero().mark(1),
            ),
            (
                Value::number(10),
                Value::number(10).mark(2),
                Value::zero().mark(2),
            ),
            (
                Value::number(10).mark(1),
                Value::number(10).mark(2),
                Value::zero().with_marks([ValueMarks::from_marks([1, 2])]),
            ),
            (
                Value::parse_number("967323432120515089486873574508975134568969931547"),
                Value::number(10),
                Value::number(7),
            ),
        ];

        for (i, (lhs, rhs, expected)) in tests.iter().enumerate() {
            let got = lhs.modulo(rhs);
            assert!(
                got.raw_equals(expected),
                "case {i}: {lhs:?}.modulo({rhs:?}) returned {got:?}; want {expected:?}"
            );
        }
    }
}
