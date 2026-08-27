//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/function/stdlib/number_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::function::stdlib;
use cty::{Type, Value};

// upstream: cty/function/stdlib/number_test.go TestAbsolute
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

// upstream: cty/function/stdlib/number_test.go TestAdd
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

// upstream: cty/function/stdlib/number_test.go TestSubtract
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

// upstream: cty/function/stdlib/number_test.go TestMultiply
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

// upstream: cty/function/stdlib/number_test.go TestDivide
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

// upstream: cty/function/stdlib/number_test.go TestModulo
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

// upstream: cty/function/stdlib/number_test.go TestNegate
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

// upstream: cty/function/stdlib/number_test.go TestLessThan
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

// upstream: cty/function/stdlib/number_test.go TestLessThanOrEqualTo
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

// upstream: cty/function/stdlib/number_test.go TestGreaterThan
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

// upstream: cty/function/stdlib/number_test.go TestGreaterThanOrEqualTo
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

// upstream: cty/function/stdlib/number_test.go TestMin
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

// upstream: cty/function/stdlib/number_test.go TestMax
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

// upstream: cty/function/stdlib/number_test.go TestInt
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

// upstream: cty/function/stdlib/number_test.go TestCeil
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

// upstream: cty/function/stdlib/number_test.go TestFloor
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

// upstream: cty/function/stdlib/number_test.go TestLog
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

// upstream: cty/function/stdlib/number_test.go TestPow
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

// upstream: cty/function/stdlib/number_test.go TestSignum
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

// upstream: cty/function/stdlib/number_test.go TestParseInt
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
