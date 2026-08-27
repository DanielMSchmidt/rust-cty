//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/function/stdlib/bool_test.go
//!   cty/function/stdlib/bytes_test.go
//!   cty/function/stdlib/csv_test.go
//!   cty/function/stdlib/conversion_test.go
//!   cty/function/stdlib/datetime_test.go
//!   cty/function/stdlib/general_test.go
//!   cty/function/stdlib/json_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::function::stdlib;
use cty::{Type, Value};

// ---------------------------------------------------------------------------
// Ported from https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/bool_test.go
// ---------------------------------------------------------------------------

// Ported from TestNot:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/bool_test.go#L10
#[test]
#[ignore = "not yet implemented"]
fn not() {
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

    for (i, (input, want)) in tests.iter().enumerate() {
        let got =
            stdlib::not(input).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestAnd:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/bool_test.go#L52
#[test]
#[ignore = "not yet implemented"]
fn and() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (Value::bool(false), Value::bool(false), Value::bool(false)),
        (Value::bool(false), Value::bool(true), Value::bool(false)),
        (Value::bool(true), Value::bool(false), Value::bool(false)),
        (Value::bool(true), Value::bool(true), Value::bool(true)),
        (
            Value::bool(true),
            Value::unknown(Type::bool()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::bool()),
            Value::unknown(Type::bool()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::bool(true),
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
        let got =
            stdlib::and(a, b).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestOr:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/bool_test.go#L115
#[test]
#[ignore = "not yet implemented"]
fn or() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (Value::bool(false), Value::bool(false), Value::bool(false)),
        (Value::bool(false), Value::bool(true), Value::bool(true)),
        (Value::bool(true), Value::bool(false), Value::bool(true)),
        (Value::bool(true), Value::bool(true), Value::bool(true)),
        (
            Value::bool(true),
            Value::unknown(Type::bool()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::bool()),
            Value::unknown(Type::bool()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::bool(true),
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
        let got =
            stdlib::or(a, b).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// ---------------------------------------------------------------------------
// Ported from https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/bytes_test.go
// ---------------------------------------------------------------------------

// Ported from TestBytesLen:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/bytes_test.go#L10
#[test]
#[ignore = "not yet implemented"]
fn bytes_len() {
    let tests: Vec<(Value, Value)> = vec![
        (stdlib::bytes_val(b"".to_vec()), Value::number_int(0)),
        (stdlib::bytes_val(b"a".to_vec()), Value::number_int(1)),
        (stdlib::bytes_val(b"abc".to_vec()), Value::number_int(3)),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got = stdlib::bytes_len(input).unwrap_or_else(|err| panic!("case {i}: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestBytesSlice:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/bytes_test.go#L47
#[test]
#[ignore = "not yet implemented"]
fn bytes_slice() {
    let tests: Vec<(Value, Value, Value, Value)> = vec![
        (
            stdlib::bytes_val(b"".to_vec()),
            Value::number_int(0),
            Value::number_int(0),
            stdlib::bytes_val(b"".to_vec()),
        ),
        (
            stdlib::bytes_val(b"a".to_vec()),
            Value::number_int(0),
            Value::number_int(1),
            stdlib::bytes_val(b"a".to_vec()),
        ),
        (
            stdlib::bytes_val(b"abc".to_vec()),
            Value::number_int(0),
            Value::number_int(2),
            stdlib::bytes_val(b"ab".to_vec()),
        ),
        (
            stdlib::bytes_val(b"abc".to_vec()),
            Value::number_int(1),
            Value::number_int(2),
            stdlib::bytes_val(b"bc".to_vec()),
        ),
        (
            stdlib::bytes_val(b"abc".to_vec()),
            Value::number_int(0),
            Value::number_int(3),
            stdlib::bytes_val(b"abc".to_vec()),
        ),
    ];

    for (i, (input, offset, length, want)) in tests.iter().enumerate() {
        let got = stdlib::bytes_slice(input, offset, length)
            .unwrap_or_else(|err| panic!("case {i}: {err}"));

        let got_bytes = got
            .encapsulated_value()
            .downcast_ref::<Vec<u8>>()
            .unwrap()
            .clone();
        let want_bytes = want
            .encapsulated_value()
            .downcast_ref::<Vec<u8>>()
            .unwrap()
            .clone();

        assert_eq!(got_bytes, want_bytes, "case {i}: wrong result");
    }
}

// ---------------------------------------------------------------------------
// Ported from https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/csv_test.go
// ---------------------------------------------------------------------------

const CSV_TEST: &str = r#""name","size","type"
"foo","100","tiny"
"bar","","huge"
"baz","50","weedy"
"#;

// Ported from TestCSVDecode:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/csv_test.go#L10
#[test]
#[ignore = "not yet implemented"]
fn csv_decode() {
    let tests: Vec<(Value, Value, &str)> = vec![
        (
            Value::string(CSV_TEST),
            Value::list([
                Value::object([
                    ("name", Value::string("foo")),
                    ("size", Value::string("100")),
                    ("type", Value::string("tiny")),
                ]),
                Value::object([
                    ("name", Value::string("bar")),
                    ("size", Value::string("")),
                    ("type", Value::string("huge")),
                ]),
                Value::object([
                    ("name", Value::string("baz")),
                    ("size", Value::string("50")),
                    ("type", Value::string("weedy")),
                ]),
            ]),
            "",
        ),
        (
            Value::string(r#""just","header","line""#),
            Value::list_empty(Type::object([
                ("just", Type::string()),
                ("header", Type::string()),
                ("line", Type::string()),
            ])),
            "",
        ),
        (Value::string(""), Value::dynamic(), "missing header line"),
        (
            Value::string("not csv at all"),
            Value::list_empty(Type::object([("not csv at all", Type::string())])),
            "",
        ),
        (
            Value::string(r#"invalid"thing""#),
            Value::dynamic(),
            r#"CSV parse error on line 1: bare " in non-quoted-field"#,
        ),
        (
            Value::unknown(Type::string()),
            Value::dynamic(), // need to know the value to determine the type
            "",
        ),
        (Value::dynamic(), Value::dynamic(), ""),
        (
            Value::bool(true),
            Value::dynamic(),
            "string required, but received bool",
        ),
        (
            Value::null(Type::string()),
            Value::dynamic(),
            "argument must not be null",
        ),
    ];

    for (i, (input, want, want_err)) in tests.iter().enumerate() {
        let result = stdlib::csv_decode(input);
        match result {
            Err(err) => {
                assert_eq!(
                    err.to_string(),
                    *want_err,
                    "case {i}: wrong error\ngot:  {err}\nwant: {want_err}"
                );
            }
            Ok(got) => {
                assert_eq!(
                    *want_err, "",
                    "case {i}: succeeded; want error {want_err:?}"
                );
                assert_eq!(got, *want, "case {i}: wrong result");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Ported from https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/conversion_test.go
// ---------------------------------------------------------------------------

// Ported from TestTo:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/conversion_test.go#L10
#[test]
#[ignore = "not yet implemented"]
fn to() {
    let tests: Vec<(Value, Type, Value, &str)> = vec![
        (Value::string("a"), Type::string(), Value::string("a"), ""),
        (
            Value::unknown(Type::string()),
            Type::string(),
            Value::unknown(Type::string()),
            "",
        ),
        (
            Value::null(Type::string()),
            Type::string(),
            Value::null(Type::string()),
            "",
        ),
        (Value::bool(true), Type::string(), Value::string("true"), ""),
        (
            Value::string("a"),
            Type::bool(),
            Value::dynamic(),
            r#"cannot convert "a" to bool; only the strings "true" or "false" are allowed"#,
        ),
        (
            Value::string("a"),
            Type::number(),
            Value::dynamic(),
            r#"cannot convert "a" to number; given string must be a decimal representation of a number"#,
        ),
        (
            Value::null(Type::string()),
            Type::number(),
            Value::null(Type::number()),
            "",
        ),
        (
            Value::null(Type::dynamic()),
            Type::number(),
            Value::null(Type::number()),
            "",
        ),
        (
            Value::unknown(Type::bool()),
            Type::string(),
            Value::unknown(Type::string()),
            "",
        ),
        (
            Value::unknown(Type::string()),
            Type::bool(),
            Value::unknown(Type::bool()), // conversion is optimistic
            "",
        ),
        (
            Value::tuple([Value::string("hello"), Value::bool(true)]),
            Type::list(Type::string()),
            Value::list([Value::string("hello"), Value::string("true")]),
            "",
        ),
        (
            Value::tuple([Value::string("hello"), Value::bool(true)]),
            Type::set(Type::string()),
            Value::set([Value::string("hello"), Value::string("true")]),
            "",
        ),
        (
            Value::object([("foo", Value::string("hello")), ("bar", Value::bool(true))]),
            Type::map(Type::string()),
            Value::map([
                ("foo", Value::string("hello")),
                ("bar", Value::string("true")),
            ]),
            "",
        ),
        (
            Value::empty_tuple(),
            Type::string(),
            Value::dynamic(),
            "cannot convert tuple to string",
        ),
        (
            Value::unknown(Type::empty_tuple()),
            Type::string(),
            Value::dynamic(),
            "cannot convert tuple to string",
        ),
        (
            Value::empty_object(),
            Type::object([("foo", Type::string())]),
            Value::dynamic(),
            r#"incompatible object type for conversion: attribute "foo" is required"#,
        ),
    ];

    for (i, (value, target_ty, want, want_err)) in tests.iter().enumerate() {
        let f = stdlib::make_to_func(target_ty.clone());
        let result = f.call(std::slice::from_ref(value));

        if !want_err.is_empty() {
            let err = match result {
                Ok(_) => panic!("case {i}: succeeded; want error"),
                Err(err) => err,
            };
            assert_eq!(
                err.to_string(),
                *want_err,
                "case {i}: wrong error\ngot:  {err}\nwant: {want_err}"
            );
            continue;
        }

        let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// ---------------------------------------------------------------------------
// Ported from https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/datetime_test.go
// ---------------------------------------------------------------------------

// Ported from TestFormatDate (format table):
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/datetime_test.go#L11
#[test]
#[ignore = "not yet implemented"]
fn format_date() {
    // NOTE(port): upstream uses cty.NilVal as the (unused) Want in the error
    // cases; represented here as None since NilVal has no Rust analogue.
    let tests: Vec<(Value, Option<Value>, &str)> = vec![
        (
            Value::string(""), // pointless, but valid
            Some(Value::string("")),
            "",
        ),
        (
            Value::string("YYYY-MM-DD"),
            Some(Value::string("2006-01-02")),
            "",
        ),
        (
            Value::string("EEE, MMM D ''YY"),
            Some(Value::string("Mon, Jan 2 '06")),
            "",
        ),
        (
            Value::string("hh:mm:ss"),
            Some(Value::string("15:04:05")),
            "",
        ),
        (
            Value::string("H 'o''clock' AA"),
            Some(Value::string("3 o'clock PM")),
            "",
        ),
        (
            Value::string("H 'o''clock'"),
            Some(Value::string("3 o'clock")),
            "",
        ),
        (
            Value::string("hh:mm:ssZZZZ"),
            Some(Value::string("15:04:05+0000")),
            "",
        ),
        (
            Value::string("hh:mm:ssZZZZZ"),
            Some(Value::string("15:04:05+00:00")),
            "",
        ),
        (Value::string("MMMM"), Some(Value::string("January")), ""),
        (Value::string("EEEE"), Some(Value::string("Monday")), ""),
        (Value::string("aa"), Some(Value::string("pm")), ""),
        // Some common standard machine-oriented formats
        (
            Value::string("YYYY-MM-DD'T'hh:mm:ssZ"),     // RFC3339
            Some(Value::string("2006-01-02T15:04:05Z")), // (since RFC3339 is the input format too, this is a bit pointless)
            "",
        ),
        (
            Value::string("DD MMM YYYY hh:mm ZZZ"), // RFC822
            Some(Value::string("02 Jan 2006 15:04 UTC")),
            "",
        ),
        (
            Value::string("EEEE, DD-MMM-YY hh:mm:ss ZZZ"), // RFC850
            Some(Value::string("Monday, 02-Jan-06 15:04:05 UTC")),
            "",
        ),
        (
            Value::string("EEE, DD MMM YYYY hh:mm:ss ZZZ"), // RFC1123
            Some(Value::string("Mon, 02 Jan 2006 15:04:05 UTC")),
            "",
        ),
        // Invalids
        (
            Value::string("Y"),
            None,
            r#"invalid date format verb "Y": year must either be "YY" or "YYYY""#,
        ),
        (
            Value::string("YYYYY"),
            None,
            r#"invalid date format verb "YYYYY": year must either be "YY" or "YYYY""#,
        ),
        (
            Value::string("A"),
            None,
            r#"invalid date format verb "A": must be "AA""#,
        ),
        (
            Value::string("a"),
            None,
            r#"invalid date format verb "a": must be "aa""#,
        ),
        (Value::string("'blah blah"), None, "unterminated literal '"),
        (Value::string("'"), None, "unterminated literal '"),
    ];

    // Upstream: time.Date(2006, time.January, 2, 15, 04, 05, 0, time.UTC)
    // formatted as RFC3339.
    let time_val = Value::string("2006-01-02T15:04:05Z");

    for (i, (format, want, want_err)) in tests.iter().enumerate() {
        let result = stdlib::format_date(format, &time_val);

        if !want_err.is_empty() {
            let err = match result {
                Ok(_) => panic!("case {i}: no error; want error {want_err:?}"),
                Err(err) => err,
            };
            assert_eq!(
                err.to_string(),
                *want_err,
                "case {i}: wrong error\ngot:  {err}\nwant: {want_err}"
            );
        } else {
            let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want.as_ref().unwrap(), "case {i}: wrong result");
        }
    }
}

// Ported from TestFormatDate (parse error tests):
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/datetime_test.go#L11
#[test]
#[ignore = "not yet implemented"]
fn format_date_parse_errors() {
    let parse_err_tests: Vec<(Value, &str)> = vec![
        (
            Value::string(""),
            "not a valid RFC3339 timestamp: end of string before year",
        ),
        (
            Value::string("2017-01-02"),
            "not a valid RFC3339 timestamp: missing required time introducer 'T'",
        ),
        (
            Value::string("2017-12-02t00:00:00Z"),
            "not a valid RFC3339 timestamp: missing required time introducer 'T'",
        ),
        (
            Value::string("2017:01:02"),
            r#"not a valid RFC3339 timestamp: found ":01:02" where "-" is expected"#,
        ),
        (
            Value::string("2017"),
            r#"not a valid RFC3339 timestamp: end of string where "-" is expected"#,
        ),
        (
            Value::string("2017-01-02T"),
            "not a valid RFC3339 timestamp: end of string before hour",
        ),
        (
            Value::string("2017-01-02T00"),
            r#"not a valid RFC3339 timestamp: end of string where ":" is expected"#,
        ),
        (
            Value::string("2017-01-02T00:00:00"),
            "not a valid RFC3339 timestamp: end of string before UTC offset",
        ),
        (
            Value::string("2017-01-02T26:00:00Z"),
            "not a valid RFC3339 timestamp: hour must be between 0 and 23 inclusive",
        ),
        (
            Value::string("2017-13-02T00:00:00Z"),
            // This one generates an odd message due to an apparent quirk in
            // the Go time parser. Ideally it would use "13" as the errant string.
            r#"not a valid RFC3339 timestamp: cannot use "-02T00:00:00Z" as month"#,
        ),
        (
            Value::string("2017-02-31T00:00:00Z"),
            "not a valid RFC3339 timestamp: day out of range",
        ),
        (
            Value::string(r#""2017-12-02T00:00:00Z""#),
            r#"not a valid RFC3339 timestamp: cannot use "\"2017-12-02T00:00:00Z\"" as year"#,
        ),
        (
            Value::string("2-12-02T00:00:00Z"),
            // Go parser seems to be trying to parse the whole thing as a year
            // here, producing a confusing error message.
            r#"not a valid RFC3339 timestamp: cannot use "2-12-02T00:00:00Z" as year"#,
        ),
        (
            Value::string("2000-01-01T1:12:34Z"),
            "not a valid RFC3339 timestamp: hour must have exactly two digits",
        ),
        (
            Value::string("2000-01-01T01:1:34Z"),
            "not a valid RFC3339 timestamp: minute must have exactly two digits",
        ),
        (
            Value::string("2000-01-01T01:01:1Z"),
            r#"not a valid RFC3339 timestamp: cannot use "1Z" as second"#,
        ),
        (
            Value::string("2000-01-01T00:00:00,000Z"),
            r#"not a valid RFC3339 timestamp: cannot use "," as timestamp segment"#,
        ),
        (
            Value::string("2000-01-01T00:00:00+24:00"),
            r#"not a valid RFC3339 timestamp: cannot use "+24:00" as UTC offset"#,
        ),
        (
            Value::string("2000-01-01T00:00:00+00:60"),
            r#"not a valid RFC3339 timestamp: cannot use "+00:60" as UTC offset"#,
        ),
    ];

    for (i, (timestamp, want_err)) in parse_err_tests.iter().enumerate() {
        let result = stdlib::format_date(&Value::string(""), timestamp);

        let err = match result {
            Ok(_) => panic!("case {i}: no error; want error {want_err:?}"),
            Err(err) => err,
        };
        assert_eq!(
            err.to_string(),
            *want_err,
            "case {i}: wrong error\ngot:  {err}\nwant: {want_err}"
        );
    }
}

// Ported from TestFormatDate (parse success tests):
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/datetime_test.go#L11
#[test]
#[ignore = "not yet implemented"]
fn format_date_parse_success() {
    let parse_success_tests: Vec<(&str, &str, &str)> = vec![
        (
            "2022-03-01T00:23:45Z",
            "2022-03-01T00:23:45Z",
            "Tuesday, 01-Mar-22 00:23:45 UTC",
        ),
        (
            "2022-03-01T00:23:45+00:00",
            "2022-03-01T00:23:45Z",
            "Tuesday, 01-Mar-22 00:23:45 UTC",
        ),
        (
            "2022-03-01T00:23:45+01:00",
            "2022-03-01T00:23:45+01:00",
            "Tuesday, 01-Mar-22 00:23:45 +0100",
        ),
        (
            "2022-03-01T00:23:45-01:00",
            "2022-03-01T00:23:45-01:00",
            "Tuesday, 01-Mar-22 00:23:45 -0100",
        ),
        (
            "1900-01-01T00:00:00Z",
            "1900-01-01T00:00:00Z",
            "Monday, 01-Jan-00 00:00:00 UTC",
        ),
    ];

    const RFC3339_FORMAT: &str = "YYYY-MM-DD'T'hh:mm:ssZ";
    const RFC850_FORMAT: &str = "EEEE, DD-MMM-YY hh:mm:ss ZZZ";

    for (i, (input, want_rfc3339, want_rfc850)) in parse_success_tests.iter().enumerate() {
        // RFC3339
        let got = stdlib::format_date(&Value::string(RFC3339_FORMAT), &Value::string(*input))
            .unwrap_or_else(|err| panic!("case {i} RFC3339: unexpected error: {err}"));
        assert_eq!(
            got.as_string(),
            *want_rfc3339,
            "case {i} RFC3339: wrong result"
        );

        // RFC850
        let got = stdlib::format_date(&Value::string(RFC850_FORMAT), &Value::string(*input))
            .unwrap_or_else(|err| panic!("case {i} RFC850: unexpected error: {err}"));
        assert_eq!(
            got.as_string(),
            *want_rfc850,
            "case {i} RFC850: wrong result"
        );
    }
}

// ---------------------------------------------------------------------------
// Ported from https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/general_test.go
// ---------------------------------------------------------------------------

// Ported from TestEqual:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/general_test.go#L10
#[test]
#[ignore = "not yet implemented"]
fn equal() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::number_int(1),
            Value::number_int(2),
            Value::bool(false),
        ),
        (
            Value::number_int(2),
            Value::number_int(2),
            Value::bool(true),
        ),
        (
            Value::null(Type::number()),
            Value::null(Type::number()),
            Value::bool(true),
        ),
        (
            Value::number_int(2),
            Value::null(Type::number()),
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
        let got =
            stdlib::equal(a, b).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestCoalesce:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/general_test.go#L73
#[test]
#[ignore = "not yet implemented"]
fn coalesce() {
    let tests: Vec<(Vec<Value>, Value)> = vec![
        (vec![Value::bool(true)], Value::bool(true)),
        (
            vec![Value::null(Type::bool()), Value::bool(true)],
            Value::bool(true),
        ),
        (
            vec![Value::null(Type::bool()), Value::bool(false)],
            Value::bool(false),
        ),
        (
            vec![
                Value::null(Type::bool()),
                Value::bool(false),
                Value::string("hello"),
            ],
            Value::string("false"),
        ),
        (
            vec![Value::bool(true), Value::unknown(Type::bool())],
            Value::bool(true),
        ),
        (
            vec![Value::unknown(Type::bool()), Value::bool(true)],
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            vec![Value::unknown(Type::bool()), Value::string("hello")],
            Value::unknown(Type::string()).refine_not_null(),
        ),
        (
            vec![Value::dynamic(), Value::bool(true)],
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (vec![Value::dynamic()], Value::dynamic()),
    ];

    for (i, (values, want)) in tests.iter().enumerate() {
        let got = stdlib::coalesce(values)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// ---------------------------------------------------------------------------
// Ported from https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/json_test.go
// ---------------------------------------------------------------------------

// Ported from TestJSONEncode:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/json_test.go#L10
#[test]
#[ignore = "not yet implemented"]
fn json_encode() {
    let tests: Vec<(Value, Value)> = vec![
        // This does not comprehensively test all possible inputs because
        // the underlying functions in package json already have tests of
        // their own. Here we are mainly concerned with seeing that the
        // function's definition accepts all reasonable values.
        (Value::number_int(15), Value::string("15")),
        (Value::string("hello"), Value::string(r#""hello""#)),
        (Value::bool(true), Value::string("true")),
        (Value::list_empty(Type::number()), Value::string("[]")),
        (
            Value::list([Value::bool(true), Value::bool(false)]),
            Value::string("[true,false]"),
        ),
        (
            Value::object([("true", Value::bool(true)), ("false", Value::bool(false))]),
            Value::string(r#"{"false":false,"true":true}"#),
        ),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::string()).refine_not_null(),
        ),
        (
            Value::object([
                ("dunno", Value::unknown(Type::bool())),
                ("false", Value::bool(false)),
            ]),
            Value::unknown(Type::string())
                .refine()
                .not_null()
                .string_prefix_full("{")
                .new_value(),
        ),
        (
            Value::list([Value::unknown(Type::string())]),
            Value::unknown(Type::string())
                .refine()
                .not_null()
                .string_prefix_full("[")
                .new_value(),
        ),
        (
            Value::unknown(Type::string()),
            Value::unknown(Type::string()).refine_not_null(), // Can't refine the prefix because the input might be null
        ),
        (
            Value::unknown(Type::string()).refine_not_null(),
            Value::unknown(Type::string())
                .refine()
                .not_null()
                .string_prefix_full("\"")
                .new_value(),
        ),
        (
            Value::unknown(Type::number()),
            Value::unknown(Type::string()).refine_not_null(),
        ),
        (
            Value::unknown(Type::bool()),
            Value::unknown(Type::string()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::unknown(Type::string()).refine_not_null(),
        ),
        (Value::null(Type::string()), Value::string("null")),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got = stdlib::json_encode(input)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestJSONDecode:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/json_test.go#L96
#[test]
#[ignore = "not yet implemented"]
fn json_decode() {
    let tests: Vec<(Value, Value)> = vec![
        (Value::string("15"), Value::number_int(15)),
        (Value::string(r#""hello""#), Value::string("hello")),
        (Value::string("true"), Value::bool(true)),
        (Value::string("[]"), Value::empty_tuple()),
        (
            Value::string("[true,false]"),
            Value::tuple([Value::bool(true), Value::bool(false)]),
        ),
        (
            Value::string(r#"{"false":false,"true":true}"#),
            Value::object([("true", Value::bool(true)), ("false", Value::bool(false))]),
        ),
        (
            Value::unknown(Type::string()),
            Value::dynamic(), // need to know the value to determine the type
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix_full("1")
                .new_value(),
            Value::unknown(Type::number()), // deduced from refinement
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix_full("-")
                .new_value(),
            Value::unknown(Type::number()), // deduced from refinement
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix_full(".")
                .new_value(),
            Value::unknown(Type::number()), // deduced from refinement
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix_full("t")
                .new_value(),
            Value::unknown(Type::bool()), // deduced from refinement
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix_full("f")
                .new_value(),
            Value::unknown(Type::bool()), // deduced from refinement
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix_full("\"blurt")
                .new_value(),
            Value::unknown(Type::string()), // deduced from refinement
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix_full("{")
                .new_value(),
            Value::dynamic(), // can't deduce the result type, but potentially valid syntax
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix_full("[")
                .new_value(),
            Value::dynamic(), // can't deduce the result type, but potentially valid syntax
        ),
        (Value::dynamic(), Value::dynamic()),
        (Value::string("true").mark(1), Value::bool(true).mark(1)),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got = stdlib::json_decode(input)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }

    let error_tests: Vec<(Value, &str)> = vec![
        (
            Value::string("aaaa"),
            "invalid character 'a' looking for beginning of value",
        ),
        (
            Value::string("nope"),
            "invalid character 'o' in literal null (expecting 'u')", // (the 'n' looked like the beginning of 'null')
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix_full("a")
                .new_value(),
            "a JSON document cannot begin with the character 'a'", // error deduced from refinement, despite full value being unknown
        ),
    ];

    for (i, (input, want_err)) in error_tests.iter().enumerate() {
        let err = match stdlib::json_decode(input) {
            Ok(_) => panic!("error case {i}: unexpected success"),
            Err(err) => err,
        };
        assert_eq!(
            err.to_string(),
            *want_err,
            "error case {i}: wrong error\ngot:  {err}\nwant: {want_err}"
        );
    }
}
