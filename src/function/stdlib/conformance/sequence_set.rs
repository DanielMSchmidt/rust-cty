//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/function/stdlib/sequence_test.go
//!   cty/function/stdlib/set_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use crate::function::stdlib::{
    concat, range, set_intersection, set_subtract, set_symmetric_difference, set_union,
};
use crate::{Type, Value, ValueMarks};

// Ported from TestConcat:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/sequence_test.go#L10
//
// Upstream TestConcat is a single table covering several concepts at once.
// It is split below into one test per concept so they can be activated one at
// a time. Rows are transcribed verbatim and keep upstream order within a test;
// nothing is added, dropped, or rewritten.
mod concat_test {
    use super::*;

    /// Runs transcribed upstream rows; fixture plumbing only, every
    /// expected value is a literal from the upstream table.
    fn check(tests: &[(Vec<Value>, Value)]) {
        for (i, (input, want)) in tests.iter().enumerate() {
            let got = match concat(input) {
                Ok(got) => got,
                Err(err) => panic!("case {i}: unexpected error: {err}"),
            };
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }

    // TestConcat, primitives:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/sequence_test.go#L10
    #[test]
    #[ignore = "not yet implemented"]
    fn primitives() {
        let tests: Vec<(Vec<Value>, Value)> = vec![
            (
                vec![Value::list_empty(Type::number())],
                Value::list_empty(Type::number()),
            ),
            (vec![Value::empty_tuple()], Value::empty_tuple()),
        ];

        check(&tests);
    }

    // TestConcat, collections:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/sequence_test.go#L10
    #[test]
    #[ignore = "not yet implemented"]
    fn collections() {
        let tests: Vec<(Vec<Value>, Value)> = vec![
            (
                vec![Value::list([
                    Value::number(1),
                    Value::number(2),
                    Value::number(3),
                ])],
                Value::list([Value::number(1), Value::number(2), Value::number(3)]),
            ),
            (
                vec![
                    Value::list([Value::number(1)]),
                    Value::list([Value::number(2), Value::number(3)]),
                ],
                Value::list([Value::number(1), Value::number(2), Value::number(3)]),
            ),
            (
                vec![
                    Value::list([Value::number(1)]),
                    Value::list([Value::string("foo")]),
                    Value::list([Value::bool(true)]),
                ],
                Value::list([
                    Value::string("1"),
                    Value::string("foo"),
                    Value::string("true"),
                ]),
            ),
            (
                vec![
                    Value::list([Value::number(1)]),
                    Value::list([Value::string("foo"), Value::string("bar")]),
                ],
                Value::list([
                    Value::string("1"),
                    Value::string("foo"),
                    Value::string("bar"),
                ]),
            ),
            (
                vec![Value::tuple([
                    Value::number(1),
                    Value::bool(true),
                    Value::number(3),
                ])],
                Value::tuple([Value::number(1), Value::bool(true), Value::number(3)]),
            ),
            (
                vec![
                    Value::tuple([Value::number(1)]),
                    Value::tuple([Value::bool(true), Value::number(3)]),
                ],
                Value::tuple([Value::number(1), Value::bool(true), Value::number(3)]),
            ),
            (
                vec![
                    Value::list([Value::number(1)]),
                    Value::tuple([Value::bool(true), Value::number(3)]),
                ],
                Value::tuple([Value::number(1), Value::bool(true), Value::number(3)]),
            ),
            (
                vec![
                    Value::tuple([Value::number(1), Value::bool(true)]),
                    Value::list([Value::number(3)]),
                ],
                Value::tuple([Value::number(1), Value::bool(true), Value::number(3)]),
            ),
            (
                // Two lists with unconvertable element types become a tuple.
                vec![
                    Value::list([Value::number(1)]),
                    Value::list([Value::list_empty(Type::bool())]),
                ],
                Value::tuple([Value::number(1), Value::list_empty(Type::bool())]),
            ),
        ];

        check(&tests);
    }

    // TestConcat, marks:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/sequence_test.go#L10
    #[test]
    #[ignore = "not yet implemented"]
    fn marks() {
        let tests: Vec<(Vec<Value>, Value)> = vec![
            (
                vec![
                    Value::list([Value::number(1)]),
                    Value::list([Value::number(2), Value::number(3)]).mark("a"),
                ],
                Value::list([Value::number(1), Value::number(2), Value::number(3)]).mark("a"),
            ),
            (
                vec![
                    Value::list([Value::number(1)]),
                    Value::list([Value::number(2).mark("b"), Value::number(3)]),
                ],
                Value::list([
                    Value::number(1),
                    Value::number(2).mark("b"),
                    Value::number(3),
                ]),
            ),
            (
                vec![
                    Value::list([Value::number(1)]).mark("a"),
                    Value::list([Value::number(2).mark("b"), Value::number(3)]),
                ],
                Value::list([
                    Value::number(1),
                    Value::number(2).mark("b"),
                    Value::number(3),
                ])
                .mark("a"),
            ),
            (
                vec![
                    Value::list_empty(Type::dynamic()).mark("a"),
                    Value::list([Value::number(2).mark("b"), Value::number(3)]).mark("c"),
                ],
                Value::list([Value::number(2).mark("b"), Value::number(3)])
                    .with_marks([ValueMarks::from_marks(["a", "c"])]),
            ),
            (
                vec![
                    Value::list_empty(Type::dynamic()).mark("a"),
                    Value::tuple([Value::number(2).mark("b"), Value::number(3)]).mark("c"),
                ],
                Value::tuple([Value::number(2).mark("b"), Value::number(3)])
                    .with_marks([ValueMarks::from_marks(["a", "c"])]),
            ),
        ];

        check(&tests);
    }
}

// Ported from TestRange:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/sequence_test.go#L259
//
// Upstream TestRange is a single table covering several concepts at once.
// It is split below into one test per concept so they can be activated one at
// a time. Rows are transcribed verbatim and keep upstream order within a test;
// nothing is added, dropped, or rewritten.
mod range_test {
    use super::*;

