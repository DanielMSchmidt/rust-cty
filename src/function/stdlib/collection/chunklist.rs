//! `chunklist` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`chunklist`] (go-cty: `stdlib.ChunklistFunc`).
pub fn chunklist_func() -> Function {
    todo!()
}

/// The list split into fixed-size chunks (go-cty: `stdlib.Chunklist`).
pub fn chunklist(list: &Value, size: &Value) -> Result<Value, CtyError> {
    let _ = (list, size);
    todo!()
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/function/stdlib/collection_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::function::stdlib;
    use crate::{Type, Value};

    // Ported from TestChunklist:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L94
    //
    // Upstream TestChunklist is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod chunklist {
        use super::*;

        // NOTE(port): upstream's `Want` field is `cty.NilVal` in the error cases;
        // those cases carry `None` here.
        struct Case {
            list: Value,
            len: Value,
            want: Option<Value>,
            err: &'static str,
        }

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[Case]) {
            for (i, case) in tests.iter().enumerate() {
                let got = stdlib::chunklist(&case.list, &case.len);
                if !case.err.is_empty() {
                    let err = got.err().unwrap_or_else(|| {
                        panic!("case {i}: succeeded; want error");
                    });
                    assert_eq!(err.to_string(), case.err, "case {i}: wrong error");
                    continue;
                }
                let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
                assert_eq!(got, *case.want.as_ref().unwrap(), "case {i}: wrong result");
            }
        }

        // TestChunklist, collections:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L94
        #[test]
        #[ignore = "not yet implemented"]
        fn collections() {
            let tests: Vec<Case> = vec![
                Case {
                    list: Value::list_empty(Type::string()),
                    len: Value::number(2),
                    want: Some(Value::list_empty(Type::list(Type::string()))),
                    err: "",
                },
                Case {
                    list: Value::list([Value::string("a")]),
                    len: Value::number(2),
                    want: Some(Value::list([Value::list([Value::string("a")])])),
                    err: "",
                },
                Case {
                    list: Value::list([Value::string("a"), Value::string("b")]),
                    len: Value::number(2),
                    want: Some(Value::list([Value::list([
                        Value::string("a"),
                        Value::string("b"),
                    ])])),
                    err: "",
                },
                // Multiple result elements, one shorter
                Case {
                    list: Value::list([Value::string("a"), Value::string("b"), Value::string("c")]),
                    len: Value::number(2),
                    want: Some(Value::list([
                        Value::list([Value::string("a"), Value::string("b")]),
                        Value::list([Value::string("c")]),
                    ])),
                    err: "",
                },
                // Multiple result elements, all "full"
                Case {
                    list: Value::list([
                        Value::string("a"),
                        Value::string("b"),
                        Value::string("c"),
                        Value::string("d"),
                        Value::string("e"),
                        Value::string("f"),
                    ]),
                    len: Value::number(2),
                    want: Some(Value::list([
                        Value::list([Value::string("a"), Value::string("b")]),
                        Value::list([Value::string("c"), Value::string("d")]),
                        Value::list([Value::string("e"), Value::string("f")]),
                    ])),
                    err: "",
                },
                // We treat length zero as infinite length
                Case {
                    list: Value::list([Value::string("a")]),
                    len: Value::zero(),
                    want: Some(Value::list([Value::list([Value::string("a")])])),
                    err: "",
                },
                Case {
                    list: Value::list_empty(Type::string()),
                    len: Value::number(-1),
                    want: None,
                    err: "the size argument must be positive",
                },
                Case {
                    list: Value::list_empty(Type::string()),
                    len: Value::positive_infinity(),
                    want: None,
                    err: "invalid size: value must be a whole number, between -9223372036854775808 and 9223372036854775807",
                },
                Case {
                    list: Value::list_empty(Type::string()),
                    len: Value::number(1.5),
                    want: None,
                    err: "invalid size: value must be a whole number, between -9223372036854775808 and 9223372036854775807",
                },
            ];

            check(&tests);
        }

        // TestChunklist, unknowns:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L94
        #[test]
        #[ignore = "not yet implemented"]
        fn unknowns() {
            let tests: Vec<Case> = vec![
                Case {
                    list: Value::unknown(Type::list(Type::string())),
                    len: Value::number(2),
                    want: Some(
                        Value::unknown(Type::list(Type::list(Type::string()))).refine_not_null(),
                    ),
                    err: "",
                },
                Case {
                    list: Value::list([Value::unknown(Type::string())]),
                    len: Value::number(2),
                    want: Some(Value::list([Value::list([Value::unknown(Type::string())])])),
                    err: "",
                },
            ];

            check(&tests);
        }

        // TestChunklist, marks:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L94
        #[test]
        #[ignore = "not yet implemented"]
        fn marks() {
            let tests: Vec<Case> = vec![
                Case {
                    list: Value::list([Value::string("a").mark("b")]),
                    len: Value::number(2),
                    want: Some(Value::list([Value::list([Value::string("a").mark("b")])])),
                    err: "",
                },
                Case {
                    list: Value::list([Value::string("a")]).mark("a"),
                    len: Value::number(2),
                    want: Some(Value::list([Value::list([Value::string("a")])]).mark("a")),
                    err: "",
                },
                Case {
                    list: Value::list([Value::string("a").mark("b")]).mark("a"),
                    len: Value::number(2),
                    want: Some(
                        Value::list([Value::list([Value::string("a").mark("b")])]).mark("a"),
                    ),
                    err: "",
                },
                Case {
                    list: Value::list([Value::string("a")]).mark("a"),
                    len: Value::zero(),
                    want: Some(Value::list([Value::list([Value::string("a")])]).mark("a")),
                    err: "",
                },
                Case {
                    list: Value::list([Value::string("a")]),
                    len: Value::zero().mark("a"),
                    want: Some(Value::list([Value::list([Value::string("a")])]).mark("a")),
                    err: "",
                },
                Case {
                    list: Value::list([Value::string("a").mark("b")]),
                    len: Value::zero(),
                    want: Some(Value::list([Value::list([Value::string("a").mark("b")])])),
                    err: "",
                },
            ];

            check(&tests);
        }
    }
}
