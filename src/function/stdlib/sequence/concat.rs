//! `concat` (go-cty: `cty/function/stdlib/sequence.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`concat`] (go-cty: `stdlib.ConcatFunc`).
pub fn concat_func() -> Function {
    todo!()
}

/// The given sequences concatenated into a single tuple or list
/// (go-cty: `stdlib.Concat`).
pub fn concat(seqs: &[Value]) -> Result<Value, CtyError> {
    let _ = seqs;
    todo!()
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/function/stdlib/sequence_test.go
    //!   cty/function/stdlib/set_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::function::stdlib::concat;
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
}
