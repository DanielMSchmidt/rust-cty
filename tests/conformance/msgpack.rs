//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/msgpack/roundtrip_test.go
//!   cty/msgpack/type_implied_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::msgpack::{implied_type, marshal, unmarshal};
use cty::{Type, Value, convert};

// Ported from TestRoundTrip:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/msgpack/roundtrip_test.go#L12
#[test]
#[ignore = "not yet implemented"]
fn round_trip() {
    let big_number_val = Value::parse_number(
        "9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999",
    )
    .unwrap();
    // awkward because it can't be represented exactly in binary
    let awkward_fraction_val = Value::parse_number("0.8").unwrap();

    let tests: Vec<(Value, Type)> = vec![
        (Value::string("hello"), Type::string()),
        (Value::string(""), Type::string()),
        (Value::null(Type::string()), Type::string()),
        (Value::unknown(Type::string()), Type::string()),
        (
            Value::unknown(Type::string()).refine_not_null(),
            Type::string(),
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix("foo-")
                .new_value(),
            Type::string(),
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .not_null()
                .string_prefix("foo-")
                .new_value(),
            Type::string(),
        ),
        (Value::bool(true), Type::bool()),
        (Value::bool(false), Type::bool()),
        (Value::null(Type::bool()), Type::bool()),
        (Value::unknown(Type::bool()), Type::bool()),
        (Value::unknown(Type::bool()).refine_not_null(), Type::bool()),
        (Value::number_int(1), Type::number()),
        (Value::number_float(1.5), Type::number()),
        (big_number_val, Type::number()),
        (
            Value::parse_number("9223372036854775807").unwrap(),
            Type::number(),
        ),
        (
            Value::parse_number("9223372036854775808").unwrap(),
            Type::number(),
        ),
        (
            Value::parse_number("9223372036854775809").unwrap(),
            Type::number(),
        ),
        (
            Value::parse_number("18446744073709551616").unwrap(),
            Type::number(),
        ),
        (awkward_fraction_val, Type::number()),
        (Value::positive_infinity(), Type::number()),
        (Value::negative_infinity(), Type::number()),
        (Value::unknown(Type::number()), Type::number()),
        (
            Value::unknown(Type::number()).refine_not_null(),
            Type::number(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::zero(), true)
                .new_value(),
            Type::number(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::zero(), false)
                .new_value(),
            Type::number(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_upper_bound(Value::zero(), true)
                .new_value(),
            Type::number(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_upper_bound(Value::zero(), false)
                .new_value(),
            Type::number(),
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_inclusive(Value::zero(), Value::number_int(1))
                .new_value(),
            Type::number(),
        ),
        (
            Value::list([Value::string("hello")]),
            Type::list(Type::string()),
        ),
        (
            Value::list([Value::unknown(Type::string())]),
            Type::list(Type::string()),
        ),
        (
            Value::list([Value::null(Type::string())]),
            Type::list(Type::string()),
        ),
        (
            Value::null(Type::list(Type::string())),
            Type::list(Type::string()),
        ),
        (
            Value::list_empty(Type::string()),
            Type::list(Type::string()),
        ),
        (
            Value::unknown(Type::list(Type::string())),
            Type::list(Type::string()),
        ),
        (
            Value::unknown(Type::list(Type::string())).refine_not_null(),
            Type::list(Type::string()),
        ),
        (
            Value::unknown(Type::list(Type::string()))
                .refine()
                .collection_length_lower_bound(1)
                .new_value(),
            Type::list(Type::string()),
        ),
        (
            Value::unknown(Type::list(Type::string()))
                .refine()
                .collection_length_upper_bound(1)
                .new_value(),
            Type::list(Type::string()),
        ),
        (
            Value::unknown(Type::list(Type::string()))
                .refine()
                .collection_length_lower_bound(1)
                .collection_length_upper_bound(2)
                .new_value(),
            Type::list(Type::string()),
        ),
        (
            // NOTE: This refinement should collapse to a known 2-element list with unknown elements
            Value::unknown(Type::list(Type::string()))
                .refine()
                .collection_length_lower_bound(2)
                .collection_length_upper_bound(2)
                .new_value(),
            Type::list(Type::string()),
        ),
        (
            Value::unknown(Type::list(Type::string()))
                .refine()
                .collection_length_upper_bound(1)
                .not_null()
                .new_value(),
            Type::list(Type::string()),
        ),
        (
            Value::set([Value::string("hello")]),
            Type::set(Type::string()),
        ),
        (
            Value::set([Value::unknown(Type::string())]),
            Type::set(Type::string()),
        ),
        (
            Value::set([Value::null(Type::string())]),
            Type::set(Type::string()),
        ),
        (Value::set_empty(Type::string()), Type::set(Type::string())),
        (
            Value::map([("greeting", Value::string("hello"))]),
            Type::map(Type::string()),
        ),
        (
            Value::map([("greeting", Value::unknown(Type::string()))]),
            Type::map(Type::string()),
        ),
        (
            Value::map([("greeting", Value::null(Type::string()))]),
            Type::map(Type::string()),
        ),
        (Value::map_empty(Type::string()), Type::map(Type::string())),
        (
            Value::tuple([Value::string("hello")]),
            Type::tuple([Type::string()]),
        ),
        (
            Value::tuple([Value::unknown(Type::string())]),
            Type::tuple([Type::string()]),
        ),
        (
            Value::tuple([Value::null(Type::string())]),
            Type::tuple([Type::string()]),
        ),
        (Value::empty_tuple(), Type::empty_tuple()),
        (
            Value::object([("greeting", Value::string("hello"))]),
            Type::object([("greeting", Type::string())]),
        ),
        (
            Value::object([("greeting", Value::unknown(Type::string()))]),
            Type::object([("greeting", Type::string())]),
        ),
        (
            Value::object([("greeting", Value::null(Type::string()))]),
            Type::object([("greeting", Type::string())]),
        ),
        (
            Value::object([
                ("a", Value::null(Type::string())),
                ("b", Value::null(Type::string())),
            ]),
            Type::object([("a", Type::string()), ("b", Type::string())]),
        ),
        (
            Value::object([
                ("a", Value::unknown(Type::string())),
                ("b", Value::unknown(Type::string())),
            ]),
            Type::object([("a", Type::string()), ("b", Type::string())]),
        ),
        (Value::empty_object(), Type::empty_object()),
        (Value::null(Type::string()), Type::dynamic()),
        (Value::dynamic(), Type::dynamic()),
        (
            Value::list([Value::string("hello")]),
            Type::list(Type::dynamic()),
        ),
        (
            Value::list([Value::null(Type::string())]),
            Type::list(Type::dynamic()),
        ),
        (Value::list([Value::dynamic()]), Type::list(Type::dynamic())),
    ];

    for (i, (value, ty)) in tests.iter().enumerate() {
        let b = marshal(value, ty).unwrap_or_else(|err| panic!("case {i}: marshal error: {err}"));

        let got =
            unmarshal(&b, ty).unwrap_or_else(|err| panic!("case {i}: unmarshal error: {err}"));

        assert_eq!(
            got, *value,
            "case {i}: value did not round-trip\ninput:  {value:?}\nresult: {got:?}"
        );
    }
}

// Ported from TestRoundTrip_fromString:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/msgpack/roundtrip_test.go#L371
#[test]
#[ignore = "not yet implemented"]
fn round_trip_from_string() {
    let tests: Vec<(&str, Type)> = vec![
        ("0", Type::number()),
        ("1", Type::number()),
        ("-1", Type::number()),
        ("9223372036854775807", Type::number()),
        ("9223372036854775808", Type::number()),
        ("9223372036854775809", Type::number()),
        ("18446744073709551616", Type::number()),
        ("-9223372036854775807", Type::number()),
        ("-9223372036854775808", Type::number()),
        ("-9223372036854775809", Type::number()),
        ("-18446744073709551616", Type::number()),
        ("true", Type::bool()),
        ("false", Type::bool()),
    ];

    for (i, (value, ty)) in tests.iter().enumerate() {
        let string_val = Value::string(*value);

        let original = convert::convert(&string_val, ty).unwrap_or_else(|err| {
            panic!("case {i}: input type must be convertible from string: {err}")
        });

        {
            // We'll first make sure that the conversion works even without
            // MessagePack involved, since otherwise we might falsely blame
            // the MessagePack encoding for bugs in package convert.
            let string_got = convert::convert(&original, &Type::string()).unwrap_or_else(|err| {
                panic!("case {i}: result must be convertible to string: {err}")
            });

            assert_eq!(
                string_got, string_val,
                "case {i}: value did not round-trip to string even without msgpack\ninput:  {value:?}\nresult: {string_got:?}"
            );
        }

        let b =
            marshal(&original, ty).unwrap_or_else(|err| panic!("case {i}: marshal error: {err}"));

        let got =
            unmarshal(&b, ty).unwrap_or_else(|err| panic!("case {i}: unmarshal error: {err}"));

        assert_eq!(
            got, original,
            "case {i}: value did not round-trip\ninput:  {value:?}\nresult: {got:?}"
        );

        let string_got = convert::convert(&got, &Type::string())
            .unwrap_or_else(|err| panic!("case {i}: result must be convertible to string: {err}"));

        assert_eq!(
            string_got, string_val,
            "case {i}: value did not round-trip to string\ninput:  {value:?}\nresult: {string_got:?}"
        );
    }
}

// Ported from TestRoundTrip_truncatesStringPrefixRefinement:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/msgpack/roundtrip_test.go#L487
//
// Unknown values with very long string prefix refinements do not round-trip
// losslessly. If the prefix is longer than 256 bytes it will be truncated to
// a maximum of 256 bytes.
#[test]
#[ignore = "not yet implemented"]
fn round_trip_truncates_string_prefix_refinement() {
    let tests: Vec<(Value, Type, Value)> = vec![
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix("a".repeat(1024))
                .new_value(),
            Type::string(),
            Value::unknown(Type::string())
                .refine()
                .string_prefix("a".repeat(255))
                .new_value(),
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .not_null()
                .string_prefix("b".repeat(1024))
                .new_value(),
            Type::string(),
            Value::unknown(Type::string())
                .refine()
                .not_null()
                .string_prefix("b".repeat(255))
                .new_value(),
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix("c".repeat(255) + "-")
                .new_value(),
            Type::string(),
            Value::unknown(Type::string())
                .refine()
                .string_prefix("c".repeat(255) + "-")
                .new_value(),
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix("d".repeat(255) + "\u{1f937}\u{1f937}")
                .new_value(),
            Type::string(),
            Value::unknown(Type::string())
                .refine()
                .string_prefix("d".repeat(255))
                .new_value(),
        ),
    ];

    for (i, (value, ty, round_trip_value)) in tests.iter().enumerate() {
        let b = marshal(value, ty).unwrap_or_else(|err| panic!("case {i}: marshal error: {err}"));

        let got =
            unmarshal(&b, ty).unwrap_or_else(|err| panic!("case {i}: unmarshal error: {err}"));

        assert_eq!(
            got, *round_trip_value,
            "case {i}: unexpected value after round-trip\ninput:  {value:?}\nexpect: {round_trip_value:?}\nresult: {got:?}"
        );
    }
}

// Ported from TestImpliedType:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/msgpack/type_implied_test.go#L10
#[test]
#[ignore = "not yet implemented"]
fn implied_type_test() {
    let tests: Vec<(&[u8], Type)> = vec![
        (b"\xc0", Type::dynamic()),
        (b"\x01", Type::number()),                 // positive fixnum
        (b"\xff", Type::number()),                 // negative fixnum
        (b"\xcc\x04", Type::number()),             // uint8
        (b"\xcd\x00\x04", Type::number()),         // uint16
        (b"\xce\x00\x04\x02\x01", Type::number()), // uint32
        (b"\xcf\x00\x04\x02\x01\x00\x04\x02\x01", Type::number()), // uint64
        (b"\xd0\x04", Type::number()),             // int8
        (b"\xd1\x00\x04", Type::number()),         // int16
        (b"\xd2\x00\x04\x02\x01", Type::number()), // int32
        (b"\xd3\x00\x04\x02\x01\x00\x04\x02\x01", Type::number()), // int64
        (b"\xca\x01\x01\x01\x01", Type::number()), // float32
        (b"\xcb\x01\x01\x01\x01\x01\x01\x01\x01", Type::number()), // float64
        (b"\xd4\x00\x00", Type::dynamic()),        // fixext1 (unknown value)
        (b"\xd5\x00\x00\x00", Type::dynamic()),    // fixext2 (unknown value)
        (b"\xa0", Type::string()),                 // fixstr (length zero)
        (b"\xa1\xff", Type::string()),             // fixstr (length one)
        (b"\xd9\x00", Type::string()),             // str8 (length zero)
        (b"\xd9\x01\xff", Type::string()),         // str8 (length one)
        (b"\xda\x00\x00", Type::string()),         // str16 (length zero)
        (b"\xda\x00\x01\xff", Type::string()),     // str16 (length one)
        (b"\xdb\x00\x00\x00\x00", Type::string()), // str32 (length zero)
        (b"\xdb\x00\x00\x00\x01\xff", Type::string()), // str32 (length one)
        (b"\xc2", Type::bool()),                   // false
        (b"\xc3", Type::bool()),                   // true
        (b"\x90", Type::empty_tuple()),            // fixarray (length zero)
        // fixarray (length one, element is empty string)
        (b"\x91\xa0", Type::tuple([Type::string()])),
        (b"\xdc\x00\x00", Type::empty_tuple()), // array16 (length zero)
        // array16 (length one, element is bool)
        (b"\xdc\x00\x01\xc2", Type::tuple([Type::bool()])),
        (b"\xdd\x00\x00\x00\x00", Type::empty_tuple()), // array32 (length zero)
        // array32 (length one, element is bool)
        (b"\xdd\x00\x00\x00\x01\xc2", Type::tuple([Type::bool()])),
        (b"\x80", Type::empty_object()), // fixmap (length zero)
        // fixmap (length one, "a" => bool)
        (b"\x81\xa1a\xc2", Type::object([("a", Type::bool())])),
        (b"\xde\x00\x00", Type::empty_object()), // map16 (length zero)
        // map16 (length one, "a" => bool)
        (
            b"\xde\x00\x01\xa1a\xc2",
            Type::object([("a", Type::bool())]),
        ),
        (b"\xdf\x00\x00\x00\x00", Type::empty_object()), // map32 (length zero)
        // map32 (length one, "a" => bool)
        (
            b"\xdf\x00\x00\x00\x01\xa1a\xc2",
            Type::object([("a", Type::bool())]),
        ),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got =
            implied_type(input).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));

        assert!(
            got.equals(want),
            "case {i}: wrong type\ninput: {input:?}\ngot:   {got:?}\nwant:  {want:?}"
        );
    }
}
