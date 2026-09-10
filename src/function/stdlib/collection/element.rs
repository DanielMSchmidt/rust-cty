//! `element` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`element`] (go-cty: `stdlib.ElementFunc`).
pub fn element_func() -> Function {
    todo!()
}

/// The element of the list at the given index, wrapping around past the end
/// (go-cty: `stdlib.Element`).
pub fn element(list: &Value, index: &Value) -> Result<Value, CtyError> {
    let _ = (list, index);
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

    // Ported from TestElement:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L1163
    //
    // Upstream TestElement is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod element {
        use super::*;

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[(Value, Value, Value, bool)]) {
            for (i, (list, idx, want, want_err)) in tests.iter().enumerate() {
                let got = stdlib::element(list, idx);
                if *want_err {
                    assert!(got.is_err(), "case {i}: succeeded; want error");
                    continue;
                }
                let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
                assert_eq!(got, *want, "case {i}: wrong result");
            }
        }

        // TestElement, primitives:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L1163
        #[test]
        #[ignore = "not yet implemented"]
        fn primitives() {
            // NOTE(deviation): cannot pass. Numbers are `f64` (deviation 1 in the
            // workbench's docs/deviations.md); upstream numbers are arbitrary-precision.
            // Out of range here: -9223372036854775809 and 9223372036854775808 below.
            // Kept as transcribed: the assertions state upstream behavior, which is the
            // point. Do not trim the table or relax them to make this green.
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
                    Value::number(2),
                    Value::string("brown"),
                    false,
                ),
                // negative index can be out of bounds too
                (
                    list_of_strings.clone(),
                    Value::number(-6),
                    Value::string("brown"),
                    false,
                ),
                // minimum valid index
                (
                    list_of_strings.clone(),
                    Value::number(f64::MIN),
                    Value::string("the"),
                    false,
                ),
                // maximum valid index
                (
                    list_of_strings.clone(),
                    Value::number(f64::MAX),
                    Value::string("fox"),
                    false,
                ),
                (
                    list_of_ints.clone(),
                    Value::number(2),
                    Value::number(3),
                    false,
                ),
                // index out of bounds of int64
                (
                    list_of_strings.clone(),
                    Value::parse_number("-9223372036854775809"),
                    Value::string("the"),
                    true,
                ),
                // index out of bounds of int64
                (
                    list_of_strings.clone(),
                    Value::parse_number("9223372036854775808"),
                    Value::string("fox"),
                    true,
                ),
            ];

            check(&tests);
        }

        // TestElement, collections:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L1163
        #[test]
        #[ignore = "not yet implemented"]
        fn collections() {
            let list_of_strings = Value::list([
                Value::string("the"),
                Value::string("quick"),
                Value::string("brown"),
                Value::string("fox"),
            ]);
            let tuple = Value::tuple([
                Value::string("the"),
                Value::unknown(Type::string()),
                Value::string("brown"),
                Value::bool(false),
            ]);
            let tests: Vec<(Value, Value, Value, bool)> = vec![
                // index greater than length(list)
                (
                    list_of_strings.clone(),
                    Value::number(5),
                    Value::string("quick"),
                    false,
                ),
                // negative index counts from the end of the list
                (
                    list_of_strings.clone(),
                    Value::number(-1),
                    Value::string("fox"),
                    false,
                ),
                // list of lists
                (
                    Value::list([list_of_strings.clone(), list_of_strings.clone()]),
                    Value::number(0),
                    list_of_strings.clone(),
                    false,
                ),
                (tuple.clone(), Value::number(0), Value::string("the"), false),
                (tuple.clone(), Value::number(3), Value::bool(false), false),
                (tuple.clone(), Value::number(4), Value::string("the"), false),
                (
                    tuple.clone(),
                    Value::number(10),
                    Value::string("brown"),
                    false,
                ),
                (tuple.clone(), Value::number(-1), Value::bool(false), false),
                (
                    tuple.clone(),
                    Value::number(-6),
                    Value::string("brown"),
                    false,
                ),
            ];

            check(&tests);
        }

        // TestElement, unknowns:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L1163
        #[test]
        #[ignore = "not yet implemented"]
        fn unknowns() {
            let list_of_strings = Value::list([
                Value::string("the"),
                Value::string("quick"),
                Value::string("brown"),
                Value::string("fox"),
            ]);
            let list_with_unknown = Value::list([
                Value::string("the"),
                Value::string("quick"),
                Value::string("brown"),
                Value::unknown(Type::string()),
            ]);
            let tuple = Value::tuple([
                Value::string("the"),
                Value::unknown(Type::string()),
                Value::string("brown"),
                Value::bool(false),
            ]);
            let unknown_tuple = Value::unknown(Type::tuple([
                Type::string(),
                Type::string(),
                Type::string(),
                Type::bool(),
            ]));
            let tests: Vec<(Value, Value, Value, bool)> = vec![
                (
                    list_of_strings.clone(),
                    Value::unknown(Type::number()),
                    Value::unknown(Type::string()),
                    false,
                ),
                (
                    list_with_unknown.clone(),
                    Value::number(2),
                    Value::string("brown"),
                    false,
                ),
                (
                    list_with_unknown.clone(),
                    Value::number(3),
                    Value::unknown(Type::string()),
                    false,
                ),
                (
                    list_of_strings.clone(),
                    Value::string("brown"), // definitely not an index
                    Value::dynamic(),
                    true,
                ),
                (
                    list_of_strings.clone(),
                    Value::number(0.5),
                    Value::dynamic(),
                    true,
                ),
                (
                    tuple.clone(),
                    Value::number(1),
                    Value::unknown(Type::string()),
                    false,
                ),
                (
                    unknown_tuple.clone(),
                    Value::number(0),
                    Value::unknown(Type::string()),
                    false,
                ),
                (
                    unknown_tuple.clone(),
                    Value::number(1),
                    Value::unknown(Type::string()),
                    false,
                ),
                (
                    unknown_tuple.clone(),
                    Value::number(3),
                    Value::unknown(Type::bool()),
                    false,
                ),
                (
                    unknown_tuple.clone(),
                    Value::number(4),
                    Value::unknown(Type::string()),
                    false,
                ),
                (
                    unknown_tuple.clone(),
                    Value::number(10),
                    Value::unknown(Type::string()),
                    false,
                ),
                (
                    unknown_tuple.clone(),
                    Value::number(-1),
                    Value::unknown(Type::bool()),
                    false,
                ),
                (
                    unknown_tuple.clone(),
                    Value::number(-6),
                    Value::unknown(Type::string()),
                    false,
                ),
            ];

            check(&tests);
        }

        // TestElement, marks:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L1163
        #[test]
        #[ignore = "not yet implemented"]
        fn marks() {
            let list_with_marks = Value::list([
                Value::string("the"),
                Value::string("quick"),
                Value::string("brown").mark("fox"),
                Value::unknown(Type::string()),
            ]);
            let tests: Vec<(Value, Value, Value, bool)> = vec![
                // preserve marks
                (
                    list_with_marks.clone(),
                    Value::number(2),
                    Value::string("brown").mark("fox"),
                    false,
                ),
                // marked items
                (
                    list_with_marks.clone(),
                    Value::number(1),
                    Value::string("quick"),
                    false,
                ),
                // The entire list is marked
                (
                    list_with_marks.clone().mark("thewholeshebang"),
                    Value::number(2),
                    Value::string("brown")
                        .with_marks([ValueMarks::from_marks(["thewholeshebang", "fox"])]),
                    false,
                ),
            ];

            check(&tests);
        }
    }
}
