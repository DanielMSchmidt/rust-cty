//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/value_ops_test.go (TestValueNot, TestValueAnd, TestValueOr, TestLessThan, TestGreaterThan, TestLessThanOrEqualTo, TestGreaterThanOrEqualTo)
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Type, Value};

// upstream: cty/value_ops_test.go TestValueNot
#[test]
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

// upstream: cty/value_ops_test.go TestValueAnd
#[test]
fn value_and() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (Value::bool(false), Value::bool(false), Value::bool(false)),
        (Value::bool(false), Value::bool(true), Value::bool(false)),
        (Value::bool(true), Value::bool(false), Value::bool(false)),
        (Value::bool(true), Value::bool(true), Value::bool(true)),
        (
            Value::unknown(Type::bool()),
            Value::unknown(Type::bool()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::bool(true),
            Value::unknown(Type::bool()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::bool()),
            Value::bool(true),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::bool(false),
            Value::unknown(Type::bool()),
            Value::bool(false),
        ),
        (
            Value::unknown(Type::bool()),
            Value::bool(false),
            Value::bool(false),
        ),
        (
            Value::dynamic(),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::bool(true),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::bool(true),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (Value::bool(false), Value::dynamic(), Value::bool(false)),
        (Value::dynamic(), Value::bool(false), Value::bool(false)),
        (
            Value::bool(true).mark(1),
            Value::bool(true),
            Value::bool(true).mark(1),
        ),
        (
            Value::bool(true),
            Value::bool(true).mark(1),
            Value::bool(true).mark(1),
        ),
        (
            Value::bool(true).mark(1),
            Value::bool(true).mark(1),
            Value::bool(true).mark(1),
        ),
    ];

    for (i, (receiver, other, expected)) in tests.iter().enumerate() {
        let got = receiver.and(other);
        assert_eq!(
            got, *expected,
            "case {i}: {receiver:?}.and({other:?}) returned {got:?}; want {expected:?}"
        );
    }
}

// upstream: cty/value_ops_test.go TestValueOr
#[test]
fn value_or() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (Value::bool(false), Value::bool(false), Value::bool(false)),
        (Value::bool(false), Value::bool(true), Value::bool(true)),
        (Value::bool(true), Value::bool(false), Value::bool(true)),
        (Value::bool(true), Value::bool(true), Value::bool(true)),
        (
            Value::unknown(Type::bool()),
            Value::unknown(Type::bool()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::bool(true),
            Value::unknown(Type::bool()),
            Value::bool(true),
        ),
        (
            Value::unknown(Type::bool()),
            Value::bool(true),
            Value::bool(true),
        ),
        (
            Value::bool(false),
            Value::unknown(Type::bool()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::bool()),
            Value::bool(false),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (Value::bool(true), Value::dynamic(), Value::bool(true)),
        (Value::dynamic(), Value::bool(true), Value::bool(true)),
        (
            Value::bool(false),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::bool(false),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::bool(true).mark(1),
            Value::bool(false),
            Value::bool(true).mark(1),
        ),
        (
            Value::bool(true),
            Value::bool(false).mark(1),
            Value::bool(true).mark(1),
        ),
        (
            Value::bool(true).mark(1),
            Value::bool(false).mark(1),
            Value::bool(true).mark(1),
        ),
    ];

    for (i, (receiver, other, expected)) in tests.iter().enumerate() {
        let got = receiver.or(other);
        assert_eq!(
            got, *expected,
            "case {i}: {receiver:?}.or({other:?}) returned {got:?}; want {expected:?}"
        );
    }
}