    /// Runs transcribed upstream rows; fixture plumbing only, every
    /// expected value is a literal from the upstream table.
    fn check(tests: &[(Vec<Value>, Value)]) {
        for (i, (args, want)) in tests.iter().enumerate() {
            let got = match range(args) {
                Ok(got) => got,
                Err(err) => panic!("case {i}: unexpected error: {err}"),
            };
            assert_eq!(got, *want, "case {i}: wrong result\nargs: {args:?}");
        }
    }

    // TestRange, one argument:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/sequence_test.go#L259
    #[test]
    #[ignore = "not yet implemented"]
    fn one_argument() {
        let tests: Vec<(Vec<Value>, Value)> = vec![
            // One argument
            (
                vec![Value::number(5)],
                Value::list([
                    Value::number(0),
                    Value::number(1),
                    Value::number(2),
                    Value::number(3),
                    Value::number(4),
                ]),
            ),
            (
                vec![Value::number(-5)],
                Value::list([
                    Value::number(0),
                    Value::number(-1),
                    Value::number(-2),
                    Value::number(-3),
                    Value::number(-4),
                ]),
            ),
            (vec![Value::number(1)], Value::list([Value::number(0)])),
            (vec![Value::number(0)], Value::list_empty(Type::number())),
            (
                vec![Value::parse_number("5.5")],
                Value::list([
                    Value::number(0),
                    Value::number(1),
                    Value::number(2),
                    Value::number(3),
                    Value::number(4),
                    Value::number(5), // because 5 < 5.5
                ]),
            ),
        ];

        check(&tests);
    }

    // TestRange, two arguments:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/sequence_test.go#L259
    #[test]
    #[ignore = "not yet implemented"]
    fn two_arguments() {
        let tests: Vec<(Vec<Value>, Value)> = vec![
            // Two arguments
            (
                vec![Value::number(1), Value::number(5)],
                Value::list([
                    Value::number(1),
                    Value::number(2),
                    Value::number(3),
                    Value::number(4),
                ]),
            ),
            (
                vec![Value::number(5), Value::number(1)],
                Value::list([
                    Value::number(5),
                    Value::number(4),
                    Value::number(3),
                    Value::number(2),
                ]),
            ),
            (
                vec![Value::number(1.5), Value::number(5)],
                Value::list([
                    Value::number(1.5),
                    Value::number(2.5),
                    Value::number(3.5),
                    Value::number(4.5),
                ]),
            ),
            (
                vec![Value::number(1), Value::number(2)],
                Value::list([Value::number(1)]),
            ),
            (
                vec![Value::number(1), Value::number(1)],
                Value::list_empty(Type::number()),
            ),
        ];

        check(&tests);
    }

