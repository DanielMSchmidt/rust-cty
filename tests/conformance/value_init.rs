//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/value_init_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Value, ValueMarks};

// Ported from TestSetVal:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_init_test.go#L8
#[test]
#[ignore = "not yet implemented"]
fn set_val() {
    let plain = Value::set([Value::bool(true)]);
    let marked = Value::set([Value::bool(true)]).mark(1_i64);
    let deep_marked = Value::set([Value::bool(true).mark(2_i64), Value::bool(true).mark(3_i64)]);

    assert_ne!(
        plain, marked,
        "plain should be unequal to marked\nplain:  {plain:?}\nmarked: {marked:?}"
    );
    assert_ne!(
        marked, deep_marked,
        "marked should be unequal to deepMarked\nmarked:      {marked:?}\ndeepmarked: {deep_marked:?}"
    );
    {
        let got = marked.marks();
        let want = ValueMarks::from_marks([1_i64]);
        assert_eq!(got, want, "wrong marks for marked");
    }
    {
        let got = deep_marked.marks();
        let want = ValueMarks::from_marks([2_i64, 3_i64]);
        // Both 2 and 3 marks are preserved even though both of them are
        // marking the same value True, and thus the resulting set contains
        // only one element.
        assert_eq!(got, want, "wrong marks for deepMarked");
    }

    // NOTE(port): upstream calls the unexported `unmarkForce`, which is
    // `Unmark` with the returned marks discarded.
    {
        let (got, _) = deep_marked.unmark();
        let want = Value::set([Value::bool(true)]);
        assert_eq!(got, want, "wrong unmarked value for deepMarked");
    }
}

// Ported from TestSetVal_nestedStructures:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_init_test.go#L34
#[test]
#[ignore = "not yet implemented"]
fn set_val_nested_structures() {
    let test_cases: Vec<(&str, Vec<Value>)> = vec![
        ("set", vec![Value::set([Value::number_int(5)])]),
        (
            "doubly nested set",
            vec![Value::set([Value::set([Value::number_int(5)])])],
        ),
        ("list", vec![Value::list([Value::number_int(5)])]),
        (
            "doubly nested list",
            vec![Value::list([Value::list([Value::number_int(5)])])],
        ),
        ("map", vec![Value::map([("key", Value::number_int(5))])]),
        (
            "doubly nested map",
            vec![Value::map([(
                "key",
                Value::map([("child", Value::string("hello world"))]),
            )])],
        ),
        ("tuple", vec![Value::tuple([Value::number_int(5)])]),
        (
            "doubly nested tuple",
            vec![Value::tuple([Value::tuple([Value::number_int(5)])])],
        ),
    ];

    for (i, (name, elems)) in test_cases.into_iter().enumerate() {
        // Each case just needs to construct without panicking.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| Value::set(elems)));
        assert!(result.is_ok(), "case {i}-{name}: Value::set panicked");
    }
}

// Ported from TestCanListVal:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_init_test.go#L120
#[test]
#[ignore = "not yet implemented"]
fn can_list_val() {
    let test_cases: Vec<(Vec<Value>, bool)> = vec![
        // Valid lists
        (vec![Value::string("Hello"), Value::string("World")], true),
        (vec![Value::number_int(13), Value::number_int(31)], true),
        (vec![Value::bool(true), Value::bool(false)], true),
        (
            vec![
                Value::list([Value::string("Hello"), Value::string("World")]),
                Value::list([
                    Value::string("beep"),
                    Value::string("boop"),
                    Value::string("bloop"),
                ]),
            ],
            true,
        ),
        (
            vec![
                Value::map([("a", Value::string("Hello"))]),
                Value::map([("c", Value::string("World"))]),
            ],
            true,
        ),
        (
            vec![
                Value::set([Value::string("Hello"), Value::string("World")]),
                Value::set([
                    Value::string("beep"),
                    Value::string("boop"),
                    Value::string("bloop"),
                ]),
            ],
            true,
        ),
        // invalid list elements
        (vec![Value::string("hello"), Value::number_int(13)], false),
        (
            vec![
                Value::list([Value::string("Hello"), Value::string("World")]),
                Value::map([("a", Value::string("bloop"))]),
            ],
            false,
        ),
        // List of string and List of lists
        (
            vec![
                Value::list([Value::string("Hello"), Value::string("World")]),
                Value::list([
                    Value::list([Value::string("a"), Value::string("b")]),
                    Value::list([Value::string("c"), Value::string("d")]),
                ]),
            ],
            false,
        ),
        // Inconsistent map elements
        (
            vec![
                Value::map([("a", Value::string("Hello"))]),
                Value::map([("a", Value::bool(true))]),
            ],
            false,
        ),
    ];

    for (i, (elems, want)) in test_cases.iter().enumerate() {
        let got = Value::can_list(elems);
        assert_eq!(
            got, *want,
            "case {i}: wrong result for elements {elems:?}:\ngot {got}, want {want}"
        );
    }
}

