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

// Ported from TestValueSubtract:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L1896
#[test]
#[ignore = "not yet implemented"]
fn value_subtract() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (Value::number(1), Value::number(2), Value::number(-1)),
        (Value::number(1), Value::number(-2), Value::number(3)),
        (Value::number(1), Value::number(0.5), Value::number(0.5)),
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
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number(2), true)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_upper_bound(Value::number(-1), true)
                .new_value(),
        ),
        (
            Value::zero(),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number(2), true)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_upper_bound(Value::number(-2), true)
                .new_value(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number(2), true)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number(2), true)
                .new_value(),
            // We don't currently refine this case
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number(1), true)
                .number_range_upper_bound(Value::number(2), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number(2), true)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_upper_bound(Value::number(0), true)
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
                .number_range_lower_bound(Value::number(-2), true)
                .number_range_upper_bound(Value::number(0), true)
                .new_value(),
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
        let got = lhs.subtract(rhs);
        assert!(
            got.raw_equals(expected),
            "case {i}: {lhs:?}.subtract({rhs:?}): wrong result\ngot:  {got:?}\nwant: {expected:?}"
        );
    }
}

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
