//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/value_ops_test.go (TestValueGoString, TestHasWhollyKnownType, TestFloatCopy)
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Type, Value};

// Ported from TestValueGoString:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L3667
#[test]
fn value_go_string() {
    let tests: Vec<(Value, &str)> = vec![
        (
            Value::null(Type::dynamic()),
            r#"cty.NullVal(cty.DynamicPseudoType)"#,
        ),
        (Value::null(Type::string()), r#"cty.NullVal(cty.String)"#),
        (
            Value::null(Type::tuple([Type::string(), Type::bool()])),
            r#"cty.NullVal(cty.Tuple([]cty.Type{cty.String, cty.Bool}))"#,
        ),
        (Value::unknown(Type::dynamic()), r#"cty.DynamicVal"#),
        (
            Value::unknown(Type::string()),
            r#"cty.UnknownVal(cty.String)"#,
        ),
        (
            Value::unknown(Type::tuple([Type::string(), Type::bool()])),
            r#"cty.UnknownVal(cty.Tuple([]cty.Type{cty.String, cty.Bool}))"#,
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .not_null()
                .new_value(),
            r#"cty.UnknownVal(cty.String).RefineNotNull()"#,
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .not_null()
                .string_prefix("a-")
                .new_value(),
            r#"cty.UnknownVal(cty.String).Refine().NotNull().StringPrefixFull("a-").NewValue()"#,
        ),
        (
            // The last character of the prefix gets discarded in case the
            // next character is a combining diacritic
            Value::unknown(Type::string())
                .refine()
                .not_null()
                .string_prefix("foo")
                .new_value(),
            r#"cty.UnknownVal(cty.String).Refine().NotNull().StringPrefixFull("fo").NewValue()"#,
        ),
        (
            Value::unknown(Type::bool()).refine().not_null().new_value(),
            r#"cty.UnknownVal(cty.Bool).RefineNotNull()"#,
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_inclusive(Value::zero(), Value::unknown(Type::number()))
                .new_value(),
            r#"cty.UnknownVal(cty.Number).Refine().NumberLowerBound(cty.NumberIntVal(0), true).NewValue()"#,
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_inclusive(Value::zero(), Value::number_int(1))
                .new_value(),
            r#"cty.UnknownVal(cty.Number).Refine().NumberLowerBound(cty.NumberIntVal(0), true).NumberUpperBound(cty.NumberIntVal(1), true).NewValue()"#,
        ),
        (Value::string(""), r#"cty.StringVal("")"#),
        (Value::string("hello"), r#"cty.StringVal("hello")"#),
        (Value::zero(), r#"cty.NumberIntVal(0)"#),
        (Value::number_float(1.2), r#"cty.NumberFloatVal(1.2)"#),
        (
            // the "float-ness" of the input is lost because its value is a
            // whole number
            Value::number_float(1.0),
            r#"cty.NumberIntVal(1)"#,
        ),
        (
            Value::parse_number("3.14159265358979323846264338327950288419716939937510582097494459")
                .unwrap(),
            r#"cty.MustParseNumberVal("3.14159265358979323846264338327950288419716939937510582097494459")"#,
        ),
        (Value::bool(true), r#"cty.True"#),
        (Value::bool(false), r#"cty.False"#),
        (
            Value::list_empty(Type::string()),
            r#"cty.ListValEmpty(cty.String)"#,
        ),
        (
            Value::list_empty(Type::list(Type::string())),
            r#"cty.ListValEmpty(cty.List(cty.String))"#,
        ),
        (
            Value::list([Value::bool(true)]),
            r#"cty.ListVal([]cty.Value{cty.True})"#,
        ),
        (
            Value::set_empty(Type::string()),
            r#"cty.SetValEmpty(cty.String)"#,
        ),
        (
            Value::set_empty(Type::map(Type::string())),
            r#"cty.SetValEmpty(cty.Map(cty.String))"#,
        ),
        (
            Value::set([Value::bool(true)]),
            r#"cty.SetVal([]cty.Value{cty.True})"#,
        ),
        (Value::empty_tuple(), r#"cty.EmptyTupleVal"#),
        (Value::tuple([] as [Value; 0]), r#"cty.EmptyTupleVal"#),
        (
            Value::tuple([Value::bool(true)]),
            r#"cty.TupleVal([]cty.Value{cty.True})"#,
        ),
        (
            Value::map_empty(Type::string()),
            r#"cty.MapValEmpty(cty.String)"#,
        ),
        (
            Value::map_empty(Type::set(Type::string())),
            r#"cty.MapValEmpty(cty.Set(cty.String))"#,
        ),
        (
            Value::map([("boop", Value::bool(true))]),
            r#"cty.MapVal(map[string]cty.Value{"boop":cty.True})"#,
        ),
        (Value::empty_object(), r#"cty.EmptyObjectVal"#),
        (
            Value::object([] as [(&str, Value); 0]),
            r#"cty.EmptyObjectVal"#,
        ),
        (
            Value::object([("foo", Value::bool(true))]),
            r#"cty.ObjectVal(map[string]cty.Value{"foo":cty.True})"#,
        ),
    ];

    for (i, (value, want)) in tests.iter().enumerate() {
        let got = value.go_string();
        assert_eq!(got, *want, "case {i}: wrong go_string result");
    }
}

// Rust-syntax twin of value_go_string: the same table with the expectations
// translated into this crate's constructor syntax, pinning `Display`.
// Display twin of TestValueGoString:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L3667
#[test]
fn value_display() {
    let tests: Vec<(Value, &str)> = vec![
        (Value::null(Type::dynamic()), "Value::null(Type::dynamic())"),
        (Value::null(Type::string()), "Value::null(Type::string())"),
        (
            Value::null(Type::tuple([Type::string(), Type::bool()])),
            "Value::null(Type::tuple([Type::string(), Type::bool()]))",
        ),
        (Value::unknown(Type::dynamic()), "Value::dynamic()"),
        (
            Value::unknown(Type::string()),
            "Value::unknown(Type::string())",
        ),
        (
            Value::unknown(Type::tuple([Type::string(), Type::bool()])),
            "Value::unknown(Type::tuple([Type::string(), Type::bool()]))",
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .not_null()
                .new_value(),
            "Value::unknown(Type::string()).refine_not_null()",
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .not_null()
                .string_prefix("a-")
                .new_value(),
            r#"Value::unknown(Type::string()).refine().not_null().string_prefix_full("a-").new_value()"#,
        ),
        (
            // The last character of the prefix gets discarded in case the
            // next character is a combining diacritic
            Value::unknown(Type::string())
                .refine()
                .not_null()
                .string_prefix("foo")
                .new_value(),
            r#"Value::unknown(Type::string()).refine().not_null().string_prefix_full("fo").new_value()"#,
        ),
        (
            Value::unknown(Type::bool()).refine().not_null().new_value(),
            "Value::unknown(Type::bool()).refine_not_null()",
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_inclusive(Value::zero(), Value::unknown(Type::number()))
                .new_value(),
            "Value::unknown(Type::number()).refine().number_range_lower_bound(Value::number_int(0), true).new_value()",
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_inclusive(Value::zero(), Value::number_int(1))
                .new_value(),
            "Value::unknown(Type::number()).refine().number_range_lower_bound(Value::number_int(0), true).number_range_upper_bound(Value::number_int(1), true).new_value()",
        ),
        (Value::string(""), r#"Value::string("")"#),
        (Value::string("hello"), r#"Value::string("hello")"#),
        (Value::zero(), "Value::number_int(0)"),
        (Value::number_float(1.2), "Value::number_float(1.2)"),
        (
            // the "float-ness" of the input is lost because its value is a
            // whole number
            Value::number_float(1.0),
            "Value::number_int(1)",
        ),
        (
            Value::parse_number("3.14159265358979323846264338327950288419716939937510582097494459")
                .unwrap(),
            r#"Value::parse_number("3.14159265358979323846264338327950288419716939937510582097494459").unwrap()"#,
        ),
        (Value::bool(true), "Value::bool(true)"),
        (Value::bool(false), "Value::bool(false)"),
        (
            Value::list_empty(Type::string()),
            "Value::list_empty(Type::string())",
        ),
        (
            Value::list_empty(Type::list(Type::string())),
            "Value::list_empty(Type::list(Type::string()))",
        ),
        (
            Value::list([Value::bool(true)]),
            "Value::list([Value::bool(true)])",
        ),
        (
            Value::set_empty(Type::string()),
            "Value::set_empty(Type::string())",
        ),
        (
            Value::set_empty(Type::map(Type::string())),
            "Value::set_empty(Type::map(Type::string()))",
        ),
        (
            Value::set([Value::bool(true)]),
            "Value::set([Value::bool(true)])",
        ),
        (Value::empty_tuple(), "Value::empty_tuple()"),
        (Value::tuple([] as [Value; 0]), "Value::empty_tuple()"),
        (
            Value::tuple([Value::bool(true)]),
            "Value::tuple([Value::bool(true)])",
        ),
        (
            Value::map_empty(Type::string()),
            "Value::map_empty(Type::string())",
        ),
        (
            Value::map_empty(Type::set(Type::string())),
            "Value::map_empty(Type::set(Type::string()))",
        ),
        (
            Value::map([("boop", Value::bool(true))]),
            r#"Value::map([("boop", Value::bool(true))])"#,
        ),
        (Value::empty_object(), "Value::empty_object()"),
        (
            Value::object([] as [(&str, Value); 0]),
            "Value::empty_object()",
        ),
        (
            Value::object([("foo", Value::bool(true))]),
            r#"Value::object([("foo", Value::bool(true))])"#,
        ),
    ];

    for (i, (value, want)) in tests.iter().enumerate() {
        let got = value.to_string();
        assert_eq!(got, *want, "case {i}: wrong Display result");
    }
}

// Ported from TestHasWhollyKnownType:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L3833
#[test]
fn has_wholly_known_type() {
    let tests: Vec<(Value, bool)> = vec![
        (Value::dynamic(), false),
        (Value::object([("dyn", Value::dynamic())]), false),
        (Value::null(Type::object([("dyn", Type::dynamic())])), true),
        (
            Value::tuple([Value::string("a"), Value::null(Type::dynamic())]),
            true,
        ),
        (
            Value::list([Value::object([("null", Value::null(Type::dynamic()))])]),
            true,
        ),
        (
            Value::list([Value::null(Type::object([("dyn", Type::dynamic())]))]),
            true,
        ),
        (
            Value::object([(
                "tuple",
                Value::tuple([Value::string("a"), Value::null(Type::dynamic())]),
            )]),
            true,
        ),
        (
            Value::object([(
                "tuple",
                Value::tuple([Value::object([("dyn", Value::dynamic())])]),
            )]),
            false,
        ),
    ];

    for (i, (value, want)) in tests.iter().enumerate() {
        let got = value.has_wholly_known_type();
        assert_eq!(got, *want, "case {i}: wrong result for {value:?}");
    }
}

// NOTE(port): upstream TestFloatCopy
// (https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L4128)
// pins that
// mutating the *big.Float returned by `Value.AsBigFloat()` (via
// `SetInt64(1)`) does not alias the number stored inside the cty.Value.
// This crate's numeric accessor is `as_f64()`, which returns an owned
// primitive `f64`, so there is no shared mutable big-float state to
// protect against and no faithful Rust analogue. Deliberately omitted.