// upstream: cty/value_ops_test.go TestLessThan
#[test]
fn less_than() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(0),
            Value::number_int(1),
            Value::bool(true),
        ),
        (
            Value::number_int(1),
            Value::number_int(0),
            Value::bool(false),
        ),
        (
            Value::number_int(0),
            Value::number_int(0),
            Value::bool(false),
        ),
        (
            Value::number_float(0.1),
            Value::number_float(0.2),
            Value::bool(true),
        ),
        (
            Value::number_float(0.2),
            Value::number_float(0.1),
            Value::bool(false),
        ),
        (
            Value::number_int(0),
            Value::number_float(0.2),
            Value::bool(true),
        ),
        (
            Value::number_float(0.2),
            Value::number_int(0),
            Value::bool(false),
        ),
        (
            Value::number_float(0.2),
            Value::number_float(0.2),
            Value::bool(false),
        ),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::number()),
            Value::number_int(1),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_upper_bound(Value::zero(), true)
                .new_value(),
            Value::number_int(1),
            Value::bool(true), // Deduction from the refinement
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), true)
                .new_value(),
            Value::number_int(1),
            Value::bool(false), // Deduction from the refinement
        ),
        (
            Value::dynamic(),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::number_int(1),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(0).mark(1),
            Value::number_int(1),
            Value::bool(true).mark(1),
        ),
        (
            Value::number_int(0),
            Value::number_int(1).mark(1),
            Value::bool(true).mark(1),
        ),
        (
            Value::number_int(0).mark(1),
            Value::number_int(1).mark(1),
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

// upstream: cty/value_ops_test.go TestGreaterThan
#[test]
fn greater_than() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(0),
            Value::number_int(1),
            Value::bool(false),
        ),
        (
            Value::number_int(1),
            Value::number_int(0),
            Value::bool(true),
        ),
        (
            Value::number_int(0),
            Value::number_int(0),
            Value::bool(false),
        ),
        (
            Value::number_float(0.1),
            Value::number_float(0.2),
            Value::bool(false),
        ),
        (
            Value::number_float(0.2),
            Value::number_float(0.1),
            Value::bool(true),
        ),
        (
            Value::number_int(0),
            Value::number_float(0.2),
            Value::bool(false),
        ),
        (
            Value::number_float(0.2),
            Value::number_int(0),
            Value::bool(true),
        ),
        (
            Value::number_float(0.2),
            Value::number_float(0.2),
            Value::bool(false),
        ),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::number()),
            Value::number_int(1),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), true)
                .new_value(),
            Value::number_int(1),
            Value::bool(true), // Deduction based on the refinements
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_upper_bound(Value::number_int(0), true)
                .new_value(),
            Value::number_int(1),
            Value::bool(false), // Deduction based on the refinements
        ),
        (
            Value::dynamic(),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::number_int(1),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(1).mark(1),
            Value::number_int(0),
            Value::bool(true).mark(1),
        ),
        (
            Value::number_int(1),
            Value::number_int(0).mark(1),
            Value::bool(true).mark(1),
        ),
        (
            Value::number_int(1).mark(1),
            Value::number_int(0).mark(1),
            Value::bool(true).mark(1),
        ),
    ];

    for (i, (receiver, other, expected)) in tests.iter().enumerate() {
        let got = receiver.greater_than(other);
        assert_eq!(
            got, *expected,
            "case {i}: {receiver:?}.greater_than({other:?}) returned {got:?}; want {expected:?}"
        );
    }
}

// upstream: cty/value_ops_test.go TestLessThanOrEqualTo
#[test]
fn less_than_or_equal_to() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(0),
            Value::number_int(1),
            Value::bool(true),
        ),
        (
            Value::number_int(1),
            Value::number_int(0),
            Value::bool(false),
        ),
        (
            Value::number_int(0),
            Value::number_int(0),
            Value::bool(true),
        ),
        (
            Value::number_float(0.1),
            Value::number_float(0.2),
            Value::bool(true),
        ),
        (
            Value::number_float(0.2),
            Value::number_float(0.1),
            Value::bool(false),
        ),
        (
            Value::number_int(0),
            Value::number_float(0.2),
            Value::bool(true),
        ),
        (
            Value::number_float(0.2),
            Value::number_int(0),
            Value::bool(false),
        ),
        (
            Value::number_float(0.2),
            Value::number_float(0.2),
            Value::bool(true),
        ),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::number()),
            Value::number_int(1),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::number_int(1),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(0).mark(1),
            Value::number_int(1),
            Value::bool(true).mark(1),
        ),
        (
            Value::number_int(0),
            Value::number_int(1).mark(1),
            Value::bool(true).mark(1),
        ),
        (
            Value::number_int(0).mark(1),
            Value::number_int(1).mark(1),
            Value::bool(true).mark(1),
        ),
    ];

    for (i, (receiver, other, expected)) in tests.iter().enumerate() {
        let got = receiver.less_than_or_equal_to(other);
        assert_eq!(
            got, *expected,
            "case {i}: {receiver:?}.less_than_or_equal_to({other:?}) returned {got:?}; want {expected:?}"
        );
    }
}

// upstream: cty/value_ops_test.go TestGreaterThanOrEqualTo
#[test]
fn greater_than_or_equal_to() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(0),
            Value::number_int(1),
            Value::bool(false),
        ),
        (
            Value::number_int(1),
            Value::number_int(0),
            Value::bool(true),
        ),
        (
            Value::number_int(0),
            Value::number_int(0),
            Value::bool(true),
        ),
        (
            Value::number_float(0.1),
            Value::number_float(0.2),
            Value::bool(false),
        ),
        (
            Value::number_float(0.2),
            Value::number_float(0.1),
            Value::bool(true),
        ),
        (
            Value::number_int(0),
            Value::number_float(0.2),
            Value::bool(false),
        ),
        (
            Value::number_float(0.2),
            Value::number_int(0),
            Value::bool(true),
        ),
        (
            Value::number_float(0.2),
            Value::number_float(0.2),
            Value::bool(true),
        ),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::number()),
            Value::number_int(1),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::number_int(1),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(0).mark(1),
            Value::number_int(1),
            Value::bool(false).mark(1),
        ),
        (
            Value::number_int(0),
            Value::number_int(1).mark(1),
            Value::bool(false).mark(1),
        ),
        (
            Value::number_int(0).mark(1),
            Value::number_int(1).mark(1),
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
