//! `Value::greater_than_or_equal_to` (go-cty: `cty/value_ops.go`).

use crate::value::Value;

impl Value {
    /// Whether this number is greater than or equal to another
    /// (go-cty: `Value.GreaterThanOrEqualTo`).
    pub fn greater_than_or_equal_to(&self, other: &Value) -> Value {
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

    // Ported from TestGreaterThanOrEqualTo:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L3564
    #[test]
    #[ignore = "not yet implemented"]
    fn greater_than_or_equal_to() {
        let tests: Vec<(Value, Value, Value)> = vec![
            (Value::number(0), Value::number(1), Value::bool(false)),
            (Value::number(1), Value::number(0), Value::bool(true)),
            (Value::number(0), Value::number(0), Value::bool(true)),
            (Value::number(0.1), Value::number(0.2), Value::bool(false)),
            (Value::number(0.2), Value::number(0.1), Value::bool(true)),
            (Value::number(0), Value::number(0.2), Value::bool(false)),
            (Value::number(0.2), Value::number(0), Value::bool(true)),
            (Value::number(0.2), Value::number(0.2), Value::bool(true)),
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
                Value::bool(false).mark(1),
            ),
            (
                Value::number(0),
                Value::number(1).mark(1),
                Value::bool(false).mark(1),
            ),
            (
                Value::number(0).mark(1),
                Value::number(1).mark(1),
                Value::bool(false).mark(1),
            ),
        ];

        for (i, (receiver, other, expected)) in tests.iter().enumerate() {
            let got = receiver.greater_than_or_equal_to(other);
            assert_eq!(
                got, *expected,
                "case {i}: {receiver:?}.greater_than_or_equal_to({other:?}) returned {got:?}; want {expected:?}"
            );
        }
    }
}
