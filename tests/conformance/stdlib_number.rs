//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/function/stdlib/number_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::function::stdlib;
use cty::{Type, Value};

// Ported from TestAbsolute:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L12
#[test]
fn absolute() {
    let tests: Vec<(Value, Value)> = vec![
        (Value::number_int(15), Value::number_int(15)),
        (Value::number_int(-15), Value::number_int(15)),
        (Value::number_int(0), Value::number_int(0)),
        (Value::positive_infinity(), Value::positive_infinity()),
        (Value::negative_infinity(), Value::positive_infinity()),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::unknown(Type::number()).refine_not_null(),
        ),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got = stdlib::absolute(input)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestAdd:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L62
#[test]
fn add() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(1),
            Value::number_int(2),
            Value::number_int(3),
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
    ];

    for (i, (a, b, want)) in tests.iter().enumerate() {
        let got =
            stdlib::add(a, b).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestSubtract:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L110
#[test]
fn subtract() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(1),
            Value::number_int(2),
            Value::number_int(-1),
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
    ];

    for (i, (a, b, want)) in tests.iter().enumerate() {
        let got = stdlib::subtract(a, b)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestMultiply:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L158
#[test]
fn multiply() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(5),
            Value::number_int(2),
            Value::number_int(10),
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
    ];

    for (i, (a, b, want)) in tests.iter().enumerate() {
        let got = stdlib::multiply(a, b)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestDivide:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L206
#[test]
fn divide() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(5),
            Value::number_int(2),
            Value::number_float(2.5),
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
            Value::positive_infinity(),
            Value::zero(),
        ),
        (
            Value::number_int(1),
            Value::negative_infinity(),
            Value::zero(),
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
    ];

    for (i, (a, b, want)) in tests.iter().enumerate() {
        let got =
            stdlib::divide(a, b).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestModulo:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L274
#[test]
fn modulo() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(15),
            Value::number_int(10),
            Value::number_int(5),
        ),
        (
            Value::number_int(0),
            Value::number_int(0),
            Value::number_int(0),
        ),
        (
            Value::positive_infinity(),
            Value::number_int(1),
            Value::positive_infinity(),
        ),
        (
            Value::negative_infinity(),
            Value::number_int(1),
            Value::negative_infinity(),
        ),
        (
            Value::number_int(1),
            Value::positive_infinity(),
            Value::positive_infinity(),
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
    ];

    for (i, (a, b, want)) in tests.iter().enumerate() {
        let got =
            stdlib::modulo(a, b).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestNegate:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L342
#[test]
fn negate() {
    let tests: Vec<(Value, Value)> = vec![
        (Value::number_int(15), Value::number_int(-15)),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::unknown(Type::number()).refine_not_null(),
        ),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got =
            stdlib::negate(input).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestLessThan:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L376
#[test]
fn less_than() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(1),
            Value::number_int(2),
            Value::bool(true),
        ),
        (
            Value::number_int(2),
            Value::number_int(1),
            Value::bool(false),
        ),
        (
            Value::number_int(2),
            Value::number_int(2),
            Value::bool(false),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(2), true)
                .new_value(),
            Value::bool(true), // deduced from refinement
        ),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
    ];

    for (i, (a, b, want)) in tests.iter().enumerate() {
        let got = stdlib::less_than(a, b)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestLessThanOrEqualTo:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L439
#[test]
fn less_than_or_equal_to() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(1),
            Value::number_int(2),
            Value::bool(true),
        ),
        (
            Value::number_int(2),
            Value::number_int(1),
            Value::bool(false),
        ),
        (
            Value::number_int(2),
            Value::number_int(2),
            Value::bool(true),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
    ];

    for (i, (a, b, want)) in tests.iter().enumerate() {
        let got = stdlib::less_than_or_equal_to(a, b)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestGreaterThan:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L497
#[test]
fn greater_than() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(1),
            Value::number_int(2),
            Value::bool(false),
        ),
        (
            Value::number_int(2),
            Value::number_int(1),
            Value::bool(true),
        ),
        (
            Value::number_int(2),
            Value::number_int(2),
            Value::bool(false),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
    ];

    for (i, (a, b, want)) in tests.iter().enumerate() {
        let got = stdlib::greater_than(a, b)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestGreaterThanOrEqualTo:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L555
#[test]
fn greater_than_or_equal_to() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(1),
            Value::number_int(2),
            Value::bool(false),
        ),
        (
            Value::number_int(2),
            Value::number_int(1),
            Value::bool(true),
        ),
        (
            Value::number_int(2),
            Value::number_int(2),
            Value::bool(true),
        ),
        (
            Value::number_int(1),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::number_int(1),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
    ];

    for (i, (a, b, want)) in tests.iter().enumerate() {
        let got = stdlib::greater_than_or_equal_to(a, b)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestMin:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L613
#[test]
fn min() {
    let tests: Vec<(Vec<Value>, Value)> = vec![
        (vec![Value::number_int(0)], Value::number_int(0)),
        (vec![Value::number_int(-12)], Value::number_int(-12)),
        (vec![Value::number_int(12)], Value::number_int(12)),
        (
            vec![
                Value::number_int(-12),
                Value::number_int(0),
                Value::number_int(2),
            ],
            Value::number_int(-12),
        ),
        (
            vec![Value::negative_infinity(), Value::number_int(0)],
            Value::negative_infinity(),
        ),
        (
            vec![Value::positive_infinity(), Value::number_int(0)],
            Value::number_int(0),
        ),
        (vec![Value::negative_infinity()], Value::negative_infinity()),
        (
            vec![Value::positive_infinity(), Value::unknown(Type::number())],
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            vec![Value::positive_infinity(), Value::dynamic()],
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            vec![Value::zero().mark(1_i64), Value::number_int(1)],
            Value::zero().mark(1_i64),
        ),
    ];

    for (i, (inputs, want)) in tests.iter().enumerate() {
        let got =
            stdlib::min(inputs).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestMax:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L675
#[test]
fn max() {
    let tests: Vec<(Vec<Value>, Value)> = vec![
        (vec![Value::number_int(0)], Value::number_int(0)),
        (vec![Value::number_int(-12)], Value::number_int(-12)),
        (vec![Value::number_int(12)], Value::number_int(12)),
        (
            vec![
                Value::number_int(-12),
                Value::number_int(0),
                Value::number_int(2),
            ],
            Value::number_int(2),
        ),
        (
            vec![Value::negative_infinity(), Value::number_int(0)],
            Value::number_int(0),
        ),
        (
            vec![Value::positive_infinity(), Value::number_int(0)],
            Value::positive_infinity(),
        ),
        (vec![Value::negative_infinity()], Value::negative_infinity()),
        (
            vec![Value::positive_infinity(), Value::unknown(Type::number())],
            Value::unknown(Type::number()).refine_not_null(),
        ),
        (
            vec![Value::positive_infinity(), Value::dynamic()],
            Value::unknown(Type::number()).refine_not_null(),
        ),
    ];

    for (i, (inputs, want)) in tests.iter().enumerate() {
        let got =
            stdlib::max(inputs).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestInt:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L733
#[test]
fn int() {
    // NOTE(port): upstream builds the two big-precision cases with
    // `cty.NumberVal(mustParseFloat("…"))`; the same values are expressed
    // here via `Value::parse_number`.
    let tests: Vec<(Value, Value)> = vec![
        (Value::number_int(0), Value::number_int(0)),
        (Value::number_int(1), Value::number_int(1)),
        (Value::number_int(-1), Value::number_int(-1)),
        (Value::number_float(1.3), Value::number_int(1)),
        (Value::number_float(-1.7), Value::number_int(-1)),
        (Value::number_float(-1.3), Value::number_int(-1)),
        (Value::number_float(-1.7), Value::number_int(-1)),
        (
            Value::parse_number("999999999999999999999999999999999999999999999999999999999999.7")
                .unwrap(),
            Value::parse_number("999999999999999999999999999999999999999999999999999999999999")
                .unwrap(),
        ),
        (
            Value::parse_number("-999999999999999999999999999999999999999999999999999999999999.7")
                .unwrap(),
            Value::parse_number("-999999999999999999999999999999999999999999999999999999999999")
                .unwrap(),
        ),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got =
            stdlib::int(input).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestCeil:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L799
#[test]
fn ceil() {
    let tests: Vec<(Value, Value, bool)> = vec![
        (Value::number_float(-1.8), Value::number_float(-1.0), false),
        (Value::number_float(1.2), Value::number_float(2.0), false),
        (
            Value::number_float(f64::INFINITY),
            Value::number_float(f64::INFINITY),
            false,
        ),
        (
            Value::number_float(f64::NEG_INFINITY),
            Value::number_float(f64::NEG_INFINITY),
            false,
        ),
        (
            Value::parse_number("99999999999999999999999999999999999999999999999999998.123")
                .unwrap(),
            Value::parse_number("99999999999999999999999999999999999999999999999999999").unwrap(),
            false,
        ),
        (
            Value::parse_number("-99999999999999999999999999999999999999999999999999998.123")
                .unwrap(),
            Value::parse_number("-99999999999999999999999999999999999999999999999999998").unwrap(),
            false,
        ),
    ];

    for (i, (num, want, want_err)) in tests.iter().enumerate() {
        let result = stdlib::ceil(num);
        if *want_err {
            assert!(result.is_err(), "case {i}: succeeded; want error");
            continue;
        }
        let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestFloor:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L857
#[test]
fn floor() {
    let tests: Vec<(Value, Value, bool)> = vec![
        (Value::number_float(-1.8), Value::number_float(-2.0), false),
        (Value::number_float(1.2), Value::number_float(1.0), false),
        (
            Value::number_float(f64::INFINITY),
            Value::number_float(f64::INFINITY),
            false,
        ),
        (
            Value::number_float(f64::NEG_INFINITY),
            Value::number_float(f64::NEG_INFINITY),
            false,
        ),
        (
            Value::parse_number("99999999999999999999999999999999999999999999999999999.123")
                .unwrap(),
            Value::parse_number("99999999999999999999999999999999999999999999999999999").unwrap(),
            false,
        ),
        (
            Value::parse_number("-99999999999999999999999999999999999999999999999999998.123")
                .unwrap(),
            Value::parse_number("-99999999999999999999999999999999999999999999999999999").unwrap(),
            false,
        ),
    ];

    for (i, (num, want, want_err)) in tests.iter().enumerate() {
        let result = stdlib::floor(num);
        if *want_err {
            assert!(result.is_err(), "case {i}: succeeded; want error");
            continue;
        }
        let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestLog:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L915
#[test]
fn log() {
    let tests: Vec<(Value, Value, Value, bool)> = vec![
        (
            Value::number_float(1.0),
            Value::number_float(10.0),
            Value::number_float(0.0),
            false,
        ),
        (
            Value::number_float(10.0),
            Value::number_float(10.0),
            Value::number_float(1.0),
            false,
        ),
        (
            Value::number_float(0.0),
            Value::number_float(10.0),
            Value::negative_infinity(),
            false,
        ),
        (
            Value::number_float(10.0),
            Value::number_float(0.0),
            // NOTE(port): upstream writes `cty.NumberFloatVal(-0)`; Go's
            // untyped constant `-0` is exactly zero, so this is `0.0` rather
            // than Rust's distinct negative-zero literal `-0.0`.
            Value::number_float(0.0),
            false,
        ),
    ];

    for (i, (num, base, want, want_err)) in tests.iter().enumerate() {
        let result = stdlib::log(num, base);
        if *want_err {
            assert!(result.is_err(), "case {i}: succeeded; want error");
            continue;
        }
        let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestPow:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L969
#[test]
fn pow() {
    let tests: Vec<(Value, Value, Value, bool)> = vec![
        (
            Value::number_float(1.0),
            Value::number_float(0.0),
            Value::number_float(1.0),
            false,
        ),
        (
            Value::number_float(1.0),
            Value::number_float(1.0),
            Value::number_float(1.0),
            false,
        ),
        (
            Value::number_float(2.0),
            Value::number_float(0.0),
            Value::number_float(1.0),
            false,
        ),
        (
            Value::number_float(2.0),
            Value::number_float(1.0),
            Value::number_float(2.0),
            false,
        ),
        (
            Value::number_float(3.0),
            Value::number_float(2.0),
            Value::number_float(9.0),
            false,
        ),
        (
            Value::number_float(-3.0),
            Value::number_float(2.0),
            Value::number_float(9.0),
            false,
        ),
        (
            Value::number_float(2.0),
            Value::number_float(-2.0),
            Value::number_float(0.25),
            false,
        ),
        (
            Value::number_float(0.0),
            Value::number_float(2.0),
            Value::number_float(0.0),
            false,
        ),
    ];

    for (i, (num, power, want, want_err)) in tests.iter().enumerate() {
        let result = stdlib::pow(num, power);
        if *want_err {
            assert!(result.is_err(), "case {i}: succeeded; want error");
            continue;
        }
        let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestSignum:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L1047
#[test]
#[allow(clippy::approx_constant)] // upstream's literal really is 3.14
fn signum() {
    let tests: Vec<(Value, Value, bool)> = vec![
        (Value::number_float(0.0), Value::number_float(0.0), false),
        (Value::number_float(12.0), Value::number_float(1.0), false),
        (Value::number_float(-29.0), Value::number_float(-1.0), false),
        (Value::number_float(-9.2), Value::number_float(-1.0), false),
        (Value::number_float(3.14), Value::number_float(1.0), false),
        (Value::number_float(0.25), Value::number_float(1.0), false),
        (
            Value::number_float(f64::INFINITY),
            Value::number_float(1.0),
            false,
        ),
        (
            Value::number_float(f64::NEG_INFINITY),
            Value::number_float(-1.0),
            false,
        ),
        (
            Value::parse_number("2e+500").unwrap(),
            Value::number_float(1.0),
            false,
        ),
        (
            Value::parse_number("-2e+500").unwrap(),
            Value::number_float(-1.0),
            false,
        ),
    ];

    for (i, (num, want, want_err)) in tests.iter().enumerate() {
        let result = stdlib::signum(num);
        if *want_err {
            assert!(result.is_err(), "case {i}: succeeded; want error");
            continue;
        }
        let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestParseInt:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L1125
#[test]
fn parse_int() {
    let tests: Vec<(Value, Value, Value, bool)> = vec![
        (
            Value::string("128"),
            Value::number_int(10),
            Value::number_int(128),
            false,
        ),
        (
            Value::string("-128"),
            Value::number_int(10),
            Value::number_int(-128),
            false,
        ),
        (
            Value::string("00128"),
            Value::number_int(10),
            Value::number_int(128),
            false,
        ),
        (
            Value::string("-00128"),
            Value::number_int(10),
            Value::number_int(-128),
            false,
        ),
        (
            Value::string("FF00"),
            Value::number_int(16),
            Value::number_int(65280),
            false,
        ),
        (
            Value::string("ff00"),
            Value::number_int(16),
            Value::number_int(65280),
            false,
        ),
        (
            Value::string("-FF00"),
            Value::number_int(16),
            Value::number_int(-65280),
            false,
        ),
        (
            Value::string("00FF00"),
            Value::number_int(16),
            Value::number_int(65280),
            false,
        ),
        (
            Value::string("-00FF00"),
            Value::number_int(16),
            Value::number_int(-65280),
            false,
        ),
        (
            Value::string("1011111011101111"),
            Value::number_int(2),
            Value::number_int(48879),
            false,
        ),
        (
            Value::string("aA"),
            Value::number_int(62),
            Value::number_int(656),
            false,
        ),
        (
            Value::string("Aa"),
            Value::number_int(62),
            Value::number_int(2242),
            false,
        ),
        (
            Value::string("999999999999999999999999999999999999999999999999999999999999"),
            Value::number_int(10),
            Value::parse_number("999999999999999999999999999999999999999999999999999999999999")
                .unwrap(),
            false,
        ),
        (
            Value::string("FF"),
            Value::number_int(10),
            Value::unknown(Type::number()).refine_not_null(),
            true,
        ),
        (
            Value::string("00FF"),
            Value::number_int(10),
            Value::unknown(Type::number()).refine_not_null(),
            true,
        ),
        (
            Value::string("-00FF"),
            Value::number_int(10),
            Value::unknown(Type::number()).refine_not_null(),
            true,
        ),
        (
            Value::number_int(2),
            Value::number_int(10),
            Value::unknown(Type::number()).refine_not_null(),
            true,
        ),
        (
            Value::string("1"),
            Value::number_int(63),
            Value::unknown(Type::number()).refine_not_null(),
            true,
        ),
        (
            Value::string("1"),
            Value::number_int(-1),
            Value::unknown(Type::number()).refine_not_null(),
            true,
        ),
        (
            Value::string("1"),
            Value::number_int(1),
            Value::unknown(Type::number()).refine_not_null(),
            true,
        ),
        (
            Value::string("1"),
            Value::number_int(0),
            Value::unknown(Type::number()).refine_not_null(),
            true,
        ),
        (
            Value::string("1.2"),
            Value::number_int(10),
            Value::unknown(Type::number()).refine_not_null(),
            true,
        ),
    ];

    for (i, (num, base, want, want_err)) in tests.iter().enumerate() {
        let result = stdlib::parse_int(num, base);
        if *want_err {
            assert!(result.is_err(), "case {i}: succeeded; want error");
            continue;
        }
        let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}
