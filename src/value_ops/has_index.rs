//! `Value::has_index` (go-cty: `cty/value_ops.go`).

use crate::value::Value;

impl Value {
    /// Whether the collection has an element at the given key, as a bool value
    /// (go-cty: `Value.HasIndex`).
    pub fn has_index(&self, key: &Value) -> Value {
        let _ = key;
        todo!()
    }
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/value_ops_test.go (TestValueGetAttr, TestValueIndex, TestValueHasIndex, TestValueForEachElement, TestHasElement, TestElements)
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::{Type, Value};

    // Ported from TestValueHasIndex:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2629
    //
    // Upstream TestValueHasIndex is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod value_has_index {
        use super::*;

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[(Value, Value, Value)]) {
            for (i, (collection, key, expected)) in tests.iter().enumerate() {
                let got = collection.has_index(key);
                assert_eq!(
                    got, *expected,
                    "case {i}: {collection:?}.has_index({key:?}) returned {got:?}; want {expected:?}"
                );
            }
        }

        // TestValueHasIndex, collections:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2629
        #[test]
        #[ignore = "not yet implemented"]
        fn collections() {
            let tests: Vec<(Value, Value, Value)> = vec![
                (
                    Value::list([Value::string("hello")]),
                    Value::number(0),
                    Value::bool(true),
                ),
                (
                    Value::list([Value::string("hello"), Value::string("world")]),
                    Value::number(1),
                    Value::bool(true),
                ),
                (
                    Value::list([Value::string("hello"), Value::string("world")]),
                    Value::number(2),
                    Value::bool(false),
                ),
                (
                    Value::list([Value::string("hello"), Value::string("world")]),
                    Value::number(-1),
                    Value::bool(false),
                ),
                (
                    Value::list([Value::string("hello"), Value::string("world")]),
                    Value::number(0.5),
                    Value::bool(false),
                ),
                (
                    Value::list([Value::string("hello"), Value::string("world")]),
                    Value::string("greeting"),
                    Value::bool(false),
                ),
                (
                    Value::list([Value::string("hello"), Value::string("world")]),
                    Value::bool(true),
                    Value::bool(false),
                ),
                (
                    Value::map([("greeting", Value::string("hello"))]),
                    Value::string("greeting"),
                    Value::bool(true),
                ),
                (
                    Value::map([("gre\u{301}eting", Value::string("hello"))]), // e with combining acute accent
                    Value::string("gr\u{e9}eting"),                            // precombined é
                    Value::bool(true),
                ),
                (
                    Value::map([("greeting", Value::string("hello"))]),
                    Value::string("grouting"),
                    Value::bool(false),
                ),
                (
                    Value::map([("greeting", Value::string("hello"))]),
                    Value::string(""),
                    Value::bool(false),
                ),
                (
                    Value::map([("greeting", Value::string("hello"))]),
                    Value::zero(),
                    Value::bool(false),
                ),
                (
                    Value::map([("greeting", Value::string("hello"))]),
                    Value::bool(true),
                    Value::bool(false),
                ),
                (
                    Value::tuple([Value::string("hello")]),
                    Value::number(0),
                    Value::bool(true),
                ),
                (
                    Value::tuple([Value::string("hello"), Value::string("world")]),
                    Value::number(1),
                    Value::bool(true),
                ),
                (
                    Value::tuple([Value::string("hello"), Value::string("world")]),
                    Value::number(2),
                    Value::bool(false),
                ),
                (
                    Value::tuple([Value::string("hello"), Value::string("world")]),
                    Value::number(-1),
                    Value::bool(false),
                ),
                (
                    Value::tuple([Value::string("hello"), Value::string("world")]),
                    Value::number(0.5),
                    Value::bool(false),
                ),
                (
                    Value::tuple([Value::string("hello"), Value::string("world")]),
                    Value::string("greeting"),
                    Value::bool(false),
                ),
                (
                    Value::tuple([Value::string("hello"), Value::string("world")]),
                    Value::bool(true),
                    Value::bool(false),
                ),
            ];

            check(&tests);
        }

        // TestValueHasIndex, unknowns:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2629
        #[test]
        #[ignore = "not yet implemented"]
        fn unknowns() {
            let tests: Vec<(Value, Value, Value)> = vec![
                (
                    Value::list([Value::string("hello")]),
                    Value::unknown(Type::number()),
                    Value::unknown(Type::bool()).refine_not_null(),
                ),
                (
                    Value::list([Value::string("hello")]),
                    Value::dynamic(),
                    Value::unknown(Type::bool()).refine_not_null(),
                ),
                (
                    Value::unknown(Type::list(Type::string())),
                    Value::number(0),
                    Value::unknown(Type::bool()).refine_not_null(),
                ),
                (
                    Value::unknown(Type::list(Type::string())),
                    Value::string("hello"),
                    Value::bool(false),
                ),
                (
                    Value::map([("greeting", Value::string("hello"))]),
                    Value::unknown(Type::string()),
                    Value::unknown(Type::bool()).refine_not_null(),
                ),
                (
                    Value::map([("greeting", Value::string("hello"))]),
                    Value::dynamic(),
                    Value::unknown(Type::bool()).refine_not_null(),
                ),
                (
                    Value::unknown(Type::map(Type::string())),
                    Value::string("hello"),
                    Value::unknown(Type::bool()).refine_not_null(),
                ),
                (
                    Value::unknown(Type::map(Type::string())),
                    Value::number(0),
                    Value::bool(false),
                ),
                (
                    Value::tuple([Value::string("hello")]),
                    Value::unknown(Type::number()),
                    Value::unknown(Type::bool()).refine_not_null(),
                ),
                (
                    Value::unknown(Type::tuple([Type::string()])),
                    Value::number(0),
                    Value::bool(true),
                ),
                (
                    Value::tuple([Value::string("hello")]),
                    Value::dynamic(),
                    Value::unknown(Type::bool()).refine_not_null(),
                ),
                (
                    Value::dynamic(),
                    Value::string("hello"),
                    Value::unknown(Type::bool()).refine_not_null(),
                ),
                (
                    Value::dynamic(),
                    Value::number(0),
                    Value::unknown(Type::bool()).refine_not_null(),
                ),
            ];

            check(&tests);
        }

        // TestValueHasIndex, marks:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2629
        #[test]
        #[ignore = "not yet implemented"]
        fn marks() {
            let tests: Vec<(Value, Value, Value)> = vec![
                (
                    Value::list([Value::string("hello")]).mark(1),
                    Value::number(0),
                    Value::bool(true).mark(1),
                ),
                (
                    Value::list([Value::string("hello")]),
                    Value::number(0).mark(1),
                    Value::bool(true).mark(1),
                ),
            ];

            check(&tests);
        }
    }
}
