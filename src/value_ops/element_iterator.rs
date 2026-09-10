//! `Value::element_iterator` (go-cty: `cty/value_ops.go`).

use super::ElementIterator;
use crate::value::Value;

impl Value {
    /// An iterator over the `(key, value)` element pairs of a known collection,
    /// tuple, or object value (go-cty: `Value.ElementIterator` / `Value.Elements`).
    ///
    /// # Panics
    /// Panics if [`Value::can_iterate_elements`] would return `false`.
    pub fn element_iterator(&self) -> ElementIterator {
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

    // Ported from TestElements:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L4048
    // NOTE(port): upstream `Elements()` is a Go 1.23 iter.Seq2 range function; the
    // Rust analogue is the std Iterator returned by `element_iterator()`.
    #[test]
    #[ignore = "not yet implemented"]
    fn elements() {
        let tests: Vec<(Value, Vec<(Value, Value)>)> = vec![
            (Value::list_empty(Type::string()), vec![]),
            (
                Value::list([Value::string("hello"), Value::string("world")]),
                vec![
                    (Value::number(0), Value::string("hello")),
                    (Value::number(1), Value::string("world")),
                ],
            ),
            (
                Value::tuple([Value::string("hello"), Value::string("world")]),
                vec![
                    (Value::number(0), Value::string("hello")),
                    (Value::number(1), Value::string("world")),
                ],
            ),
            (
                Value::set([Value::string("hello"), Value::string("world")]),
                vec![
                    // When the element type is string, the results are returned
                    // in lexicographical order. Otherwise the order is unspecified.
                    (Value::string("hello"), Value::string("hello")),
                    (Value::string("world"), Value::string("world")),
                ],
            ),
            (
                Value::map([
                    ("greeting", Value::string("hello")),
                    ("greetee", Value::string("world")),
                ]),
                vec![
                    (Value::string("greetee"), Value::string("world")),
                    (Value::string("greeting"), Value::string("hello")),
                ],
            ),
            (
                Value::object([
                    ("greeting", Value::string("hello")),
                    ("greetee", Value::string("world")),
                ]),
                vec![
                    (Value::string("greetee"), Value::string("world")),
                    (Value::string("greeting"), Value::string("hello")),
                ],
            ),
        ];

        for (i, (input, want)) in tests.iter().enumerate() {
            let got: Vec<(Value, Value)> = input.element_iterator().collect();
            assert_eq!(got, *want, "case {i}: wrong elements from {input:?}");
        }
    }
}
