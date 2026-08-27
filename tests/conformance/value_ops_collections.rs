//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/value_ops_test.go (TestValueGetAttr, TestValueIndex, TestValueHasIndex, TestValueForEachElement, TestHasElement, TestElements)
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Type, Value};

// upstream: cty/value_ops_test.go TestValueGetAttr
#[test]
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

// upstream: cty/value_ops_test.go TestValueIndex
#[test]
fn value_index() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::list([Value::string("hello")]),
            Value::number_int(0),
            Value::string("hello"),
        ),
        (
            Value::list([Value::string("hello"), Value::string("world")]),
            Value::number_int(1),
            Value::string("world"),
        ),
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
            Value::number_int(0),
            Value::unknown(Type::string()),
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
        (Value::dynamic(), Value::number_int(0), Value::dynamic()),
        (
            Value::tuple([Value::string("hello")]),
            Value::number_int(0),
            Value::string("hello"),
        ),
        (
            Value::tuple([Value::string("hello"), Value::number_int(5)]),
            Value::number_int(0),
            Value::string("hello"),
        ),
        (
            Value::tuple([Value::string("hello"), Value::number_int(5)]),
            Value::number_int(1),
            Value::number_int(5),
        ),
        (
            Value::tuple([Value::string("hello"), Value::dynamic()]),
            Value::number_int(0),
            Value::string("hello"),
        ),
        (
            Value::tuple([Value::string("hello"), Value::dynamic()]),
            Value::number_int(1),
            Value::dynamic(),
        ),
        (
            Value::tuple([Value::string("hello"), Value::unknown(Type::number())]),
            Value::number_int(0),
            Value::string("hello"),
        ),
        (
            Value::tuple([Value::string("hello"), Value::unknown(Type::number())]),
            Value::number_int(1),
            Value::unknown(Type::number()),
        ),
        (
            Value::tuple([Value::string("hello"), Value::unknown(Type::number())]),
            Value::unknown(Type::number()),
            Value::dynamic(),
        ),
        (
            Value::unknown(Type::tuple([Type::string()])),
            Value::number_int(0),
            Value::unknown(Type::string()),
        ),
        (
            Value::list([Value::string("hello")]).mark(1),
            Value::number_int(0),
            Value::string("hello").mark(1),
        ),
        (
            Value::list([Value::string("hello")]),
            Value::number_int(0).mark(1),
            Value::string("hello").mark(1),
        ),
    ];

    for (i, (collection, key, expected)) in tests.iter().enumerate() {
        let got = collection.index(key);
        assert_eq!(
            got, *expected,
            "case {i}: {collection:?}.index({key:?}) returned {got:?}; want {expected:?}"
        );
    }
}

// upstream: cty/value_ops_test.go TestValueHasIndex
#[test]
fn value_has_index() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::list([Value::string("hello")]),
            Value::number_int(0),
            Value::bool(true),
        ),
        (
            Value::list([Value::string("hello"), Value::string("world")]),
            Value::number_int(1),
            Value::bool(true),
        ),
        (
            Value::list([Value::string("hello"), Value::string("world")]),
            Value::number_int(2),
            Value::bool(false),
        ),
        (
            Value::list([Value::string("hello"), Value::string("world")]),
            Value::number_int(-1),
            Value::bool(false),
        ),
        (
            Value::list([Value::string("hello"), Value::string("world")]),
            Value::number_float(0.5),
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
            Value::number_int(0),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::list(Type::string())),
            Value::string("hello"),
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
            Value::number_int(0),
            Value::bool(false),
        ),
        (
            Value::tuple([Value::string("hello")]),
            Value::number_int(0),
            Value::bool(true),
        ),
        (
            Value::tuple([Value::string("hello"), Value::string("world")]),
            Value::number_int(1),
            Value::bool(true),
        ),
        (
            Value::tuple([Value::string("hello"), Value::string("world")]),
            Value::number_int(2),
            Value::bool(false),
        ),
        (
            Value::tuple([Value::string("hello"), Value::string("world")]),
            Value::number_int(-1),
            Value::bool(false),
        ),
        (
            Value::tuple([Value::string("hello"), Value::string("world")]),
            Value::number_float(0.5),
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
        (
            Value::tuple([Value::string("hello")]),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::tuple([Type::string()])),
            Value::number_int(0),
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
            Value::number_int(0),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::list([Value::string("hello")]).mark(1),
            Value::number_int(0),
            Value::bool(true).mark(1),
        ),
        (
            Value::list([Value::string("hello")]),
            Value::number_int(0).mark(1),
            Value::bool(true).mark(1),
        ),
    ];

    for (i, (collection, key, expected)) in tests.iter().enumerate() {
        let got = collection.has_index(key);
        assert_eq!(
            got, *expected,
            "case {i}: {collection:?}.has_index({key:?}) returned {got:?}; want {expected:?}"
        );
    }
}

