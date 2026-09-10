//! `Value::add` (go-cty: `cty/value_ops.go`).

use crate::value::Value;

impl Value {
    /// Sum of two numbers (go-cty: `Value.Add`).
    pub fn add(&self, other: &Value) -> Value {
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

    // Ported from TestValueAdd:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L1768
    #[test]
    #[ignore = "not yet implemented"]
    fn value_add() {
        let tests: Vec<(Value, Value, Value)> = vec![
            (Value::number(1), Value::number(2), Value::number(3)),
            (Value::number(1), Value::number(-2), Value::number(-1)),
            (Value::number(1), Value::number(0.5), Value::number(1.5)),
            (
                Value::number(1),
                Value::unknown(Type::number()),
                Value::unknown(Type::number()).refine_not_null(),
            ),
            (
                Value::number(1),
                Value::unknown(Type::number())
                    .refine()
                    .number_range_lower_bound(Value::number(2), false)
                    .new_value(),
                Value::unknown(Type::number())
                    .refine()
                    .not_null()
                    .number_range_lower_bound(Value::number(3), true)
                    .new_value(),
            ),
            (
                Value::zero(),
                Value::unknown(Type::number())
                    .refine()
                    .number_range_lower_bound(Value::number(2), false)
                    .new_value(),
                Value::unknown(Type::number())
                    .refine()
                    .not_null()
                    .number_range_lower_bound(Value::number(2), true)
                    .new_value(),
            ),
            (
                Value::unknown(Type::number())
                    .refine()
                    .number_range_lower_bound(Value::number(2), false)
                    .new_value(),
                Value::unknown(Type::number())
                    .refine()
                    .number_range_lower_bound(Value::number(2), false)
                    .new_value(),
                Value::unknown(Type::number())
                    .refine()
                    .not_null()
                    .number_range_lower_bound(Value::number(4), true)
                    .new_value(),
            ),
            (
                Value::unknown(Type::number())
                    .refine()
                    .number_range_lower_bound(Value::number(1), true)
                    .number_range_upper_bound(Value::number(2), false)
                    .new_value(),
                Value::unknown(Type::number())
                    .refine()
                    .number_range_lower_bound(Value::number(2), false)
                    .new_value(),
                Value::unknown(Type::number())
                    .refine()
                    .not_null()
                    .number_range_lower_bound(Value::number(3), true)
                    .new_value(),
            ),
            (
                Value::unknown(Type::number())
                    .refine()
                    .number_range_lower_bound(Value::number(1), true)
                    .number_range_upper_bound(Value::number(2), false)
                    .new_value(),
                Value::unknown(Type::number())
                    .refine()
                    .number_range_lower_bound(Value::number(2), false)
                    .number_range_upper_bound(Value::number(3), false)
                    .new_value(),
                Value::unknown(Type::number())
                    .refine()
                    .not_null()
                    .number_range_lower_bound(Value::number(3), true)
                    .number_range_upper_bound(Value::number(5), true)
                    .new_value(),
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
            (Value::zero().mark(1), Value::zero(), Value::zero().mark(1)),
            (Value::zero(), Value::zero().mark(2), Value::zero().mark(2)),
            (
                Value::zero().mark(1),
                Value::zero().mark(2),
                Value::zero().with_marks([ValueMarks::from_marks([1, 2])]),
            ),
        ];

        for (i, (lhs, rhs, expected)) in tests.iter().enumerate() {
            let got = lhs.add(rhs);
            assert!(
                got.raw_equals(expected),
                "case {i}: {lhs:?}.add({rhs:?}): wrong result\ngot:  {got:?}\nwant: {expected:?}"
            );
        }
    }
}
