//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/marks_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Path, PathStep, PathValueMarks, Type, Value, ValueMarks};

// Ported from TestContainsMarked:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/marks_test.go#L11
#[test]
#[ignore = "not yet implemented"]
fn contains_marked() {
    let test_cases: Vec<(Value, bool)> = vec![
        (Value::string("a"), false),
        (Value::number_int(1).mark("a"), true),
        (
            Value::list([Value::number_int(1), Value::number_int(2)]),
            false,
        ),
        (
            Value::list([Value::number_int(1), Value::number_int(2).mark("a")]),
            true,
        ),
        (
            Value::list([Value::number_int(1), Value::number_int(2)]).mark("a"),
            true,
        ),
        (Value::list_empty(Type::string()).mark("c"), true),
        (
            Value::map([
                ("a", Value::string("b").mark("c")),
                ("x", Value::string("y").mark("z")),
            ]),
            true,
        ),
        (
            Value::tuple([Value::number_int(1).mark("a"), Value::string("y").mark("z")]),
            true,
        ),
        (
            Value::set([
                Value::number_int(1).mark("a"),
                Value::number_int(2).mark("z"),
            ]),
            true,
        ),
        (
            Value::object([
                (
                    "x",
                    Value::list([Value::number_int(1).mark("a"), Value::number_int(2)]),
                ),
                ("y", Value::string("y")),
                ("z", Value::bool(true)),
            ]),
            true,
        ),
    ];

    for (i, (val, want)) in test_cases.iter().enumerate() {
        let got = val.contains_marked();
        assert_eq!(
            got, *want,
            "case {i}: wrong result (got {got}, want {want}) for {val:?}"
        );
    }
}

// Ported from TestIsMarked:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/marks_test.go#L72
#[test]
#[ignore = "not yet implemented"]
fn is_marked() {
    let test_cases: Vec<(Value, bool)> = vec![
        (Value::string("a"), false),
        (Value::number_int(1).mark("a"), true),
        (
            Value::list([Value::number_int(1), Value::number_int(2)]),
            false,
        ),
        (
            Value::list([Value::number_int(1), Value::number_int(2).mark("a")]),
            false,
        ),
        (
            Value::list([Value::number_int(1), Value::number_int(2)]).mark("a"),
            true,
        ),
    ];

    for (i, (val, want)) in test_cases.iter().enumerate() {
        let got = val.is_marked();
        assert_eq!(
            got, *want,
            "case {i}: wrong result (got {got}, want {want}) for {val:?}"
        );
    }
}

// Ported from TestValueMarks:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/marks_test.go#L106
#[test]
#[ignore = "not yet implemented"]
fn value_marks() {
    let v = Value::bool(true);
    let v1 = v.mark(1i64);
    let v2 = v.mark(2i64);

    assert_eq!(v.marks(), ValueMarks::new(), "wrong v marks");
    assert_eq!(v1.marks(), ValueMarks::from_marks([1i64]), "wrong v1 marks");
    assert_eq!(v2.marks(), ValueMarks::from_marks([2i64]), "wrong v2 marks");

    let v12 = Value::bool(false).with_same_marks(&[&v, &v1, &v2]);
    assert_eq!(
        v12.marks(),
        ValueMarks::from_marks([1i64, 2]),
        "wrong v12 marks"
    );

    let v12_again = v12.mark(1i64);
    assert_eq!(
        v12_again.marks(),
        ValueMarks::from_marks([1i64, 2]),
        "wrong v12Again marks"
    );

    let v1234 = v12.with_marks([ValueMarks::from_marks([2i64, 3, 4])]);
    assert_eq!(
        v1234.marks(),
        ValueMarks::from_marks([1i64, 2, 3, 4]),
        "wrong v1234 marks"
    );
    assert!(v1234.has_mark(2i64), "v1234 should have mark 2");
    assert!(!v1234.has_mark(5i64), "v1234 should not have mark 5");

    let (v, marks1234) = v1234.unmark();
    assert_eq!(
        v.marks(),
        ValueMarks::new(),
        "wrong v marks after unmarking"
    );
    assert_eq!(
        marks1234,
        ValueMarks::from_marks([1i64, 2, 3, 4]),
        "wrong marks1234"
    );
    assert_eq!(v, Value::bool(false), "wrong v after unmarking");

    // One more test for a more interesting/realistic situation involving
    // a number of different operations.
    let a = Value::number_int(2).mark("a");
    let b = Value::number_int(5).mark("b");
    let c = Value::number_int(1).mark("c");
    let d = Value::number_int(12).mark("d");
    let result = a.multiply(&b).subtract(&c).greater_than_or_equal_to(&d);
    assert_eq!(
        result,
        Value::bool(false).with_marks([ValueMarks::from_marks(["a", "b", "c", "d"])]),
        "wrong result"
    );

    // Unmark the result and capture the paths
    let (unmarked_result, pvm) = result.unmark_deep_with_paths();
    // Remark the result with those paths
    let remarked = unmarked_result.mark_with_paths(&pvm);
    assert_eq!(
        remarked,
        Value::bool(false).with_marks([ValueMarks::from_marks(["a", "b", "c", "d"])]),
        "wrong result"
    );

    // If we call MarkWithPaths without any matching paths, we should get the unmarked result
    let marked_with_no_paths = unmarked_result.mark_with_paths(&[PathValueMarks {
        path: Path::from_steps([PathStep::Index(Value::number_int(0))]),
        marks: ValueMarks::from_marks(["z"]),
    }]);
    assert_eq!(marked_with_no_paths, Value::bool(false), "wrong result");
}

