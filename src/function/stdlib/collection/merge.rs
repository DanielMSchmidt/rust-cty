//! `merge` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`merge`] (go-cty: `stdlib.MergeFunc`).
pub fn merge_func() -> Function {
    todo!()
}

/// The maps merged left-to-right, later values overriding earlier
/// (go-cty: `stdlib.Merge`).
pub fn merge(maps: &[Value]) -> Result<Value, CtyError> {
    let _ = maps;
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
    use crate::{Type, Value, ValueMarks};

    // Ported from TestMerge:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L474
    //
    // Upstream TestMerge is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod merge {
        use super::*;

        // NOTE(port): upstream's `Want` field is `cty.NilVal` in the error cases;
        // those cases carry `None` here.
        struct Case {
            values: Vec<Value>,
            want: Option<Value>,
            err: bool,
        }

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[Case]) {
            for (i, case) in tests.iter().enumerate() {
                let got = stdlib::merge(&case.values);
                if case.err {
                    assert!(got.is_err(), "case {i}: succeeded; want error");
                    continue;
                }
                let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
                assert_eq!(got, *case.want.as_ref().unwrap(), "case {i}: wrong result");
            }
        }

        // TestMerge, primitives:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L474
        #[test]
        #[ignore = "not yet implemented"]
        fn primitives() {
            let tests: Vec<Case> = vec![
                // Empty maps are allowed in merge
                Case {
                    values: vec![
                        Value::map_empty(Type::string()),
                        Value::map_empty(Type::string()),
                    ],
                    want: Some(Value::map_empty(Type::string())),
                    err: false,
                },
            ];

            check(&tests);
        }

        // TestMerge, collections:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L474
        #[test]
        #[ignore = "not yet implemented"]
        fn collections() {
            let tests: Vec<Case> = vec![
                Case {
                    values: vec![
                        Value::map([("a", Value::string("b"))]),
                        Value::map([("c", Value::string("d"))]),
                    ],
                    want: Some(Value::map([
                        ("a", Value::string("b")),
                        ("c", Value::string("d")),
                    ])),
                    err: false,
                },
                // handle null map
                Case {
                    values: vec![
                        Value::null(Type::map(Type::string())),
                        Value::map([("c", Value::string("d"))]),
                    ],
                    want: Some(Value::map([("c", Value::string("d"))])),
                    err: false,
                },
                // all inputs are null
                Case {
                    values: vec![
                        Value::null(Type::map(Type::string())),
                        Value::null(Type::object([("a", Type::list(Type::string()))])),
                    ],
                    want: Some(Value::empty_object()),
                    err: false,
                },
                // single null object with attributes
                Case {
                    values: vec![Value::null(Type::object([("a", Type::string())]))],
                    want: Some(Value::empty_object()),
                    err: false,
                },
                // multible null objects with the same attributes
                Case {
                    values: vec![
                        Value::null(Type::object([("a", Type::string())])),
                        Value::null(Type::object([("a", Type::string())])),
                    ],
                    want: Some(Value::empty_object()),
                    err: false,
                },
                // multible null objects with the differing attributes
                Case {
                    values: vec![
                        Value::null(Type::object([("a", Type::string())])),
                        Value::null(Type::object([("b", Type::string())])),
                    ],
                    want: Some(Value::empty_object()),
                    err: false,
                },
                // mixture of null and non-null objects of the same type
                Case {
                    values: vec![
                        Value::null(Type::object([("a", Type::string())])),
                        Value::object([("a", Value::string("a value"))]),
                    ],
                    want: Some(Value::object([("a", Value::string("a value"))])),
                    err: false,
                },
                // single empty map
                Case {
                    values: vec![Value::map_empty(Type::string())],
                    want: Some(Value::map_empty(Type::string())),
                    err: false,
                },
                // handle null object
                Case {
                    values: vec![
                        Value::map([("c", Value::string("d"))]),
                        Value::null(Type::object([("a", Type::list(Type::string()))])),
                    ],
                    want: Some(Value::object([("c", Value::string("d"))])),
                    err: false,
                },
                // merge with conflicts is ok, last in wins
                Case {
                    values: vec![
                        Value::map([("a", Value::string("b")), ("c", Value::string("d"))]),
                        Value::map([("a", Value::string("x"))]),
                    ],
                    want: Some(Value::map([
                        ("a", Value::string("x")),
                        ("c", Value::string("d")),
                    ])),
                    err: false,
                },
                // only accept maps
                Case {
                    values: vec![
                        Value::map([("a", Value::string("b")), ("c", Value::string("d"))]),
                        Value::list([Value::string("a"), Value::string("x")]),
                    ],
                    want: None,
                    err: true,
                },
                // argument error, for a null type
                Case {
                    values: vec![
                        Value::map([("a", Value::string("b"))]),
                        Value::null(Type::string()),
                    ],
                    want: None,
                    err: true,
                },
                // merge maps of maps
                Case {
                    values: vec![
                        Value::map([("a", Value::map([("b", Value::string("c"))]))]),
                        Value::map([("d", Value::map([("e", Value::string("f"))]))]),
                    ],
                    want: Some(Value::map([
                        ("a", Value::map([("b", Value::string("c"))])),
                        ("d", Value::map([("e", Value::string("f"))])),
                    ])),
                    err: false,
                },
                // map of lists
                Case {
                    values: vec![
                        Value::map([("a", Value::list([Value::string("b"), Value::string("c")]))]),
                        Value::map([("d", Value::list([Value::string("e"), Value::string("f")]))]),
                    ],
                    want: Some(Value::map([
                        ("a", Value::list([Value::string("b"), Value::string("c")])),
                        ("d", Value::list([Value::string("e"), Value::string("f")])),
                    ])),
                    err: false,
                },
                // merge map of various kinds
                Case {
                    values: vec![
                        Value::map([("a", Value::list([Value::string("b"), Value::string("c")]))]),
                        Value::map([("d", Value::map([("e", Value::string("f"))]))]),
                    ],
                    want: Some(Value::object([
                        ("a", Value::list([Value::string("b"), Value::string("c")])),
                        ("d", Value::map([("e", Value::string("f"))])),
                    ])),
                    err: false,
                },
                // merge maps and objects
                Case {
                    values: vec![
                        Value::map([("a", Value::list([Value::string("b")]))]),
                        Value::object([("d", Value::number(2))]),
                    ],
                    want: Some(Value::object([
                        ("a", Value::list([Value::string("b")])),
                        ("d", Value::number(2)),
                    ])),
                    err: false,
                },
                // attr a type and value is overridden
                Case {
                    values: vec![
                        Value::object([
                            ("a", Value::list([Value::string("b")])),
                            ("b", Value::string("b")),
                        ]),
                        Value::object([("a", Value::object([("e", Value::string("f"))]))]),
                    ],
                    want: Some(Value::object([
                        ("a", Value::object([("e", Value::string("f"))])),
                        ("b", Value::string("b")),
                    ])),
                    err: false,
                },
                // argument error: non map type
                Case {
                    values: vec![
                        Value::map([("a", Value::list([Value::string("b"), Value::string("c")]))]),
                        Value::list([Value::string("d"), Value::string("e")]),
                    ],
                    want: None,
                    err: true,
                },
            ];

            check(&tests);
        }

        // TestMerge, unknowns:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L474
        #[test]
        #[ignore = "not yet implemented"]
        fn unknowns() {
            let tests: Vec<Case> = vec![
                // handle unknowns
                Case {
                    values: vec![
                        Value::map([("a", Value::unknown(Type::string()))]),
                        Value::map([("c", Value::string("d"))]),
                    ],
                    want: Some(Value::map([
                        ("a", Value::unknown(Type::string())),
                        ("c", Value::string("d")),
                    ])),
                    err: false,
                },
                // handle unknowns
                Case {
                    values: vec![
                        Value::unknown(Type::map(Type::string())),
                        Value::map([("c", Value::string("d"))]),
                    ],
                    want: Some(Value::unknown(Type::map(Type::string())).refine_not_null()),
                    err: false,
                },
                // handle dynamic unknown
                Case {
                    values: vec![
                        Value::unknown(Type::dynamic()),
                        Value::map([("c", Value::string("d"))]),
                    ],
                    want: Some(Value::dynamic()),
                    err: false,
                },
                // merge objects of various shapes
                Case {
                    values: vec![
                        Value::object([("a", Value::list([Value::string("b")]))]),
                        Value::object([("d", Value::dynamic())]),
                    ],
                    want: Some(Value::object([
                        ("a", Value::list([Value::string("b")])),
                        ("d", Value::dynamic()),
                    ])),
                    err: false,
                },
            ];

            check(&tests);
        }

        // TestMerge, marks:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L474
        #[test]
        #[ignore = "not yet implemented"]
        fn marks() {
            let tests: Vec<Case> = vec![
                // Preserve marks from chosen elements
                Case {
                    values: vec![
                        Value::map([
                            ("a", Value::string("a").mark("first")),
                            ("c", Value::string("c")),
                            ("d", Value::string("d").mark("first")),
                        ]),
                        Value::map([
                            ("a", Value::string("a")),
                            ("b", Value::string("b").mark("second")),
                            ("c", Value::string("c").mark("second")),
                        ]),
                    ],
                    want: Some(Value::map([
                        ("a", Value::string("a")),
                        ("b", Value::string("b").mark("second")),
                        ("c", Value::string("c").mark("second")),
                        ("d", Value::string("d").mark("first")),
                    ])),
                    err: false,
                },
                // Marks on the collections must be merged, even if empty
                Case {
                    values: vec![
                        Value::map([("a", Value::string("a"))]).mark("first"),
                        Value::map([("a", Value::string("a")), ("b", Value::string("b"))])
                            .mark("second"),
                        Value::map_empty(Type::string()).mark("third"),
                    ],
                    want: Some(
                        Value::map([("a", Value::string("a")), ("b", Value::string("b"))])
                            .with_marks([ValueMarks::from_marks(["first", "second", "third"])]),
                    ),
                    err: false,
                },
                // Similar test but where all args are the same object type
                Case {
                    values: vec![
                        Value::object([
                            ("a", Value::string("a")),
                            ("b", Value::null(Type::string())),
                        ])
                        .mark("first"),
                        Value::object([("a", Value::string("A")), ("b", Value::string("B"))])
                            .mark("second"),
                    ],
                    want: Some(
                        Value::object([("a", Value::string("A")), ("b", Value::string("B"))])
                            .with_marks([ValueMarks::from_marks(["first", "second"])]),
                    ),
                    err: false,
                },
            ];

            check(&tests);
        }
    }
}
