//! `Value::negate` (go-cty: `cty/value_ops.go`).

use crate::value::Value;

impl Value {
    /// Additive inverse of a number (go-cty: `Value.Negate`).
    pub fn negate(&self) -> Value {
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

    // Ported from TestValueNegate:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2021
    #[test]
    #[ignore = "not yet implemented"]
    fn value_negate() {
        let tests: Vec<(Value, Value)> = vec![
            (Value::number(1), Value::number(-1)),
            (Value::number(0.5), Value::number(-0.5)),
            (
                Value::unknown(Type::number()),
                Value::unknown(Type::number()).refine_not_null(),
            ),
            (
                Value::dynamic(),
                Value::unknown(Type::number()).refine_not_null(),
            ),
            (Value::zero().mark(1), Value::zero().mark(1)),
        ];

        for (i, (receiver, expected)) in tests.iter().enumerate() {
            let got = receiver.negate();
            assert!(
                got.raw_equals(expected),
                "case {i}: {receiver:?}.negate(): wrong result\ngot:  {got:?}\nwant: {expected:?}"
            );
        }
    }
}