// Ported from TestValueMarksInsert:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/marks_test.go#L179
#[test]
#[ignore = "not yet implemented"]
fn value_marks_insert() {
    let mut marks = ValueMarks::from_marks([0i64]);
    marks.insert([2i64, 1i64]);
    assert!(marks.has(0i64), "marks set does not contain 0");
    assert!(marks.has(1i64), "marks set does not contain 1");
    assert!(marks.has(2i64), "marks set does not contain 2");
    assert_eq!(
        marks.len(),
        3,
        "marks set has {} elements; want 3",
        marks.len()
    );
}

// Ported from TestPathValueMarksEqual:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/marks_test.go#L196
#[test]
#[ignore = "not yet implemented"]
fn path_value_marks_equal() {
    let tests: Vec<(PathValueMarks, PathValueMarks, bool)> = vec![
        (
            PathValueMarks {
                path: Path::from_steps([PathStep::Index(Value::number_int(0))]),
                marks: ValueMarks::from_marks(["a"]),
            },
            PathValueMarks {
                path: Path::from_steps([PathStep::Index(Value::number_int(0))]),
                marks: ValueMarks::from_marks(["a"]),
            },
            true,
        ),
        (
            PathValueMarks {
                path: Path::from_steps([PathStep::Index(Value::string("p"))]),
                marks: ValueMarks::from_marks([123i64]),
            },
            PathValueMarks {
                path: Path::from_steps([PathStep::Index(Value::string("p"))]),
                marks: ValueMarks::from_marks([123i64]),
            },
            true,
        ),
        (
            PathValueMarks {
                path: Path::from_steps([PathStep::Index(Value::number_int(0))]),
                marks: ValueMarks::from_marks(["a"]),
            },
            PathValueMarks {
                path: Path::from_steps([PathStep::Index(Value::number_int(1))]),
                marks: ValueMarks::from_marks(["a"]),
            },
            false,
        ),
        (
            PathValueMarks {
                path: Path::from_steps([PathStep::Index(Value::number_int(0))]),
                marks: ValueMarks::from_marks(["a"]),
            },
            PathValueMarks {
                path: Path::from_steps([PathStep::Index(Value::number_int(0))]),
                marks: ValueMarks::from_marks(["b"]),
            },
            false,
        ),
        (
            PathValueMarks {
                path: Path::from_steps([PathStep::Index(Value::number_int(0))]),
                marks: ValueMarks::from_marks(["a"]),
            },
            PathValueMarks {
                path: Path::from_steps([PathStep::Index(Value::number_int(1))]),
                marks: ValueMarks::from_marks(["b"]),
            },
            false,
        ),
    ];

    for (i, (original, compare, want)) in tests.iter().enumerate() {
        let got = original == compare;
        assert_eq!(
            got, *want,
            "case {i}: comparing {original:?} to {compare:?}: wrong result (got {got}, want {want})"
        );
    }
}

