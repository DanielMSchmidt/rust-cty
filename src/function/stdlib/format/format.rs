//! `format` (go-cty: `cty/function/stdlib/format.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`format`] (go-cty: `stdlib.FormatFunc`).
pub fn format_func() -> Function {
    todo!()
}

/// Produces a string by formatting values per a printf-like format string
/// (go-cty: `stdlib.Format`).
pub fn format(format: &Value, vals: &[Value]) -> Result<Value, CtyError> {
    let _ = (format, vals);
    todo!()
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/function/stdlib/format_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::function::stdlib::format;
    use crate::{Type, Value, ValueMarks};

    // Ported from TestFormat:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/format_test.go#L10
    //
    // NOTE(port): upstream's `Want: cty.NilVal` (no expected value because an
    // error is expected) is represented as `None`; as upstream, the `want` value
    // is not consulted when `want_err` is non-empty.
    //
    // Upstream TestFormat is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod format_test {
        use super::*;

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[(Value, Vec<Value>, Option<Value>, &str)]) {
            for (i, (format_val, args, want, want_err)) in tests.iter().enumerate() {
                let result = format(format_val, args);

                if want_err.is_empty() {
                    let got = result.unwrap_or_else(|err| {
                        panic!("case {i} ({format_val:?}): unexpected error: {err}")
                    });
                    let want = want
                        .as_ref()
                        .unwrap_or_else(|| panic!("case {i} ({format_val:?}): missing want value"));
                    assert!(
                        got == *want,
                        "case {i} ({format_val:?}): wrong result\ngot:  {got:?}\nwant: {want:?}"
                    );
                } else {
                    let err = match result {
                        Ok(got) => {
                            panic!(
                                "case {i} ({format_val:?}): unexpected success {got:?}; want error"
                            )
                        }
                        Err(err) => err,
                    };
                    let err_str = err.to_string();
                    assert_eq!(
                        err_str, *want_err,
                        "case {i} ({format_val:?}): wrong error\ngot:  {err_str}\nwant: {want_err}"
                    );
                }
            }
        }

        // TestFormat, general:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/format_test.go#L10
        #[test]
        #[ignore = "not yet implemented"]
        fn general() {
            let tests: Vec<(Value, Vec<Value>, Option<Value>, &str)> = vec![
                (Value::string(""), vec![], Some(Value::string("")), ""),
                (
                    Value::string("hello"),
                    vec![],
                    Some(Value::string("hello")),
                    "",
                ),
                (
                    Value::string("100%% successful"),
                    vec![],
                    Some(Value::string("100% successful")),
                    "",
                ),
                (
                    Value::string("100%%"),
                    vec![],
                    Some(Value::string("100%")),
                    "",
                ),
            ];

            check(&tests);
        }

        // TestFormat, default formats:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/format_test.go#L10
        #[test]
        #[ignore = "not yet implemented"]
        fn default_formats() {
            let tests: Vec<(Value, Vec<Value>, Option<Value>, &str)> = vec![
                // Default formats
                (
                    Value::string("string %v"),
                    vec![Value::string("hello")],
                    Some(Value::string("string hello")),
                    "",
                ),
                (
                    Value::string("string %[2]v"),
                    vec![Value::bool(true), Value::string("hello")],
                    Some(Value::string("string hello")),
                    "",
                ),
                (
                    Value::string("string %#v"),
                    vec![Value::string("hello")],
                    Some(Value::string(r#"string "hello""#)),
                    "",
                ),
                (
                    Value::string("number %v"),
                    vec![Value::number(2)],
                    Some(Value::string("number 2")),
                    "",
                ),
                (
                    Value::string("number %#v"),
                    vec![Value::number(2)],
                    Some(Value::string("number 2")),
                    "",
                ),
                (
                    Value::string("bool %v"),
                    vec![Value::bool(true)],
                    Some(Value::string("bool true")),
                    "",
                ),
                (
                    Value::string("bool %#v"),
                    vec![Value::bool(true)],
                    Some(Value::string("bool true")),
                    "",
                ),
                (
                    Value::string("object %v"),
                    vec![Value::empty_object()],
                    Some(Value::string("object {}")),
                    "",
                ),
                (
                    Value::string("tuple %v"),
                    vec![Value::empty_tuple()],
                    Some(Value::string("tuple []")),
                    "",
                ),
                (
                    Value::string("tuple with unknown %v"),
                    vec![Value::tuple([Value::unknown(Type::string())])],
                    Some(
                        Value::unknown(Type::string())
                            .refine()
                            .not_null()
                            .string_prefix_full("tuple with unknown ")
                            .new_value(),
                    ),
                    "",
                ),
                (
                    Value::string("%%%v"),
                    vec![Value::bool(false)],
                    Some(Value::string("%false")),
                    "",
                ),
                (
                    Value::string("%v"),
                    vec![Value::null(Type::bool())],
                    Some(Value::string("null")),
                    "",
                ),
                (
                    Value::string("%v"),
                    vec![Value::null(Type::dynamic())],
                    Some(Value::string("null")),
                    "",
                ),
            ];

            check(&tests);
        }

        // TestFormat, strings:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/format_test.go#L10
        #[test]
        #[ignore = "not yet implemented"]
        fn strings() {
            let tests: Vec<(Value, Vec<Value>, Option<Value>, &str)> = vec![
                // Strings
                (
                    Value::string("Hello, %s!"),
                    vec![Value::string("Ermintrude")],
                    Some(Value::string("Hello, Ermintrude!")),
                    "",
                ),
                (
                    Value::string("Hello, %[2]s!"),
                    vec![Value::string("Stephen"), Value::string("Ermintrude")],
                    Some(Value::string("Hello, Ermintrude!")),
                    "",
                ),
                (
                    Value::string("Hello, %q... if that _is_ your real name!"),
                    vec![Value::string("Ermintrude")],
                    Some(Value::string(
                        r#"Hello, "Ermintrude"... if that _is_ your real name!"#,
                    )),
                    "",
                ),
                (
                    Value::string("This statement is %s"),
                    vec![Value::bool(false)],
                    Some(Value::string("This statement is false")),
                    "",
                ),
                (
                    Value::string("This statement is %q"),
                    vec![Value::bool(false)],
                    Some(Value::string(r#"This statement is "false""#)),
                    "",
                ),
                (
                    Value::string("%s"),
                    vec![Value::null(Type::string())],
                    None,
                    r#"unsupported value for "%s" at 0: null value cannot be formatted"#,
                ),
                (
                    Value::string("%s"),
                    vec![Value::null(Type::dynamic())],
                    None,
                    r#"unsupported value for "%s" at 0: null value cannot be formatted"#,
                ),
                (
                    Value::string("%10s"),
                    vec![Value::string("hello")],
                    Some(Value::string("     hello")),
                    "",
                ),
                (
                    Value::string("%-10s"),
                    vec![Value::string("hello")],
                    Some(Value::string("hello     ")),
                    "",
                ),
                (
                    Value::string("%4s"),
                    vec![Value::string("💃🏿")],
                    Some(Value::string("   💃🏿")), // three spaces because this emoji sequence is a single grapheme cluster
                    "",
                ),
                (
                    Value::string("%-4s"),
                    vec![Value::string("💃🏿")],
                    Some(Value::string("💃🏿   ")), // three spaces because this emoji sequence is a single grapheme cluster
                    "",
                ),
                (
                    Value::string("%q"),
                    vec![Value::string("💃🏿")],
                    Some(Value::string(r#""💃🏿""#)),
                    "",
                ),
                (
                    Value::string("%6q"),
                    vec![Value::string("💃🏿")],
                    Some(Value::string(r#"   "💃🏿""#)), // three spaces because this emoji sequence is a single grapheme cluster
                    "",
                ),
                (
                    Value::string("%-6q"),
                    vec![Value::string("💃🏿")],
                    Some(Value::string(r#""💃🏿"   "#)), // three spaces because this emoji sequence is a single grapheme cluster
                    "",
                ),
                (
                    Value::string("%.2s"),
                    vec![Value::string("hello")],
                    Some(Value::string("he")),
                    "",
                ),
                (
                    Value::string("%.2q"),
                    vec![Value::string("hello")],
                    Some(Value::string(r#""he""#)),
                    "",
                ),
                (
                    Value::string("%.5s"),
                    vec![Value::string("日本語日本語")],
                    Some(Value::string("日本語日本")),
                    "",
                ),
                (
                    Value::string("%.1q"),
                    vec![Value::string("日本語日本語")],
                    Some(Value::string(r#""日""#)),
                    "",
                ),
                (
                    Value::string("%.10s"),
                    vec![Value::string("hello")],
                    Some(Value::string("hello")),
                    "",
                ),
                (
                    Value::string("%4.2s"),
                    vec![Value::string("hello")],
                    Some(Value::string("  he")),
                    "",
                ),
                (
                    Value::string("%6.2q"),
                    vec![Value::string("hello")],
                    Some(Value::string(r#"  "he""#)),
                    "",
                ),
                (
                    Value::string("%-4.2s"),
                    vec![Value::string("hello")],
                    Some(Value::string("he  ")),
                    "",
                ),
                (
                    Value::string("%q"),
                    vec![Value::string("Hello\nWorld")],
                    Some(Value::string(r#""Hello\nWorld""#)),
                    "",
                ),
            ];

            check(&tests);
        }

        // TestFormat, booleans:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/format_test.go#L10
        #[test]
        #[ignore = "not yet implemented"]
        fn booleans() {
            let tests: Vec<(Value, Vec<Value>, Option<Value>, &str)> = vec![
                // Booleans
                (
                    Value::string("This statement is %t"),
                    vec![Value::bool(false)],
                    Some(Value::string("This statement is false")),
                    "",
                ),
                (
                    Value::string("This statement is %[2]t"),
                    vec![Value::bool(true), Value::bool(false)],
                    Some(Value::string("This statement is false")),
                    "",
                ),
                (
                    Value::string("This statement is %t"),
                    vec![Value::bool(true)],
                    Some(Value::string("This statement is true")),
                    "",
                ),
                (
                    Value::string("This statement is %t"),
                    vec![Value::string("false")],
                    Some(Value::string("This statement is false")),
                    "",
                ),
                (
                    Value::string("This statement is %t"),
                    vec![Value::null(Type::bool())],
                    None,
                    r#"unsupported value for "%t" at 18: null value cannot be formatted"#,
                ),
                (
                    Value::string("This statement is %t"),
                    vec![Value::null(Type::dynamic())],
                    None,
                    r#"unsupported value for "%t" at 18: null value cannot be formatted"#,
                ),
            ];

            check(&tests);
        }

        // TestFormat, integer numbers:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/format_test.go#L10
        #[test]
        #[ignore = "not yet implemented"]
        fn integer_numbers() {
            let tests: Vec<(Value, Vec<Value>, Option<Value>, &str)> = vec![
                // Integer Numbers
                (
                    Value::string("%d green bottles standing on the wall"),
                    vec![Value::number(10)],
                    Some(Value::string("10 green bottles standing on the wall")),
                    "",
                ),
                (
                    Value::string("%[2]d things"),
                    vec![Value::number(1), Value::number(10)],
                    Some(Value::string("10 things")),
                    "",
                ),
                (
                    Value::string("%+d green bottles standing on the wall"),
                    vec![Value::number(10)],
                    Some(Value::string("+10 green bottles standing on the wall")),
                    "",
                ),
                (
                    Value::string("% d green bottles standing on the wall"),
                    vec![Value::number(10)],
                    Some(Value::string(" 10 green bottles standing on the wall")),
                    "",
                ),
                (
                    Value::string("%5d green bottles standing on the wall"),
                    vec![Value::number(10)],
                    Some(Value::string("   10 green bottles standing on the wall")),
                    "",
                ),
                (
                    Value::string("%-5d green bottles standing on the wall"),
                    vec![Value::number(10)],
                    Some(Value::string("10    green bottles standing on the wall")),
                    "",
                ),
                (
                    Value::string("%d green bottles standing on the wall"),
                    vec![Value::bool(true)],
                    None,
                    r#"unsupported value for "%d" at 0: number required, but have bool"#,
                ),
                (
                    Value::string("%d green bottles standing on the wall"),
                    vec![Value::null(Type::number())],
                    None,
                    r#"unsupported value for "%d" at 0: null value cannot be formatted"#,
                ),
                (
                    Value::string("%d green bottles standing on the wall"),
                    vec![Value::null(Type::empty_tuple())],
                    None,
                    r#"unsupported value for "%d" at 0: null value cannot be formatted"#,
                ),
                (
                    Value::string("%d green bottles standing on the wall"),
                    vec![Value::null(Type::dynamic())],
                    None,
                    r#"unsupported value for "%d" at 0: null value cannot be formatted"#,
                ),
                (
                    Value::string("%b"),
                    vec![Value::number(5)],
                    Some(Value::string("101")),
                    "",
                ),
                (
                    Value::string("%o"),
                    vec![Value::number(9)],
                    Some(Value::string("11")),
                    "",
                ),
                (
                    Value::string("%x"),
                    vec![Value::number(254)],
                    Some(Value::string("fe")),
                    "",
                ),
                (
                    Value::string("%X"),
                    vec![Value::number(254)],
                    Some(Value::string("FE")),
                    "",
                ),
            ];

            check(&tests);
        }

        // TestFormat, floating point numbers:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/format_test.go#L10
        #[test]
        #[ignore = "not yet implemented"]
        fn floating_point_numbers() {
            let tests: Vec<(Value, Vec<Value>, Option<Value>, &str)> = vec![
                // Floating-point numbers
                (
                    Value::string("%f things"),
                    vec![Value::number(10)],
                    Some(Value::string("10.000000 things")),
                    "",
                ),
                (
                    Value::string("%[2]f things"),
                    vec![Value::number(1), Value::number(10)],
                    Some(Value::string("10.000000 things")),
                    "",
                ),
                (
                    Value::string("%+f things"),
                    vec![Value::number(10)],
                    Some(Value::string("+10.000000 things")),
                    "",
                ),
                (
                    Value::string("% f things"),
                    vec![Value::number(10)],
                    Some(Value::string(" 10.000000 things")),
                    "",
                ),
                (
                    Value::string("%+f things"),
                    vec![Value::number(-10)],
                    Some(Value::string("-10.000000 things")),
                    "",
                ),
                (
                    Value::string("% f things"),
                    vec![Value::number(-10)],
                    Some(Value::string("-10.000000 things")),
                    "",
                ),
                (
                    Value::string("%f things"),
                    vec![Value::string("100000000000000000000000000000000000001")],
                    Some(Value::string(
                        "100000000000000000000000000000000000001.000000 things",
                    )),
                    "",
                ),
                (
                    Value::string("%f things"),
                    vec![Value::string("1.00000000000000000000000000000000000001")],
                    Some(Value::string("1.000000 things")),
                    "",
                ),
                (
                    Value::string("%.4f things"),
                    vec![Value::string("1.00000000000000000000000000000000000001")],
                    Some(Value::string("1.0000 things")),
                    "",
                ),
                (
                    Value::string("%.1f things"),
                    vec![Value::string("1.06")],
                    Some(Value::string("1.1 things")),
                    "",
                ),
                (
                    Value::string("%e things"),
                    vec![Value::number(1000)],
                    Some(Value::string("1.000000e+03 things")),
                    "",
                ),
                (
                    Value::string("%E things"),
                    vec![Value::number(1000)],
                    Some(Value::string("1.000000E+03 things")),
                    "",
                ),
                (
                    Value::string("%g things"),
                    vec![Value::number(1000)],
                    Some(Value::string("1000 things")),
                    "",
                ),
                (
                    Value::string("%G things"),
                    vec![Value::number(1000)],
                    Some(Value::string("1000 things")),
                    "",
                ),
                (
                    Value::string("%g things"),
                    vec![Value::string("0.00000000000000000000001")],
                    Some(Value::string("1e-23 things")),
                    "",
                ),
                (
                    Value::string("%G things"),
                    vec![Value::string("0.00000000000000000000001")],
                    Some(Value::string("1E-23 things")),
                    "",
                ),
            ];

            check(&tests);
        }

        // TestFormat, unknowns:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/format_test.go#L10
        #[test]
        #[ignore = "not yet implemented"]
        fn unknowns() {
            let tests: Vec<(Value, Vec<Value>, Option<Value>, &str)> = vec![
                // Unknowns
                (
                    Value::unknown(Type::string()),
                    vec![Value::bool(true)],
                    Some(Value::unknown(Type::string()).refine_not_null()),
                    "",
                ),
                (
                    Value::unknown(Type::bool()),
                    vec![Value::bool(true)],
                    None,
                    "string required, but received bool",
                ),
                (
                    Value::string("Hello, %s!"),
                    vec![Value::unknown(Type::string())],
                    Some(
                        Value::unknown(Type::string())
                            .refine()
                            .not_null()
                            .string_prefix_full("Hello, ")
                            .new_value(),
                    ),
                    "",
                ),
                (
                    Value::string("Hello%s"),
                    vec![Value::unknown(Type::string())],
                    // We lose the trailing "o" in the prefix here because the unknown
                    // value could potentially start with a combining diacritic, which
                    // would therefore combine into a different character.
                    Some(
                        Value::unknown(Type::string())
                            .refine()
                            .not_null()
                            .string_prefix_full("Hell")
                            .new_value(),
                    ),
                    "",
                ),
                (
                    Value::string("Hello, %[2]s!"),
                    vec![Value::unknown(Type::string()), Value::string("Ermintrude")],
                    Some(
                        Value::unknown(Type::string())
                            .refine()
                            .not_null()
                            .string_prefix_full("Hello, ")
                            .new_value(),
                    ),
                    "",
                ),
                (
                    Value::string("%s!"),
                    vec![Value::unknown(Type::string())],
                    Some(Value::unknown(Type::string()).refine_not_null()),
                    "",
                ),
                (
                    Value::string("%v"),
                    vec![Value::dynamic()],
                    Some(Value::unknown(Type::string()).refine_not_null()),
                    "",
                ),
            ];

            check(&tests);
        }

        // TestFormat, invalids:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/format_test.go#L10
        #[test]
        #[ignore = "not yet implemented"]
        fn invalids() {
            let tests: Vec<(Value, Vec<Value>, Option<Value>, &str)> = vec![
                // Invalids
                (
                    Value::string("%s is not in the args list"),
                    vec![],
                    None,
                    r#"not enough arguments for "%s" at 0: need index 1 but have 0 total"#,
                ),
                (
                    Value::string("%[3]s is not in the args list"),
                    vec![Value::bool(true), Value::bool(true)],
                    None,
                    r#"not enough arguments for "%[3]s" at 0: need index 3 but have 2 total"#,
                ),
                (
                    Value::string("%[0]s is not valid because args are 1-based"),
                    vec![Value::bool(true), Value::bool(true)],
                    None,
                    "unrecognized format character '0' at offset 2",
                ),
                (
                    Value::string("%v %v %v"),
                    vec![Value::bool(true), Value::bool(true)],
                    None,
                    r#"not enough arguments for "%v" at 6: need index 3 but have 2 total"#,
                ),
                (
                    Value::string("%z is not a valid sequence"),
                    vec![Value::number(10)],
                    None,
                    r#"unsupported format verb 'z' in "%z" at offset 0"#,
                ),
                (
                    Value::string("%#z is not a valid sequence"),
                    vec![Value::number(10)],
                    None,
                    r#"unsupported format verb 'z' in "%#z" at offset 0"#,
                ),
                (
                    Value::string("%012z is not a valid sequence"),
                    vec![Value::number(10)],
                    None,
                    r#"unsupported format verb 'z' in "%012z" at offset 0"#,
                ),
                (
                    Value::string("%☠ is not a valid sequence"),
                    vec![Value::number(10)],
                    None,
                    "unrecognized format character '☠' at offset 1",
                ),
                (
                    Value::string("%💃🏿 is not a valid sequence"),
                    vec![Value::number(10)],
                    None,
                    "unrecognized format character '💃' at offset 1", // since this is a grammar-level error, we don't get the full grapheme cluster
                ),
                (
                    Value::null(Type::string()),
                    vec![Value::number(10)],
                    None,
                    "argument must not be null",
                ),
                (
                    Value::string("no format verbs at all"),
                    vec![Value::number(10)],
                    None,
                    "too many arguments; no verbs in format string",
                ),
                (
                    Value::string("only one verb %d"),
                    vec![Value::number(10), Value::number(11)],
                    None,
                    "too many arguments; only 1 used by format string",
                ),
            ];

            check(&tests);
        }

        // TestFormat, marked values:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/format_test.go#L10
        #[test]
        #[ignore = "not yet implemented"]
        fn marked_values() {
            let tests: Vec<(Value, Vec<Value>, Option<Value>, &str)> = vec![
                // Marked values
                (
                    Value::string("hello %s").mark(1),
                    vec![Value::string("world")],
                    Some(Value::string("hello world").mark(1)),
                    "",
                ),
                (
                    Value::string("hello %s"),
                    vec![Value::string("world").mark(1)],
                    Some(Value::string("hello world").mark(1)),
                    "",
                ),
                (
                    Value::string("hello %s").mark(0),
                    vec![Value::string("world").mark(1)],
                    Some(Value::string("hello world").with_marks([ValueMarks::from_marks([0, 1])])),
                    "",
                ),
            ];

            check(&tests);
        }
    }
}
