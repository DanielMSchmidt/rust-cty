//! `Value::get_attr` (go-cty: `cty/value_ops.go`).

use crate::value::Value;

impl Value {
    /// The value of the named attribute of an object value
    /// (go-cty: `Value.GetAttr`).
    ///
    /// # Panics
    /// Panics if this is not an object value or has no such attribute.
    pub fn get_attr(&self, name: &str) -> Value {
        let _ = name;
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

    // Ported from TestValueGetAttr:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2445
    #[test]
    #[ignore = "not yet implemented"]
    fn value_get_attr() {
        let tests: Vec<(Value, &str, Value)> = vec![
            (
                Value::object([("greeting", Value::string("hello"))]),
                "greeting",
                Value::string("hello"),
            ),
            (
                Value::object([("greeting", Value::string("hello"))]),
                "greeting",
                Value::string("hello"),
            ),
            (
                Value::unknown(Type::object([("gr\u{e9}eting", Type::string())])), // precombined é
                "gre\u{301}eting", // e with combining acute accent
                Value::unknown(Type::string()),
            ),
            (Value::dynamic(), "hello", Value::dynamic()),
            (
                Value::object([("greeting", Value::string("hello"))]).mark(1),
                "greeting",
                Value::string("hello").mark(1),
            ),
        ];

        for (i, (object, attr_name, expected)) in tests.iter().enumerate() {
            let got = object.get_attr(attr_name);
            assert_eq!(
                got, *expected,
                "case {i}: {object:?}.get_attr({attr_name:?}) returned {got:?}; want {expected:?}"
            );
        }
    }
}