// Ported from TestMarks:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/marks_test.go#L238
#[test]
#[ignore = "not yet implemented"]
fn marks() {
    fn want_marks(marks: &ValueMarks, expected: &[&str]) {
        assert_eq!(marks.len(), expected.len(), "wrong marks: {marks:?}");
        for mark in expected {
            assert!(marks.has(*mark), "missing mark {mark:?}: {marks:?}");
        }
    }

    // Single mark
    let val = Value::string("foo").mark("a");
    want_marks(&val.marks(), &["a"]);
    let (val, marks) = val.unmark();
    assert!(!val.is_marked(), "still marked after unmark: {marks:?}");
    want_marks(&marks, &["a"]);

    // Multiple marks
    let val = val.with_marks([ValueMarks::from_marks(["a", "b", "c"])]);
    want_marks(&val.marks(), &["a", "b", "c"]);
    let (val, marks) = val.unmark();
    assert!(!val.is_marked(), "still marked after unmark: {marks:?}");
    want_marks(&marks, &["a", "b", "c"]);

    // Multiple marks, applied separately
    let val = val.mark("a").mark("b");
    want_marks(&val.marks(), &["a", "b"]);
    let (val, marks) = val.unmark();
    assert!(!val.is_marked(), "still marked after unmark: {marks:?}");
    want_marks(&marks, &["a", "b"]);
}

// Ported from TestUnmarkDeep:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/marks_test.go#L278
#[test]
#[ignore = "not yet implemented"]
fn unmark_deep() {
    let test_cases: Vec<(&str, Value, Value, ValueMarks)> = vec![
        (
            "unmarked string",
            Value::string("a"),
            Value::string("a"),
            ValueMarks::new(),
        ),
        (
            "marked number",
            Value::number_int(1).mark("a"),
            Value::number_int(1),
            ValueMarks::from_marks(["a"]),
        ),
        (
            "unmarked list",
            Value::list([Value::number_int(1), Value::number_int(2)]),
            Value::list([Value::number_int(1), Value::number_int(2)]),
            ValueMarks::new(),
        ),
        (
            "list with some elements marked",
            Value::list([Value::number_int(1).mark("a"), Value::number_int(2)]),
            Value::list([Value::number_int(1), Value::number_int(2)]),
            ValueMarks::from_marks(["a"]),
        ),
        (
            "marked list with all elements marked",
            Value::list([
                Value::number_int(1).mark("a"),
                Value::number_int(2).mark("b"),
            ])
            .mark("c"),
            Value::list([Value::number_int(1), Value::number_int(2)]),
            ValueMarks::from_marks(["a", "b", "c"]),
        ),
        (
            "marked empty list",
            Value::list_empty(Type::string()).mark("c"),
            Value::list_empty(Type::string()),
            ValueMarks::from_marks(["c"]),
        ),
        (
            "map with elements marked",
            Value::map([
                ("a", Value::string("b").mark("c")),
                ("x", Value::string("y").mark("z")),
            ]),
            Value::map([("a", Value::string("b")), ("x", Value::string("y"))]),
            ValueMarks::from_marks(["c", "z"]),
        ),
        (
            "tuple with elements marked",
            Value::tuple([Value::number_int(1).mark("a"), Value::string("y").mark("z")]),
            Value::tuple([Value::number_int(1), Value::string("y")]),
            ValueMarks::from_marks(["a", "z"]),
        ),
        (
            "set with elements marked",
            Value::set([
                Value::number_int(1).mark("a"),
                Value::number_int(2).mark("z"),
            ]),
            Value::set([Value::number_int(1), Value::number_int(2)]),
            ValueMarks::from_marks(["a", "z"]),
        ),
        (
            "complex marked object with lots of marks",
            Value::object([
                (
                    "x",
                    Value::list([
                        Value::number_int(3).mark("a"),
                        Value::number_int(5).mark("b"),
                    ])
                    .with_marks([ValueMarks::from_marks(["c", "d"])]),
                ),
                ("y", Value::string("y").mark("e")),
                ("z", Value::bool(true).mark("f")),
            ])
            .mark("g"),
            Value::object([
                (
                    "x",
                    Value::list([Value::number_int(3), Value::number_int(5)]),
                ),
                ("y", Value::string("y")),
                ("z", Value::bool(true)),
            ]),
            ValueMarks::from_marks(["a", "b", "c", "d", "e", "f", "g"]),
        ),
    ];

    for (name, val, want, want_marks) in test_cases.iter() {
        let (got, marks) = val.unmark_deep();
        assert_eq!(got, *want, "{name}: wrong value");
        assert_eq!(marks, *want_marks, "{name}: wrong marks");
    }
}

