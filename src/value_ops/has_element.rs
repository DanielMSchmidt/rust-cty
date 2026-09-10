//! `Value::has_element` (go-cty: `cty/value_ops.go`).

use crate::value::Value;

impl Value {
    /// Whether a set contains the given element, as a bool value
    /// (go-cty: `Value.HasElement`).
    pub fn has_element(&self, element: &Value) -> Value {
        let _ = element;
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

    // Ported from TestHasElement:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L3908
    #[test]
    #[ignore = "not yet implemented"]
    fn has_element() {
        let tests: Vec<(Value, Value, Value)> = vec![
            (
                Value::set_empty(Type::string()),
                Value::string("hello"),
                Value::bool(false),
            ),
            (
                Value::set([Value::string("hello")]),
                Value::string("hello"),
                Value::bool(true),
            ),
            (
                Value::set([Value::string("hello"), Value::string("world")]),
                Value::string("hello"),
                Value::bool(true),
            ),
            (
                Value::set([Value::string("hello"), Value::string("world")]),
                Value::string("hi"),
                Value::bool(false),
            ),
            (
                Value::set([Value::string("hello"), Value::unknown(Type::string())]),
                Value::string("hello"),
                // "hello" is definitely present regardless of what the unknown value is
                Value::bool(true),
            ),
            (
                Value::set([Value::string("hello"), Value::unknown(Type::string())]),
                Value::string("world"),
                // The unknown value might turn out to be "world"
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::set([Value::unknown(Type::string())]),
                Value::string("world"),
                // The unknown value might turn out to be "world"
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::set([
                    Value::unknown(Type::string()),
                    Value::unknown(Type::string()),
                ]),
                Value::string("world"),
                // One of the unknown values might turn out to be "world"
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::set([Value::string("hello"), Value::unknown(Type::string())]),
                Value::bool(true),
                // A set of string cannot possibly contain a bool
                Value::bool(false),
            ),
            (
                Value::set([Value::string("hello"), Value::unknown(Type::string())]),
                Value::unknown(Type::string()),
                // The unknowns are placeholders for values, not values themselves, so the presence of an unknown
                // in the set doesn't cause this to return true: there's no guarantee that both of the unknowns
                // above will be equal once finalized.
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::set([Value::string("hello"), Value::string("world")]),
                Value::unknown(Type::string()),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::set([Value::string("hello"), Value::string("world")]),
                Value::dynamic(),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::dynamic(),
                Value::string("hello"),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::set([Value::null(Type::dynamic())]),
                Value::null(Type::dynamic()),
                Value::bool(true),
            ),
            (
                Value::set([Value::dynamic()]),
                Value::null(Type::dynamic()),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::set([Value::dynamic()]),
                Value::dynamic(),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
        ];

        for (i, (set, elem, want)) in tests.iter().enumerate() {
            let got = set.has_element(elem);
            assert_eq!(
                got, *want,
                "case {i}: {set:?}.has_element({elem:?}) returned {got:?}; want {want:?}"
            );
        }
    }
}
