//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/marks_wrangle_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.
//!
//! Upstream's `cty.WrangleFunc` returns `(ctymarks.WrangleAction, error)` with
//! the two results independent (a nil action means "no opinion", and an action
//! may be accompanied by an error that gets accumulated), and
//! `Value.WrangleMarksDeep` returns `(Value, error)` where the value is
//! asserted even when the error is set. The Rust signatures mirror that:
//! wranglers return `(Option<WrangleAction>, Option<Error>)` and
//! `wrangle_marks_deep` returns `(Value, Option<Error>)`.

use cty::{Error, Mark, Path, PathStep, Type, Value, WrangleAction, WrangleFunc};

/// A boxed wrangler callback, so heterogeneous per-case closures can live in
/// one table.
type Wrangler = Box<dyn FnMut(&Mark, &Path) -> (Option<WrangleAction>, Option<Error>)>;

/// Go's `%#v` rendering of a `cty.Path`: `cty.Path(nil)` for the nil (empty)
/// path, otherwise `cty.Path{...}` listing each step's GoString.
fn path_go_string(path: &Path) -> String {
    if path.is_empty() {
        "cty.Path(nil)".to_string()
    } else {
        let steps: Vec<String> = path.steps().iter().map(PathStep::go_string).collect();
        format!("cty.Path{{{}}}", steps.join(", "))
    }
}

/// Mirrors the upstream closures' error construction:
/// `fmt.Errorf("found mark %q at path %#v", mark, path)`.
fn found_mark_err(mark: &Mark, path: &Path) -> Error {
    let mark_str = mark
        .downcast_ref::<String>()
        .map(String::as_str)
        .unwrap_or("<non-string mark>");
    Error::new(format!(
        "found mark \"{mark_str}\" at path {}",
        path_go_string(path)
    ))
}

/// The upstream closures' placeholder error for calls that must not decide the
/// outcome: `fmt.Errorf("this error should not be observed")`.
fn unobserved_err() -> Error {
    Error::new("this error should not be observed")
}

struct Case {
    name: &'static str,
    input: Value,
    funcs: Vec<Wrangler>,
    want: Value,
    want_err: Option<&'static str>,
}