// Ported from TestPathValueMarks:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/marks_test.go#L363
#[test]
#[ignore = "not yet implemented"]
fn path_value_marks() {
    let test_cases: Vec<(&str, Value, Value, Vec<PathValueMarks>)> = vec![
        (
            "unmarked string",
            Value::string("a"),
            Value::string("a"),
            vec![],
        ),
        (
            "marked number",
            Value::number_int(1).mark("a"),
            Value::number_int(1),
            vec![PathValueMarks {
                path: Path::new(),
                marks: ValueMarks::from_marks(["a"]),
            }],
        ),
        (
            "list with some elements marked",
            Value::list([Value::number_int(1).mark("a"), Value::number_int(2)]),
            Value::list([Value::number_int(1), Value::number_int(2)]),
            vec![PathValueMarks {
                path: Path::new().index_int(0),
                marks: ValueMarks::from_marks(["a"]),
            }],
        ),
        (
            "marked list with all elements marked",
            Value::list([
                Value::number_int(1).mark("a"),
                Value::number_int(2).mark("b"),
            ])
            .mark("c"),
            Value::list([Value::number_int(1), Value::number_int(2)]),
            vec![
                PathValueMarks {
                    path: Path::new(),
                    marks: ValueMarks::from_marks(["c"]),
                },
                PathValueMarks {
                    path: Path::new().index_int(0),
                    marks: ValueMarks::from_marks(["a"]),
                },
                PathValueMarks {
                    path: Path::new().index_int(1),
                    marks: ValueMarks::from_marks(["b"]),
                },
            ],
        ),
        (
            "marked empty list",
            Value::list_empty(Type::string()).mark("c"),
            Value::list_empty(Type::string()),
            vec![PathValueMarks {
                path: Path::new(),
                marks: ValueMarks::from_marks(["c"]),
            }],
        ),
        (
            "map with elements marked",
            Value::map([
                ("a", Value::string("b").mark("c")),
                ("x", Value::string("y").mark("z")),
            ]),
            Value::map([("a", Value::string("b")), ("x", Value::string("y"))]),
            vec![
                PathValueMarks {
                    path: Path::new().index_string("a"),
                    marks: ValueMarks::from_marks(["c"]),
                },
                PathValueMarks {
                    path: Path::new().index_string("x"),
                    marks: ValueMarks::from_marks(["z"]),
                },
            ],
        ),
        (
            "tuple with elements marked",
            Value::tuple([
                Value::number_int(1).mark("a"),
                Value::string("y").mark("z"),
                Value::object([("x", Value::bool(true))]).mark("o"),
            ]),
            Value::tuple([
                Value::number_int(1),
                Value::string("y"),
                Value::object([("x", Value::bool(true))]),
            ]),
            vec![
                PathValueMarks {
                    path: Path::new().index_int(0),
                    marks: ValueMarks::from_marks(["a"]),
                },
                PathValueMarks {
                    path: Path::new().index_int(1),
                    marks: ValueMarks::from_marks(["z"]),
                },
                PathValueMarks {
                    path: Path::new().index_int(2),
                    marks: ValueMarks::from_marks(["o"]),
                },
            ],
        ),
        (
            "set with elements marked",
            Value::set([
                Value::number_int(1).mark("a"),
                Value::number_int(2).mark("z"),
            ]),
            Value::set([Value::number_int(1), Value::number_int(2)]),
            vec![PathValueMarks {
                path: Path::new(),
                marks: ValueMarks::from_marks(["a", "z"]),
            }],
        ),
        (
            "complex marked object with lots of marks",
            Value::object([
                (
                    "x",
                    Value::list([
                        Value::number_int(3).mark("a"),
                        Value::number_int(5).mark("b"),
                    ])
                    .with_marks([ValueMarks::from_marks(["c", "d"])]),
                ),
                ("y", Value::string("y").mark("e")),
                ("z", Value::bool(true).mark("f")),
            ])
            .mark("g"),
            Value::object([
                (
                    "x",
                    Value::list([Value::number_int(3), Value::number_int(5)]),
                ),
                ("y", Value::string("y")),
                ("z", Value::bool(true)),
            ]),
            vec![
                PathValueMarks {
                    path: Path::new(),
                    marks: ValueMarks::from_marks(["g"]),
                },
                PathValueMarks {
                    path: Path::new().attr("x"),
                    marks: ValueMarks::from_marks(["c", "d"]),
                },
                PathValueMarks {
                    path: Path::new().attr("x").index_int(0),
                    marks: ValueMarks::from_marks(["a"]),
                },
                PathValueMarks {
                    path: Path::new().attr("x").index_int(1),
                    marks: ValueMarks::from_marks(["b"]),
                },
                PathValueMarks {
                    path: Path::new().attr("y"),
                    marks: ValueMarks::from_marks(["e"]),
                },
                PathValueMarks {
                    path: Path::new().attr("z"),
                    marks: ValueMarks::from_marks(["f"]),
                },
            ],
        ),
        (
            "path array reuse regression test",
            Value::object([(
                "environment",
                Value::list([Value::object([(
                    "variables",
                    Value::map([
                        ("bar", Value::string("secret").mark("sensitive")),
                        ("foo", Value::string("secret").mark("sensitive")),
                    ]),
                )])]),
            )]),
            Value::object([(
                "environment",
                Value::list([Value::object([(
                    "variables",
                    Value::map([
                        ("bar", Value::string("secret")),
                        ("foo", Value::string("secret")),
                    ]),
                )])]),
            )]),
            vec![
                PathValueMarks {
                    path: Path::new()
                        .attr("environment")
                        .index_int(0)
                        .attr("variables")
                        .index_string("bar"),
                    marks: ValueMarks::from_marks(["sensitive"]),
                },
                PathValueMarks {
                    path: Path::new()
                        .attr("environment")
                        .index_int(0)
                        .attr("variables")
                        .index_string("foo"),
                    marks: ValueMarks::from_marks(["sensitive"]),
                },
            ],
        ),
    ];

    for (name, marked, unmarked, want_pvms) in test_cases.iter() {
        // unmark direction
        let (got, pvms) = marked.unmark_deep_with_paths();
        assert_eq!(got, *unmarked, "unmark: {name}: wrong value");

        assert_eq!(
            pvms.len(),
            want_pvms.len(),
            "unmark: {name}: wrong length\n got: {}\nwant: {}",
            pvms.len(),
            want_pvms.len()
        );

        for want_pvm in want_pvms.iter() {
            let found = pvms.iter().any(|got_pvm| {
                got_pvm.path.equals(&want_pvm.path) && got_pvm.marks == want_pvm.marks
            });
            assert!(
                found,
                "unmark: {name}: missing {want_pvm:?}\nnot found in: {pvms:?}"
            );
        }

        // mark direction
        let got = unmarked.mark_with_paths(want_pvms);
        assert_eq!(got, *marked, "mark: {name}: wrong value");
    }
}

