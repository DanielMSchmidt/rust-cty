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

use cty::{Type, Value, ValueMarks};

// Ported from TestValueAdd:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L1768
#[test]
fn value_add() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(1),
            Value::number_int(2),
            Value::number_int(3),
        ),
        (
            Value::number_int(1),
            Value::number_int(-2),
            Value::number_int(-1),
        ),
        (
            Value::number_int(1),
            Value::number_float(0.5),
            Value::number_float(1.5),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number()),
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::number_int(3), true)
                .new_value(),
        ),
        (
            Value::zero(),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::number_int(2), true)
                .new_value(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::number_int(4), true)
                .new_value(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(1), true)
                .number_range_upper_bound(Value::number_int(2), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::number_int(3), true)
                .new_value(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(1), true)
                .number_range_upper_bound(Value::number_int(2), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), false)
                .number_range_upper_bound(Value::number_int(3), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::number_int(3), true)
                .number_range_upper_bound(Value::number_int(5), true)
                .new_value(),
        ),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::number()),
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::number_int(1),
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
fn value_subtract() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(1),
            Value::number_int(2),
            Value::number_int(-1),
        ),
        (
            Value::number_int(1),
            Value::number_int(-2),
            Value::number_int(3),
        ),
        (
            Value::number_int(1),
            Value::number_float(0.5),
            Value::number_float(0.5),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number()),
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::number()),
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), true)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_upper_bound(Value::number_int(-1), true)
                .new_value(),
        ),
        (
            Value::zero(),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), true)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_upper_bound(Value::number_int(-2), true)
                .new_value(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), true)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), true)
                .new_value(),
            // We don't currently refine this case
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(1), true)
                .number_range_upper_bound(Value::number_int(2), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), true)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_upper_bound(Value::number_int(0), true)
                .new_value(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(1), true)
                .number_range_upper_bound(Value::number_int(2), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), false)
                .number_range_upper_bound(Value::number_int(3), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::number_int(-2), true)
                .number_range_upper_bound(Value::number_int(0), true)
                .new_value(),
        ),
        (
            Value::number_int(1),
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
fn value_negate() {
    let tests: Vec<(Value, Value)> = vec![
        (Value::number_int(1), Value::number_int(-1)),
        (Value::number_float(0.5), Value::number_float(-0.5)),
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
#[test]
fn value_multiply() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(4),
            Value::number_int(2),
            Value::number_int(8),
        ),
        (
            Value::number_int(1),
            Value::number_int(-2),
            Value::number_int(-2),
        ),
        (
            Value::number_int(5),
            Value::number_float(0.5),
            Value::number_float(2.5),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number()),
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::number()),
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::number_int(3),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::number_int(6), true)
                .new_value(),
        ),
        (Value::zero(), Value::unknown(Type::number()), Value::zero()),
        (Value::unknown(Type::number()), Value::zero(), Value::zero()),
        (
            Value::zero(),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), false)
                .new_value(),
            Value::zero(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(4), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::number_int(8), true)
                .new_value(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(3), true)
                .number_range_upper_bound(Value::number_int(4), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::number_int(6), true)
                .new_value(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(1), true)
                .number_range_upper_bound(Value::number_int(2), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), false)
                .number_range_upper_bound(Value::number_int(3), false)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::number_int(2), true)
                .number_range_upper_bound(Value::number_int(6), true)
                .new_value(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(1), true)
                .number_range_upper_bound(Value::number_int(2), false)
                .new_value(),
            Value::zero(),
            Value::zero(), // deduced by refinement
        ),
        (
            Value::number_int(1),
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
        (
            Value::parse_number("967323432120515089486873574508975134568969931547").unwrap(),
            Value::number_float(12345.0),
            Value::parse_number("11941607769527758779715454277313298036253933804947715").unwrap(),
        ),
        (
            Value::number_float(22337203685475.5),
            Value::number_float(22337203685475.5),
            Value::parse_number("498950668486420259929661100.2").unwrap(),
        ),
    ];

    for (i, (lhs, rhs, expected)) in tests.iter().enumerate() {
        let got = lhs.multiply(rhs);
        assert!(
            got.raw_equals(expected),
            "case {i}: {lhs:?}.multiply({rhs:?}): wrong result\ngot:  {got:?}\nwant: {expected:?}"
        );
    }
}

// Ported from TestValueDivide:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2211
#[test]
fn value_divide() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(10),
            Value::number_int(2),
            Value::number_int(5),
        ),
        (
            Value::number_int(1),
            Value::number_int(-2),
            Value::number_float(-0.5),
        ),
        (
            Value::number_int(5),
            Value::number_float(0.5),
            Value::number_int(10),
        ),
        (
            Value::number_int(5),
            Value::number_int(0),
            Value::positive_infinity(),
        ),
        (
            Value::number_int(-5),
            Value::number_int(0),
            Value::negative_infinity(),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number()),
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::number()),
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::number_int(1),
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
            Value::number_int(1),
            Value::zero().mark(1),
        ),
        (
            Value::zero(),
            Value::number_int(1).mark(2),
            Value::zero().mark(2),
        ),
        (
            Value::zero().mark(1),
            Value::number_int(1).mark(2),
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
fn value_modulo() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(10),
            Value::number_int(2),
            Value::number_int(0),
        ),
        (
            Value::number_int(-10),
            Value::number_int(2),
            Value::number_int(0),
        ),
        (
            Value::number_int(11),
            Value::number_int(2),
            Value::number_int(1),
        ),
        (
            Value::number_int(-11),
            Value::number_int(2),
            Value::number_int(-1),
        ),
        (
            Value::number_int(1),
            Value::number_int(-2),
            Value::number_float(1.0),
        ),
        (
            Value::number_int(5),
            Value::number_float(0.5),
            Value::number_int(0),
        ),
        (
            Value::number_int(5),
            Value::number_float(1.5),
            Value::number_float(0.5),
        ),
        (
            Value::number_int(5),
            Value::number_int(0),
            Value::number_int(5),
        ),
        (
            Value::number_int(-5),
            Value::number_int(0),
            Value::number_int(-5),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number()),
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::number()),
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::dynamic(),
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::dynamic(),
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::number_int(10).mark(1),
            Value::number_int(10),
            Value::zero().mark(1),
        ),
        (
            Value::number_int(10),
            Value::number_int(10).mark(2),
            Value::zero().mark(2),
        ),
        (
            Value::number_int(10).mark(1),
            Value::number_int(10).mark(2),
            Value::zero().with_marks([ValueMarks::from_marks([1, 2])]),
        ),
        (
            Value::parse_number("967323432120515089486873574508975134568969931547").unwrap(),
            Value::number_int(10),
            Value::number_int(7),
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
fn value_absolute() {
    let tests: Vec<(Value, Value)> = vec![
        (Value::number_int(1), Value::number_int(1)),
        (Value::number_int(-1), Value::number_int(1)),
        (Value::number_float(0.5), Value::number_float(0.5)),
        (Value::number_float(-0.5), Value::number_float(0.5)),
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
        (Value::number_int(-1).mark(1), Value::number_int(1).mark(1)),
    ];

    for (i, (receiver, expected)) in tests.iter().enumerate() {
        let got = receiver.absolute();
        assert!(
            got.raw_equals(expected),
            "case {i}: {receiver:?}.absolute(): wrong result\ngot:  {got:?}\nwant: {expected:?}"
        );
    }
}