// upstream: cty/value_ops_test.go TestValueForEachElement
#[test]
fn value_for_each_element() {
    // Upstream's `type call struct { Key, Element Value }`.
    type Call = (Value, Value);
    // Each case is (receiver, expected (key, element) calls, expected stopped).
    let tests: Vec<(Value, Vec<Call>, bool)> = vec![
        (Value::list_empty(Type::string()), vec![], false),
        (
            Value::list([Value::number_int(1), Value::number_int(2)]),
            vec![
                (Value::number_int(0), Value::number_int(1)),
                (Value::number_int(1), Value::number_int(2)),
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
                (Value::number_int(0), Value::string("hey")),
                (Value::number_int(1), Value::string("stop")),
            ],
            true,
        ),
        (Value::set_empty(Type::string()), vec![], false),
        (
            Value::set([
                Value::number_int(1),
                Value::number_int(10),
                Value::number_int(2),
            ]),
            vec![
                // Numbers in sets are always iterated in numerical order.
                (Value::number_int(1), Value::number_int(1)),
                (Value::number_int(2), Value::number_int(2)),
                (Value::number_int(10), Value::number_int(10)),
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
            Value::map([
                ("second", Value::number_int(2)),
                ("first", Value::number_int(1)),
            ]),
            vec![
                (Value::string("first"), Value::number_int(1)),
                (Value::string("second"), Value::number_int(2)),
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
        (Value::empty_tuple(), vec![], false),
        (
            Value::tuple([Value::string("hello"), Value::number_int(2)]),
            vec![
                (Value::number_int(0), Value::string("hello")),
                (Value::number_int(1), Value::number_int(2)),
            ],
            false,
        ),
        (
            Value::tuple([
                Value::number_int(5),
                Value::string("stop"),
                Value::bool(true),
            ]),
            vec![
                (Value::number_int(0), Value::number_int(5)),
                (Value::number_int(1), Value::string("stop")),
            ],
            true,
        ),
        (Value::empty_object(), vec![], false),
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

// upstream: cty/value_ops_test.go TestHasElement
#[test]
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

// upstream: cty/value_ops_test.go TestElements
// NOTE(port): upstream `Elements()` is a Go 1.23 iter.Seq2 range function; the
// Rust analogue is the std Iterator returned by `element_iterator()`.
#[test]
fn elements() {
    let tests: Vec<(Value, Vec<(Value, Value)>)> = vec![
        (Value::list_empty(Type::string()), vec![]),
        (
            Value::list([Value::string("hello"), Value::string("world")]),
            vec![
                (Value::number_int(0), Value::string("hello")),
                (Value::number_int(1), Value::string("world")),
            ],
        ),
        (
            Value::tuple([Value::string("hello"), Value::string("world")]),
            vec![
                (Value::number_int(0), Value::string("hello")),
                (Value::number_int(1), Value::string("world")),
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
