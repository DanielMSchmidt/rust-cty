//! `flatten` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`flatten`] (go-cty: `stdlib.FlattenFunc`).
pub fn flatten_func() -> Function {
    todo!()
}

/// The sequence with nested sequences flattened, recursively
/// (go-cty: `stdlib.Flatten`).
pub fn flatten(list: &Value) -> Result<Value, CtyError> {
    let _ = list;
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

    // Ported from TestFlatten:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L2010
    //
    // Upstream TestFlatten is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod flatten {
        use super::*;

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[(Value, Value, &'static str)]) {
            for (i, (list, want, want_err)) in tests.iter().enumerate() {
                let got = stdlib::flatten(list);
                if !want_err.is_empty() {
                    let err = got.err().unwrap_or_else(|| {
                        panic!("case {i}: succeeded; want error");
                    });
                    assert_eq!(err.to_string(), *want_err, "case {i}: wrong error");
                    continue;
                }
                let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
                assert_eq!(got, *want, "case {i}: wrong result");
            }
        }

        // TestFlatten, primitives:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L2010
        #[test]
        #[ignore = "not yet implemented"]
        fn primitives() {
            let tests: Vec<(Value, Value, &'static str)> = vec![
                // Empty case is easy
                (Value::list_empty(Type::string()), Value::empty_tuple(), ""),
                (Value::list_empty(Type::number()), Value::empty_tuple(), ""),
            ];

            check(&tests);
        }

        // TestFlatten, collections:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L2010
        #[test]
        #[ignore = "not yet implemented"]
        fn collections() {
            let tests: Vec<(Value, Value, &'static str)> = vec![
                (
                    Value::tuple([
                        Value::string("a"),
                        Value::list([Value::string("b")]),
                        Value::tuple([
                            Value::list([Value::string("c")]),
                            Value::list([Value::string("d"), Value::string("e")]),
                        ]),
                    ]),
                    Value::tuple([
                        Value::string("a"),
                        Value::string("b"),
                        Value::string("c"),
                        Value::string("d"),
                        Value::string("e"),
                    ]),
                    "",
                ),
                // null of a string type
                (
                    Value::tuple([Value::null(Type::string()), Value::bool(true)]),
                    Value::tuple([Value::null(Type::string()), Value::bool(true)]),
                    "",
                ),
                // null of a list type
                (
                    Value::tuple([Value::null(Type::list(Type::string())), Value::bool(true)]),
                    Value::tuple([Value::null(Type::list(Type::string())), Value::bool(true)]),
                    "",
                ),
                // null of a tuple type
                (
                    Value::tuple([Value::null(Type::empty_tuple()), Value::bool(true)]),
                    Value::tuple([Value::null(Type::empty_tuple()), Value::bool(true)]),
                    "",
                ),
                // nested null of a string type
                (
                    Value::tuple([
                        Value::tuple([Value::null(Type::string())]),
                        Value::bool(true),
                    ]),
                    Value::tuple([Value::null(Type::string()), Value::bool(true)]),
                    "",
                ),
                // nested null of a list type
                (
                    Value::tuple([
                        Value::tuple([Value::null(Type::list(Type::string()))]),
                        Value::bool(true),
                    ]),
                    Value::tuple([Value::null(Type::list(Type::string())), Value::bool(true)]),
                    "",
                ),
                // nested null of a tuple type
                (
                    Value::tuple([
                        Value::tuple([Value::null(Type::empty_tuple())]),
                        Value::bool(true),
                    ]),
                    Value::tuple([Value::null(Type::empty_tuple()), Value::bool(true)]),
                    "",
                ),
            ];

            check(&tests);
        }

        // TestFlatten, unknowns:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L2010
        #[test]
        #[ignore = "not yet implemented"]
        fn unknowns() {
            let tests: Vec<(Value, Value, &'static str)> = vec![
                // Lists can contain unknown values
                (
                    Value::list([
                        Value::list([Value::unknown(Type::string()), Value::string("a")]),
                        Value::list([
                            Value::unknown(Type::string()),
                            Value::string("b"),
                            Value::unknown(Type::string()),
                        ]),
                    ]),
                    Value::tuple([
                        Value::unknown(Type::string()),
                        Value::string("a"),
                        Value::unknown(Type::string()),
                        Value::string("b"),
                        Value::unknown(Type::string()),
                    ]),
                    "",
                ),
                // If the list itself is unknown this is the best we can do
                (
                    Value::unknown(Type::list(Type::list(Type::string()))),
                    Value::unknown(Type::dynamic()),
                    "",
                ),
                // Type error
                (
                    Value::map_empty(Type::string()),
                    Value::dynamic(),
                    "can only flatten lists, sets and tuples",
                ),
                (Value::list([Value::dynamic()]), Value::dynamic(), ""),
                (
                    Value::tuple([
                        Value::list([Value::object([("blop", Value::list([Value::dynamic()]))])]),
                        Value::list([Value::object([("bloop", Value::dynamic())])]),
                    ]),
                    Value::tuple([
                        Value::object([("blop", Value::list([Value::dynamic()]))]),
                        Value::object([("bloop", Value::dynamic())]),
                    ]),
                    "",
                ),
                (
                    Value::list([
                        Value::list([Value::object([("bloop", Value::dynamic())])]),
                        Value::list([Value::object([("bloop", Value::dynamic())])]),
                    ]),
                    Value::tuple([
                        Value::object([("bloop", Value::dynamic())]),
                        Value::object([("bloop", Value::dynamic())]),
                    ]),
                    "",
                ),
                (
                    Value::tuple([
                        Value::tuple([Value::string("a"), Value::string("b")]),
                        Value::null(Type::dynamic()),
                        Value::tuple([Value::string("c")]),
                    ]),
                    Value::tuple([
                        Value::string("a"),
                        Value::string("b"),
                        Value::null(Type::dynamic()),
                        Value::string("c"),
                    ]),
                    "",
                ),
                (
                    Value::tuple([
                        Value::tuple([Value::string("a"), Value::string("b")]),
                        Value::dynamic(),
                        Value::tuple([Value::string("c")]),
                    ]),
                    Value::unknown(Type::dynamic()),
                    "",
                ),
                // null of an unknown type
                (
                    Value::tuple([Value::null(Type::dynamic()), Value::bool(true)]),
                    Value::tuple([Value::null(Type::dynamic()), Value::bool(true)]),
                    "",
                ),
                // nested null of an unknown type
                (
                    Value::tuple([
                        Value::tuple([Value::null(Type::dynamic())]),
                        Value::bool(true),
                    ]),
                    Value::tuple([Value::null(Type::dynamic()), Value::bool(true)]),
                    "",
                ),
            ];

            check(&tests);
        }

        // TestFlatten, marks:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L2010
        #[test]
        #[ignore = "not yet implemented"]
        fn marks() {
            let tests: Vec<(Value, Value, &'static str)> = vec![
                // Top-level list marks should carry over
                (
                    Value::list([
                        Value::list([Value::string("a")]),
                        Value::list([Value::string("b"), Value::string("c")]),
                        Value::list_empty(Type::string()),
                    ])
                    .mark("mark"),
                    Value::tuple([Value::string("a"), Value::string("b"), Value::string("c")])
                        .mark("mark"),
                    "",
                ),
                // Inner list marks should apply to the result collection
                (
                    Value::list([
                        Value::list([Value::string("a")]).mark("first"),
                        Value::list([Value::string("b"), Value::string("c")]).mark("second"),
                        Value::list_empty(Type::string()).mark("third"),
                    ]),
                    Value::tuple([Value::string("a"), Value::string("b"), Value::string("c")])
                        .with_marks([ValueMarks::from_marks(["first", "second", "third"])]),
                    "",
                ),
                // Non-list element marks should be retained on the element only
                (
                    Value::list([
                        Value::list([Value::string("a").mark("a")]),
                        Value::list([Value::string("b").mark("b"), Value::string("c").mark("b")]),
                    ]),
                    Value::tuple([
                        Value::string("a").mark("a"),
                        Value::string("b").mark("b"),
                        Value::string("c").mark("b"),
                    ]),
                    "",
                ),
                // Nested unknown lists/sets/tuples should still propagate marks
                (
                    Value::list([
                        Value::list([Value::string("a")]).mark("first"),
                        Value::unknown(Type::list(Type::string())).mark("second"),
                        Value::list([Value::string("c")]).mark("third"),
                    ]),
                    Value::unknown(Type::dynamic())
                        .with_marks([ValueMarks::from_marks(["first", "second", "third"])]),
                    "",
                ),
                // Empty marked list retains marks
                (
                    Value::list_empty(Type::string()).mark("a"),
                    Value::empty_tuple().mark("a"),
                    "",
                ),
                (
                    Value::tuple([
                        Value::list([Value::list([Value::dynamic()])]),
                        Value::list([Value::list([Value::dynamic()]).mark("marked")]),
                    ]),
                    Value::dynamic().mark("marked"),
                    "",
                ),
            ];

            check(&tests);
        }
    }
}