// Ported from TestReapplyMarks:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/marks_test.go#L513
#[test]
#[ignore = "not yet implemented"]
fn reapply_marks() {
    // Re-applying the same marks to an object value should not change the result.
    let obj = Value::object([(
        "nested",
        Value::object([("attr", Value::string("not directly marked"))]),
    )]);

    let pvm = [PathValueMarks {
        path: Path::new().attr("nested"),
        marks: ValueMarks::from_marks(["mark"]),
    }];

    let first = obj.mark_with_paths(&pvm);
    let second = first.mark_with_paths(&pvm);

    assert_eq!(
        first, second,
        "Value changed re-applying marks\n1st: {first:?}\n2nd: {second:?}"
    );
}

// Ported from TestHasMarkDeep:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/marks_test.go#L531
#[test]
#[ignore = "not yet implemented"]
fn has_mark_deep() {
    let obj = Value::object([(
        "nested",
        Value::object([("marked", Value::bool(true).mark("boop"))]),
    )]);
    assert!(obj.has_mark_deep("boop"), "did not find nested mark");
}

// Ported from TestValueMarksOfType:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/marks_test.go#L542
#[test]
#[ignore = "not yet implemented"]
fn value_marks_of_type() {
    // shallow
    {
        let obj = Value::object([(
            "nested",
            Value::object([
                ("marked 1", Value::bool(true).mark("nested")),
                ("marked 2", Value::bool(true).mark(2i64)),
            ]),
        )])
        .mark("shallow")
        .mark(2i64);
        let got = obj.marks_of_type::<String>();
        assert_eq!(got, ["shallow"], "shallow: wrong result");
    }
    // only nested
    {
        let obj = Value::object([(
            "nested",
            Value::object([
                ("marked 1", Value::bool(true).mark("nested")),
                ("marked 2", Value::bool(true).mark(2i64)),
            ]),
        )]);
        let got = obj.marks_of_type::<String>();
        assert_eq!(got, [] as [&str; 0], "only nested: wrong result");
    }
}

// Ported from TestValueMarksOfTypeDeep:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/marks_test.go#L571
#[test]
#[ignore = "not yet implemented"]
fn value_marks_of_type_deep() {
    let obj = Value::object([(
        "nested",
        Value::object([
            ("marked 1", Value::bool(true).mark("boop")),
            ("marked 2", Value::bool(true).mark(2i64)),
        ]),
    )]);
    let got = obj.marks_of_type_deep::<String>();
    assert_eq!(got, ["boop"], "wrong result");
}
