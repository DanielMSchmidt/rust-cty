//! `Value::not` (go-cty: `cty/value_ops.go`).

use crate::value::Value;

impl Value {
    /// Logical NOT (go-cty: `Value.Not`).
    pub fn not(&self) -> Value {
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

    // Ported from TestValueNot:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2992
    #[test]
    #[ignore = "not yet implemented"]
    fn value_not() {
        let tests: Vec<(Value, Value)> = vec![
            (Value::bool(true), Value::bool(false)),
            (Value::bool(false), Value::bool(true)),
            (
                Value::unknown(Type::bool()),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::dynamic(),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (Value::bool(true).mark(1), Value::bool(false).mark(1)),
        ];

        for (i, (receiver, expected)) in tests.iter().enumerate() {
            let got = receiver.not();
            assert_eq!(
                got, *expected,
                "case {i}: {receiver:?}.not() returned {got:?}; want {expected:?}"
            );
        }
    }
}
