//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/path_test.go
//!   cty/path_set_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Path, PathSet, PathStep, Type, Value};

// upstream: cty/path_test.go TestPathApply
#[test]
fn path_apply() {
    // NOTE(port): upstream's `Want` field is `cty.NilVal` in every case where
    // `WantErr` is set; `Path.Apply` returns `Result` here, so those cases
    // carry `None` for the want value rather than a nil placeholder.
    struct Case {
        start: Value,
        path: Path,
        want: Option<Value>,
        want_err: Option<&'static str>,
    }

    let tests: Vec<Case> = vec![
        Case {
            start: Value::string("hello"),
            path: Path::new(),
            want: Some(Value::string("hello")),
            want_err: None,
        },
        Case {
            start: Value::string("hello"),
            path: Path::new().index(Value::string("boop")),
            want: None,
            want_err: Some(r#"at step 0: not a map type"#),
        },
        Case {
            start: Value::string("hello"),
            path: Path::new().index(Value::number_int(0)),
            want: None,
            want_err: Some(r#"at step 0: not a list type"#),
        },
        Case {
            start: Value::list([Value::string("hello")]),
            path: Path::new().index(Value::number_int(0)),
            want: Some(Value::string("hello")),
            want_err: None,
        },
        Case {
            start: Value::tuple([Value::string("hello")]),
            path: Path::new().index(Value::number_int(0)),
            want: Some(Value::string("hello")),
            want_err: None,
        },
        Case {
            start: Value::list_empty(Type::string()),
            path: Path::new().index(Value::number_int(0)),
            want: None,
            want_err: Some(r#"at step 0: value does not have given index key"#),
        },
        Case {
            start: Value::list([Value::string("hello")]),
            path: Path::new().index(Value::number_int(1)),
            want: None,
            want_err: Some(r#"at step 0: value does not have given index key"#),
        },
        Case {
            start: Value::list([Value::string("hello")]),
            path: Path::new().index(Value::number_int(0)).attr("foo"),
            want: None,
            want_err: Some(r#"at step 1: not an object type"#),
        },
        Case {
            start: Value::list([Value::empty_object()]),
            path: Path::new().index(Value::number_int(0)).attr("foo"),
            want: None,
            want_err: Some(r#"at step 1: object has no attribute "foo""#),
        },
        Case {
            start: Value::null(Type::list(Type::string())),
            path: Path::new().index(Value::number_int(0)),
            want: None,
            want_err: Some(r#"at step 0: cannot index a null value"#),
        },
        Case {
            start: Value::null(Type::map(Type::string())),
            path: Path::new().index(Value::number_int(0)),
            want: None,
            want_err: Some(r#"at step 0: cannot index a null value"#),
        },
        Case {
            start: Value::null(Type::empty_object()),
            path: Path::new().attr("foo"),
            want: None,
            want_err: Some(r#"at step 0: cannot access attributes on a null value"#),
        },
        Case {
            start: Value::list([Value::list([Value::string("hello")]).mark(2)]).mark(1),
            path: Path::new()
                .index(Value::number_int(0))
                .index(Value::number_int(0)),
            want: Some(Value::string("hello").mark(1).mark(2)),
            want_err: None,
        },
        Case {
            start: Value::tuple([Value::list([Value::string("hello")]).mark(2)]).mark(1),
            path: Path::new()
                .index(Value::number_int(0))
                .index(Value::number_int(0)),
            want: Some(Value::string("hello").mark(1).mark(2)),
            want_err: None,
        },
        Case {
            start: Value::map([("hello", Value::string("there"))]).mark(1),
            path: Path::new().index(Value::string("hello")),
            want: Some(Value::string("there").mark(1)),
            want_err: None,
        },
        Case {
            start: Value::object([("hello", Value::string("there"))]).mark(1),
            path: Path::new().attr("hello"),
            want: Some(Value::string("there").mark(1)),
            want_err: None,
        },
        Case {
            start: Value::list([Value::string("hello").mark(1)]),
            path: Path::new().index(Value::number_int(0)),
            want: Some(Value::string("hello").mark(1)),
            want_err: None,
        },
        Case {
            start: Value::tuple([Value::string("hello").mark(1)]),
            path: Path::new().index(Value::number_int(0)),
            want: Some(Value::string("hello").mark(1)),
            want_err: None,
        },
        Case {
            start: Value::map([("hello", Value::string("there").mark(1))]),
            path: Path::new().index(Value::string("hello")),
            want: Some(Value::string("there").mark(1)),
            want_err: None,
        },
        Case {
            start: Value::object([("hello", Value::string("there").mark(1))]),
            path: Path::new().attr("hello"),
            want: Some(Value::string("there").mark(1)),
            want_err: None,
        },
        Case {
            start: Value::set([
                Value::string("hello").mark(1), // Note: this mark is automatically hoisted to the set as a whole
            ])
            .mark(2),
            path: Path::new().index(Value::string("hello")),
            want: Some(Value::string("hello").mark(1).mark(2)),
            want_err: None,
        },
        Case {
            start: Value::set([
                Value::string("hello").mark(1), // Note: this mark is automatically hoisted to the set as a whole
            ])
            .mark(2),
            path: Path::new().index(Value::string("not present")),
            want: None,
            want_err: Some(r#"at step 0: set does not contain the requested element"#),
        },
        Case {
            start: Value::set([
                Value::string("hello").mark(1), // Note: this mark is automatically hoisted to the set as a whole
                Value::unknown(Type::string()),
            ])
            .mark(2),
            path: Path::new().index(Value::string("not present")),
            want: Some(Value::unknown(Type::string()).mark(1).mark(2)),
            want_err: None,
        },
        Case {
            start: Value::set([
                Value::string("hello").mark(1), // Note: this mark is automatically hoisted to the set as a whole
            ])
            .mark(2),
            // type mismatch is treated the same as value not present, constistent with Value.HasElement
            path: Path::new().index(Value::bool(true)),
            want: None,
            want_err: Some(r#"at step 0: set does not contain the requested element"#),
        },
        Case {
            start: Value::set([
                Value::string("hello").mark(1), // Note: this mark is automatically hoisted to the set as a whole
            ])
            .mark(2),
            // null is a valid set element, but it isn't present in this set
            path: Path::new().index(Value::null(Type::string())),
            want: None,
            want_err: Some(r#"at step 0: set does not contain the requested element"#),
        },
        Case {
            start: Value::set([
                Value::null(Type::string()).mark(1), // Note: this mark is automatically hoisted to the set as a whole
            ])
            .mark(2),
            path: Path::new().index(Value::null(Type::string())),
            want: Some(Value::null(Type::string()).mark(1).mark(2)),
            want_err: None,
        },
        Case {
            start: Value::set([
                Value::string("hello").mark(1), // Note: this mark is automatically hoisted to the set as a whole
            ])
            .mark(2),
            path: Path::new().index(Value::unknown(Type::string())),
            want: Some(Value::unknown(Type::string()).mark(1).mark(2)),
            want_err: None,
        },
        Case {
            start: Value::unknown(Type::set(Type::string())).mark(2),
            path: Path::new().index(Value::string("hello")),
            want: Some(Value::unknown(Type::string()).mark(2)),
            want_err: None,
        },
        Case {
            start: Value::null(Type::set(Type::string())).mark(2),
            path: Path::new().index(Value::string("hello")),
            want: None,
            want_err: Some(r#"at step 0: cannot index a null value"#),
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let got = test.path.apply(&test.start);

        if let Some(want_err) = test.want_err {
            match &got {
                Ok(_) => panic!("case {i}: succeeded, but want error\nwant error: {want_err}"),
                Err(err) => {
                    let got_err_str = err.to_string();
                    assert_eq!(
                        got_err_str, want_err,
                        "case {i}: wrong error\ngot error:  {got_err_str}\nwant error: {want_err}"
                    );
                }
            }
            continue;
        }

        match got {
            Err(err) => panic!("case {i}: failed, but want success\ngot error: {err}"),
            Ok(got) => {
                let want = test.want.as_ref().unwrap();
                assert_eq!(
                    &got, want,
                    "case {i}: wrong result\ngot:  {got:?}\nwant: {want:?}"
                );
            }
        }
    }
}

// upstream: cty/path_test.go TestPathEquals
#[test]
fn path_equals() {
    struct Case {
        a: Path,
        b: Path,
        equal: bool,
        prefix: bool,
    }

    fn steps(steps: impl IntoIterator<Item = PathStep>) -> Path {
        Path::from_steps(steps)
    }

    let tests: Vec<Case> = vec![
        // upstream: A: nil, B: nil
        Case {
            a: Path::new(),
            b: Path::new(),
            equal: true,
            prefix: true,
        },
        // upstream: A: cty.Path{}, B: cty.Path{}
        Case {
            a: Path::new(),
            b: Path::new(),
            equal: true,
            prefix: true,
        },
        // NOTE(port): upstream case `A: cty.Path{nil}, B: cty.Path{cty.GetAttrStep{Name: "attr"}}`
        // pins the behavior of a Go nil `PathStep` interface value inside a
        // path; a nil step has no Rust analogue (`PathStep` is an enum), so the
        // case is deliberately omitted.
        Case {
            a: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::unknown(Type::string())),
                PathStep::GetAttr("attr".into()),
            ]),
            b: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::string("key")),
                PathStep::GetAttr("attr".into()),
            ]),
            equal: false,
            prefix: false,
        },
        Case {
            a: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::list([Value::unknown(Type::string())])),
                PathStep::GetAttr("attr".into()),
            ]),
            b: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::list([Value::string("known")])),
                PathStep::GetAttr("attr".into()),
            ]),
            equal: false,
            prefix: false,
        },
        Case {
            a: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::unknown(Type::string())),
            ]),
            b: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::string("known")),
                PathStep::GetAttr("attr".into()),
            ]),
            equal: false,
            prefix: false,
        },
        Case {
            a: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::string("known")),
            ]),
            b: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::string("known")),
                PathStep::GetAttr("attr".into()),
            ]),
            equal: false,
            prefix: false,
        },
        Case {
            a: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::string("known")),
                PathStep::GetAttr("attr".into()),
            ]),
            b: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::string("known")),
            ]),
            equal: false,
            prefix: true,
        },
        Case {
            a: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::unknown(Type::string())),
            ]),
            b: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::unknown(Type::string())),
            ]),
            equal: true,
            prefix: true,
        },
        Case {
            a: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::number_float(0.0)),
                PathStep::GetAttr("attr".into()),
            ]),
            b: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::number_int(0)),
                PathStep::GetAttr("attr".into()),
            ]),
            equal: true,
            prefix: true,
        },
        Case {
            a: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::number_int(1)),
                PathStep::GetAttr("attr".into()),
            ]),
            b: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::number_int(0)),
                PathStep::GetAttr("attr".into()),
            ]),
            equal: false,
            prefix: false,
        },
        // tests for convenience methods
        Case {
            a: steps([PathStep::GetAttr("attr".into())]),
            b: Path::new().attr("attr"),
            equal: true,
            prefix: true,
        },
        Case {
            a: steps([PathStep::Index(Value::number_int(0))]),
            b: Path::new().index(Value::number_int(0)),
            equal: true,
            prefix: true,
        },
        Case {
            a: steps([PathStep::Index(Value::number_int(0))]),
            b: Path::new().index_int(0),
            equal: true,
            prefix: true,
        },
        Case {
            a: steps([PathStep::Index(Value::string("key"))]),
            b: Path::new().index_string("key"),
            equal: true,
            prefix: true,
        },
        Case {
            a: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::number_int(0)),
            ]),
            b: Path::new().attr("attr").index_int(0),
            equal: true,
            prefix: true,
        },
        Case {
            a: steps([
                PathStep::GetAttr("attr".into()),
                PathStep::Index(Value::string("key")),
            ]),
            b: Path::new().attr("attr").index_string("key"),
            equal: true,
            prefix: true,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(
            test.a.equals(&test.b),
            test.equal,
            "case {i}: {:?}.equals({:?}) != {}",
            test.a,
            test.b,
            test.equal
        );
        assert_eq!(
            test.a.has_prefix(&test.b),
            test.prefix,
            "case {i}: {:?}.has_prefix({:?}) != {}",
            test.a,
            test.b,
            test.prefix
        );
    }
}

