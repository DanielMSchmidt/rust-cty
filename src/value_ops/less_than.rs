//! `Value::less_than` (go-cty: `cty/value_ops.go`).

use crate::value::Value;

impl Value {
    /// Whether this number is less than another (go-cty: `Value.LessThan`).
    pub fn less_than(&self, other: &Value) -> Value {
        let _ = other;
        todo!()
    }
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/value_ops_test.go (TestValueNot, TestValueAnd, TestValueOr, TestLessThan, TestGreaterThan, TestLessThanOrEqualTo, TestGreaterThanOrEqualTo)
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::{Type, Value};

    // Ported from TestLessThan:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L3235
    #[test]
    #[ignore = "not yet implemented"]
    fn less_than() {
        let tests: Vec<(Value, Value, Value)> = vec![
            (Value::number(0), Value::number(1), Value::bool(true)),
            (Value::number(1), Value::number(0), Value::bool(false)),
            (Value::number(0), Value::number(0), Value::bool(false)),
            (Value::number(0.1), Value::number(0.2), Value::bool(true)),
            (Value::number(0.2), Value::number(0.1), Value::bool(false)),
            (Value::number(0), Value::number(0.2), Value::bool(true)),
            (Value::number(0.2), Value::number(0), Value::bool(false)),
            (Value::number(0.2), Value::number(0.2), Value::bool(false)),
            (
                Value::unknown(Type::number()),
                Value::unknown(Type::number()),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::number(1),
                Value::unknown(Type::number()),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::unknown(Type::number()),
                Value::number(1),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::unknown(Type::number())
                    .refine()
                    .number_range_upper_bound(Value::zero(), true)
                    .new_value(),
                Value::number(1),
                Value::bool(true), // Deduction from the refinement
            ),
            (
                Value::unknown(Type::number())
                    .refine()
                    .number_range_lower_bound(Value::number(2), true)
                    .new_value(),
                Value::number(1),
                Value::bool(false), // Deduction from the refinement
            ),
            (
                Value::dynamic(),
                Value::dynamic(),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::number(1),
                Value::dynamic(),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::dynamic(),
                Value::number(1),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::number(0).mark(1),
                Value::number(1),
                Value::bool(true).mark(1),
            ),
            (
                Value::number(0),
                Value::number(1).mark(1),
                Value::bool(true).mark(1),
            ),
            (
                Value::number(0).mark(1),
                Value::number(1).mark(1),
                Value::bool(true).mark(1),
            ),
        ];

        for (i, (receiver, other, expected)) in tests.iter().enumerate() {
            let got = receiver.less_than(other);
            assert_eq!(
                got, *expected,
                "case {i}: {receiver:?}.less_than({other:?}) returned {got:?}; want {expected:?}"
            );
        }
    }
}
