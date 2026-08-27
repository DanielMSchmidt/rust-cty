//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/function/stdlib/collection_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::function::stdlib;
use cty::{Type, Value, ValueMarks};

// upstream: cty/function/stdlib/collection_test.go TestHasIndex
#[test]
fn has_index() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::list_empty(Type::number()),
            Value::number_int(2),
            Value::bool(false),
        ),
        (
            Value::list([Value::bool(true)]),
            Value::number_int(0),
            Value::bool(true),
        ),
        (
            Value::list([Value::bool(true)]),
            Value::string("hello"),
            Value::bool(false),
        ),
        (
            Value::map_empty(Type::bool()),
            Value::string("hello"),
            Value::bool(false),
        ),
        (
            Value::map([("hello", Value::bool(true))]),
            Value::string("hello"),
            Value::bool(true),
        ),
        (
            Value::empty_tuple(),
            Value::string("hello"),
            Value::bool(false),
        ),
        (
            Value::empty_tuple(),
            Value::number_int(0),
            Value::bool(false),
        ),
        (
            Value::tuple([Value::bool(true)]),
            Value::number_int(0),
            Value::bool(true),
        ),
        (
            Value::list_empty(Type::number()),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::unknown(Type::list(Type::bool())),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::list_empty(Type::number()),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
        (
            Value::dynamic(),
            Value::dynamic(),
            Value::unknown(Type::bool()).refine_not_null(),
        ),
    ];

    for (i, (collection, key, want)) in tests.iter().enumerate() {
        let got = stdlib::has_index(collection, key)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/collection_test.go TestChunklist
#[test]
fn chunklist() {
    // NOTE(port): upstream's `Want` field is `cty.NilVal` in the error cases;
    // those cases carry `None` here.
    struct Case {
        list: Value,
        len: Value,
        want: Option<Value>,
        err: &'static str,
    }

    let tests: Vec<Case> = vec![
        Case {
            list: Value::list_empty(Type::string()),
            len: Value::number_int(2),
            want: Some(Value::list_empty(Type::list(Type::string()))),
            err: "",
        },
        Case {
            list: Value::unknown(Type::list(Type::string())),
            len: Value::number_int(2),
            want: Some(Value::unknown(Type::list(Type::list(Type::string()))).refine_not_null()),
            err: "",
        },
        Case {
            list: Value::list([Value::string("a")]),
            len: Value::number_int(2),
            want: Some(Value::list([Value::list([Value::string("a")])])),
            err: "",
        },
        Case {
            list: Value::list([Value::string("a").mark("b")]),
            len: Value::number_int(2),
            want: Some(Value::list([Value::list([Value::string("a").mark("b")])])),
            err: "",
        },
        Case {
            list: Value::list([Value::string("a")]).mark("a"),
            len: Value::number_int(2),
            want: Some(Value::list([Value::list([Value::string("a")])]).mark("a")),
            err: "",
        },
        Case {
            list: Value::list([Value::string("a").mark("b")]).mark("a"),
            len: Value::number_int(2),
            want: Some(Value::list([Value::list([Value::string("a").mark("b")])]).mark("a")),
            err: "",
        },
        Case {
            list: Value::list([Value::unknown(Type::string())]),
            len: Value::number_int(2),
            want: Some(Value::list([Value::list([Value::unknown(Type::string())])])),
            err: "",
        },
        Case {
            list: Value::list([Value::string("a"), Value::string("b")]),
            len: Value::number_int(2),
            want: Some(Value::list([Value::list([
                Value::string("a"),
                Value::string("b"),
            ])])),
            err: "",
        },
        // Multiple result elements, one shorter
        Case {
            list: Value::list([Value::string("a"), Value::string("b"), Value::string("c")]),
            len: Value::number_int(2),
            want: Some(Value::list([
                Value::list([Value::string("a"), Value::string("b")]),
                Value::list([Value::string("c")]),
            ])),
            err: "",
        },
        // Multiple result elements, all "full"
        Case {
            list: Value::list([
                Value::string("a"),
                Value::string("b"),
                Value::string("c"),
                Value::string("d"),
                Value::string("e"),
                Value::string("f"),
            ]),
            len: Value::number_int(2),
            want: Some(Value::list([
                Value::list([Value::string("a"), Value::string("b")]),
                Value::list([Value::string("c"), Value::string("d")]),
                Value::list([Value::string("e"), Value::string("f")]),
            ])),
            err: "",
        },
        // We treat length zero as infinite length
        Case {
            list: Value::list([Value::string("a")]),
            len: Value::zero(),
            want: Some(Value::list([Value::list([Value::string("a")])])),
            err: "",
        },
        Case {
            list: Value::list([Value::string("a")]).mark("a"),
            len: Value::zero(),
            want: Some(Value::list([Value::list([Value::string("a")])]).mark("a")),
            err: "",
        },
        Case {
            list: Value::list([Value::string("a")]),
            len: Value::zero().mark("a"),
            want: Some(Value::list([Value::list([Value::string("a")])]).mark("a")),
            err: "",
        },
        Case {
            list: Value::list([Value::string("a").mark("b")]),
            len: Value::zero(),
            want: Some(Value::list([Value::list([Value::string("a").mark("b")])])),
            err: "",
        },
        Case {
            list: Value::list_empty(Type::string()),
            len: Value::number_int(-1),
            want: None,
            err: "the size argument must be positive",
        },
        Case {
            list: Value::list_empty(Type::string()),
            len: Value::positive_infinity(),
            want: None,
            err: "invalid size: value must be a whole number, between -9223372036854775808 and 9223372036854775807",
        },
        Case {
            list: Value::list_empty(Type::string()),
            len: Value::number_float(1.5),
            want: None,
            err: "invalid size: value must be a whole number, between -9223372036854775808 and 9223372036854775807",
        },
    ];

    for (i, case) in tests.iter().enumerate() {
        let got = stdlib::chunklist(&case.list, &case.len);
        if !case.err.is_empty() {
            let err = got.err().unwrap_or_else(|| {
                panic!("case {i}: succeeded; want error");
            });
            assert_eq!(err.to_string(), case.err, "case {i}: wrong error");
            continue;
        }
        let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *case.want.as_ref().unwrap(), "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/collection_test.go TestContains
#[test]
fn contains() {
    let list_of_strings = Value::list([
        Value::string("the"),
        Value::string("quick"),
        Value::string("brown"),
        Value::string("fox"),
    ]);
    let list_of_ints = Value::list([
        Value::number_int(1),
        Value::number_int(2),
        Value::number_int(3),
        Value::number_int(4),
    ]);
    let list_with_unknown = Value::list([
        Value::string("the"),
        Value::string("quick"),
        Value::string("brown"),
        Value::unknown(Type::string()),
    ]);

    let tests: Vec<(Value, Value, Value, bool)> = vec![
        (
            list_of_strings.clone(),
            Value::string("the"),
            Value::bool(true),
            false,
        ),
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
        (
            list_of_strings.clone(),
            Value::string("penguin"),
            Value::bool(false),
            false,
        ),
        (
            list_of_ints.clone(),
            Value::number_int(1),
            Value::bool(true),
            false,
        ),
        (
            list_of_ints.clone(),
            Value::number_int(42),
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
        // tuple val
        (
            Value::tuple([
                Value::string("quick"),
                Value::string("brown"),
                Value::number_int(3),
            ]),
            Value::number_int(3),
            Value::bool(true),
            false,
        ),
    ];

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

// upstream: cty/function/stdlib/collection_test.go TestMerge
#[test]
fn merge() {
    // NOTE(port): upstream's `Want` field is `cty.NilVal` in the error cases;
    // those cases carry `None` here.
    struct Case {
        values: Vec<Value>,
        want: Option<Value>,
        err: bool,
    }

    let tests: Vec<Case> = vec![
        Case {
            values: vec![
                Value::map([("a", Value::string("b"))]),
                Value::map([("c", Value::string("d"))]),
            ],
            want: Some(Value::map([
                ("a", Value::string("b")),
                ("c", Value::string("d")),
            ])),
            err: false,
        },
        // handle unknowns
        Case {
            values: vec![
                Value::map([("a", Value::unknown(Type::string()))]),
                Value::map([("c", Value::string("d"))]),
            ],
            want: Some(Value::map([
                ("a", Value::unknown(Type::string())),
                ("c", Value::string("d")),
            ])),
            err: false,
        },
        // handle null map
        Case {
            values: vec![
                Value::null(Type::map(Type::string())),
                Value::map([("c", Value::string("d"))]),
            ],
            want: Some(Value::map([("c", Value::string("d"))])),
            err: false,
        },
        // all inputs are null
        Case {
            values: vec![
                Value::null(Type::map(Type::string())),
                Value::null(Type::object([("a", Type::list(Type::string()))])),
            ],
            want: Some(Value::empty_object()),
            err: false,
        },
        // single null object with attributes
        Case {
            values: vec![Value::null(Type::object([("a", Type::string())]))],
            want: Some(Value::empty_object()),
            err: false,
        },
        // multible null objects with the same attributes
        Case {
            values: vec![
                Value::null(Type::object([("a", Type::string())])),
                Value::null(Type::object([("a", Type::string())])),
            ],
            want: Some(Value::empty_object()),
            err: false,
        },
        // multible null objects with the differing attributes
        Case {
            values: vec![
                Value::null(Type::object([("a", Type::string())])),
                Value::null(Type::object([("b", Type::string())])),
            ],
            want: Some(Value::empty_object()),
            err: false,
        },
        // mixture of null and non-null objects of the same type
        Case {
            values: vec![
                Value::null(Type::object([("a", Type::string())])),
                Value::object([("a", Value::string("a value"))]),
            ],
            want: Some(Value::object([("a", Value::string("a value"))])),
            err: false,
        },
        // single empty map
        Case {
            values: vec![Value::map_empty(Type::string())],
            want: Some(Value::map_empty(Type::string())),
            err: false,
        },
        // handle null object
        Case {
            values: vec![
                Value::map([("c", Value::string("d"))]),
                Value::null(Type::object([("a", Type::list(Type::string()))])),
            ],
            want: Some(Value::object([("c", Value::string("d"))])),
            err: false,
        },
        // handle unknowns
        Case {
            values: vec![
                Value::unknown(Type::map(Type::string())),
                Value::map([("c", Value::string("d"))]),
            ],
            want: Some(Value::unknown(Type::map(Type::string())).refine_not_null()),
            err: false,
        },
        // handle dynamic unknown
        Case {
            values: vec![
                Value::unknown(Type::dynamic()),
                Value::map([("c", Value::string("d"))]),
            ],
            want: Some(Value::dynamic()),
            err: false,
        },
        // merge with conflicts is ok, last in wins
        Case {
            values: vec![
                Value::map([("a", Value::string("b")), ("c", Value::string("d"))]),
                Value::map([("a", Value::string("x"))]),
            ],
            want: Some(Value::map([
                ("a", Value::string("x")),
                ("c", Value::string("d")),
            ])),
            err: false,
        },
        // only accept maps
        Case {
            values: vec![
                Value::map([("a", Value::string("b")), ("c", Value::string("d"))]),
                Value::list([Value::string("a"), Value::string("x")]),
            ],
            want: None,
            err: true,
        },
        // argument error, for a null type
        Case {
            values: vec![
                Value::map([("a", Value::string("b"))]),
                Value::null(Type::string()),
            ],
            want: None,
            err: true,
        },
        // merge maps of maps
        Case {
            values: vec![
                Value::map([("a", Value::map([("b", Value::string("c"))]))]),
                Value::map([("d", Value::map([("e", Value::string("f"))]))]),
            ],
            want: Some(Value::map([
                ("a", Value::map([("b", Value::string("c"))])),
                ("d", Value::map([("e", Value::string("f"))])),
            ])),
            err: false,
        },
        // map of lists
        Case {
            values: vec![
                Value::map([("a", Value::list([Value::string("b"), Value::string("c")]))]),
                Value::map([("d", Value::list([Value::string("e"), Value::string("f")]))]),
            ],
            want: Some(Value::map([
                ("a", Value::list([Value::string("b"), Value::string("c")])),
                ("d", Value::list([Value::string("e"), Value::string("f")])),
            ])),
            err: false,
        },
        // merge map of various kinds
        Case {
            values: vec![
                Value::map([("a", Value::list([Value::string("b"), Value::string("c")]))]),
                Value::map([("d", Value::map([("e", Value::string("f"))]))]),
            ],
            want: Some(Value::object([
                ("a", Value::list([Value::string("b"), Value::string("c")])),
                ("d", Value::map([("e", Value::string("f"))])),
            ])),
            err: false,
        },
        // merge objects of various shapes
        Case {
            values: vec![
                Value::object([("a", Value::list([Value::string("b")]))]),
                Value::object([("d", Value::dynamic())]),
            ],
            want: Some(Value::object([
                ("a", Value::list([Value::string("b")])),
                ("d", Value::dynamic()),
            ])),
            err: false,
        },
        // merge maps and objects
        Case {
            values: vec![
                Value::map([("a", Value::list([Value::string("b")]))]),
                Value::object([("d", Value::number_int(2))]),
            ],
            want: Some(Value::object([
                ("a", Value::list([Value::string("b")])),
                ("d", Value::number_int(2)),
            ])),
            err: false,
        },
        // attr a type and value is overridden
        Case {
            values: vec![
                Value::object([
                    ("a", Value::list([Value::string("b")])),
                    ("b", Value::string("b")),
                ]),
                Value::object([("a", Value::object([("e", Value::string("f"))]))]),
            ],
            want: Some(Value::object([
                ("a", Value::object([("e", Value::string("f"))])),
                ("b", Value::string("b")),
            ])),
            err: false,
        },
        // argument error: non map type
        Case {
            values: vec![
                Value::map([("a", Value::list([Value::string("b"), Value::string("c")]))]),
                Value::list([Value::string("d"), Value::string("e")]),
            ],
            want: None,
            err: true,
        },
        // Empty maps are allowed in merge
        Case {
            values: vec![
                Value::map_empty(Type::string()),
                Value::map_empty(Type::string()),
            ],
            want: Some(Value::map_empty(Type::string())),
            err: false,
        },
        // Preserve marks from chosen elements
        Case {
            values: vec![
                Value::map([
                    ("a", Value::string("a").mark("first")),
                    ("c", Value::string("c")),
                    ("d", Value::string("d").mark("first")),
                ]),
                Value::map([
                    ("a", Value::string("a")),
                    ("b", Value::string("b").mark("second")),
                    ("c", Value::string("c").mark("second")),
                ]),
            ],
            want: Some(Value::map([
                ("a", Value::string("a")),
                ("b", Value::string("b").mark("second")),
                ("c", Value::string("c").mark("second")),
                ("d", Value::string("d").mark("first")),
            ])),
            err: false,
        },
        // Marks on the collections must be merged, even if empty
        Case {
            values: vec![
                Value::map([("a", Value::string("a"))]).mark("first"),
                Value::map([("a", Value::string("a")), ("b", Value::string("b"))]).mark("second"),
                Value::map_empty(Type::string()).mark("third"),
            ],
            want: Some(
                Value::map([("a", Value::string("a")), ("b", Value::string("b"))])
                    .with_marks([ValueMarks::from_marks(["first", "second", "third"])]),
            ),
            err: false,
        },
        // Similar test but where all args are the same object type
        Case {
            values: vec![
                Value::object([
                    ("a", Value::string("a")),
                    ("b", Value::null(Type::string())),
                ])
                .mark("first"),
                Value::object([("a", Value::string("A")), ("b", Value::string("B"))])
                    .mark("second"),
            ],
            want: Some(
                Value::object([("a", Value::string("A")), ("b", Value::string("B"))])
                    .with_marks([ValueMarks::from_marks(["first", "second"])]),
            ),
            err: false,
        },
    ];

    for (i, case) in tests.iter().enumerate() {
        let got = stdlib::merge(&case.values);
        if case.err {
            assert!(got.is_err(), "case {i}: succeeded; want error");
            continue;
        }
        let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *case.want.as_ref().unwrap(), "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/collection_test.go TestIndex
#[test]
fn index() {
    let tests: Vec<(Value, Value, Value)> = vec![
        (
            Value::list([Value::bool(true)]),
            Value::number_int(0),
            Value::bool(true),
        ),
        (
            Value::map([("hello", Value::bool(true))]),
            Value::string("hello"),
            Value::bool(true),
        ),
        (
            Value::tuple([Value::bool(true), Value::string("hello")]),
            Value::number_int(0),
            Value::bool(true),
        ),
        (
            Value::tuple([Value::bool(true), Value::string("hello")]),
            Value::number_int(1),
            Value::string("hello"),
        ),
        (
            Value::list_empty(Type::number()),
            Value::unknown(Type::number()),
            Value::unknown(Type::number()),
        ),
        (
            Value::unknown(Type::list(Type::bool())),
            Value::unknown(Type::number()),
            Value::unknown(Type::bool()),
        ),
        (
            Value::list_empty(Type::number()),
            Value::dynamic(),
            Value::unknown(Type::number()),
        ),
        (
            Value::map_empty(Type::number()),
            Value::dynamic(),
            Value::unknown(Type::number()),
        ),
        (Value::dynamic(), Value::string("hello"), Value::dynamic()),
        (Value::dynamic(), Value::dynamic(), Value::dynamic()),
    ];

    for (i, (collection, key, want)) in tests.iter().enumerate() {
        let got = stdlib::index(collection, key)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/collection_test.go TestLength
#[test]
fn length() {
    let tests: Vec<(Value, Value)> = vec![
        (Value::list_empty(Type::number()), Value::number_int(0)),
        (Value::list([Value::bool(true)]), Value::number_int(1)),
        (Value::set_empty(Type::number()), Value::number_int(0)),
        (Value::set([Value::bool(true)]), Value::number_int(1)),
        (
            Value::set([Value::bool(true), Value::bool(false)]),
            Value::number_int(2),
        ),
        (
            Value::set([Value::bool(true), Value::unknown(Type::bool())]),
            // Don't know if the unknown in the input represents cty.True or cty.False,
            // so it may or may not coalesce with the one known value.
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_inclusive(Value::number_int(1), Value::number_int(2))
                .new_value(),
        ),
        (
            Value::set([Value::unknown(Type::bool())]),
            Value::number_int(1), // Will be one regardless of what value the unknown in the input is representing
        ),
        (Value::map_empty(Type::bool()), Value::number_int(0)),
        (
            Value::map([("hello", Value::bool(true))]),
            Value::number_int(1),
        ),
        (Value::empty_tuple(), Value::number_int(0)),
        (Value::tuple([Value::bool(true)]), Value::number_int(1)),
        (
            Value::unknown(Type::list(Type::bool())),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_inclusive(Value::zero(), Value::number_int(i64::MAX))
                .new_value(),
        ),
        (
            Value::dynamic(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_inclusive(Value::zero(), Value::number_int(i64::MAX))
                .new_value(),
        ),
        (
            Value::unknown(Type::list(Type::bool()))
                .refine()
                .collection_length_upper_bound(2)
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_inclusive(Value::zero(), Value::number_int(2))
                .new_value(),
        ),
        // Marked collections return a marked length
        (
            Value::list([Value::string("hello"), Value::string("world")]).mark("secret"),
            Value::number_int(2).mark("secret"),
        ),
        // Marks on values in unmarked collections do not propagate
        (
            Value::list([
                Value::string("hello").mark("a"),
                Value::string("world").mark("b"),
            ]),
            Value::number_int(2),
        ),
    ];

    for (i, (collection, want)) in tests.iter().enumerate() {
        let got = stdlib::length(collection)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/collection_test.go TestLookup
#[test]
fn lookup() {
    let tests: Vec<(Value, Value, Value, Value)> = vec![
        (
            Value::map_empty(Type::string()),
            Value::string("baz"),
            Value::string("foo"),
            Value::string("foo"),
        ),
        (
            Value::map([("foo", Value::string("bar"))]),
            Value::string("foo"),
            Value::string("nope"),
            Value::string("bar"),
        ),
        // successful marked collection lookup returns marked value
        (
            Value::map([("boop", Value::string("beep"))]).mark("a"),
            Value::string("boop"),
            Value::string("nope"),
            Value::string("beep").mark("a"),
        ),
        // apply collection marks to unknown return vaue
        (
            Value::map([
                ("boop", Value::string("beep")),
                ("frob", Value::unknown(Type::string())),
            ])
            .mark("a"),
            Value::string("boop"),
            Value::string("nope"),
            Value::unknown(Type::string()).mark("a"),
        ),
        // propagate collection marks to default when returning
        (
            Value::map([("boop", Value::string("beep"))]).mark("a"),
            Value::string("frob"),
            Value::string("nope").mark("b"),
            Value::string("nope").with_marks([ValueMarks::from_marks(["a", "b"])]),
        ),
        // on unmarked collection, return only marks from found value
        (
            Value::map([
                ("boop", Value::string("beep").mark("a")),
                ("frob", Value::string("honk").mark("b")),
            ]),
            Value::string("frob"),
            Value::string("nope").mark("c"),
            Value::string("honk").mark("b"),
        ),
        // on unmarked collection, return default exactly on missing
        (
            Value::map([
                ("boop", Value::string("beep").mark("a")),
                ("frob", Value::string("honk").mark("b")),
            ]),
            Value::string("squish"),
            Value::string("nope").mark("c"),
            Value::string("nope").mark("c"),
        ),
        // retain marks on default if converted
        (
            Value::map([
                ("boop", Value::string("beep").mark("a")),
                ("frob", Value::string("honk").mark("b")),
            ]),
            Value::string("squish"),
            Value::number_int(5).mark("c"),
            Value::string("5").mark("c"),
        ),
        // propagate marks from key
        (
            Value::map([
                ("boop", Value::string("beep")),
                ("frob", Value::string("honk")),
            ]),
            Value::string("boop").mark("a"),
            Value::string("nope"),
            Value::string("beep").mark("a"),
        ),
    ];

    for (i, (collection, key, default, want)) in tests.iter().enumerate() {
        let got = stdlib::lookup(collection, key, default)
            .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/collection_test.go TestElement
#[test]
fn element() {
    let list_of_strings = Value::list([
        Value::string("the"),
        Value::string("quick"),
        Value::string("brown"),
        Value::string("fox"),
    ]);
    let list_of_ints = Value::list([
        Value::number_int(1),
        Value::number_int(2),
        Value::number_int(3),
        Value::number_int(4),
    ]);
    let list_with_unknown = Value::list([
        Value::string("the"),
        Value::string("quick"),
        Value::string("brown"),
        Value::unknown(Type::string()),
    ]);
    let list_with_marks = Value::list([
        Value::string("the"),
        Value::string("quick"),
        Value::string("brown").mark("fox"),
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
            Value::number_int(2),
            Value::string("brown"),
            false,
        ),
        // index greater than length(list)
        (
            list_of_strings.clone(),
            Value::number_int(5),
            Value::string("quick"),
            false,
        ),
        // negative index counts from the end of the list
        (
            list_of_strings.clone(),
            Value::number_int(-1),
            Value::string("fox"),
            false,
        ),
        // negative index can be out of bounds too
        (
            list_of_strings.clone(),
            Value::number_int(-6),
            Value::string("brown"),
            false,
        ),
        // minimum valid index
        (
            list_of_strings.clone(),
            Value::number_int(i64::MIN),
            Value::string("the"),
            false,
        ),
        // maximum valid index
        (
            list_of_strings.clone(),
            Value::number_int(i64::MAX),
            Value::string("fox"),
            false,
        ),
        // list of lists
        (
            Value::list([list_of_strings.clone(), list_of_strings.clone()]),
            Value::number_int(0),
            list_of_strings.clone(),
            false,
        ),
        (
            list_of_strings.clone(),
            Value::unknown(Type::number()),
            Value::unknown(Type::string()),
            false,
        ),
        (
            list_of_ints.clone(),
            Value::number_int(2),
            Value::number_int(3),
            false,
        ),
        (
            list_with_unknown.clone(),
            Value::number_int(2),
            Value::string("brown"),
            false,
        ),
        (
            list_with_unknown.clone(),
            Value::number_int(3),
            Value::unknown(Type::string()),
            false,
        ),
        // preserve marks
        (
            list_with_marks.clone(),
            Value::number_int(2),
            Value::string("brown").mark("fox"),
            false,
        ),
        // marked items
        (
            list_with_marks.clone(),
            Value::number_int(1),
            Value::string("quick"),
            false,
        ),
        // The entire list is marked
        (
            list_with_marks.clone().mark("thewholeshebang"),
            Value::number_int(2),
            Value::string("brown").with_marks([ValueMarks::from_marks(["thewholeshebang", "fox"])]),
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
            Value::number_float(0.5),
            Value::dynamic(),
            true,
        ),
        // index out of bounds of int64
        (
            list_of_strings.clone(),
            Value::parse_number("-9223372036854775809").unwrap(),
            Value::string("the"),
            true,
        ),
        // index out of bounds of int64
        (
            list_of_strings.clone(),
            Value::parse_number("9223372036854775808").unwrap(),
            Value::string("fox"),
            true,
        ),
        (
            tuple.clone(),
            Value::number_int(0),
            Value::string("the"),
            false,
        ),
        (
            tuple.clone(),
            Value::number_int(1),
            Value::unknown(Type::string()),
            false,
        ),
        (
            tuple.clone(),
            Value::number_int(3),
            Value::bool(false),
            false,
        ),
        (
            tuple.clone(),
            Value::number_int(4),
            Value::string("the"),
            false,
        ),
        (
            tuple.clone(),
            Value::number_int(10),
            Value::string("brown"),
            false,
        ),
        (
            tuple.clone(),
            Value::number_int(-1),
            Value::bool(false),
            false,
        ),
        (
            tuple.clone(),
            Value::number_int(-6),
            Value::string("brown"),
            false,
        ),
        (
            unknown_tuple.clone(),
            Value::number_int(0),
            Value::unknown(Type::string()),
            false,
        ),
        (
            unknown_tuple.clone(),
            Value::number_int(1),
            Value::unknown(Type::string()),
            false,
        ),
        (
            unknown_tuple.clone(),
            Value::number_int(3),
            Value::unknown(Type::bool()),
            false,
        ),
        (
            unknown_tuple.clone(),
            Value::number_int(4),
            Value::unknown(Type::string()),
            false,
        ),
        (
            unknown_tuple.clone(),
            Value::number_int(10),
            Value::unknown(Type::string()),
            false,
        ),
        (
            unknown_tuple.clone(),
            Value::number_int(-1),
            Value::unknown(Type::bool()),
            false,
        ),
        (
            unknown_tuple.clone(),
            Value::number_int(-6),
            Value::unknown(Type::string()),
            false,
        ),
    ];

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

// upstream: cty/function/stdlib/collection_test.go TestCoalesceList
#[test]
fn coalesce_list() {
    // NOTE(port): upstream's `Want` field is `cty.NilVal` in the error cases;
    // those cases carry `None` here.
    struct Case {
        name: &'static str,
        values: Vec<Value>,
        want: Option<Value>,
        err: bool,
    }

    let tests: Vec<Case> = vec![
        Case {
            name: "returns first list if non-empty",
            values: vec![
                Value::list([Value::string("a"), Value::string("b")]),
                Value::list([Value::string("c"), Value::string("d")]),
            ],
            want: Some(Value::list([Value::string("a"), Value::string("b")])),
            err: false,
        },
        Case {
            name: "returns second list if first is empty",
            values: vec![
                Value::list_empty(Type::string()),
                Value::list([Value::string("c"), Value::string("d")]),
            ],
            want: Some(Value::list([Value::string("c"), Value::string("d")])),
            err: false,
        },
        Case {
            name: "return type is dynamic, not unified",
            values: vec![
                Value::list_empty(Type::string()),
                Value::list([Value::number_int(3), Value::number_int(4)]),
            ],
            want: Some(Value::list([Value::number_int(3), Value::number_int(4)])),
            err: false,
        },
        Case {
            name: "works with tuples",
            values: vec![
                Value::empty_tuple(),
                Value::tuple([Value::string("c"), Value::string("d")]),
            ],
            want: Some(Value::tuple([Value::string("c"), Value::string("d")])),
            err: false,
        },
        Case {
            name: "unknown arguments",
            values: vec![
                Value::unknown(Type::list(Type::string())),
                Value::list([Value::string("c"), Value::string("d")]),
            ],
            want: Some(Value::dynamic()),
            err: false,
        },
        Case {
            name: "null arguments",
            values: vec![
                Value::null(Type::list(Type::string())),
                Value::list([Value::string("c"), Value::string("d")]),
            ],
            want: Some(Value::list([Value::string("c"), Value::string("d")])),
            err: false,
        },
        Case {
            name: "all null arguments",
            values: vec![
                Value::null(Type::list(Type::string())),
                Value::null(Type::list(Type::string())),
            ],
            want: None,
            err: true,
        },
        Case {
            name: "invalid arguments",
            values: vec![
                Value::map([("a", Value::bool(true))]),
                Value::object([("b", Value::bool(false))]),
            ],
            want: None,
            err: true,
        },
        Case {
            name: "no arguments",
            values: vec![],
            want: None,
            err: true,
        },
    ];

    for case in tests.iter() {
        let name = case.name;
        let got = stdlib::coalesce_list(&case.values);
        if case.err {
            assert!(got.is_err(), "{name}: succeeded; want error");
            continue;
        }
        let got = got.unwrap_or_else(|err| panic!("{name}: unexpected error: {err}"));
        assert_eq!(got, *case.want.as_ref().unwrap(), "{name}: wrong result");
    }
}

// upstream: cty/function/stdlib/collection_test.go TestValues
#[test]
fn values() {
    // NOTE(port): upstream's `Want` field is `cty.NilVal` in the error cases;
    // those cases carry `None` here.
    struct Case {
        collection: Value,
        want: Option<Value>,
        err: &'static str,
    }

    let tests: Vec<Case> = vec![
        Case {
            collection: Value::map_empty(Type::string()),
            want: Some(Value::list_empty(Type::string())),
            err: "",
        },
        Case {
            collection: Value::map_empty(Type::string()).mark("a"),
            want: Some(Value::list_empty(Type::string()).mark("a")),
            err: "",
        },
        Case {
            collection: Value::null(Type::map(Type::string())),
            want: None,
            err: "argument must not be null",
        },
        Case {
            collection: Value::unknown(Type::map(Type::string())),
            want: Some(Value::unknown(Type::list(Type::string())).refine_not_null()),
            err: "",
        },
        Case {
            collection: Value::map([("hello", Value::string("world"))]),
            want: Some(Value::list([Value::string("world")])),
            err: "",
        },
        // The map itself is not marked, just an inner element.
        Case {
            collection: Value::map([("hello", Value::string("world").mark("a"))]),
            want: Some(Value::list([Value::string("world").mark("a")])),
            err: "",
        },
        // The entire map is marked, so the resulting list is also marked.
        Case {
            collection: Value::map([("hello", Value::string("world"))]).mark("a"),
            want: Some(Value::list([Value::string("world")]).mark("a")),
            err: "",
        },
        // Marked both inside and outside.
        Case {
            collection: Value::map([("hello", Value::string("world").mark("a"))]).mark("a"),
            want: Some(Value::list([Value::string("world").mark("a")]).mark("a")),
            err: "",
        },
        Case {
            collection: Value::object([("hello", Value::string("world"))]),
            want: Some(Value::tuple([Value::string("world")])),
            err: "",
        },
        Case {
            collection: Value::empty_object(),
            want: Some(Value::empty_tuple()),
            err: "",
        },
        Case {
            collection: Value::empty_object().mark("a"),
            want: Some(Value::empty_tuple().mark("a")),
            err: "",
        },
        Case {
            collection: Value::null(Type::empty_object()),
            want: None,
            err: "argument must not be null",
        },
        Case {
            collection: Value::unknown(Type::empty_object()),
            want: Some(Value::unknown(Type::empty_tuple()).refine_not_null()),
            err: "",
        },
        Case {
            collection: Value::unknown(Type::object([("a", Type::string())])),
            want: Some(Value::unknown(Type::tuple([Type::string()])).refine_not_null()),
            err: "",
        },
        // The object itself is not marked, just an inner attribute value.
        Case {
            collection: Value::object([("hello", Value::string("world").mark("a"))]),
            want: Some(Value::tuple([Value::string("world").mark("a")])),
            err: "",
        },
        // The entire object is marked, so the resulting tuple is also marked.
        Case {
            collection: Value::object([("hello", Value::string("world"))]).mark("a"),
            want: Some(Value::tuple([Value::string("world")]).mark("a")),
            err: "",
        },
        // Marked both inside and outside.
        Case {
            collection: Value::object([("hello", Value::string("world").mark("a"))]).mark("a"),
            want: Some(Value::tuple([Value::string("world").mark("a")]).mark("a")),
            err: "",
        },
    ];

    for (i, case) in tests.iter().enumerate() {
        let got = stdlib::values(&case.collection);
        if !case.err.is_empty() {
            let err = got.err().unwrap_or_else(|| {
                panic!("case {i}: succeeded; want error");
            });
            assert_eq!(err.to_string(), case.err, "case {i}: wrong error");
            continue;
        }
        let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *case.want.as_ref().unwrap(), "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/collection_test.go TestZipMap
#[test]
fn zipmap() {
    // NOTE(port): upstream's `Want` field is `cty.NilVal` in the error cases;
    // those cases carry `None` here.
    struct Case {
        keys: Value,
        values: Value,
        want: Option<Value>,
        err: &'static str,
    }

    let tests: Vec<Case> = vec![
        // Lists of values (map result)
        Case {
            keys: Value::list_empty(Type::string()),
            values: Value::list_empty(Type::string()),
            want: Some(Value::map_empty(Type::string())),
            err: "",
        },
        Case {
            keys: Value::list([Value::string("bleep")]),
            values: Value::list([Value::string("bloop")]),
            want: Some(Value::map([("bleep", Value::string("bloop"))])),
            err: "",
        },
        Case {
            keys: Value::list([Value::string("bleep"), Value::string("beep")]),
            values: Value::list([Value::string("bloop"), Value::string("boop")]),
            want: Some(Value::map([
                ("beep", Value::string("boop")),
                ("bleep", Value::string("bloop")),
            ])),
            err: "",
        },
        Case {
            keys: Value::unknown(Type::list(Type::string())),
            values: Value::unknown(Type::list(Type::string())),
            want: Some(Value::unknown(Type::map(Type::string())).refine_not_null()),
            err: "",
        },
        Case {
            keys: Value::unknown(Type::list(Type::string())),
            values: Value::list_empty(Type::string()),
            want: Some(Value::unknown(Type::map(Type::string())).refine_not_null()),
            err: "",
        },
        Case {
            keys: Value::list_empty(Type::string()),
            values: Value::unknown(Type::list(Type::string())),
            want: Some(Value::unknown(Type::map(Type::string())).refine_not_null()),
            err: "",
        },
        Case {
            keys: Value::list([Value::string("bleep")]).mark("a"),
            values: Value::list([Value::string("bloop")]),
            want: Some(Value::map([("bleep", Value::string("bloop"))]).mark("a")),
            err: "",
        },
        Case {
            keys: Value::list([Value::string("bleep")]),
            values: Value::list([Value::string("bloop")]).mark("b"),
            want: Some(Value::map([("bleep", Value::string("bloop"))]).mark("b")),
            err: "",
        },
        Case {
            keys: Value::list([Value::string("bleep")]).mark("a"),
            values: Value::list([Value::string("bloop")]).mark("b"),
            want: Some(
                Value::map([("bleep", Value::string("bloop"))])
                    .mark("a")
                    .mark("b"),
            ),
            err: "",
        },
        // cty map keys don't have individual marks, so marks on elements
        // in the keys list aggregate with the resulting map as a whole.
        Case {
            keys: Value::list([Value::string("bleep").mark("a")]),
            values: Value::list([Value::string("bloop")]),
            want: Some(Value::map([("bleep", Value::string("bloop"))]).mark("a")),
            err: "",
        },
        // cty map _values_ can have individual marks, so individual
        // elements in the values list should have their marks preserved.
        Case {
            keys: Value::list([Value::string("bleep")]),
            values: Value::list([Value::string("bloop").mark("a")]),
            want: Some(Value::map([("bleep", Value::string("bloop").mark("a"))])),
            err: "",
        },
        Case {
            keys: Value::list([Value::string("boop")]),
            values: Value::list_empty(Type::string()),
            want: None,
            err: "number of keys (1) does not match number of values (0)",
        },
        Case {
            keys: Value::list_empty(Type::string()),
            values: Value::list([Value::string("boop")]),
            want: None,
            err: "number of keys (0) does not match number of values (1)",
        },
        // Tuple of values (object result)
        Case {
            keys: Value::list_empty(Type::string()),
            values: Value::empty_tuple(),
            want: Some(Value::empty_object()),
            err: "",
        },
        Case {
            keys: Value::list([Value::string("bleep")]),
            values: Value::tuple([Value::string("bloop")]),
            want: Some(Value::object([("bleep", Value::string("bloop"))])),
            err: "",
        },
        Case {
            keys: Value::list([Value::string("bleep"), Value::string("beep")]),
            values: Value::tuple([Value::string("bloop"), Value::string("boop")]),
            want: Some(Value::object([
                ("beep", Value::string("boop")),
                ("bleep", Value::string("bloop")),
            ])),
            err: "",
        },
        Case {
            keys: Value::unknown(Type::list(Type::string())),
            values: Value::unknown(Type::empty_tuple()),
            want: Some(Value::dynamic()),
            err: "",
        },
        Case {
            keys: Value::unknown(Type::list(Type::string())),
            values: Value::empty_tuple(),
            want: Some(Value::dynamic()),
            err: "",
        },
        Case {
            keys: Value::list_empty(Type::string()),
            values: Value::unknown(Type::empty_tuple()),
            want: Some(Value::unknown(Type::empty_object()).refine_not_null()),
            err: "",
        },
        Case {
            keys: Value::list([Value::string("bleep")]).mark("a"),
            values: Value::tuple([Value::string("bloop")]),
            want: Some(Value::object([("bleep", Value::string("bloop"))]).mark("a")),
            err: "",
        },
        Case {
            keys: Value::list([Value::string("bleep")]),
            values: Value::tuple([Value::string("bloop")]).mark("b"),
            want: Some(Value::object([("bleep", Value::string("bloop"))]).mark("b")),
            err: "",
        },
        Case {
            keys: Value::list([Value::string("bleep")]).mark("a"),
            values: Value::tuple([Value::string("bloop")]).mark("b"),
            want: Some(
                Value::object([("bleep", Value::string("bloop"))])
                    .mark("a")
                    .mark("b"),
            ),
            err: "",
        },
        // cty object attributes don't have individual marks, so marks on
        // elements in the keys list aggregate with the resulting object as
        // a whole.
        Case {
            keys: Value::list([Value::string("bleep").mark("a")]),
            values: Value::tuple([Value::string("bloop")]),
            want: Some(Value::object([("bleep", Value::string("bloop"))]).mark("a")),
            err: "",
        },
        // cty attribute _values_ can have individual marks, so individual
        // elements in the values list should have their marks preserved.
        Case {
            keys: Value::list([Value::string("bleep")]),
            values: Value::tuple([Value::string("bloop").mark("a")]),
            want: Some(Value::object([("bleep", Value::string("bloop").mark("a"))])),
            err: "",
        },
        Case {
            keys: Value::list([Value::string("boop")]),
            values: Value::empty_tuple(),
            want: None,
            err: "number of keys (1) does not match number of values (0)",
        },
        Case {
            keys: Value::list_empty(Type::string()),
            values: Value::tuple([Value::string("boop")]),
            want: None,
            err: "number of keys (0) does not match number of values (1)",
        },
    ];

    for (i, case) in tests.iter().enumerate() {
        let got = stdlib::zipmap(&case.keys, &case.values);
        if !case.err.is_empty() {
            let err = got.err().unwrap_or_else(|| {
                panic!("case {i}: succeeded; want error");
            });
            assert_eq!(err.to_string(), case.err, "case {i}: wrong error");
            continue;
        }
        let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *case.want.as_ref().unwrap(), "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/collection_test.go TestKeys
#[test]
fn keys() {
    // NOTE(port): upstream's `Want` field is `cty.NilVal` in the error cases;
    // those cases carry `None` here.
    struct Case {
        collection: Value,
        want: Option<Value>,
        err: &'static str,
    }

    let tests: Vec<Case> = vec![
        Case {
            collection: Value::map_empty(Type::string()),
            want: Some(Value::list_empty(Type::string())),
            err: "",
        },
        Case {
            collection: Value::map_empty(Type::string()).mark("a"),
            want: Some(Value::list_empty(Type::string()).mark("a")),
            err: "",
        },
        Case {
            collection: Value::null(Type::map(Type::string())),
            want: None,
            err: "argument must not be null",
        },
        Case {
            collection: Value::map([("hello", Value::string("world"))]),
            want: Some(Value::list([Value::string("hello")])),
            err: "",
        },
        // The map itself is not marked, just an inner element.
        Case {
            collection: Value::map([("hello", Value::string("world").mark("a"))]),
            want: Some(Value::list([Value::string("hello")])),
            err: "",
        },
        // The entire map is marked, so the resulting list is also marked.
        Case {
            collection: Value::map([("hello", Value::string("world"))]).mark("a"),
            want: Some(Value::list([Value::string("hello")]).mark("a")),
            err: "",
        },
        // Marked both inside and outside.
        Case {
            collection: Value::map([("hello", Value::string("world").mark("a"))]).mark("a"),
            want: Some(Value::list([Value::string("hello")]).mark("a")),
            err: "",
        },
        Case {
            collection: Value::object([("hello", Value::string("world"))]),
            want: Some(Value::tuple([Value::string("hello")])),
            err: "",
        },
        Case {
            collection: Value::empty_object(),
            want: Some(Value::empty_tuple()),
            err: "",
        },
        Case {
            collection: Value::empty_object().mark("a"),
            want: Some(Value::empty_tuple().mark("a")),
            err: "",
        },
        Case {
            collection: Value::null(Type::empty_object()),
            want: None,
            err: "argument must not be null",
        },
        Case {
            collection: Value::unknown(Type::empty_object()),
            want: Some(Value::empty_tuple()),
            err: "",
        },
        Case {
            collection: Value::unknown(Type::object([("a", Type::string())])),
            want: Some(Value::tuple([Value::string("a")])),
            err: "",
        },
        // The object itself is not marked, just an inner attribute value.
        Case {
            collection: Value::object([("hello", Value::string("world").mark("a"))]),
            want: Some(Value::tuple([Value::string("hello")])),
            err: "",
        },
        // The entire object is marked, so the resulting tuple is also marked.
        Case {
            collection: Value::object([("hello", Value::string("world"))]).mark("a"),
            want: Some(Value::tuple([Value::string("hello")]).mark("a")),
            err: "",
        },
        // Marked both inside and outside.
        Case {
            collection: Value::object([("hello", Value::string("world").mark("a"))]).mark("a"),
            want: Some(Value::tuple([Value::string("hello")]).mark("a")),
            err: "",
        },
    ];

    for (i, case) in tests.iter().enumerate() {
        let got = stdlib::keys(&case.collection);
        if !case.err.is_empty() {
            let err = got.err().unwrap_or_else(|| {
                panic!("case {i}: succeeded; want error");
            });
            assert_eq!(err.to_string(), case.err, "case {i}: wrong error");
            continue;
        }
        let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *case.want.as_ref().unwrap(), "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/collection_test.go TestFlatten
#[test]
fn flatten() {
    let tests: Vec<(Value, Value, &'static str)> = vec![
        // Empty case is easy
        (Value::list_empty(Type::string()), Value::empty_tuple(), ""),
        // Lists can contain unknown values
        (
            Value::list([
                Value::list([Value::unknown(Type::string()), Value::string("a")]),
                Value::list([
                    Value::unknown(Type::string()),
                    Value::string("b"),
                    Value::unknown(Type::string()),
                ]),
            ]),
            Value::tuple([
                Value::unknown(Type::string()),
                Value::string("a"),
                Value::unknown(Type::string()),
                Value::string("b"),
                Value::unknown(Type::string()),
            ]),
            "",
        ),
        // If the list itself is unknown this is the best we can do
        (
            Value::unknown(Type::list(Type::list(Type::string()))),
            Value::unknown(Type::dynamic()),
            "",
        ),
        // Type error
        (
            Value::map_empty(Type::string()),
            Value::dynamic(),
            "can only flatten lists, sets and tuples",
        ),
        // Top-level list marks should carry over
        (
            Value::list([
                Value::list([Value::string("a")]),
                Value::list([Value::string("b"), Value::string("c")]),
                Value::list_empty(Type::string()),
            ])
            .mark("mark"),
            Value::tuple([Value::string("a"), Value::string("b"), Value::string("c")]).mark("mark"),
            "",
        ),
        // Inner list marks should apply to the result collection
        (
            Value::list([
                Value::list([Value::string("a")]).mark("first"),
                Value::list([Value::string("b"), Value::string("c")]).mark("second"),
                Value::list_empty(Type::string()).mark("third"),
            ]),
            Value::tuple([Value::string("a"), Value::string("b"), Value::string("c")])
                .with_marks([ValueMarks::from_marks(["first", "second", "third"])]),
            "",
        ),
        // Non-list element marks should be retained on the element only
        (
            Value::list([
                Value::list([Value::string("a").mark("a")]),
                Value::list([Value::string("b").mark("b"), Value::string("c").mark("b")]),
            ]),
            Value::tuple([
                Value::string("a").mark("a"),
                Value::string("b").mark("b"),
                Value::string("c").mark("b"),
            ]),
            "",
        ),
        // Nested unknown lists/sets/tuples should still propagate marks
        (
            Value::list([
                Value::list([Value::string("a")]).mark("first"),
                Value::unknown(Type::list(Type::string())).mark("second"),
                Value::list([Value::string("c")]).mark("third"),
            ]),
            Value::unknown(Type::dynamic())
                .with_marks([ValueMarks::from_marks(["first", "second", "third"])]),
            "",
        ),
        // Empty marked list retains marks
        (
            Value::list_empty(Type::string()).mark("a"),
            Value::empty_tuple().mark("a"),
            "",
        ),
        (Value::list_empty(Type::number()), Value::empty_tuple(), ""),
        (Value::list([Value::dynamic()]), Value::dynamic(), ""),
        (
            Value::tuple([
                Value::list([Value::list([Value::dynamic()])]),
                Value::list([Value::list([Value::dynamic()]).mark("marked")]),
            ]),
            Value::dynamic().mark("marked"),
            "",
        ),
        (
            Value::tuple([
                Value::list([Value::object([("blop", Value::list([Value::dynamic()]))])]),
                Value::list([Value::object([("bloop", Value::dynamic())])]),
            ]),
            Value::tuple([
                Value::object([("blop", Value::list([Value::dynamic()]))]),
                Value::object([("bloop", Value::dynamic())]),
            ]),
            "",
        ),
        (
            Value::list([
                Value::list([Value::object([("bloop", Value::dynamic())])]),
                Value::list([Value::object([("bloop", Value::dynamic())])]),
            ]),
            Value::tuple([
                Value::object([("bloop", Value::dynamic())]),
                Value::object([("bloop", Value::dynamic())]),
            ]),
            "",
        ),
        (
            Value::tuple([
                Value::string("a"),
                Value::list([Value::string("b")]),
                Value::tuple([
                    Value::list([Value::string("c")]),
                    Value::list([Value::string("d"), Value::string("e")]),
                ]),
            ]),
            Value::tuple([
                Value::string("a"),
                Value::string("b"),
                Value::string("c"),
                Value::string("d"),
                Value::string("e"),
            ]),
            "",
        ),
        (
            Value::tuple([
                Value::tuple([Value::string("a"), Value::string("b")]),
                Value::null(Type::dynamic()),
                Value::tuple([Value::string("c")]),
            ]),
            Value::tuple([
                Value::string("a"),
                Value::string("b"),
                Value::null(Type::dynamic()),
                Value::string("c"),
            ]),
            "",
        ),
        (
            Value::tuple([
                Value::tuple([Value::string("a"), Value::string("b")]),
                Value::dynamic(),
                Value::tuple([Value::string("c")]),
            ]),
            Value::unknown(Type::dynamic()),
            "",
        ),
        // null of an unknown type
        (
            Value::tuple([Value::null(Type::dynamic()), Value::bool(true)]),
            Value::tuple([Value::null(Type::dynamic()), Value::bool(true)]),
            "",
        ),
        // null of a string type
        (
            Value::tuple([Value::null(Type::string()), Value::bool(true)]),
            Value::tuple([Value::null(Type::string()), Value::bool(true)]),
            "",
        ),
        // null of a list type
        (
            Value::tuple([Value::null(Type::list(Type::string())), Value::bool(true)]),
            Value::tuple([Value::null(Type::list(Type::string())), Value::bool(true)]),
            "",
        ),
        // null of a tuple type
        (
            Value::tuple([Value::null(Type::empty_tuple()), Value::bool(true)]),
            Value::tuple([Value::null(Type::empty_tuple()), Value::bool(true)]),
            "",
        ),
        // nested null of an unknown type
        (
            Value::tuple([
                Value::tuple([Value::null(Type::dynamic())]),
                Value::bool(true),
            ]),
            Value::tuple([Value::null(Type::dynamic()), Value::bool(true)]),
            "",
        ),
        // nested null of a string type
        (
            Value::tuple([
                Value::tuple([Value::null(Type::string())]),
                Value::bool(true),
            ]),
            Value::tuple([Value::null(Type::string()), Value::bool(true)]),
            "",
        ),
        // nested null of a list type
        (
            Value::tuple([
                Value::tuple([Value::null(Type::list(Type::string()))]),
                Value::bool(true),
            ]),
            Value::tuple([Value::null(Type::list(Type::string())), Value::bool(true)]),
            "",
        ),
        // nested null of a tuple type
        (
            Value::tuple([
                Value::tuple([Value::null(Type::empty_tuple())]),
                Value::bool(true),
            ]),
            Value::tuple([Value::null(Type::empty_tuple()), Value::bool(true)]),
            "",
        ),
    ];

    for (i, (list, want, want_err)) in tests.iter().enumerate() {
        let got = stdlib::flatten(list);
        if !want_err.is_empty() {
            let err = got.err().unwrap_or_else(|| {
                panic!("case {i}: succeeded; want error");
            });
            assert_eq!(err.to_string(), *want_err, "case {i}: wrong error");
            continue;
        }
        let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/collection_test.go TestSetproduct
#[test]
fn setproduct() {
    // NOTE(port): upstream's `Want` field is `cty.NilVal` in the error case;
    // that case carries `None` here.
    struct Case {
        collections: Vec<Value>,
        want: Option<Value>,
        err: &'static str,
    }

    let tests: Vec<Case> = vec![
        Case {
            collections: vec![Value::list_empty(Type::string())],
            want: None,
            err: "at least two arguments are required",
        },
        Case {
            collections: vec![
                Value::list_empty(Type::empty_object()),
                Value::list([Value::string("quick"), Value::string("fox")]),
            ],
            want: Some(Value::list_empty(Type::tuple([
                Type::empty_object(),
                Type::string(),
            ]))),
            err: "",
        },
        Case {
            collections: vec![
                Value::set_empty(Type::empty_object()),
                Value::set([Value::string("quick"), Value::string("fox")]),
            ],
            want: Some(Value::set_empty(Type::tuple([
                Type::empty_object(),
                Type::string(),
            ]))),
            err: "",
        },
        Case {
            collections: vec![
                Value::list_empty(Type::empty_object()),
                Value::list_empty(Type::empty_object()),
            ],
            want: Some(Value::list_empty(Type::tuple([
                Type::empty_object(),
                Type::empty_object(),
            ]))),
            err: "",
        },
        Case {
            collections: vec![
                Value::set_empty(Type::empty_object()),
                Value::set_empty(Type::empty_object()),
            ],
            want: Some(Value::set_empty(Type::tuple([
                Type::empty_object(),
                Type::empty_object(),
            ]))),
            err: "",
        },
        Case {
            collections: vec![
                Value::list([Value::list_empty(Type::string())]),
                Value::list([Value::list_empty(Type::string())]),
            ],
            want: Some(Value::list([Value::tuple([
                Value::list_empty(Type::string()),
                Value::list_empty(Type::string()),
            ])])),
            err: "",
        },
        Case {
            collections: vec![
                Value::set([Value::list_empty(Type::string())]),
                Value::set([Value::list_empty(Type::string())]),
            ],
            want: Some(Value::set([Value::tuple([
                Value::list_empty(Type::string()),
                Value::list_empty(Type::string()),
            ])])),
            err: "",
        },
        Case {
            collections: vec![
                Value::set([Value::list_empty(Type::string()).mark("a")]),
                Value::set([Value::list_empty(Type::string())]),
            ],
            want: Some(Value::set([Value::tuple([
                Value::list_empty(Type::string()).mark("a"),
                Value::list_empty(Type::string()),
            ])])),
            err: "",
        },
        Case {
            collections: vec![
                Value::tuple([Value::string("the"), Value::string("brown")]),
                Value::tuple([Value::string("fox"), Value::number_int(3)]),
            ],
            want: Some(Value::list([
                Value::tuple([Value::string("the"), Value::string("fox")]),
                Value::tuple([Value::string("the"), Value::string("3")]),
                Value::tuple([Value::string("brown"), Value::string("fox")]),
                Value::tuple([Value::string("brown"), Value::string("3")]),
            ])),
            err: "",
        },
        Case {
            collections: vec![
                Value::set([Value::string("the"), Value::string("brown")]),
                Value::set([Value::string("quick"), Value::string("fox")]),
            ],
            want: Some(Value::set([
                Value::tuple([Value::string("the"), Value::string("quick")]),
                Value::tuple([Value::string("the"), Value::string("fox")]),
                Value::tuple([Value::string("brown"), Value::string("quick")]),
                Value::tuple([Value::string("brown"), Value::string("fox")]),
            ])),
            err: "",
        },
        // The collection itself is not marked, just some elements
        Case {
            collections: vec![
                Value::set([Value::string("the"), Value::string("brown").mark("a")]),
                Value::set([Value::string("quick"), Value::string("fox").mark("b")]),
            ],
            // Sets don't allow individually-marked elements, so the marks
            // end up aggregating on the set itself anyway in this case.
            want: Some(
                Value::set([
                    Value::tuple([Value::string("the"), Value::string("quick")]),
                    Value::tuple([Value::string("the"), Value::string("fox")]),
                    Value::tuple([Value::string("brown"), Value::string("quick")]),
                    Value::tuple([Value::string("brown"), Value::string("fox")]),
                ])
                .mark("a")
                .mark("b"),
            ),
            err: "",
        },
        // The collections are marked
        Case {
            collections: vec![
                Value::set([Value::string("the"), Value::string("brown")]).mark("a"),
                Value::set([Value::string("quick"), Value::string("fox")]).mark("b"),
            ],
            want: Some(
                Value::set([
                    Value::tuple([Value::string("the"), Value::string("quick")]),
                    Value::tuple([Value::string("the"), Value::string("fox")]),
                    Value::tuple([Value::string("brown"), Value::string("quick")]),
                    Value::tuple([Value::string("brown"), Value::string("fox")]),
                ])
                .mark("a")
                .mark("b"),
            ),
            err: "",
        },
        // One collection is marked
        Case {
            collections: vec![
                Value::set([Value::string("the"), Value::string("brown")]).mark("a"),
                Value::set([Value::string("quick"), Value::string("fox")]),
            ],
            want: Some(
                Value::set([
                    Value::tuple([Value::string("the"), Value::string("quick")]),
                    Value::tuple([Value::string("the"), Value::string("fox")]),
                    Value::tuple([Value::string("brown"), Value::string("quick")]),
                    Value::tuple([Value::string("brown"), Value::string("fox")]),
                ])
                .mark("a"),
            ),
            err: "",
        },
        // Inner and outer marks
        Case {
            collections: vec![
                Value::set([Value::string("the"), Value::string("brown").mark("a")]).mark("b"),
                Value::set([Value::string("quick"), Value::string("fox").mark("c")]),
            ],
            want: Some(
                Value::set([
                    Value::tuple([Value::string("the"), Value::string("quick")]),
                    Value::tuple([Value::string("the"), Value::string("fox")]),
                    Value::tuple([Value::string("brown"), Value::string("quick")]),
                    Value::tuple([Value::string("brown"), Value::string("fox")]),
                ])
                .with_marks([ValueMarks::from_marks(["b", "c", "a"])]),
            ),
            err: "",
        },
        // SetproductFunc supports lists too, in which case it preserves the
        // input order and returns a list as the result. In this case we can
        // preserve the marks more precisely.
        // The collection itself is not marked, just some elements
        Case {
            collections: vec![
                Value::list([Value::string("the"), Value::string("brown").mark("a")]),
                Value::list([Value::string("quick"), Value::string("fox").mark("b")]),
            ],
            want: Some(Value::list([
                Value::tuple([Value::string("the"), Value::string("quick")]),
                Value::tuple([Value::string("the"), Value::string("fox").mark("b")]),
                Value::tuple([Value::string("brown").mark("a"), Value::string("quick")]),
                Value::tuple([
                    Value::string("brown").mark("a"),
                    Value::string("fox").mark("b"),
                ]),
            ])),
            err: "",
        },
        // The collections are marked
        Case {
            collections: vec![
                Value::list([Value::string("the"), Value::string("brown")]).mark("a"),
                Value::list([Value::string("quick"), Value::string("fox")]).mark("b"),
            ],
            want: Some(
                Value::list([
                    Value::tuple([Value::string("the"), Value::string("quick")]),
                    Value::tuple([Value::string("the"), Value::string("fox")]),
                    Value::tuple([Value::string("brown"), Value::string("quick")]),
                    Value::tuple([Value::string("brown"), Value::string("fox")]),
                ])
                .mark("a")
                .mark("b"),
            ),
            err: "",
        },
        // One collection is marked
        Case {
            collections: vec![
                Value::list([Value::string("the"), Value::string("brown")]).mark("a"),
                Value::list([Value::string("quick"), Value::string("fox")]),
            ],
            want: Some(
                Value::list([
                    Value::tuple([Value::string("the"), Value::string("quick")]),
                    Value::tuple([Value::string("the"), Value::string("fox")]),
                    Value::tuple([Value::string("brown"), Value::string("quick")]),
                    Value::tuple([Value::string("brown"), Value::string("fox")]),
                ])
                .mark("a"),
            ),
            err: "",
        },
        // Inner and outer marks
        Case {
            collections: vec![
                Value::list([Value::string("the"), Value::string("brown").mark("a")]).mark("b"),
                Value::list([Value::string("quick"), Value::string("fox").mark("c")]),
            ],
            want: Some(
                Value::list([
                    Value::tuple([Value::string("the"), Value::string("quick")]),
                    Value::tuple([Value::string("the"), Value::string("fox").mark("c")]),
                    Value::tuple([Value::string("brown").mark("a"), Value::string("quick")]),
                    Value::tuple([
                        Value::string("brown").mark("a"),
                        Value::string("fox").mark("c"),
                    ]),
                ])
                .mark("b"),
            ),
            err: "",
        },
        // Empty lists with marks should propagate the marks
        Case {
            collections: vec![
                Value::list_empty(Type::string()).mark("a"),
                Value::list_empty(Type::bool()).mark("b"),
            ],
            want: Some(
                Value::list_empty(Type::tuple([Type::string(), Type::bool()]))
                    .with_marks([ValueMarks::from_marks(["a", "b"])]),
            ),
            err: "",
        },
        // Empty sets with marks should propagate the marks
        Case {
            collections: vec![
                Value::set_empty(Type::string()).mark("a"),
                Value::set_empty(Type::bool()).mark("b"),
            ],
            want: Some(
                Value::set_empty(Type::tuple([Type::string(), Type::bool()]))
                    .with_marks([ValueMarks::from_marks(["a", "b"])]),
            ),
            err: "",
        },
        // Arguments which are sets with partially unknown values results
        // in unknown length (since the unknown values may already be
        // present in the set). This gives an unknown result preserving all
        // marks
        Case {
            collections: vec![
                Value::set([Value::string("x"), Value::unknown(Type::string())]).mark("a"),
                Value::set([Value::bool(true), Value::bool(false)]).mark("b"),
            ],
            want: Some(
                Value::unknown(Type::set(Type::tuple([Type::string(), Type::bool()])))
                    .refine_not_null()
                    .with_marks([ValueMarks::from_marks(["a", "b"])]),
            ),
            err: "",
        },
        Case {
            collections: vec![Value::set([Value::bool(true)]), Value::dynamic()],
            want: Some(Value::dynamic()),
            err: "",
        },
        // If the inputs have unknown lengths but have length refinements then
        // we can potentially refine our unknown result too.
        Case {
            collections: vec![
                Value::unknown(Type::set(Type::string()))
                    .refine()
                    .collection_length_upper_bound(2)
                    .new_value(),
                Value::unknown(Type::set(Type::number()))
                    .refine()
                    .collection_length_upper_bound(3)
                    .new_value(),
            ],
            want: Some(
                Value::unknown(Type::set(Type::tuple([Type::string(), Type::number()])))
                    .refine()
                    .not_null()
                    .collection_length_lower_bound(1)
                    .collection_length_upper_bound(6)
                    .new_value(),
            ),
            err: "",
        },
        Case {
            collections: vec![
                Value::unknown(Type::set(Type::string()))
                    .refine()
                    .collection_length_upper_bound(2)
                    .new_value(),
                Value::set_empty(Type::number()),
            ],
            // deduced from refinements
            want: Some(Value::set_empty(Type::tuple([
                Type::string(),
                Type::number(),
            ]))),
            err: "",
        },
        // If we have any input with a very large maximum element count then we'll
        // just leave the result length unrefined to reduce the risk of integer overflow.
        Case {
            collections: vec![
                Value::unknown(Type::set(Type::string()))
                    .refine()
                    .collection_length_upper_bound(2)
                    .new_value(),
                Value::unknown(Type::set(Type::number()))
                    .refine()
                    .collection_length_upper_bound(4096)
                    .new_value(),
            ],
            want: Some(
                Value::unknown(Type::set(Type::tuple([Type::string(), Type::number()])))
                    .refine_not_null(),
            ),
            err: "",
        },
        Case {
            collections: vec![
                Value::unknown(Type::list(Type::string()))
                    .refine()
                    .collection_length_upper_bound(2)
                    .new_value(),
                Value::unknown(Type::list(Type::number()))
                    .refine()
                    .collection_length_upper_bound(3)
                    .new_value(),
            ],
            // NOTE: When the result is a list rather than a set there is no
            // coalescing and so we could potentially also calculate a more
            // refined lower bound on the collection length, but since
            // this function is primarily for sets for now we just accept a
            // set-oriented refinement. If we find that it would be productive
            // to further constrain the range of a list result then we can
            // make this more precise later.
            want: Some(
                Value::unknown(Type::list(Type::tuple([Type::string(), Type::number()])))
                    .refine()
                    .not_null()
                    .collection_length_lower_bound(1)
                    .collection_length_upper_bound(6)
                    .new_value(),
            ),
            err: "",
        },
        Case {
            collections: vec![
                Value::unknown(Type::list(Type::string()))
                    .refine()
                    .collection_length_upper_bound(2)
                    .new_value(),
                Value::list_empty(Type::number()),
            ],
            // deduced from refinements
            want: Some(Value::list_empty(Type::tuple([
                Type::string(),
                Type::number(),
            ]))),
            err: "",
        },
        Case {
            collections: vec![
                Value::unknown(Type::tuple([Type::string(), Type::string()])),
                Value::unknown(Type::tuple([
                    Type::number(),
                    Type::number(),
                    Type::number(),
                ])),
            ],
            // NOTE: When the result is a list rather than a set there is no
            // coalescing and so we could potentially also calculate a more
            // refined lower bound on the collection length, but since
            // this function is primarily for sets for now we just accept a
            // set-oriented refinement. If we find that it would be productive
            // to further constrain the range of a list result then we can
            // make this more precise later.
            want: Some(
                Value::unknown(Type::list(Type::tuple([Type::string(), Type::number()])))
                    .refine()
                    .not_null()
                    .collection_length_lower_bound(1)
                    .collection_length_upper_bound(6)
                    .new_value(),
            ),
            err: "",
        },
        Case {
            collections: vec![
                Value::unknown(Type::tuple([Type::string(), Type::string()])),
                Value::empty_tuple(),
            ],
            // NOTE: When the result is a list rather than a set there is no
            // coalescing and so we could potentially also calculate a more
            // refined lower bound on the collection length, but since
            // this function is primarily for sets for now we just accept a
            // set-oriented refinement. If we find that it would be productive
            // to further constrain the range of a list result then we can
            // make this more precise later.
            want: Some(Value::list_empty(Type::tuple([
                Type::string(),
                Type::dynamic(),
            ]))),
            err: "",
        },
    ];

    for (i, case) in tests.iter().enumerate() {
        let got = stdlib::set_product(&case.collections);
        if !case.err.is_empty() {
            let err = got.err().unwrap_or_else(|| {
                panic!("case {i}: succeeded; want error");
            });
            assert_eq!(err.to_string(), case.err, "case {i}: wrong error");
            continue;
        }
        let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *case.want.as_ref().unwrap(), "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/collection_test.go TestReverseList
#[test]
fn reverse_list() {
    // NOTE(port): upstream's first case passes `cty.NilVal` as the input and
    // expects the error "argument must not be null". Go's zero-value NilVal
    // has no Rust analogue (see docs/api-mapping.md), so that case is
    // deliberately omitted.
    let tests: Vec<(Value, Value, &'static str)> = vec![
        (
            Value::list_empty(Type::string()),
            Value::list_empty(Type::string()),
            "",
        ),
        (
            Value::list_empty(Type::string()).mark("foo"),
            Value::list_empty(Type::string()).mark("foo"),
            "",
        ),
        (
            Value::unknown(Type::list(Type::string())),
            Value::unknown(Type::list(Type::string())).refine_not_null(),
            "",
        ),
        // marks on list elements
        (
            Value::list([
                Value::string("beep").mark("boop"),
                Value::string("bop"),
                Value::string("bloop"),
            ]),
            Value::list([
                Value::string("bloop"),
                Value::string("bop"),
                Value::string("beep").mark("boop"),
            ]),
            "",
        ),
        // marks on the entire input are preserved
        (
            Value::list([
                Value::string("beep").mark("boop"),
                Value::string("bop"),
                Value::string("bloop"),
            ])
            .mark("outer"),
            Value::list([
                Value::string("bloop"),
                Value::string("bop"),
                Value::string("beep").mark("boop"),
            ])
            .mark("outer"),
            "",
        ),
        // marks on tuple elements
        (
            Value::tuple([
                Value::string("beep").mark("boop"),
                Value::string("bop"),
                Value::string("bloop"),
            ]),
            Value::tuple([
                Value::string("bloop"),
                Value::string("bop"),
                Value::string("beep").mark("boop"),
            ]),
            "",
        ),
        // Set elements don't support individual marks; any marks on elements get propegated to the entire set.
        (
            Value::set([
                Value::string("beep").mark("boop"),
                Value::string("bop"),
                Value::string("bloop"),
            ]),
            // sets end up sorted alphabetically when converted to lists
            Value::list([
                Value::string("bop"),
                Value::string("bloop"),
                Value::string("beep"),
            ])
            .mark("boop"),
            "",
        ),
    ];

    for (i, (input, want, want_err)) in tests.iter().enumerate() {
        let got = stdlib::reverse_list(input);
        if !want_err.is_empty() {
            let err = got.err().unwrap_or_else(|| {
                panic!("case {i}: succeeded; want error");
            });
            assert_eq!(err.to_string(), *want_err, "case {i}: wrong error");
            continue;
        }
        let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *want, "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/collection_test.go TestSlice
#[test]
fn slice() {
    struct Case {
        input: Value,
        start: Value,
        end: Value,
        want: Value,
        err: &'static str,
    }

    let tests: Vec<Case> = vec![
        Case {
            input: Value::list([Value::string("a"), Value::string("b"), Value::string("c")]),
            start: Value::number_int(0),
            end: Value::number_int(2),
            want: Value::list([Value::string("a"), Value::string("b")]),
            err: "",
        },
        // The entire input list is marked, so the return should be marked
        Case {
            input: Value::list([Value::string("a"), Value::string("b"), Value::string("c")])
                .mark("bloop"),
            start: Value::number_int(0),
            end: Value::number_int(2),
            want: Value::list([Value::string("a"), Value::string("b")]).mark("bloop"),
            err: "",
        },
        // individual element marks should be preserved
        Case {
            input: Value::list([
                Value::string("a"),
                Value::string("b").mark("bloop"),
                Value::string("c"),
            ]),
            start: Value::number_int(0),
            end: Value::number_int(2),
            want: Value::list([Value::string("a"), Value::string("b").mark("bloop")]),
            err: "",
        },
    ];

    for (i, case) in tests.iter().enumerate() {
        let got = stdlib::slice(&case.input, &case.start, &case.end);
        if !case.err.is_empty() {
            let err = got.err().unwrap_or_else(|| {
                panic!("case {i}: succeeded; want error");
            });
            assert_eq!(err.to_string(), case.err, "case {i}: wrong error");
            continue;
        }
        let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, case.want, "case {i}: wrong result");
    }
}

// upstream: cty/function/stdlib/collection_test.go TestDistinct
#[test]
fn distinct() {
    // NOTE(port): upstream's `Want` field is `cty.NilVal` in the error case;
    // that case carries `None` here.
    struct Case {
        list: Value,
        want: Option<Value>,
        err: &'static str,
    }

    let tests: Vec<Case> = vec![
        // Empty list (string type)
        Case {
            list: Value::list_empty(Type::string()),
            want: Some(Value::list_empty(Type::string())),
            err: "",
        },
        // Empty list (number type)
        Case {
            list: Value::list_empty(Type::number()),
            want: Some(Value::list_empty(Type::number())),
            err: "",
        },
        // Empty list (unknown element type)
        Case {
            list: Value::list_empty(Type::dynamic()),
            want: Some(Value::list_empty(Type::dynamic())),
            err: "",
        },
        // List with single element
        Case {
            list: Value::list([Value::string("single")]),
            want: Some(Value::list([Value::string("single")])),
            err: "",
        },
        // List where all elements are identical
        Case {
            list: Value::list([
                Value::number_int(42),
                Value::number_int(42),
                Value::number_int(42),
            ]),
            want: Some(Value::list([Value::number_int(42)])),
            err: "",
        },
        // List that is already distinct
        Case {
            list: Value::list([Value::string("a"), Value::string("b"), Value::string("c")]),
            want: Some(Value::list([
                Value::string("a"),
                Value::string("b"),
                Value::string("c"),
            ])),
            err: "",
        },
        // List with nested lists
        Case {
            list: Value::list([
                Value::list([Value::string("a"), Value::string("a")]),
                Value::list([Value::string("b")]),
                Value::list([Value::string("a"), Value::string("a")]),
            ]),
            want: Some(Value::list([
                Value::list([Value::string("a"), Value::string("a")]),
                Value::list([Value::string("b")]),
            ])),
            err: "",
        },
        // Wholly-unknown list
        Case {
            list: Value::unknown(Type::list(Type::string())),
            want: Some(Value::unknown(Type::list(Type::string())).refine_not_null()),
            err: "",
        },
        // List with unknown values
        Case {
            list: Value::list([
                Value::unknown(Type::string()),
                Value::string("a"),
                Value::string("b"),
                Value::unknown(Type::string()),
            ]),
            want: Some(Value::unknown(Type::list(Type::string())).refine_not_null()),
            err: "",
        },
        // null list
        Case {
            list: Value::null(Type::list(Type::string())),
            want: None,
            err: "argument must not be null",
        },
        // List with null values
        Case {
            list: Value::list([
                Value::null(Type::string()),
                Value::string("a"),
                Value::null(Type::string()),
                Value::string("b"),
            ]),
            want: Some(Value::list([
                Value::null(Type::string()),
                Value::string("a"),
                Value::string("b"),
            ])),
            err: "",
        },
    ];

    for (i, case) in tests.iter().enumerate() {
        let got = stdlib::distinct(&case.list);
        if !case.err.is_empty() {
            let err = got.err().unwrap_or_else(|| {
                panic!("case {i}: succeeded; want error");
            });
            assert_eq!(err.to_string(), case.err, "case {i}: wrong error");
            continue;
        }
        let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        assert_eq!(got, *case.want.as_ref().unwrap(), "case {i}: wrong result");
    }
}
