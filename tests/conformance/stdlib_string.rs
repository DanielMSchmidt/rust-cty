//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/function/stdlib/string_test.go
//!   cty/function/stdlib/string_replace_test.go
//!   cty/function/stdlib/regexp_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::function::stdlib;
use cty::{Type, Value, ValueMarks};

// upstream: cty/function/stdlib/string_test.go TestUpper
#[test]
fn upper() {
    let tests: Vec<(Value, Value)> = vec![
        (Value::string("hello"), Value::string("HELLO")),
        (Value::string("HELLO"), Value::string("HELLO")),
        (Value::string(""), Value::string("")),
        (Value::string("1"), Value::string("1")),
        (
            Value::string("\u{436}\u{436}"),
            Value::string("\u{416}\u{416}"),
        ),
        (
            Value::string("noe\u{308}l"),
            Value::string("NO\u{cb}L"),
        ),
        (
            // Go's case conversions don't handle this ligature, which is
            // unfortunate but is now a compatibility constraint since it
            // would be potentially-breaking to behave differently here in
            // future.
            Value::string("ba\u{fb04}e"),
            Value::string("BA\u{fb04}E"),
        ),
        (
            Value::string("\u{1f638}\u{1f63e}"),
            Value::string("\u{1f638}\u{1f63e}"),
        ),
        (
            Value::unknown(Type::string()),
            Value::unknown(Type::string()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::unknown(Type::string()).refine_not_null(),
        ),
        (
            Value::string("hello").mark(1),
            Value::string("HELLO").mark(1),
        ),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got = stdlib::upper(input).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/string_test.go TestLower
#[test]
fn lower() {
    let tests: Vec<(Value, Value)> = vec![
        (Value::string("HELLO"), Value::string("hello")),
        (Value::string("hello"), Value::string("hello")),
        (Value::string(""), Value::string("")),
        (Value::string("1"), Value::string("1")),
        (
            Value::string("\u{416}\u{416}"),
            Value::string("\u{436}\u{436}"),
        ),
        (
            Value::unknown(Type::string()),
            Value::unknown(Type::string()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::unknown(Type::string()).refine_not_null(),
        ),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got = stdlib::lower(input).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/string_test.go TestReverse
#[test]
fn reverse() {
    let tests: Vec<(Value, Value)> = vec![
        (Value::string("hello"), Value::string("olleh")),
        (Value::string(""), Value::string("")),
        (Value::string("1"), Value::string("1")),
        (
            Value::string("\u{416}\u{438}\u{432}\u{43e}\u{439} \u{416}\u{443}\u{440}\u{43d}\u{430}\u{43b}"),
            Value::string("\u{43b}\u{430}\u{43d}\u{440}\u{443}\u{416} \u{439}\u{43e}\u{432}\u{438}\u{416}"),
        ),
        (
            // note that the dieresis here is intentionally a combining
            // ligature.
            Value::string("noe\u{308}l"),
            Value::string("le\u{308}on"),
        ),
        (
            // The Es in this string has three combining acute accents.
            // This tests something that NFC-normalization cannot collapse
            // into a single precombined codepoint, since otherwise we might
            // be cheating and relying on the single-codepoint forms.
            Value::string("we\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}!"),
            Value::string("!e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}w"),
        ),
        (
            // Go's normalization forms don't handle this ligature, so we
            // will produce the wrong result but this is now a compatibility
            // constraint and so we'll test it.
            Value::string("ba\u{fb04}e"),
            Value::string("e\u{fb04}ab"),
        ),
        (
            Value::string("\u{1f638}\u{1f63e}"),
            Value::string("\u{1f63e}\u{1f638}"),
        ),
        (
            Value::unknown(Type::string()),
            Value::unknown(Type::string()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::unknown(Type::string()).refine_not_null(),
        ),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got = stdlib::reverse(input).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/string_test.go TestStrlen
#[test]
fn strlen() {
    let tests: Vec<(Value, Value)> = vec![
        (Value::string("hello"), Value::number_int(5)),
        (Value::string(""), Value::number_int(0)),
        (Value::string("1"), Value::number_int(1)),
        (
            Value::string("\u{416}\u{438}\u{432}\u{43e}\u{439} \u{416}\u{443}\u{440}\u{43d}\u{430}\u{43b}"),
            Value::number_int(12),
        ),
        (
            // note that the dieresis here is intentionally a combining
            // ligature.
            Value::string("noe\u{308}l"),
            Value::number_int(4),
        ),
        (
            // The Es in this string has three combining acute accents.
            // This tests something that NFC-normalization cannot collapse
            // into a single precombined codepoint, since otherwise we might
            // be cheating and relying on the single-codepoint forms.
            Value::string("we\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}!"),
            Value::number_int(5),
        ),
        (
            // Go's normalization forms don't handle this ligature, so we
            // will produce the wrong result but this is now a compatibility
            // constraint and so we'll test it.
            Value::string("ba\u{fb04}e"),
            Value::number_int(4),
        ),
        (
            Value::string("\u{1f638}\u{1f63e}"),
            Value::number_int(2),
        ),
        (
            Value::unknown(Type::string()),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::zero(), true)
                .new_value(),
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix("we\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}-")
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::number_int(5), true)
                .new_value(),
        ),
        (
            Value::dynamic(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::zero(), true)
                .new_value(),
        ),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got = stdlib::strlen(input).unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/string_test.go TestSubstr
#[test]
fn substr() {
    let tests: Vec<(Value, Value, Value, Value)> = vec![
        (
            Value::string("hello"),
            Value::number_int(0),
            Value::number_int(2),
            Value::string("he"),
        ),
        (
            Value::string("hello"),
            Value::number_int(1),
            Value::number_int(3),
            Value::string("ell"),
        ),
        (
            Value::string("hello"),
            Value::number_int(1),
            Value::number_int(-1),
            Value::string("ello"),
        ),
        (
            Value::string("hello"),
            Value::number_int(1),
            Value::number_int(-10), // not documented, but <0 is the same as -1
            Value::string("ello"),
        ),
        (
            Value::string("hello"),
            Value::number_int(1),
            Value::number_int(10),
            Value::string("ello"),
        ),
        (
            Value::string("hello"),
            Value::number_int(-3),
            Value::number_int(-1),
            Value::string("llo"),
        ),
        (
            Value::string("hello"),
            Value::number_int(-3),
            Value::number_int(2),
            Value::string("ll"),
        ),
        (
            Value::string("hello"),
            Value::number_int(10),
            Value::number_int(10),
            Value::string(""),
        ),
        (
            Value::string("hello"),
            Value::number_int(0),
            Value::number_int(0),
            Value::string(""),
        ),
        (
            Value::string("noe\u{308}l"),
            Value::number_int(0),
            Value::number_int(3),
            Value::string("noe\u{308}"),
        ),
        (
            Value::string("noe\u{308}l"),
            Value::number_int(3),
            Value::number_int(-1),
            Value::string("l"),
        ),
        (
            Value::string("we\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}!"),
            Value::number_int(2),
            Value::number_int(2),
            Value::string("e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}"),
        ),
        (
            Value::string("we\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}!"),
            Value::number_int(3),
            Value::number_int(2),
            Value::string("e\u{301}\u{301}\u{301}!"),
        ),
        (
            Value::string("we\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}!"),
            Value::number_int(-2),
            Value::number_int(-1),
            Value::string("e\u{301}\u{301}\u{301}!"),
        ),
        (
            Value::string("noe\u{308}l"),
            Value::number_int(-2),
            Value::number_int(-1),
            Value::string("e\u{308}l"),
        ),
        (
            Value::string("\u{1f638}\u{1f63e}"),
            Value::number_int(0),
            Value::number_int(1),
            Value::string("\u{1f638}"),
        ),
        (
            Value::string("\u{1f638}\u{1f63e}"),
            Value::number_int(1),
            Value::number_int(1),
            Value::string("\u{1f63e}"),
        ),
    ];

    for (i, (input, offset, length, want)) in tests.iter().enumerate() {
        let got = stdlib::substr(input, offset, length)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/string_test.go TestJoin
#[test]
fn join() {
    let tests: Vec<(&str, Value, Vec<Value>, Value)> = vec![
        (
            "single two-element list",
            Value::string("-"),
            vec![Value::list([
                Value::string("hello"),
                Value::string("world"),
            ])],
            Value::string("hello-world"),
        ),
        (
            "multiple single-element lists",
            Value::string("-"),
            vec![
                Value::list([Value::string("chicken")]),
                Value::list([Value::string("egg")]),
            ],
            Value::string("chicken-egg"),
        ),
        (
            "single single-element list",
            Value::string("-"),
            vec![Value::list([Value::string("chicken")])],
            Value::string("chicken"),
        ),
        (
            "blank separator",
            Value::string(""),
            vec![Value::list([
                Value::string("horse"),
                Value::string("face"),
            ])],
            Value::string("horseface"),
        ),
        (
            "marked list",
            Value::string("-"),
            vec![Value::list([
                Value::string("hello"),
                Value::string("world"),
            ])
            .mark("sensitive")],
            Value::string("hello-world").mark("sensitive"),
        ),
        (
            "marked separator",
            Value::string("-").mark("sensitive"),
            vec![Value::list([
                Value::string("hello"),
                Value::string("world"),
            ])],
            Value::string("hello-world").mark("sensitive"),
        ),
        (
            "list with some marked elements",
            Value::string("-"),
            vec![Value::list([
                Value::string("hello").mark("sensitive"),
                Value::string("world"),
            ])],
            Value::string("hello-world").mark("sensitive"),
        ),
        (
            "multiple marks",
            Value::string("-").mark("a"),
            vec![Value::list([
                Value::string("hello").mark("b"),
                Value::string("world").mark("c"),
            ])],
            Value::string("hello-world").with_marks([ValueMarks::from_marks(["a", "b", "c"])]),
        ),
    ];

    for (name, separator, lists, want) in tests.iter() {
        let got = stdlib::join(separator, lists)
            .unwrap_or_else(|err| panic!("{name}: unexpected error: {err}"));
        assert_eq!(got, *want, "{name}: wrong result");
    }
}

// upstream: cty/function/stdlib/string_test.go TestSort
#[test]
fn sort() {
    let tests: Vec<(Value, Value, &str)> = vec![
        (
            Value::list_empty(Type::string()),
            Value::list_empty(Type::string()),
            "",
        ),
        (
            Value::list([Value::string("a")]),
            Value::list([Value::string("a")]),
            "",
        ),
        (
            Value::list([Value::string("b"), Value::string("a")]),
            Value::list([Value::string("a"), Value::string("b")]),
            "",
        ),
        (
            Value::list([
                Value::string("b"),
                Value::string("a"),
                Value::string("c"),
            ]),
            Value::list([
                Value::string("a"),
                Value::string("b"),
                Value::string("c"),
            ]),
            "",
        ),
        (
            Value::unknown(Type::list(Type::string())),
            Value::unknown(Type::list(Type::string())).refine_not_null(),
            "",
        ),
        (
            // If the list contains any unknown values then we can still
            // preserve the length of the list by generating a known list
            // with unknown elements, because sort can never change the length.
            Value::list([Value::string("b"), Value::unknown(Type::string())]),
            Value::list([
                Value::unknown(Type::string()),
                Value::unknown(Type::string()),
            ]),
            "",
        ),
        (
            // For a completely unknown list we can still preserve any
            // refinements it had for its length, because sorting can never
            // change the length.
            Value::unknown(Type::list(Type::string()))
                .refine()
                .collection_length_lower_bound(1)
                .collection_length_upper_bound(2)
                .new_value(),
            Value::unknown(Type::list(Type::string()))
                .refine()
                .not_null()
                .collection_length_lower_bound(1)
                .collection_length_upper_bound(2)
                .new_value(),
            "",
        ),
    ];

    for (i, (input, want, want_err)) in tests.iter().enumerate() {
        let result = stdlib::sort(input);

        if !want_err.is_empty() {
            match result {
                Err(err) => {
                    assert_eq!(err.to_string(), *want_err, "case {i}: wrong error");
                }
                Ok(_) => panic!("case {i}: expected error, got success"),
            }
            continue;
        }

        let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/string_replace_test.go TestReplace
#[test]
fn replace() {
    let tests: Vec<(Value, Value, Value, Value)> = vec![
        (
            Value::string("hello"),
            Value::string("l"),
            Value::string(""),
            Value::string("heo"),
        ),
        (
            Value::string("\u{1f638}\u{1f638}\u{1f638}\u{1f63e}\u{1f63e}\u{1f63e}"),
            Value::string("\u{1f63e}"),
            Value::string("\u{1f638}"),
            Value::string("\u{1f638}\u{1f638}\u{1f638}\u{1f638}\u{1f638}\u{1f638}"),
        ),
        (
            Value::string("\u{1f638}\u{1f638}\u{1f638}\u{1f638}\u{1f638}\u{1f63e}"),
            Value::string("\u{1f63e}"),
            Value::string("\u{1f638}"),
            Value::string("\u{1f638}\u{1f638}\u{1f638}\u{1f638}\u{1f638}\u{1f638}"),
        ),
    ];

    for (i, (input, substr, replace, want)) in tests.iter().enumerate() {
        // Upstream runs each case as two subtests ("_replace" and
        // "_regex_replace"), both of which call Replace; ported faithfully.
        {
            let got = stdlib::replace(input, substr, replace)
                .unwrap_or_else(|err| panic!("case {i} (replace): unexpected error: {err}"));
            assert_eq!(got, *want, "case {i} (replace): wrong result");
        }
        {
            let got = stdlib::replace(input, substr, replace)
                .unwrap_or_else(|err| panic!("case {i} (regex_replace): unexpected error: {err}"));
            assert_eq!(got, *want, "case {i} (regex_replace): wrong result");
        }
    }
}

// upstream: cty/function/stdlib/string_replace_test.go TestRegexReplace
#[test]
fn regex_replace() {
    let tests: Vec<(Value, Value, Value, Value)> = vec![
        (
            Value::string("-ab-axxb-"),
            Value::string("a(x*)b"),
            Value::string("T"),
            Value::string("-T-T-"),
        ),
        (
            Value::string("-ab-axxb-"),
            Value::string("a(x*)b"),
            Value::string("${1}W"),
            Value::string("-W-xxW-"),
        ),
    ];

    for (i, (input, substr, replace, want)) in tests.iter().enumerate() {
        let got = stdlib::regex_replace(input, substr, replace)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/string_replace_test.go TestRegexReplaceInvalidRegex
#[test]
fn regex_replace_invalid_regex() {
    let result = stdlib::regex_replace(&Value::string(""), &Value::string("("), &Value::string(""));
    assert!(result.is_err(), "expected an error");
}

// upstream: cty/function/stdlib/regexp_test.go TestRegex
#[test]
fn regex() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::string("[a-z]+"),
            Value::string("135abc456def789"),
            Value::string("abc"),
        ),
        (
            Value::string("([0-9]*)([a-z]*)"),
            Value::string("135abc456def"),
            Value::tuple([Value::string("135"), Value::string("abc")]),
        ),
        (
            Value::string(
                r"^(?:(?P<scheme>[^:/?#]+):)?(?://(?P<authority>[^/?#]*))?(?P<path>[^?#]*)(?:\?(?P<query>[^#]*))?(?:#(?P<fragment>.*))?",
            ),
            Value::string("http://www.ics.uci.edu/pub/ietf/uri/#Related"),
            Value::object([
                ("scheme", Value::string("http")),
                ("authority", Value::string("www.ics.uci.edu")),
                ("path", Value::string("/pub/ietf/uri/")),
                // query portion isn't present at all, because there's no ?
                ("query", Value::null(Type::string())),
                ("fragment", Value::string("Related")),
            ]),
        ),
        (
            Value::string("([0-9]*)([a-z]*)"),
            Value::unknown(Type::string()),
            Value::unknown(Type::tuple([Type::string(), Type::string()])).refine_not_null(),
        ),
        (
            Value::string("(?P<num>[0-9]*)"),
            Value::unknown(Type::string()),
            Value::unknown(Type::object([("num", Type::string())])).refine_not_null(),
        ),
        (
            Value::unknown(Type::string()),
            Value::string("135abc456def"),
            Value::dynamic(),
        ),
        (
            Value::string("[a-z]+").mark(1),
            Value::string("135abc456def789"),
            Value::string("abc").mark(1),
        ),
        (
            Value::string("[a-z]+"),
            Value::string("135abc456def789").mark(2),
            Value::string("abc").mark(2),
        ),
    ];

    for (i, (pattern, string, want)) in tests.iter().enumerate() {
        let got = stdlib::regex(pattern, string)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(
            got, *want,
            "case {i}: wrong result for pattern {pattern:?}, string {string:?}"
        );
    }
}

// upstream: cty/function/stdlib/regexp_test.go TestRegexAll
#[test]
fn regex_all() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::string("[a-z]+"),
            Value::string("135abc456def789"),
            Value::list([Value::string("abc"), Value::string("def")]),
        ),
        (
            Value::string("([0-9]*)([a-z]*)"),
            Value::string("135abc456def"),
            Value::list([
                Value::tuple([Value::string("135"), Value::string("abc")]),
                Value::tuple([Value::string("456"), Value::string("def")]),
            ]),
        ),
        (
            Value::string(
                r"^(?:(?P<scheme>[^:/?#]+):)?(?://(?P<authority>[^/?#]*))?(?P<path>[^?#]*)(?:\?(?P<query>[^#]*))?(?:#(?P<fragment>.*))?",
            ),
            Value::string("http://www.ics.uci.edu/pub/ietf/uri/#Related"),
            Value::list([Value::object([
                ("scheme", Value::string("http")),
                ("authority", Value::string("www.ics.uci.edu")),
                ("path", Value::string("/pub/ietf/uri/")),
                // query portion isn't present at all, because there's no ?
                ("query", Value::null(Type::string())),
                ("fragment", Value::string("Related")),
            ])]),
        ),
        (
            Value::string("([0-9]*)([a-z]*)"),
            Value::unknown(Type::string()),
            Value::unknown(Type::list(Type::tuple([Type::string(), Type::string()])))
                .refine_not_null(),
        ),
        (
            Value::string("(?P<num>[0-9]*)"),
            Value::unknown(Type::string()),
            Value::unknown(Type::list(Type::object([("num", Type::string())]))).refine_not_null(),
        ),
        (
            Value::unknown(Type::string()),
            Value::string("135abc456def"),
            Value::unknown(Type::list(Type::dynamic())).refine_not_null(),
        ),
    ];

    for (i, (pattern, string, want)) in tests.iter().enumerate() {
        let got = stdlib::regex_all(pattern, string)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(
            got, *want,
            "case {i}: wrong result for pattern {pattern:?}, string {string:?}"
        );
    }
}
