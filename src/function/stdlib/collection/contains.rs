//! `contains` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`contains`] (go-cty: `stdlib.ContainsFunc`).
pub fn contains_func() -> Function {
    todo!()
}

/// Whether the list contains the given value (go-cty: `stdlib.Contains`).
pub fn contains(list: &Value, value: &Value) -> Result<Value, CtyError> {
    let _ = (list, value);
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

    // Ported from TestContains:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L321
    //
    // Upstream TestContains is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod contains {
        use super::*;

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[(Value, Value, Value, bool)]) {
            for (i, (list, value, want, want_err)) in tests.iter().enumerate() {
                let got = stdlib::contains(list, value);
                if *want_err {
                    assert!(got.is_err(), "case {i}: succeeded; want error");
                    continue;
                }
                let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
                assert_eq!(got, *want, "case {i}: wrong result");
            }
        }

        // TestContains, primitives:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L321
        #[test]
        #[ignore = "not yet implemented"]
        fn primitives() {
            let list_of_strings = Value::list([
                Value::string("the"),
                Value::string("quick"),
                Value::string("brown"),
                Value::string("fox"),
            ]);
            let list_of_ints = Value::list([
                Value::number(1),
                Value::number(2),
                Value::number(3),
                Value::number(4),
            ]);
            let tests: Vec<(Value, Value, Value, bool)> = vec![
                (
                    list_of_strings.clone(),
                    Value::string("the"),
                    Value::bool(true),
                    false,
                ),
                (
                    list_of_strings.clone(),
                    Value::string("penguin"),
                    Value::bool(false),
                    false,
                ),
                (
                    list_of_ints.clone(),
                    Value::number(1),
                    Value::bool(true),
                    false,
                ),
                (
                    list_of_ints.clone(),
                    Value::number(42),
                    Value::bool(false),
                    false,
                ),
                // And now we mix and match
                (
                    list_of_ints.clone(),
                    Value::string("1"),
                    Value::bool(false),
                    false,
                ),
            ];

            check(&tests);
        }

        // TestContains, collections:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L321
        #[test]
        #[ignore = "not yet implemented"]
        fn collections() {
            let tests: Vec<(Value, Value, Value, bool)> = vec![
                // set val
                (
                    Value::set([
                        Value::string("quick"),
                        Value::string("brown"),
                        Value::string("fox"),
                    ]),
                    Value::string("quick"),
                    Value::bool(true),
                    false,
                ),
                // tuple val
                (
                    Value::tuple([
                        Value::string("quick"),
                        Value::string("brown"),
                        Value::number(3),
                    ]),
                    Value::number(3),
                    Value::bool(true),
                    false,
                ),
            ];

            check(&tests);
        }

        // TestContains, unknowns:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L321
        #[test]
        #[ignore = "not yet implemented"]
        fn unknowns() {
            let list_with_unknown = Value::list([
                Value::string("the"),
                Value::string("quick"),
                Value::string("brown"),
                Value::unknown(Type::string()),
            ]);
            let tests: Vec<(Value, Value, Value, bool)> = vec![
                (
                    list_with_unknown.clone(),
                    Value::string("the"),
                    Value::bool(true),
                    false,
                ),
                (
                    list_with_unknown.clone(),
                    Value::string("orange"),
                    Value::unknown(Type::bool()).refine_not_null(),
                    false,
                ),
                // Check a list with an unknown value
                (
                    Value::list([
                        Value::unknown(Type::string()),
                        Value::string("quick"),
                        Value::string("brown"),
                        Value::string("fox"),
                    ]),
                    Value::string("quick"),
                    Value::bool(true),
                    false,
                ),
                (
                    Value::list([
                        Value::unknown(Type::string()),
                        Value::string("brown"),
                        Value::string("fox"),
                    ]),
                    Value::string("quick"),
                    Value::unknown(Type::bool()).refine_not_null(),
                    false,
                ),
                (
                    Value::set([
                        Value::unknown(Type::string()),
                        Value::string("brown"),
                        Value::string("fox"),
                    ]),
                    Value::string("quick"),
                    Value::unknown(Type::bool()).refine_not_null(),
                    false,
                ),
                // nested unknown
                (
                    Value::list([Value::object([("a", Value::unknown(Type::string()))])]),
                    Value::object([("a", Value::string("b"))]),
                    Value::unknown(Type::bool()).refine_not_null(),
                    false,
                ),
            ];

            check(&tests);
        }
    }
}
