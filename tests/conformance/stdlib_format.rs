//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/function/stdlib/format_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::function::stdlib::{format, format_list};
use cty::{Type, Value, ValueMarks};

// upstream: cty/function/stdlib/format_test.go TestFormat
//
// NOTE(port): upstream's `Want: cty.NilVal` (no expected value because an
// error is expected) is represented as `None`; as upstream, the `want` value
// is not consulted when `want_err` is non-empty.
#[test]
fn format_test() {
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
        (Value::string("100%%"), vec![], Some(Value::string("100%")), ""),
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
            vec![Value::number_int(2)],
            Some(Value::string("number 2")),
            "",
        ),
        (
            Value::string("number %#v"),
            vec![Value::number_int(2)],
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
        // Integer Numbers
        (
            Value::string("%d green bottles standing on the wall"),
            vec![Value::number_int(10)],
            Some(Value::string("10 green bottles standing on the wall")),
            "",
        ),
        (
            Value::string("%[2]d things"),
            vec![Value::number_int(1), Value::number_int(10)],
            Some(Value::string("10 things")),
            "",
        ),
        (
            Value::string("%+d green bottles standing on the wall"),
            vec![Value::number_int(10)],
            Some(Value::string("+10 green bottles standing on the wall")),
            "",
        ),
        (
            Value::string("% d green bottles standing on the wall"),
            vec![Value::number_int(10)],
            Some(Value::string(" 10 green bottles standing on the wall")),
            "",
        ),
        (
            Value::string("%5d green bottles standing on the wall"),
            vec![Value::number_int(10)],
            Some(Value::string("   10 green bottles standing on the wall")),
            "",
        ),
        (
            Value::string("%-5d green bottles standing on the wall"),
            vec![Value::number_int(10)],
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
            vec![Value::number_int(5)],
            Some(Value::string("101")),
            "",
        ),
        (
            Value::string("%o"),
            vec![Value::number_int(9)],
            Some(Value::string("11")),
            "",
        ),
        (
            Value::string("%x"),
            vec![Value::number_int(254)],
            Some(Value::string("fe")),
            "",
        ),
        (
            Value::string("%X"),
            vec![Value::number_int(254)],
            Some(Value::string("FE")),
            "",
        ),
        // Floating-point numbers
        (
            Value::string("%f things"),
            vec![Value::number_int(10)],
            Some(Value::string("10.000000 things")),
            "",
        ),
        (
            Value::string("%[2]f things"),
            vec![Value::number_int(1), Value::number_int(10)],
            Some(Value::string("10.000000 things")),
            "",
        ),
        (
            Value::string("%+f things"),
            vec![Value::number_int(10)],
            Some(Value::string("+10.000000 things")),
            "",
        ),
        (
            Value::string("% f things"),
            vec![Value::number_int(10)],
            Some(Value::string(" 10.000000 things")),
            "",
        ),
        (
            Value::string("%+f things"),
            vec![Value::number_int(-10)],
            Some(Value::string("-10.000000 things")),
            "",
        ),
        (
            Value::string("% f things"),
            vec![Value::number_int(-10)],
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
            vec![Value::number_int(1000)],
            Some(Value::string("1.000000e+03 things")),
            "",
        ),
        (
            Value::string("%E things"),
            vec![Value::number_int(1000)],
            Some(Value::string("1.000000E+03 things")),
            "",
        ),
        (
            Value::string("%g things"),
            vec![Value::number_int(1000)],
            Some(Value::string("1000 things")),
            "",
        ),
        (
            Value::string("%G things"),
            vec![Value::number_int(1000)],
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
            vec![Value::number_int(10)],
            None,
            r#"unsupported format verb 'z' in "%z" at offset 0"#,
        ),
        (
            Value::string("%#z is not a valid sequence"),
            vec![Value::number_int(10)],
            None,
            r#"unsupported format verb 'z' in "%#z" at offset 0"#,
        ),
        (
            Value::string("%012z is not a valid sequence"),
            vec![Value::number_int(10)],
            None,
            r#"unsupported format verb 'z' in "%012z" at offset 0"#,
        ),
        (
            Value::string("%☠ is not a valid sequence"),
            vec![Value::number_int(10)],
            None,
            "unrecognized format character '☠' at offset 1",
        ),
        (
            Value::string("%💃🏿 is not a valid sequence"),
            vec![Value::number_int(10)],
            None,
            "unrecognized format character '💃' at offset 1", // since this is a grammar-level error, we don't get the full grapheme cluster
        ),
        (
            Value::null(Type::string()),
            vec![Value::number_int(10)],
            None,
            "argument must not be null",
        ),
        (
            Value::string("no format verbs at all"),
            vec![Value::number_int(10)],
            None,
            "too many arguments; no verbs in format string",
        ),
        (
            Value::string("only one verb %d"),
            vec![Value::number_int(10), Value::number_int(11)],
            None,
            "too many arguments; only 1 used by format string",
        ),
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

    for (i, (format_val, args, want, want_err)) in tests.iter().enumerate() {
        let result = format(format_val, args);

        if want_err.is_empty() {
            let got = result
                .unwrap_or_else(|err| panic!("case {i} ({format_val:?}): unexpected error: {err}"));
            let want = want
                .as_ref()
                .unwrap_or_else(|| panic!("case {i} ({format_val:?}): missing want value"));
            assert!(
                got == *want,
                "case {i} ({format_val:?}): wrong result\ngot:  {got:?}\nwant: {want:?}"
            );
        } else {
            let err = match result {
                Ok(got) => panic!("case {i} ({format_val:?}): unexpected success {got:?}; want error"),
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

// upstream: cty/function/stdlib/format_test.go TestFormatList
//
// NOTE(port): upstream's `Want: cty.NilVal` (no expected value because an
// error is expected) is represented as `None`; as upstream, the `want` value
// is not consulted when `want_err` is non-empty.
#[test]
fn format_list_test() {
    let tests: Vec<(Value, Vec<Value>, Option<Value>, &str)> = vec![
        // 0:
        (
            Value::string(""),
            vec![],
            Some(Value::list([Value::string("")])),
            "",
        ),
        // 1:
        (
            Value::string("hello"),
            vec![],
            Some(Value::list([Value::string("hello")])),
            "",
        ),
        // 2:
        (
            Value::string("100%% successful"),
            vec![],
            Some(Value::list([Value::string("100% successful")])),
            "",
        ),
        // 3:
        (
            Value::string("100%%"),
            vec![],
            Some(Value::list([Value::string("100%")])),
            "",
        ),
        // 4:
        (
            Value::string("%s"),
            vec![Value::string("hello")],
            Some(Value::list([Value::string("hello")])),
            "",
        ),
        // 5:
        (
            Value::string("%s"),
            vec![Value::list([Value::string("hello")])],
            Some(Value::list([Value::string("hello")])),
            "",
        ),
        // 6:
        (
            Value::string("%s"),
            vec![Value::list([Value::string("hello"), Value::string("world")])],
            Some(Value::list([Value::string("hello"), Value::string("world")])),
            "",
        ),
        // 7:
        (
            Value::string("%s %s"),
            vec![
                Value::list([Value::string("hello"), Value::string("goodbye")]),
                Value::list([Value::string("world"), Value::string("universe")]),
            ],
            Some(Value::list([
                Value::string("hello world"),
                Value::string("goodbye universe"),
            ])),
            "",
        ),
        // 8:
        (
            Value::string("%s %s"),
            vec![
                Value::list([Value::string("hello"), Value::string("goodbye")]),
                Value::string("world"),
            ],
            Some(Value::list([
                Value::string("hello world"),
                Value::string("goodbye world"),
            ])),
            "",
        ),
        // 9:
        (
            Value::string("%s %s"),
            vec![
                Value::string("hello"),
                Value::list([Value::string("world"), Value::string("universe")]),
            ],
            Some(Value::list([
                Value::string("hello world"),
                Value::string("hello universe"),
            ])),
            "",
        ),
        // 10:
        (
            Value::string("%s %s"),
            vec![
                Value::list([Value::string("hello"), Value::string("goodbye")]),
                Value::list([Value::string("world")]),
            ],
            Some(Value::list_empty(Type::string())),
            "argument 2 has length 1, which is inconsistent with argument 1 of length 2",
        ),
        // 11:
        (
            Value::string("%s"),
            vec![Value::empty_object()],
            Some(Value::list_empty(Type::string())),
            r#"error on format iteration 0: unsupported value for "%s" at 0: string required, but have object"#,
        ),
        // 12:
        (
            Value::string("%v"),
            vec![Value::empty_tuple()],
            Some(Value::list_empty(Type::string())), // no items because our given tuple is empty
            "",
        ),
        // 13:
        (
            Value::string("%v"),
            vec![Value::null(Type::list(Type::string()))],
            Some(Value::list([
                Value::string("null"), // we treat a null list like a list whose elements are all null
            ])),
            "",
        ),
        // 14:
        (
            Value::unknown(Type::string()),
            vec![Value::bool(true)],
            Some(Value::unknown(Type::list(Type::string())).refine_not_null()),
            "",
        ),
        // 15:
        (
            Value::string("%v"),
            vec![Value::unknown(Type::string())],
            Some(Value::list([Value::unknown(Type::string()).refine_not_null()])),
            "",
        ),
        // 16:
        (
            Value::string("%v"),
            vec![Value::null(Type::string())],
            Some(Value::list([Value::string("null")])),
            "",
        ),
        // 17:
        (
            Value::string("%v"),
            vec![Value::unknown(Type::list(Type::string()))],
            Some(Value::unknown(Type::list(Type::string())).refine_not_null()),
            "",
        ),
        // 18:
        (
            Value::string("%v"),
            vec![Value::list([
                Value::tuple([Value::string("hello")]),
                Value::tuple([Value::unknown(Type::string())]),
                Value::tuple([Value::string("world")]),
            ])],
            Some(Value::list([
                Value::string(r#"["hello"]"#),
                Value::unknown(Type::string()).refine_not_null(),
                Value::string(r#"["world"]"#),
            ])),
            "",
        ),
        // 19:
        (
            Value::string("%v"),
            vec![Value::unknown(Type::tuple([Type::string()]))],
            Some(Value::unknown(Type::list(Type::string())).refine_not_null()),
            "",
        ),
        // 20:
        (
            Value::string("%s %s"),
            vec![
                Value::unknown(Type::tuple([Type::string()])),
                Value::unknown(Type::tuple([Type::string(), Type::string()])),
            ],
            Some(Value::unknown(Type::list(Type::string())).refine_not_null()),
            "argument 2 has length 2, which is inconsistent with argument 1 of length 1",
        ),
        // 21:
        (
            Value::string("%s %s"),
            vec![
                Value::list([Value::string("hi")]),
                Value::unknown(Type::tuple([Type::string(), Type::string()])),
            ],
            Some(Value::unknown(Type::list(Type::string())).refine_not_null()),
            "argument 2 has length 2, which is inconsistent with argument 1 of length 1",
        ),
        // 22:
        (
            Value::string("%v"),
            vec![Value::set([
                Value::string("hello"),
                Value::unknown(Type::string()),
            ])],
            Some(Value::unknown(Type::list(Type::string())).refine_not_null()),
            "",
        ),
        // 23:
        (
            Value::string("%v"),
            vec![Value::dynamic()],
            Some(Value::unknown(Type::list(Type::string())).refine_not_null()),
            "",
        ),
        // 24:
        (
            Value::string("%v"),
            vec![Value::null(Type::dynamic())],
            Some(Value::list([Value::string("null")])),
            "",
        ),
        // 25:
        (
            Value::string("%v %v"),
            vec![
                Value::null(Type::dynamic()),
                Value::list([
                    Value::string("a"),
                    Value::null(Type::string()),
                    Value::string("c"),
                ]),
            ],
            Some(Value::list([
                Value::string("null a"),
                Value::string("null null"),
                Value::string("null c"),
            ])),
            "",
        ),
        // 26:
        (
            Value::string("%v %v"),
            vec![
                Value::null(Type::dynamic()),
                Value::list([Value::null(Type::dynamic()), Value::null(Type::dynamic())]),
            ],
            Some(Value::list([
                Value::string("null null"),
                Value::string("null null"),
            ])),
            "",
        ),
    ];

    for (i, (format_val, args, want, want_err)) in tests.iter().enumerate() {
        let result = format_list(format_val, args);

        if want_err.is_empty() {
            let got = result
                .unwrap_or_else(|err| panic!("case {i} ({format_val:?}): unexpected error: {err}"));
            let want = want
                .as_ref()
                .unwrap_or_else(|| panic!("case {i} ({format_val:?}): missing want value"));
            assert!(
                got == *want,
                "case {i} ({format_val:?}): wrong result\ngot:  {got:?}\nwant: {want:?}"
            );
        } else {
            let err = match result {
                Ok(got) => panic!("case {i} ({format_val:?}): unexpected success {got:?}; want error"),
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