// Ported from TestValueWrangleMarksDeep:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/marks_wrangle_test.go#L11
#[test]
fn value_wrangle_marks_deep() {
    let mut tests: Vec<Case> = vec![
        Case {
            name: "null with no marks nor funcs",
            input: Value::null(Type::dynamic()),
            funcs: vec![],
            want: Value::null(Type::dynamic()),
            want_err: None,
        },
        Case {
            name: "null with no marks and unused func",
            input: Value::null(Type::dynamic()),
            funcs: vec![Box::new(|_mark, _path| (None, Some(unobserved_err())))],
            want: Value::null(Type::dynamic()),
            want_err: None,
        },
        Case {
            name: "null with mark but no funcs",
            input: Value::null(Type::dynamic()).mark("irrelevant"),
            funcs: vec![],
            want: Value::null(Type::dynamic()).mark("irrelevant"),
            want_err: None,
        },
        Case {
            name: "null with mark that is unaffected by func",
            input: Value::null(Type::dynamic()).mark("irrelevant"),
            funcs: vec![Box::new(|mark, path| {
                if *mark != Mark::from("irrelevant") || !path.is_empty() {
                    return (None, Some(unobserved_err()));
                }
                (None, None)
            })],
            want: Value::null(Type::dynamic()).mark("irrelevant"),
            want_err: None,
        },
        Case {
            name: "null with mark and func that's blocked by earlier func",
            input: Value::null(Type::dynamic()).mark("maybe bad"),
            funcs: vec![
                Box::new(|_mark, _path| (Some(WrangleAction::Keep), None)),
                Box::new(|_mark, _path| (None, Some(unobserved_err()))),
            ],
            want: Value::null(Type::dynamic()).mark("maybe bad"),
            want_err: None,
        },
        Case {
            name: "null with mark and func that's not blocked by earlier func",
            input: Value::null(Type::dynamic()).mark("maybe bad"),
            funcs: vec![
                Box::new(|_mark, _path| (None, None)),
                Box::new(|mark, path| {
                    (Some(WrangleAction::Drop), Some(found_mark_err(mark, path)))
                }),
            ],
            want: Value::null(Type::dynamic()),
            want_err: Some(r#"found mark "maybe bad" at path cty.Path(nil)"#),
        },
        Case {
            name: "null with marks, one of which is dropped",
            input: Value::null(Type::dynamic()).mark("keep").mark("drop"),
            funcs: vec![Box::new(|mark, _path| {
                if *mark == Mark::from("drop") {
                    return (Some(WrangleAction::Drop), None);
                }
                (None, None)
            })],
            want: Value::null(Type::dynamic()).mark("keep"),
            want_err: None,
        },
        Case {
            name: "null with marks, one of which is replaced",
            input: Value::null(Type::dynamic()).mark("keep").mark("drop"),
            funcs: vec![Box::new(|mark, _path| {
                if *mark == Mark::from("drop") {
                    return (
                        Some(WrangleAction::Replace(Mark::from("replacement"))),
                        None,
                    );
                }
                (None, None)
            })],
            want: Value::null(Type::dynamic())
                .mark("keep")
                .mark("replacement"),
            want_err: None,
        },
        Case {
            name: "null with a mark that causes an error",
            input: Value::null(Type::dynamic()).mark("bad").mark("irrelevant"),
            funcs: vec![Box::new(|mark, path| {
                if *mark == Mark::from("bad") {
                    return (Some(WrangleAction::Drop), Some(found_mark_err(mark, path)));
                }
                (None, None)
            })],
            want: Value::null(Type::dynamic()).mark("irrelevant"),
            want_err: Some(r#"found mark "bad" at path cty.Path(nil)"#),
        },
        // Sets are not really any different than primitive values for the
        // sake of this function, because they can't contain any nested values
        // that are individually marked. This single test is therefore here
        // just to check that we don't do anything weird with a set.
        Case {
            name: "set with marks, one of which is dropped",
            input: Value::set([Value::bool(true)]).mark("drop").mark("keep"),
            funcs: vec![Box::new(|mark, _path| {
                if *mark == Mark::from("drop") {
                    return (Some(WrangleAction::Drop), None);
                }
                (None, None)
            })],
            want: Value::set([Value::bool(true)]).mark("keep"),
            want_err: None,
        },
        Case {
            name: "list with no marks and inert wrangle func",
            input: Value::list([
                Value::string("unmarked 1"),
                Value::string("unmarked 2"),
                Value::string("unmarked 3"),
            ]),
            funcs: vec![Box::new(|_mark, _path| (None, Some(unobserved_err())))],
            want: Value::list([
                Value::string("unmarked 1"),
                Value::string("unmarked 2"),
                Value::string("unmarked 3"),
            ]),
            want_err: None,
        },
        Case {
            name: "list with nested marks, one of which is dropped",
            input: Value::list([
                Value::string("unmarked"),
                Value::string("marked 1").mark("drop"),
                Value::string("marked 2").mark("drop").mark("keep"),
                Value::string("marked 3").mark("keep"),
            ]),
            funcs: vec![Box::new(|mark, _path| {
                if *mark == Mark::from("drop") {
                    return (Some(WrangleAction::Drop), None);
                }
                (None, None)
            })],
            want: Value::list([
                Value::string("unmarked"),
                Value::string("marked 1"),
                Value::string("marked 2").mark("keep"),
                Value::string("marked 3").mark("keep"),
            ]),
            want_err: None,
        },
        Case {
            name: "tuple with nested marks, one of which is dropped",
            input: Value::tuple([
                Value::bool(true),
                Value::string("marked 1").mark("drop"),
                Value::string("marked 2").mark("drop").mark("keep"),
                Value::string("marked 3").mark("keep"),
            ]),
            funcs: vec![Box::new(|mark, _path| {
                if *mark == Mark::from("drop") {
                    return (Some(WrangleAction::Drop), None);
                }
                (None, None)
            })],
            want: Value::tuple([
                Value::bool(true),
                Value::string("marked 1"),
                Value::string("marked 2").mark("keep"),
                Value::string("marked 3").mark("keep"),
            ]),
            want_err: None,
        },
        Case {
            name: "list with nested marks, one of which is expanded",
            input: Value::list([
                Value::string("unmarked"),
                Value::string("marked 1").mark("expand"),
                Value::string("marked 2").mark("expand").mark("keep"),
                Value::string("marked 3").mark("keep"),
            ]),
            funcs: vec![Box::new(|mark, _path| {
                if *mark == Mark::from("expand") {
                    return (Some(WrangleAction::Expand), None);
                }
                (None, None)
            })],
            want: Value::list([
                Value::string("unmarked"),
                Value::string("marked 1").mark("expand"),
                Value::string("marked 2").mark("expand").mark("keep"),
                Value::string("marked 3").mark("keep"),
            ])
            .mark("expand"),
            want_err: None,
        },
        Case {
            name: "list with nested mark that causes error",
            input: Value::list([
                Value::string("unmarked 1"),
                Value::string("marked").mark("bad"),
                Value::string("unmarked 2"),
            ]),
            funcs: vec![Box::new(|mark, path| {
                if *mark == Mark::from("bad") {
                    return (None, Some(found_mark_err(mark, path)));
                }
                (None, None)
            })],
            want: Value::list([
                Value::string("unmarked 1"),
                Value::string("marked").mark("bad"),
                Value::string("unmarked 2"),
            ]),
            want_err: Some(
                r#"found mark "bad" at path cty.Path{cty.IndexStep{Key:cty.NumberIntVal(1)}}"#,
            ),
        },
        Case {
            name: "list with nested marks that cause error",
            input: Value::list([
                Value::string("unmarked 1"),
                Value::string("marked 1").mark("bad"),
                Value::string("marked 2").mark("bad"),
                Value::string("unmarked 2"),
            ]),
            funcs: vec![Box::new(|mark, path| {
                if *mark == Mark::from("bad") {
                    return (None, Some(found_mark_err(mark, path)));
                }
                (None, None)
            })],
            want: Value::list([
                Value::string("unmarked 1"),
                Value::string("marked 1").mark("bad"),
                Value::string("marked 2").mark("bad"),
                Value::string("unmarked 2"),
            ]),
            want_err: Some(
                "found mark \"bad\" at path cty.Path{cty.IndexStep{Key:cty.NumberIntVal(1)}}\nfound mark \"bad\" at path cty.Path{cty.IndexStep{Key:cty.NumberIntVal(2)}}",
            ),
        },
        Case {
            name: "object with no marks and inert wrangle func",
            input: Value::object([
                ("name", Value::string("Bob")),
                ("age", Value::number_int(84)),
                (
                    "friends",
                    Value::list([Value::string("Harpreet"), Value::string("Amanda")]),
                ),
            ]),
            funcs: vec![Box::new(|_mark, _path| (None, Some(unobserved_err())))],
            want: Value::object([
                ("name", Value::string("Bob")),
                ("age", Value::number_int(84)),
                (
                    "friends",
                    Value::list([Value::string("Harpreet"), Value::string("Amanda")]),
                ),
            ]),
            want_err: None,
        },
        Case {
            name: "object with marks, one of which is dropped",
            input: Value::object([
                ("name", Value::string("Bob")),
                ("age", Value::number_int(84).mark("drop").mark("keep")),
                (
                    "friends",
                    Value::list([
                        Value::string("Harpreet").mark("drop"),
                        Value::string("Amanda").mark("keep"),
                    ]),
                ),
            ]),
            funcs: vec![Box::new(|mark, _path| {
                if *mark == Mark::from("drop") {
                    return (Some(WrangleAction::Drop), None);
                }
                (None, None)
            })],
            want: Value::object([
                ("name", Value::string("Bob")),
                ("age", Value::number_int(84).mark("keep")),
                (
                    "friends",
                    Value::list([
                        Value::string("Harpreet"),
                        Value::string("Amanda").mark("keep"),
                    ]),
                ),
            ]),
            want_err: None,
        },
        Case {
            name: "object with marks, one of which is expanded",
            input: Value::object([
                ("name", Value::string("Bob")),
                ("age", Value::number_int(84).mark("keep")),
                (
                    "friends",
                    Value::list([
                        Value::string("Harpreet").mark("expand"),
                        Value::string("Amanda").mark("keep"),
                    ]),
                ),
            ])
            .mark("keep"),
            funcs: vec![Box::new(|mark, _path| {
                if *mark == Mark::from("expand") {
                    return (Some(WrangleAction::Expand), None);
                }
                (None, None)
            })],
            want: Value::object([
                ("name", Value::string("Bob")),
                ("age", Value::number_int(84).mark("keep")),
                (
                    "friends",
                    Value::list([
                        Value::string("Harpreet").mark("expand"),
                        Value::string("Amanda").mark("keep"),
                    ]),
                ),
            ])
            .mark("keep")
            .mark("expand"),
            want_err: None,
        },
        Case {
            name: "map with no marks and inert wrangle func",
            input: Value::map([
                ("foo", Value::string("bar")),
                ("baz", Value::string("beep")),
            ]),
            funcs: vec![Box::new(|_mark, _path| (None, Some(unobserved_err())))],
            want: Value::map([
                ("foo", Value::string("bar")),
                ("baz", Value::string("beep")),
            ]),
            want_err: None,
        },
        Case {
            name: "map with marks, one of which is dropped",
            input: Value::map([
                ("unmarked 1", Value::string("unmarked")),
                ("marked 1", Value::string("marked").mark("keep")),
                (
                    "marked 2",
                    Value::string("marked").mark("keep").mark("drop"),
                ),
                ("marked 3", Value::string("marked").mark("drop")),
                ("unmarked 2", Value::string("unmarked")),
            ]),
            funcs: vec![Box::new(|mark, _path| {
                if *mark == Mark::from("drop") {
                    return (Some(WrangleAction::Drop), None);
                }
                (None, None)
            })],
            want: Value::map([
                ("unmarked 1", Value::string("unmarked")),
                ("marked 1", Value::string("marked").mark("keep")),
                ("marked 2", Value::string("marked").mark("keep")),
                ("marked 3", Value::string("marked")),
                ("unmarked 2", Value::string("unmarked")),
            ]),
            want_err: None,
        },
    ];

    for (i, case) in tests.iter_mut().enumerate() {
        let name = case.name;
        let mut wranglers: Vec<WrangleFunc<'_>> =
            case.funcs.iter_mut().map(|f| &mut **f as _).collect();
        let (got, got_err) = case.input.wrangle_marks_deep(&mut wranglers);
        match (&case.want_err, &got_err) {
            (Some(want_err), None) => {
                panic!("case {i} ({name}): unexpected success\nwant error: {want_err}");
            }
            (Some(want_err), Some(got_err)) => {
                assert_eq!(
                    got_err.to_string(),
                    *want_err,
                    "case {i} ({name}): wrong error"
                );
            }
            (None, Some(got_err)) => {
                panic!("case {i} ({name}): unexpected error: {got_err}");
            }
            (None, None) => {}
        }
        assert_eq!(got, case.want, "case {i} ({name}): wrong result");
    }
}
