//! `Value::for_each_element` (go-cty: `cty/value_ops.go`).

use crate::value::Value;

impl Value {
    /// Calls `callback(key, value)` for each element; the callback returns
    /// `true` to stop early. Returns whether iteration was stopped early
    /// (go-cty: `Value.ForEachElement`).
    pub fn for_each_element(&self, callback: impl FnMut(Value, Value) -> bool) -> bool {
        let _ = &callback;
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

    // Ported from TestValueForEachElement:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2822
    //
    // Upstream TestValueForEachElement is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod value_for_each_element {
        use super::*;

        // Upstream's `type call struct { Key, Element Value }`.
        type Call = (Value, Value);

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[(Value, Vec<Call>, bool)]) {
            for (i, (receiver, expected, expected_stopped)) in tests.iter().enumerate() {
                let mut calls: Vec<(Value, Value)> = Vec::new();
                let stopped = receiver.for_each_element(|key, elem| {
                    // NOTE(port): upstream inspects the internal `elem.v == "stop"`
                    // field; the observable analogue is comparing against the string
                    // value "stop" with RawEquals semantics.
                    let stop = elem == Value::string("stop");
                    calls.push((key, elem));
                    stop
                });
                assert_eq!(
                    calls, *expected,
                    "case {i}: wrong calls from for_each_element on {receiver:?}"
                );
                assert_eq!(
                    stopped, *expected_stopped,
                    "case {i}: for_each_element returned {stopped}; want {expected_stopped}"
                );
            }
        }

        // TestValueForEachElement, primitives:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2822
        #[test]
        #[ignore = "not yet implemented"]
        fn primitives() {
            let tests: Vec<(Value, Vec<Call>, bool)> = vec![
                (Value::list_empty(Type::string()), vec![], false),
                (Value::set_empty(Type::string()), vec![], false),
                (Value::empty_tuple(), vec![], false),
                (Value::empty_object(), vec![], false),
            ];

            check(&tests);
        }

        // TestValueForEachElement, collections:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L2822
        #[test]
        #[ignore = "not yet implemented"]
        fn collections() {
            let tests: Vec<(Value, Vec<Call>, bool)> = vec![
                (
                    Value::list([Value::number(1), Value::number(2)]),
                    vec![
                        (Value::number(0), Value::number(1)),
                        (Value::number(1), Value::number(2)),
                    ],
                    false,
                ),
                (
                    Value::list([
                        Value::string("hey"),
                        Value::string("stop"),
                        Value::string("hey"),
                    ]),
                    vec![
                        (Value::number(0), Value::string("hey")),
                        (Value::number(1), Value::string("stop")),
                    ],
                    true,
                ),
                (
                    Value::set([Value::number(1), Value::number(10), Value::number(2)]),
                    vec![
                        // Numbers in sets are always iterated in numerical order.
                        (Value::number(1), Value::number(1)),
                        (Value::number(2), Value::number(2)),
                        (Value::number(10), Value::number(10)),
                    ],
                    false,
                ),
                (
                    Value::set([
                        Value::string("hi"),
                        Value::string("stop"),
                        Value::string("zzz"),
                    ]),
                    vec![
                        // Strings in sets are always iterated in lexicographical order.
                        (Value::string("hi"), Value::string("hi")),
                        (Value::string("stop"), Value::string("stop")),
                    ],
                    true,
                ),
                (
                    Value::map([("second", Value::number(2)), ("first", Value::number(1))]),
                    vec![
                        (Value::string("first"), Value::number(1)),
                        (Value::string("second"), Value::number(2)),
                    ],
                    false,
                ),
                (
                    Value::map([
                        ("item2", Value::string("value2")),
                        ("item1", Value::string("stop")),
                        ("item0", Value::string("value0")),
                    ]),
                    vec![
                        (Value::string("item0"), Value::string("value0")),
                        (Value::string("item1"), Value::string("stop")),
                    ],
                    true,
                ),
                (
                    Value::tuple([Value::string("hello"), Value::number(2)]),
                    vec![
                        (Value::number(0), Value::string("hello")),
                        (Value::number(1), Value::number(2)),
                    ],
                    false,
                ),
                (
                    Value::tuple([Value::number(5), Value::string("stop"), Value::bool(true)]),
                    vec![
                        (Value::number(0), Value::number(5)),
                        (Value::number(1), Value::string("stop")),
                    ],
                    true,
                ),
                (
                    Value::object([
                        ("bool", Value::bool(true)),
                        ("string", Value::string("hello")),
                    ]),
                    vec![
                        (Value::string("bool"), Value::bool(true)),
                        (Value::string("string"), Value::string("hello")),
                    ],
                    false,
                ),
            ];

            check(&tests);
        }
    }
}
