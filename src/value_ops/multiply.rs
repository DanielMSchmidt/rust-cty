//! `Value::multiply` (go-cty: `cty/value_ops.go`).

use crate::value::Value;

impl Value {
    /// Product of two numbers (go-cty: `Value.Multiply`).
    pub fn multiply(&self, other: &Value) -> Value {
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

    // Ported from TestValueMultiply:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2058
    //
    // Upstream TestValueMultiply is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod value_multiply {
        use super::*;

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[(Value, Value, Value)]) {
            for (i, (lhs, rhs, expected)) in tests.iter().enumerate() {
                let got = lhs.multiply(rhs);
                assert!(
                    got.raw_equals(expected),
                    "case {i}: {lhs:?}.multiply({rhs:?}): wrong result\ngot:  {got:?}\nwant: {expected:?}"
                );
            }
        }

        // TestValueMultiply, primitives:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2058
        #[test]
        #[ignore = "not yet implemented"]
        fn primitives() {
            let tests: Vec<(Value, Value, Value)> = vec![
                (Value::number(4), Value::number(2), Value::number(8)),
                (Value::number(1), Value::number(-2), Value::number(-2)),
                (Value::number(5), Value::number(0.5), Value::number(2.5)),
                (
                    Value::parse_number("967323432120515089486873574508975134568969931547"),
                    Value::number(12345.0),
                    Value::parse_number("11941607769527758779715454277313298036253933804947715"),
                ),
                (
                    Value::number(22337203685475.5),
                    Value::number(22337203685475.5),
                    Value::parse_number("498950668486420259929661100.2"),
                ),
            ];

            check(&tests);
        }

        // TestValueMultiply, unknowns:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2058
        #[test]
        #[ignore = "not yet implemented"]
        fn unknowns() {
            let tests: Vec<(Value, Value, Value)> = vec![
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
                    Value::number(3),
                    Value::unknown(Type::number())
                        .refine()
                        .number_range_lower_bound(Value::number(2), false)
                        .new_value(),
                    Value::unknown(Type::number())
                        .refine()
                        .not_null()
                        .number_range_lower_bound(Value::number(6), true)
                        .new_value(),
                ),
                (Value::zero(), Value::unknown(Type::number()), Value::zero()),
                (Value::unknown(Type::number()), Value::zero(), Value::zero()),
                (
                    Value::zero(),
                    Value::unknown(Type::number())
                        .refine()
                        .number_range_lower_bound(Value::number(2), false)
                        .new_value(),
                    Value::zero(),
                ),
                (
                    Value::unknown(Type::number())
                        .refine()
                        .number_range_lower_bound(Value::number(2), false)
                        .new_value(),
                    Value::unknown(Type::number())
                        .refine()
                        .number_range_lower_bound(Value::number(4), false)
                        .new_value(),
                    Value::unknown(Type::number())
                        .refine()
                        .not_null()
                        .number_range_lower_bound(Value::number(8), true)
                        .new_value(),
                ),
                (
                    Value::unknown(Type::number())
                        .refine()
                        .number_range_lower_bound(Value::number(3), true)
                        .number_range_upper_bound(Value::number(4), false)
                        .new_value(),
                    Value::unknown(Type::number())
                        .refine()
                        .number_range_lower_bound(Value::number(2), false)
                        .new_value(),
                    Value::unknown(Type::number())
                        .refine()
                        .not_null()
                        .number_range_lower_bound(Value::number(6), true)
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
                        .number_range_lower_bound(Value::number(2), true)
                        .number_range_upper_bound(Value::number(6), true)
                        .new_value(),
                ),
                (
                    Value::unknown(Type::number())
                        .refine()
                        .number_range_lower_bound(Value::number(1), true)
                        .number_range_upper_bound(Value::number(2), false)
                        .new_value(),
                    Value::zero(),
                    Value::zero(), // deduced by refinement
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
            ];

            check(&tests);
        }

        // TestValueMultiply, marks:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2058
        #[test]
        #[ignore = "not yet implemented"]
        fn marks() {
            let tests: Vec<(Value, Value, Value)> = vec![
                (Value::zero().mark(1), Value::zero(), Value::zero().mark(1)),
                (Value::zero(), Value::zero().mark(2), Value::zero().mark(2)),
                (
                    Value::zero().mark(1),
                    Value::zero().mark(2),
                    Value::zero().with_marks([ValueMarks::from_marks([1, 2])]),
                ),
            ];

            check(&tests);
        }
    }
}
