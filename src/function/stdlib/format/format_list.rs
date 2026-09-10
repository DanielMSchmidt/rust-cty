//! `format_list` (go-cty: `cty/function/stdlib/format.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`format_list`] (go-cty: `stdlib.FormatListFunc`).
pub fn format_list_func() -> Function {
    todo!()
}

/// Like [`format`], but iterating over sequence arguments to produce a list
/// of strings (go-cty: `stdlib.FormatList`).
pub fn format_list(format: &Value, vals: &[Value]) -> Result<Value, CtyError> {
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

    use crate::function::stdlib::format_list;
    use crate::{Type, Value};

    // Ported from TestFormatList:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/format_test.go#L657
    //
    // NOTE(port): upstream's `Want: cty.NilVal` (no expected value because an
    // error is expected) is represented as `None`; as upstream, the `want` value
    // is not consulted when `want_err` is non-empty.
    //
    // Upstream TestFormatList is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod format_list_test {
        use super::*;

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[(Value, Vec<Value>, Option<Value>, &str)]) {
            for (i, (format_val, args, want, want_err)) in tests.iter().enumerate() {
                let result = format_list(format_val, args);

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

        // TestFormatList, collections:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/format_test.go#L657
        #[test]
        #[ignore = "not yet implemented"]
        fn collections() {
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
                    vec![Value::list([
                        Value::string("hello"),
                        Value::string("world"),
                    ])],
                    Some(Value::list([
                        Value::string("hello"),
                        Value::string("world"),
                    ])),
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
                // 16:
                (
                    Value::string("%v"),
                    vec![Value::null(Type::string())],
                    Some(Value::list([Value::string("null")])),
                    "",
                ),
            ];

            check(&tests);
        }

        // TestFormatList, unknowns:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/format_test.go#L657
        #[test]
        #[ignore = "not yet implemented"]
        fn unknowns() {
            let tests: Vec<(Value, Vec<Value>, Option<Value>, &str)> = vec![
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
                    Some(Value::list([
                        Value::unknown(Type::string()).refine_not_null()
                    ])),
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

            check(&tests);
        }
    }
}
