//! `Value::divide` (go-cty: `cty/value_ops.go`).

use crate::value::Value;

impl Value {
    /// Quotient of two numbers (go-cty: `Value.Divide`). Division by zero
    /// yields infinity, as in go-cty.
    pub fn divide(&self, other: &Value) -> Value {
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

    // Ported from TestValueDivide:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2211
    #[test]
    #[ignore = "not yet implemented"]
    fn value_divide() {
        let tests: Vec<(Value, Value, Value)> = vec![
            (Value::number(10), Value::number(2), Value::number(5)),
            (Value::number(1), Value::number(-2), Value::number(-0.5)),
            (Value::number(5), Value::number(0.5), Value::number(10)),
            (
                Value::number(5),
                Value::number(0),
                Value::positive_infinity(),
            ),
            (
                Value::number(-5),
                Value::number(0),
                Value::negative_infinity(),
            ),
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
                Value::zero().mark(1),
                Value::number(1),
                Value::zero().mark(1),
            ),
            (
                Value::zero(),
                Value::number(1).mark(2),
                Value::zero().mark(2),
            ),
            (
                Value::zero().mark(1),
                Value::number(1).mark(2),
                Value::zero().with_marks([ValueMarks::from_marks([1, 2])]),
            ),
        ];

        for (i, (lhs, rhs, expected)) in tests.iter().enumerate() {
            let got = lhs.divide(rhs);
            assert!(
                got.raw_equals(expected),
                "case {i}: {lhs:?}.divide({rhs:?}): wrong result\ngot:  {got:?}\nwant: {expected:?}"
            );
        }
    }
}