// Ported from TestCanSetVal:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_init_test.go#L224
#[test]
#[ignore = "not yet implemented"]
fn can_set_val() {
    let test_cases: Vec<(Vec<Value>, bool)> = vec![
        // Valid set elements
        (vec![Value::string("Hello"), Value::string("World")], true),
        (
            vec![
                Value::string("Hello").mark(1_i64),
                Value::string("World").mark(2_i64),
            ],
            true,
        ),
        (vec![Value::number_int(13), Value::number_int(31)], true),
        (vec![Value::bool(true), Value::bool(false)], true),
        (
            vec![
                Value::list([Value::string("Hello"), Value::string("World")]),
                Value::list([
                    Value::string("beep"),
                    Value::string("boop"),
                    Value::string("bloop"),
                ]),
            ],
            true,
        ),
        (
            vec![
                Value::map([("a", Value::string("Hello"))]),
                Value::map([("c", Value::string("World"))]),
            ],
            true,
        ),
        (
            vec![
                Value::set([Value::string("Hello"), Value::string("World")]),
                Value::set([
                    Value::string("beep"),
                    Value::string("boop"),
                    Value::string("bloop"),
                ]),
            ],
            true,
        ),
        // invalid set elements
        (vec![Value::string("hello"), Value::number_int(13)], false),
        (
            vec![
                Value::list([Value::string("Hello"), Value::string("World")]),
                Value::map([("a", Value::string("bloop"))]),
            ],
            false,
        ),
        // List of string and List of lists
        (
            vec![
                Value::list([Value::string("Hello"), Value::string("World")]),
                Value::list([
                    Value::list([Value::string("a"), Value::string("b")]),
                    Value::list([Value::string("c"), Value::string("d")]),
                ]),
            ],
            false,
        ),
        // Inconsistent map elements
        (
            vec![
                Value::map([("a", Value::string("Hello"))]),
                Value::map([("a", Value::bool(true))]),
            ],
            false,
        ),
    ];

    for (i, (elems, want)) in test_cases.iter().enumerate() {
        let got = Value::can_set(elems);
        assert_eq!(
            got, *want,
            "case {i}: wrong result for elements {elems:?}:\ngot {got}, want {want}"
        );
    }
}

// Ported from TestCanMapVal:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_init_test.go#L332
#[test]
#[ignore = "not yet implemented"]
fn can_map_val() {
    fn entries<const N: usize>(pairs: [(&str, Value); N]) -> Vec<(String, Value)> {
        pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
    }

    let test_cases: Vec<(Vec<(String, Value)>, bool)> = vec![
        // Valid lists
        (
            entries([("a", Value::string("Hello")), ("b", Value::string("World"))]),
            true,
        ),
        (
            entries([
                ("one", Value::number_int(13)),
                ("two", Value::number_int(31)),
            ]),
            true,
        ),
        (
            entries([("one", Value::bool(true)), ("two", Value::bool(false))]),
            true,
        ),
        (
            entries([
                (
                    "lista",
                    Value::list([Value::string("Hello"), Value::string("World")]),
                ),
                (
                    "listb",
                    Value::list([
                        Value::string("beep"),
                        Value::string("boop"),
                        Value::string("bloop"),
                    ]),
                ),
            ]),
            true,
        ),
        (
            entries([
                ("map_a", Value::map([("a", Value::string("Hello"))])),
                ("map_b", Value::map([("c", Value::string("World"))])),
            ]),
            true,
        ),
        (
            entries([
                (
                    "set_a",
                    Value::set([Value::string("Hello"), Value::string("World")]),
                ),
                (
                    "set_b",
                    Value::set([
                        Value::string("beep"),
                        Value::string("boop"),
                        Value::string("bloop"),
                    ]),
                ),
            ]),
            true,
        ),
        // invalid map elements
        (
            entries([
                ("one", Value::string("hello")),
                ("two", Value::number_int(13)),
            ]),
            false,
        ),
        (
            entries([
                (
                    "one",
                    Value::list([Value::string("Hello"), Value::string("World")]),
                ),
                ("two", Value::map([("a", Value::string("bloop"))])),
            ]),
            false,
        ),
        (
            entries([
                (
                    "one",
                    Value::list([Value::string("Hello"), Value::string("World")]),
                ),
                (
                    "two",
                    Value::list([
                        Value::list([Value::string("a"), Value::string("b")]),
                        Value::list([Value::string("c"), Value::string("d")]),
                    ]),
                ),
            ]),
            false,
        ),
        // Inconsistent map elements
        (
            entries([
                ("one", Value::map([("a", Value::string("Hello"))])),
                ("two", Value::map([("a", Value::bool(true))])),
            ]),
            false,
        ),
    ];

    for (i, (elems, want)) in test_cases.iter().enumerate() {
        let got = Value::can_map(elems);
        assert_eq!(
            got, *want,
            "case {i}: wrong result for elements {elems:?}:\ngot {got}, want {want}"
        );
    }
}