    // TestRange, three arguments:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/sequence_test.go#L259
    #[test]
    #[ignore = "not yet implemented"]
    fn three_arguments() {
        let tests: Vec<(Vec<Value>, Value)> = vec![
            // Three arguments
            (
                vec![Value::number(0), Value::number(5), Value::number(2)],
                Value::list([Value::number(0), Value::number(2), Value::number(4)]),
            ),
            (
                vec![Value::number(0), Value::number(5), Value::number(1)],
                Value::list([
                    Value::number(0),
                    Value::number(1),
                    Value::number(2),
                    Value::number(3),
                    Value::number(4),
                ]),
            ),
            (
                vec![Value::number(0), Value::number(1), Value::number(1)],
                Value::list([Value::number(0)]),
            ),
            (
                vec![Value::number(0), Value::number(0), Value::number(1)],
                Value::list_empty(Type::number()),
            ),
            (
                vec![Value::number(5), Value::number(0), Value::number(-1)],
                Value::list([
                    Value::number(5),
                    Value::number(4),
                    Value::number(3),
                    Value::number(2),
                    Value::number(1),
                ]),
            ),
            (
                vec![Value::number(0), Value::number(5), Value::number(0.5)],
                Value::list([
                    Value::number(0),
                    Value::number(0.5),
                    Value::number(1),
                    Value::number(1.5),
                    Value::number(2),
                    Value::number(2.5),
                    Value::number(3),
                    Value::number(3.5),
                    Value::number(4),
                    Value::number(4.5),
                ]),
            ),
        ];

        check(&tests);
    }
}

// Ported from TestSetUnion:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/set_test.go#L10
#[test]
#[ignore = "not yet implemented"]
fn set_union_test() {
    let tests: Vec<(Vec<Value>, Value)> = vec![
        (
            vec![Value::set_empty(Type::string())],
            Value::set_empty(Type::string()),
        ),
        (
            vec![
                Value::set_empty(Type::string()),
                Value::set_empty(Type::string()),
            ],
            Value::set_empty(Type::string()),
        ),
        (
            vec![
                Value::set([Value::bool(true)]),
                Value::set_empty(Type::string()),
            ],
            Value::set([Value::string("true")]),
        ),
        (
            vec![
                Value::set([Value::bool(true)]),
                Value::set([Value::bool(true)]),
                Value::set([Value::bool(false)]),
            ],
            Value::set([Value::bool(true), Value::bool(false)]),
        ),
        (
            vec![
                Value::set([Value::string("a")]),
                Value::set([Value::string("b")]),
                Value::set([Value::string("b"), Value::string("c")]),
            ],
            Value::set([Value::string("a"), Value::string("b"), Value::string("c")]),
        ),
        (
            vec![
                Value::set([Value::bool(true)]),
                Value::set_empty(Type::dynamic()),
            ],
            Value::set([Value::bool(true)]),
        ),
        (
            vec![
                Value::set([Value::empty_object()]),
                Value::set_empty(Type::dynamic()),
            ],
            Value::set([Value::empty_object()]),
        ),
        (
            vec![
                Value::set_empty(Type::dynamic()),
                Value::set_empty(Type::dynamic()),
            ],
            Value::set_empty(Type::dynamic()),
        ),
        (
            vec![
                Value::set([Value::string("5")]),
                Value::unknown(Type::set(Type::number())),
            ],
            Value::unknown(Type::set(Type::string())).refine_not_null(),
        ),
        (
            vec![
                Value::set([Value::string("5")]),
                Value::set([Value::unknown(Type::string())]),
            ],
            Value::set([Value::string("5"), Value::unknown(Type::string())]),
        ),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got = match set_union(input) {
            Ok(got) => got,
            Err(err) => panic!("case {i}: unexpected error: {err}"),
        };
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestSetIntersection:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/set_test.go#L110
#[test]
#[ignore = "not yet implemented"]
fn set_intersection_test() {
    let tests: Vec<(Vec<Value>, Value)> = vec![
        (
            vec![Value::set_empty(Type::string())],
            Value::set_empty(Type::string()),
        ),
        (
            vec![
                Value::set_empty(Type::string()),
                Value::set_empty(Type::string()),
            ],
            Value::set_empty(Type::string()),
        ),
        (
            vec![
                Value::set([Value::bool(true)]),
                Value::set_empty(Type::string()),
            ],
            Value::set_empty(Type::string()),
        ),
        (
            vec![
                Value::set([Value::bool(true)]),
                Value::set([Value::bool(true), Value::bool(false)]),
                Value::set([Value::bool(true), Value::bool(false)]),
            ],
            Value::set([Value::bool(true)]),
        ),
        (
            vec![
                Value::set([Value::string("a"), Value::string("b")]),
                Value::set([Value::string("b")]),
                Value::set([Value::string("b"), Value::string("c")]),
            ],
            Value::set([Value::string("b")]),
        ),
        (
            vec![
                Value::set([Value::bool(true)]),
                Value::set_empty(Type::dynamic()),
            ],
            Value::set_empty(Type::bool()),
        ),
        (
            vec![
                Value::set([Value::empty_object()]),
                Value::set_empty(Type::dynamic()),
            ],
            Value::set_empty(Type::empty_object()),
        ),
        (
            vec![
                Value::set_empty(Type::dynamic()),
                Value::set_empty(Type::dynamic()),
            ],
            Value::set_empty(Type::dynamic()),
        ),
        (
            vec![
                Value::set([Value::string("5")]),
                Value::unknown(Type::set(Type::number())),
            ],
            Value::unknown(Type::set(Type::string())).refine_not_null(),
        ),
        (
            vec![
                Value::set([Value::string("5")]),
                Value::set([Value::unknown(Type::string())]),
            ],
            Value::unknown(Type::set(Type::string())).refine_not_null(),
        ),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got = match set_intersection(input) {
            Ok(got) => got,
            Err(err) => panic!("case {i}: unexpected error: {err}"),
        };
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestSetSubtract:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/set_test.go#L207
#[test]
#[ignore = "not yet implemented"]
fn set_subtract_test() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::set_empty(Type::string()),
            Value::set_empty(Type::string()),
            Value::set_empty(Type::string()),
        ),
        (
            Value::set([Value::bool(true)]),
            Value::set_empty(Type::string()),
            Value::set([Value::string("true")]),
        ),
        (
            Value::set([Value::bool(true)]),
            Value::set([Value::bool(false)]),
            Value::set([Value::bool(true)]),
        ),
        (
            Value::set([Value::string("a"), Value::string("b"), Value::string("c")]),
            Value::set([Value::string("a"), Value::string("c")]),
            Value::set([Value::string("b")]),
        ),
        (
            Value::set([Value::string("a")]),
            Value::set_empty(Type::dynamic()),
            Value::set([Value::string("a")]),
        ),
        (
            Value::set([Value::empty_object()]),
            Value::set_empty(Type::dynamic()),
            Value::set([Value::empty_object()]),
        ),
        (
            Value::set_empty(Type::dynamic()),
            Value::set_empty(Type::dynamic()),
            Value::set_empty(Type::dynamic()),
        ),
        (
            Value::set([Value::string("5")]),
            Value::unknown(Type::set(Type::number())),
            Value::unknown(Type::set(Type::string())).refine_not_null(),
        ),
        (
            Value::set([Value::string("5")]),
            Value::set([Value::unknown(Type::string())]),
            Value::unknown(Type::set(Type::string())).refine_not_null(),
        ),
    ];

    for (i, (input_a, input_b, want)) in tests.iter().enumerate() {
        let got = match set_subtract(input_a, input_b) {
            Ok(got) => got,
            Err(err) => panic!("case {i}: unexpected error: {err}"),
        };
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// Ported from TestSetSymmetricDifference:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/set_test.go#L284
#[test]
#[ignore = "not yet implemented"]
fn set_symmetric_difference_test() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::set_empty(Type::string()),
            Value::set_empty(Type::string()),
            Value::set_empty(Type::string()),
        ),
        (
            Value::set([Value::bool(true)]),
            Value::set_empty(Type::string()),
            Value::set([Value::string("true")]),
        ),
        (
            Value::set([Value::bool(true)]),
            Value::set([Value::bool(false)]),
            Value::set([Value::bool(true), Value::bool(false)]),
        ),
        (
            Value::set([Value::string("a"), Value::string("b"), Value::string("c")]),
            Value::set([Value::string("a"), Value::string("c")]),
            Value::set([Value::string("b")]),
        ),
        (
            Value::set([Value::string("a")]),
            Value::set_empty(Type::dynamic()),
            Value::set([Value::string("a")]),
        ),
        (
            Value::set([Value::empty_object()]),
            Value::set_empty(Type::dynamic()),
            Value::set([Value::empty_object()]),
        ),
        (
            Value::set_empty(Type::dynamic()),
            Value::set_empty(Type::dynamic()),
            Value::set_empty(Type::dynamic()),
        ),
        (
            Value::set([Value::string("5")]),
            Value::unknown(Type::set(Type::number())),
            Value::unknown(Type::set(Type::string())).refine_not_null(),
        ),
        (
            Value::set([Value::string("5")]),
            Value::set([Value::unknown(Type::number())]),
            Value::unknown(Type::set(Type::string())).refine_not_null(),
        ),
    ];

    for (i, (input_a, input_b, want)) in tests.iter().enumerate() {
        let got = match set_symmetric_difference(&[input_a.clone(), input_b.clone()]) {
            Ok(got) => got,
            Err(err) => panic!("case {i}: unexpected error: {err}"),
        };
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}
