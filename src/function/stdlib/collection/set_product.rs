//! `set_product` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`set_product`] (go-cty: `stdlib.SetProductFunc`).
pub fn set_product_func() -> Function {
    todo!()
}

/// The cartesian product of the given sets or lists
/// (go-cty: `stdlib.SetProduct`).
pub fn set_product(sets: &[Value]) -> Result<Value, CtyError> {
    let _ = sets;
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

    // Ported from TestSetproduct:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L2383
    //
    // Upstream TestSetproduct is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod setproduct {
        use super::*;

        // NOTE(port): upstream's `Want` field is `cty.NilVal` in the error case;
        // that case carries `None` here.
        struct Case {
            collections: Vec<Value>,
            want: Option<Value>,
            err: &'static str,
        }

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[Case]) {
            for (i, case) in tests.iter().enumerate() {
                let got = stdlib::set_product(&case.collections);
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

        // TestSetproduct, primitives:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L2383
        #[test]
        #[ignore = "not yet implemented"]
        fn primitives() {
            let tests: Vec<Case> = vec![Case {
                collections: vec![Value::list_empty(Type::string())],
                want: None,
                err: "at least two arguments are required",
            }];

            check(&tests);
        }

        // TestSetproduct, collections:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L2383
        #[test]
        #[ignore = "not yet implemented"]
        fn collections() {
            let tests: Vec<Case> = vec![
                Case {
                    collections: vec![
                        Value::list_empty(Type::empty_object()),
                        Value::list([Value::string("quick"), Value::string("fox")]),
                    ],
                    want: Some(Value::list_empty(Type::tuple([
                        Type::empty_object(),
                        Type::string(),
                    ]))),
                    err: "",
                },
                Case {
                    collections: vec![
                        Value::set_empty(Type::empty_object()),
                        Value::set([Value::string("quick"), Value::string("fox")]),
                    ],
                    want: Some(Value::set_empty(Type::tuple([
                        Type::empty_object(),
                        Type::string(),
                    ]))),
                    err: "",
                },
                Case {
                    collections: vec![
                        Value::list_empty(Type::empty_object()),
                        Value::list_empty(Type::empty_object()),
                    ],
                    want: Some(Value::list_empty(Type::tuple([
                        Type::empty_object(),
                        Type::empty_object(),
                    ]))),
                    err: "",
                },
                Case {
                    collections: vec![
                        Value::set_empty(Type::empty_object()),
                        Value::set_empty(Type::empty_object()),
                    ],
                    want: Some(Value::set_empty(Type::tuple([
                        Type::empty_object(),
                        Type::empty_object(),
                    ]))),
                    err: "",
                },
                Case {
                    collections: vec![
                        Value::list([Value::list_empty(Type::string())]),
                        Value::list([Value::list_empty(Type::string())]),
                    ],
                    want: Some(Value::list([Value::tuple([
                        Value::list_empty(Type::string()),
                        Value::list_empty(Type::string()),
                    ])])),
                    err: "",
                },
                Case {
                    collections: vec![
                        Value::set([Value::list_empty(Type::string())]),
                        Value::set([Value::list_empty(Type::string())]),
                    ],
                    want: Some(Value::set([Value::tuple([
                        Value::list_empty(Type::string()),
                        Value::list_empty(Type::string()),
                    ])])),
                    err: "",
                },
                Case {
                    collections: vec![
                        Value::tuple([Value::string("the"), Value::string("brown")]),
                        Value::tuple([Value::string("fox"), Value::number(3)]),
                    ],
                    want: Some(Value::list([
                        Value::tuple([Value::string("the"), Value::string("fox")]),
                        Value::tuple([Value::string("the"), Value::string("3")]),
                        Value::tuple([Value::string("brown"), Value::string("fox")]),
                        Value::tuple([Value::string("brown"), Value::string("3")]),
                    ])),
                    err: "",
                },
                Case {
                    collections: vec![
                        Value::set([Value::string("the"), Value::string("brown")]),
                        Value::set([Value::string("quick"), Value::string("fox")]),
                    ],
                    want: Some(Value::set([
                        Value::tuple([Value::string("the"), Value::string("quick")]),
                        Value::tuple([Value::string("the"), Value::string("fox")]),
                        Value::tuple([Value::string("brown"), Value::string("quick")]),
                        Value::tuple([Value::string("brown"), Value::string("fox")]),
                    ])),
                    err: "",
                },
            ];

            check(&tests);
        }

        // TestSetproduct, unknowns:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L2383
        #[test]
        #[ignore = "not yet implemented"]
        fn unknowns() {
            let tests: Vec<Case> = vec![
                Case {
                    collections: vec![Value::set([Value::bool(true)]), Value::dynamic()],
                    want: Some(Value::dynamic()),
                    err: "",
                },
                // If the inputs have unknown lengths but have length refinements then
                // we can potentially refine our unknown result too.
                Case {
                    collections: vec![
                        Value::unknown(Type::set(Type::string()))
                            .refine()
                            .collection_length_upper_bound(2)
                            .new_value(),
                        Value::unknown(Type::set(Type::number()))
                            .refine()
                            .collection_length_upper_bound(3)
                            .new_value(),
                    ],
                    want: Some(
                        Value::unknown(Type::set(Type::tuple([Type::string(), Type::number()])))
                            .refine()
                            .not_null()
                            .collection_length_lower_bound(1)
                            .collection_length_upper_bound(6)
                            .new_value(),
                    ),
                    err: "",
                },
                Case {
                    collections: vec![
                        Value::unknown(Type::set(Type::string()))
                            .refine()
                            .collection_length_upper_bound(2)
                            .new_value(),
                        Value::set_empty(Type::number()),
                    ],
                    // deduced from refinements
                    want: Some(Value::set_empty(Type::tuple([
                        Type::string(),
                        Type::number(),
                    ]))),
                    err: "",
                },
                // If we have any input with a very large maximum element count then we'll
                // just leave the result length unrefined to reduce the risk of integer overflow.
                Case {
                    collections: vec![
                        Value::unknown(Type::set(Type::string()))
                            .refine()
                            .collection_length_upper_bound(2)
                            .new_value(),
                        Value::unknown(Type::set(Type::number()))
                            .refine()
                            .collection_length_upper_bound(4096)
                            .new_value(),
                    ],
                    want: Some(
                        Value::unknown(Type::set(Type::tuple([Type::string(), Type::number()])))
                            .refine_not_null(),
                    ),
                    err: "",
                },
                Case {
                    collections: vec![
                        Value::unknown(Type::list(Type::string()))
                            .refine()
                            .collection_length_upper_bound(2)
                            .new_value(),
                        Value::unknown(Type::list(Type::number()))
                            .refine()
                            .collection_length_upper_bound(3)
                            .new_value(),
                    ],
                    // NOTE: When the result is a list rather than a set there is no
                    // coalescing and so we could potentially also calculate a more
                    // refined lower bound on the collection length, but since
                    // this function is primarily for sets for now we just accept a
                    // set-oriented refinement. If we find that it would be productive
                    // to further constrain the range of a list result then we can
                    // make this more precise later.
                    want: Some(
                        Value::unknown(Type::list(Type::tuple([Type::string(), Type::number()])))
                            .refine()
                            .not_null()
                            .collection_length_lower_bound(1)
                            .collection_length_upper_bound(6)
                            .new_value(),
                    ),
                    err: "",
                },
                Case {
                    collections: vec![
                        Value::unknown(Type::list(Type::string()))
                            .refine()
                            .collection_length_upper_bound(2)
                            .new_value(),
                        Value::list_empty(Type::number()),
                    ],
                    // deduced from refinements
                    want: Some(Value::list_empty(Type::tuple([
                        Type::string(),
                        Type::number(),
                    ]))),
                    err: "",
                },
                Case {
                    collections: vec![
                        Value::unknown(Type::tuple([Type::string(), Type::string()])),
                        Value::unknown(Type::tuple([
                            Type::number(),
                            Type::number(),
                            Type::number(),
                        ])),
                    ],
                    // NOTE: When the result is a list rather than a set there is no
                    // coalescing and so we could potentially also calculate a more
                    // refined lower bound on the collection length, but since
                    // this function is primarily for sets for now we just accept a
                    // set-oriented refinement. If we find that it would be productive
                    // to further constrain the range of a list result then we can
                    // make this more precise later.
                    want: Some(
                        Value::unknown(Type::list(Type::tuple([Type::string(), Type::number()])))
                            .refine()
                            .not_null()
                            .collection_length_lower_bound(1)
                            .collection_length_upper_bound(6)
                            .new_value(),
                    ),
                    err: "",
                },
                Case {
                    collections: vec![
                        Value::unknown(Type::tuple([Type::string(), Type::string()])),
                        Value::empty_tuple(),
                    ],
                    // NOTE: When the result is a list rather than a set there is no
                    // coalescing and so we could potentially also calculate a more
                    // refined lower bound on the collection length, but since
                    // this function is primarily for sets for now we just accept a
                    // set-oriented refinement. If we find that it would be productive
                    // to further constrain the range of a list result then we can
                    // make this more precise later.
                    want: Some(Value::list_empty(Type::tuple([
                        Type::string(),
                        Type::dynamic(),
                    ]))),
                    err: "",
                },
            ];

            check(&tests);
        }

        // TestSetproduct, marks:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L2383
        #[test]
        #[ignore = "not yet implemented"]
        fn marks() {
            let tests: Vec<Case> = vec![
                Case {
                    collections: vec![
                        Value::set([Value::list_empty(Type::string()).mark("a")]),
                        Value::set([Value::list_empty(Type::string())]),
                    ],
                    want: Some(Value::set([Value::tuple([
                        Value::list_empty(Type::string()).mark("a"),
                        Value::list_empty(Type::string()),
                    ])])),
                    err: "",
                },
                // The collection itself is not marked, just some elements
                Case {
                    collections: vec![
                        Value::set([Value::string("the"), Value::string("brown").mark("a")]),
                        Value::set([Value::string("quick"), Value::string("fox").mark("b")]),
                    ],
                    // Sets don't allow individually-marked elements, so the marks
                    // end up aggregating on the set itself anyway in this case.
                    want: Some(
                        Value::set([
                            Value::tuple([Value::string("the"), Value::string("quick")]),
                            Value::tuple([Value::string("the"), Value::string("fox")]),
                            Value::tuple([Value::string("brown"), Value::string("quick")]),
                            Value::tuple([Value::string("brown"), Value::string("fox")]),
                        ])
                        .mark("a")
                        .mark("b"),
                    ),
                    err: "",
                },
                // The collections are marked
                Case {
                    collections: vec![
                        Value::set([Value::string("the"), Value::string("brown")]).mark("a"),
                        Value::set([Value::string("quick"), Value::string("fox")]).mark("b"),
                    ],
                    want: Some(
                        Value::set([
                            Value::tuple([Value::string("the"), Value::string("quick")]),
                            Value::tuple([Value::string("the"), Value::string("fox")]),
                            Value::tuple([Value::string("brown"), Value::string("quick")]),
                            Value::tuple([Value::string("brown"), Value::string("fox")]),
                        ])
                        .mark("a")
                        .mark("b"),
                    ),
                    err: "",
                },
                // One collection is marked
                Case {
                    collections: vec![
                        Value::set([Value::string("the"), Value::string("brown")]).mark("a"),
                        Value::set([Value::string("quick"), Value::string("fox")]),
                    ],
                    want: Some(
                        Value::set([
                            Value::tuple([Value::string("the"), Value::string("quick")]),
                            Value::tuple([Value::string("the"), Value::string("fox")]),
                            Value::tuple([Value::string("brown"), Value::string("quick")]),
                            Value::tuple([Value::string("brown"), Value::string("fox")]),
                        ])
                        .mark("a"),
                    ),
                    err: "",
                },
                // Inner and outer marks
                Case {
                    collections: vec![
                        Value::set([Value::string("the"), Value::string("brown").mark("a")])
                            .mark("b"),
                        Value::set([Value::string("quick"), Value::string("fox").mark("c")]),
                    ],
                    want: Some(
                        Value::set([
                            Value::tuple([Value::string("the"), Value::string("quick")]),
                            Value::tuple([Value::string("the"), Value::string("fox")]),
                            Value::tuple([Value::string("brown"), Value::string("quick")]),
                            Value::tuple([Value::string("brown"), Value::string("fox")]),
                        ])
                        .with_marks([ValueMarks::from_marks(["b", "c", "a"])]),
                    ),
                    err: "",
                },
                // SetproductFunc supports lists too, in which case it preserves the
                // input order and returns a list as the result. In this case we can
                // preserve the marks more precisely.
                // The collection itself is not marked, just some elements
                Case {
                    collections: vec![
                        Value::list([Value::string("the"), Value::string("brown").mark("a")]),
                        Value::list([Value::string("quick"), Value::string("fox").mark("b")]),
                    ],
                    want: Some(Value::list([
                        Value::tuple([Value::string("the"), Value::string("quick")]),
                        Value::tuple([Value::string("the"), Value::string("fox").mark("b")]),
                        Value::tuple([Value::string("brown").mark("a"), Value::string("quick")]),
                        Value::tuple([
                            Value::string("brown").mark("a"),
                            Value::string("fox").mark("b"),
                        ]),
                    ])),
                    err: "",
                },
                // The collections are marked
                Case {
                    collections: vec![
                        Value::list([Value::string("the"), Value::string("brown")]).mark("a"),
                        Value::list([Value::string("quick"), Value::string("fox")]).mark("b"),
                    ],
                    want: Some(
                        Value::list([
                            Value::tuple([Value::string("the"), Value::string("quick")]),
                            Value::tuple([Value::string("the"), Value::string("fox")]),
                            Value::tuple([Value::string("brown"), Value::string("quick")]),
                            Value::tuple([Value::string("brown"), Value::string("fox")]),
                        ])
                        .mark("a")
                        .mark("b"),
                    ),
                    err: "",
                },
                // One collection is marked
                Case {
                    collections: vec![
                        Value::list([Value::string("the"), Value::string("brown")]).mark("a"),
                        Value::list([Value::string("quick"), Value::string("fox")]),
                    ],
                    want: Some(
                        Value::list([
                            Value::tuple([Value::string("the"), Value::string("quick")]),
                            Value::tuple([Value::string("the"), Value::string("fox")]),
                            Value::tuple([Value::string("brown"), Value::string("quick")]),
                            Value::tuple([Value::string("brown"), Value::string("fox")]),
                        ])
                        .mark("a"),
                    ),
                    err: "",
                },
                // Inner and outer marks
                Case {
                    collections: vec![
                        Value::list([Value::string("the"), Value::string("brown").mark("a")])
                            .mark("b"),
                        Value::list([Value::string("quick"), Value::string("fox").mark("c")]),
                    ],
                    want: Some(
                        Value::list([
                            Value::tuple([Value::string("the"), Value::string("quick")]),
                            Value::tuple([Value::string("the"), Value::string("fox").mark("c")]),
                            Value::tuple([
                                Value::string("brown").mark("a"),
                                Value::string("quick"),
                            ]),
                            Value::tuple([
                                Value::string("brown").mark("a"),
                                Value::string("fox").mark("c"),
                            ]),
                        ])
                        .mark("b"),
                    ),
                    err: "",
                },
                // Empty lists with marks should propagate the marks
                Case {
                    collections: vec![
                        Value::list_empty(Type::string()).mark("a"),
                        Value::list_empty(Type::bool()).mark("b"),
                    ],
                    want: Some(
                        Value::list_empty(Type::tuple([Type::string(), Type::bool()]))
                            .with_marks([ValueMarks::from_marks(["a", "b"])]),
                    ),
                    err: "",
                },
                // Empty sets with marks should propagate the marks
                Case {
                    collections: vec![
                        Value::set_empty(Type::string()).mark("a"),
                        Value::set_empty(Type::bool()).mark("b"),
                    ],
                    want: Some(
                        Value::set_empty(Type::tuple([Type::string(), Type::bool()]))
                            .with_marks([ValueMarks::from_marks(["a", "b"])]),
                    ),
                    err: "",
                },
                // Arguments which are sets with partially unknown values results
                // in unknown length (since the unknown values may already be
                // present in the set). This gives an unknown result preserving all
                // marks
                Case {
                    collections: vec![
                        Value::set([Value::string("x"), Value::unknown(Type::string())]).mark("a"),
                        Value::set([Value::bool(true), Value::bool(false)]).mark("b"),
                    ],
                    want: Some(
                        Value::unknown(Type::set(Type::tuple([Type::string(), Type::bool()])))
                            .refine_not_null()
                            .with_marks([ValueMarks::from_marks(["a", "b"])]),
                    ),
                    err: "",
                },
            ];

            check(&tests);
        }
    }
}
