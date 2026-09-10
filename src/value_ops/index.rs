//! `Value::index` (go-cty: `cty/value_ops.go`).

use crate::value::Value;

impl Value {
    /// The element at the given key of a list, map, or tuple
    /// (go-cty: `Value.Index`).
    ///
    /// # Panics
    /// Panics if the key does not exist; check with [`Value::has_index`] first.
    pub fn index(&self, key: &Value) -> Value {
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

    // Ported from TestValueIndex:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2496
    //
    // Upstream TestValueIndex is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod value_index {
        use super::*;

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[(Value, Value, Value)]) {
            for (i, (collection, key, expected)) in tests.iter().enumerate() {
                let got = collection.index(key);
                assert_eq!(
                    got, *expected,
                    "case {i}: {collection:?}.index({key:?}) returned {got:?}; want {expected:?}"
                );
            }
        }

        // TestValueIndex, collections:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2496
        #[test]
        #[ignore = "not yet implemented"]
        fn collections() {
            let tests: Vec<(Value, Value, Value)> = vec![
                (
                    Value::list([Value::string("hello")]),
                    Value::number(0),
                    Value::string("hello"),
                ),
                (
                    Value::list([Value::string("hello"), Value::string("world")]),
                    Value::number(1),
                    Value::string("world"),
                ),
                (
                    Value::map([("greeting", Value::string("hello"))]),
                    Value::string("greeting"),
                    Value::string("hello"),
                ),
                (
                    Value::map([("gr\u{e9}eting", Value::string("hello"))]), // precombined é
                    Value::string("gre\u{301}eting"), // e with combining acute accent
                    Value::string("hello"),
                ),
                (
                    Value::tuple([Value::string("hello")]),
                    Value::number(0),
                    Value::string("hello"),
                ),
                (
                    Value::tuple([Value::string("hello"), Value::number(5)]),
                    Value::number(0),
                    Value::string("hello"),
                ),
                (
                    Value::tuple([Value::string("hello"), Value::number(5)]),
                    Value::number(1),
                    Value::number(5),
                ),
            ];

            check(&tests);
        }

        // TestValueIndex, unknowns:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2496
        #[test]
        #[ignore = "not yet implemented"]
        fn unknowns() {
            let tests: Vec<(Value, Value, Value)> = vec![
                (
                    Value::list([Value::string("hello")]),
                    Value::unknown(Type::number()),
                    Value::unknown(Type::string()),
                ),
                (
                    Value::list([Value::string("hello")]),
                    Value::dynamic(),
                    Value::unknown(Type::string()),
                ),
                (
                    Value::unknown(Type::list(Type::string())),
                    Value::number(0),
                    Value::unknown(Type::string()),
                ),
                (
                    Value::map([("greeting", Value::bool(true))]),
                    Value::unknown(Type::string()),
                    Value::unknown(Type::bool()),
                ),
                (
                    Value::map([("greeting", Value::bool(true))]),
                    Value::dynamic(),
                    Value::unknown(Type::bool()),
                ),
                (
                    Value::unknown(Type::map(Type::string())),
                    Value::string("greeting"),
                    Value::unknown(Type::string()),
                ),
                (Value::dynamic(), Value::string("hello"), Value::dynamic()),
                (Value::dynamic(), Value::number(0), Value::dynamic()),
                (
                    Value::tuple([Value::string("hello"), Value::dynamic()]),
                    Value::number(0),
                    Value::string("hello"),
                ),
                (
                    Value::tuple([Value::string("hello"), Value::dynamic()]),
                    Value::number(1),
                    Value::dynamic(),
                ),
                (
                    Value::tuple([Value::string("hello"), Value::unknown(Type::number())]),
                    Value::number(0),
                    Value::string("hello"),
                ),
                (
                    Value::tuple([Value::string("hello"), Value::unknown(Type::number())]),
                    Value::number(1),
                    Value::unknown(Type::number()),
                ),
                (
                    Value::tuple([Value::string("hello"), Value::unknown(Type::number())]),
                    Value::unknown(Type::number()),
                    Value::dynamic(),
                ),
                (
                    Value::unknown(Type::tuple([Type::string()])),
                    Value::number(0),
                    Value::unknown(Type::string()),
                ),
            ];

            check(&tests);
        }

        // TestValueIndex, marks:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2496
        #[test]
        #[ignore = "not yet implemented"]
        fn marks() {
            let tests: Vec<(Value, Value, Value)> = vec![
                (
                    Value::list([Value::string("hello")]).mark(1),
                    Value::number(0),
                    Value::string("hello").mark(1),
                ),
                (
                    Value::list([Value::string("hello")]),
                    Value::number(0).mark(1),
                    Value::string("hello").mark(1),
                ),
            ];

            check(&tests);
        }
    }
}