// upstream: cty/path_set_test.go TestPathSet
#[test]
fn path_set() {
    // Upstream slices the path value (`helloWorld[:1]`); the analogue here is
    // rebuilding a path from a prefix of the original's steps.
    fn prefix(path: &Path, len: usize) -> Path {
        Path::from_steps(path.steps()[..len].iter().cloned())
    }

    let hello_world = Path::from_steps([
        PathStep::GetAttr("hello".into()),
        PathStep::GetAttr("world".into()),
    ]);
    let mut s = PathSet::new([hello_world.clone()]);

    assert!(
        s.has(&hello_world),
        "set does not have hello.world; should have it"
    );
    assert!(
        !s.has(&prefix(&hello_world, 1)),
        "set has hello; should not have it"
    );

    assert_eq!(
        s.list(),
        vec![hello_world.clone()],
        "wrong list result\ngot:  {:?}\nwant: {:?}",
        s.list(),
        vec![hello_world.clone()]
    );

    let foo_bar_baz = Path::from_steps([
        PathStep::GetAttr("foo".into()),
        PathStep::Index(Value::string("bar")),
        PathStep::GetAttr("baz".into()),
    ]);
    s.add_all_steps(foo_bar_baz.clone());
    assert!(
        s.has(&hello_world),
        "set does not have hello.world; should have it"
    );
    assert!(
        s.has(&foo_bar_baz),
        "set does not have foo['bar'].baz; should have it"
    );
    assert!(
        s.has(&prefix(&foo_bar_baz, 2)),
        "set does not have foo['bar']; should have it"
    );
    assert!(
        s.has(&prefix(&foo_bar_baz, 1)),
        "set does not have foo; should have it"
    );

    s.remove(&prefix(&foo_bar_baz, 2));
    assert!(
        !s.has(&prefix(&foo_bar_baz, 2)),
        "set has foo['bar']; should not have it"
    );
    assert!(
        s.has(&foo_bar_baz),
        "set does not have foo['bar'].baz; should have it"
    );
    assert!(
        s.has(&prefix(&foo_bar_baz, 1)),
        "set does not have foo; should have it"
    );

    let mut new = PathSet::new(s.list());
    assert!(s == new, "new set does not equal original; want equal sets");
    new.remove(&hello_world);
    assert!(s != new, "new set equals original; want non-equal sets");
    new.add(Path::from_steps([
        PathStep::GetAttr("goodbye".into()),
        PathStep::GetAttr("world".into()),
    ]));
    assert!(s != new, "new set equals original; want non-equal sets");
}
