//! Value marks: opaque annotations that propagate through operations.
//!
//! Mirrors go-cty's `marks.go`, `marks_wrangle.go`, and the `ctymarks` helper
//! package. In Go a mark is any comparable value (`any`); in Rust a mark is a
//! [`Mark`], constructed from any hashable, comparable value via [`Mark::of`]
//! or the `From` conversions for common cases like `&str`.

use std::any::Any;
use std::fmt::Debug;
use std::hash::Hash;

use crate::error::CtyError;
use crate::path::Path;
use crate::value::Value;

/// A single mark: a type-erased, comparable, hashable annotation value.
///
/// Two marks are equal when they wrap values of the same type that compare
/// equal, mirroring Go's interface equality.
#[derive(Debug, Clone)]
pub struct Mark {
    _priv: (),
}

impl Mark {
    /// Wraps an arbitrary native value as a mark.
    pub fn of<T>(value: T) -> Mark
    where
        T: Any + Eq + Hash + Debug + Clone + Send + Sync,
    {
        let _ = value;
        todo!()
    }

    /// Downcasts to the wrapped native value, if it has type `T`.
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        todo!()
    }
}

impl PartialEq for Mark {
    fn eq(&self, other: &Self) -> bool {
        let _ = other;
        todo!()
    }
}

impl Eq for Mark {}

impl Hash for Mark {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
        todo!()
    }
}

impl From<&str> for Mark {
    fn from(v: &str) -> Mark {
        Mark::of(v.to_string())
    }
}

impl From<String> for Mark {
    fn from(v: String) -> Mark {
        Mark::of(v)
    }
}

impl From<i64> for Mark {
    fn from(v: i64) -> Mark {
        Mark::of(v)
    }
}

impl From<bool> for Mark {
    fn from(v: bool) -> Mark {
        Mark::of(v)
    }
}

/// A set of marks associated with a value (go-cty: `cty.ValueMarks`).
#[derive(Debug, Clone)]
pub struct ValueMarks {
    _priv: (),
}

impl Default for ValueMarks {
    fn default() -> ValueMarks {
        ValueMarks::new()
    }
}

impl ValueMarks {
    /// The empty mark set.
    pub fn new() -> ValueMarks {
        todo!()
    }

    /// A mark set containing the given marks (go-cty: `cty.NewValueMarks`).
    pub fn from_marks<M: Into<Mark>>(marks: impl IntoIterator<Item = M>) -> ValueMarks {
        let _ = marks.into_iter().map(Into::into).collect::<Vec<_>>();
        todo!()
    }

    /// Whether the set contains the given mark (go-cty: `ValueMarks.Has`).
    pub fn has(&self, mark: impl Into<Mark>) -> bool {
        let _ = mark.into();
        todo!()
    }

    /// Adds all of the given marks to the set (go-cty: `ValueMarks.Insert`).
    pub fn insert<M: Into<Mark>>(&mut self, marks: impl IntoIterator<Item = M>) {
        let _ = marks.into_iter().map(Into::into).collect::<Vec<_>>();
        todo!()
    }

    /// The number of marks in the set.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Whether the set is empty.
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Iterates over the marks in the set, in unspecified order.
    pub fn iter(&self) -> impl Iterator<Item = &Mark> {
        std::iter::empty::<&Mark>()
    }

    /// The Go-syntax representation, identical to go-cty's `ValueMarks.GoString`,
    /// e.g. `cty.NewValueMarks("a")`.
    pub fn go_string(&self) -> String {
        todo!()
    }
}

/// Equality of mark sets (go-cty: `ValueMarks.Equal`).
impl PartialEq for ValueMarks {
    fn eq(&self, other: &Self) -> bool {
        let _ = other;
        todo!()
    }
}

impl Eq for ValueMarks {}

/// A set of marks associated with a path inside a value, used to attach and
/// recover marks on nested values (go-cty: `cty.PathValueMarks`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathValueMarks {
    /// The path at which the marks apply.
    pub path: Path,
    /// The marks applied at that path.
    pub marks: ValueMarks,
}

/// The action a [`WrangleFunc`] tells [`Value::wrangle_marks_deep`] to take for
/// one mark (go-cty: `ctymarks.WrangleAction`).
#[derive(Debug, Clone)]
pub enum WrangleAction {
    /// Keep the mark as-is (go-cty: `ctymarks.WrangleKeep`).
    Keep,
    /// Discard the mark (go-cty: `ctymarks.WrangleDrop`).
    Drop,
    /// Move the mark from a collection onto each of its elements
    /// (go-cty: `ctymarks.WrangleExpand`).
    Expand,
    /// Replace the mark with another mark (go-cty: `ctymarks.WrangleReplace`).
    Replace(Mark),
}

/// A callback deciding what to do with each mark encountered by
/// [`Value::wrangle_marks_deep`] (go-cty: `cty.WrangleFunc`).
///
/// Mirrors Go's `func(mark any, path cty.Path) (ctymarks.WrangleAction, error)`,
/// whose two results are independent: a `None` action means "no opinion, let
/// the next wrangler decide" (Go's nil action), and an error may accompany any
/// action — the action is still applied and the error is accumulated.
pub type WrangleFunc<'a> =
    &'a mut dyn FnMut(&Mark, &Path) -> (Option<WrangleAction>, Option<CtyError>);

impl Value {
    /// Whether this value is directly marked (go-cty: `Value.IsMarked`).
    pub fn is_marked(&self) -> bool {
        todo!()
    }

    /// Whether this value or any nested value is marked
    /// (go-cty: `Value.ContainsMarked`).
    pub fn contains_marked(&self) -> bool {
        todo!()
    }

    /// Whether this value is directly marked with the given mark
    /// (go-cty: `Value.HasMark`).
    pub fn has_mark(&self, mark: impl Into<Mark>) -> bool {
        let _ = mark.into();
        todo!()
    }

    /// Whether this value or any nested value carries the given mark
    /// (go-cty: `Value.HasMarkDeep`).
    pub fn has_mark_deep(&self, mark: impl Into<Mark>) -> bool {
        let _ = mark.into();
        todo!()
    }

    /// Whether this value has exactly the same direct marks as `other`
    /// (go-cty: `Value.HasSameMarks`).
    pub fn has_same_marks(&self, other: &Value) -> bool {
        let _ = other;
        todo!()
    }

    /// A copy of this value with the given mark added (go-cty: `Value.Mark`).
    pub fn mark(&self, mark: impl Into<Mark>) -> Value {
        let _ = mark.into();
        todo!()
    }

    /// The direct marks of this value (go-cty: `Value.Marks`).
    pub fn marks(&self) -> ValueMarks {
        todo!()
    }

    /// A copy with marks applied at the given paths within the value
    /// (go-cty: `Value.MarkWithPaths`).
    pub fn mark_with_paths(&self, path_marks: &[PathValueMarks]) -> Value {
        let _ = path_marks;
        todo!()
    }

    /// A copy with all of the given mark sets added (go-cty: `Value.WithMarks`).
    pub fn with_marks(&self, marks: impl IntoIterator<Item = ValueMarks>) -> Value {
        let _ = marks.into_iter().collect::<Vec<_>>();
        todo!()
    }

    /// A copy carrying the same direct marks as all of the given source values
    /// (go-cty: `Value.WithSameMarks`).
    pub fn with_same_marks(&self, sources: &[&Value]) -> Value {
        let _ = sources;
        todo!()
    }

    /// Removes and returns this value's direct marks (go-cty: `Value.Unmark`).
    pub fn unmark(&self) -> (Value, ValueMarks) {
        todo!()
    }

    /// Removes marks from this value and all nested values, returning the
    /// union of all removed marks (go-cty: `Value.UnmarkDeep`).
    pub fn unmark_deep(&self) -> (Value, ValueMarks) {
        todo!()
    }

    /// Removes marks from this value and all nested values, returning them
    /// with the paths they were found at (go-cty: `Value.UnmarkDeepWithPaths`).
    pub fn unmark_deep_with_paths(&self) -> (Value, Vec<PathValueMarks>) {
        todo!()
    }

    /// The direct marks of this value that wrap a native value of type `T`
    /// (go-cty: `cty.ValueMarksOfType[T]`).
    pub fn marks_of_type<T: Any + Clone>(&self) -> Vec<T> {
        todo!()
    }

    /// The marks of this value and all nested values that wrap a native value
    /// of type `T` (go-cty: `cty.ValueMarksOfTypeDeep[T]`).
    pub fn marks_of_type_deep<T: Any + Clone>(&self) -> Vec<T> {
        todo!()
    }

    /// Rewrites the marks of this value and all nested values by applying the
    /// given wrangler callbacks in turn (go-cty: `Value.WrangleMarksDeep`).
    ///
    /// Mirrors Go's `(Value, error)` result pair: the returned value is
    /// meaningful even when errors were accumulated (wrangling continues past
    /// errors), so this returns `(Value, Option<CtyError>)` rather than a
    /// `Result`. Multiple accumulated errors are joined into one `Error`
    /// whose message is their newline-joined messages (Go's `errors.Join`).
    pub fn wrangle_marks_deep(
        &self,
        wranglers: &mut [WrangleFunc<'_>],
    ) -> (Value, Option<CtyError>) {
        let _ = wranglers;
        todo!()
    }
}

#[cfg(test)]
mod conformance {
    //! Conformance suites transcribed from the go-cty test tables for this
    //! module, one file per upstream test file.

    mod marks {
        //! Conformance tests transcribed from go-cty
        //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
        //!   cty/marks_test.go
        //!
        //! Expected values are literals from the upstream tables; see
        //! docs/api-mapping.md for the Go→Rust API correspondence.

        use crate::{Path, PathStep, PathValueMarks, Type, Value, ValueMarks};

        // Ported from TestContainsMarked:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/marks_test.go#L11
        #[test]
        #[ignore = "not yet implemented"]
        fn contains_marked() {
            let test_cases: Vec<(Value, bool)> = vec![
                (Value::string("a"), false),
                (Value::number(1).mark("a"), true),
                (Value::list([Value::number(1), Value::number(2)]), false),
                (
                    Value::list([Value::number(1), Value::number(2).mark("a")]),
                    true,
                ),
                (
                    Value::list([Value::number(1), Value::number(2)]).mark("a"),
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
                    Value::tuple([Value::number(1).mark("a"), Value::string("y").mark("z")]),
                    true,
                ),
                (
                    Value::set([Value::number(1).mark("a"), Value::number(2).mark("z")]),
                    true,
                ),
                (
                    Value::object([
                        (
                            "x",
                            Value::list([Value::number(1).mark("a"), Value::number(2)]),
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
                (Value::number(1).mark("a"), true),
                (Value::list([Value::number(1), Value::number(2)]), false),
                (
                    Value::list([Value::number(1), Value::number(2).mark("a")]),
                    false,
                ),
                (
                    Value::list([Value::number(1), Value::number(2)]).mark("a"),
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
            let a = Value::number(2).mark("a");
            let b = Value::number(5).mark("b");
            let c = Value::number(1).mark("c");
            let d = Value::number(12).mark("d");
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
                path: Path::from_steps([PathStep::Index(Value::number(0))]),
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
                        path: Path::from_steps([PathStep::Index(Value::number(0))]),
                        marks: ValueMarks::from_marks(["a"]),
                    },
                    PathValueMarks {
                        path: Path::from_steps([PathStep::Index(Value::number(0))]),
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
                        path: Path::from_steps([PathStep::Index(Value::number(0))]),
                        marks: ValueMarks::from_marks(["a"]),
                    },
                    PathValueMarks {
                        path: Path::from_steps([PathStep::Index(Value::number(1))]),
                        marks: ValueMarks::from_marks(["a"]),
                    },
                    false,
                ),
                (
                    PathValueMarks {
                        path: Path::from_steps([PathStep::Index(Value::number(0))]),
                        marks: ValueMarks::from_marks(["a"]),
                    },
                    PathValueMarks {
                        path: Path::from_steps([PathStep::Index(Value::number(0))]),
                        marks: ValueMarks::from_marks(["b"]),
                    },
                    false,
                ),
                (
                    PathValueMarks {
                        path: Path::from_steps([PathStep::Index(Value::number(0))]),
                        marks: ValueMarks::from_marks(["a"]),
                    },
                    PathValueMarks {
                        path: Path::from_steps([PathStep::Index(Value::number(1))]),
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
                    Value::number(1).mark("a"),
                    Value::number(1),
                    ValueMarks::from_marks(["a"]),
                ),
                (
                    "unmarked list",
                    Value::list([Value::number(1), Value::number(2)]),
                    Value::list([Value::number(1), Value::number(2)]),
                    ValueMarks::new(),
                ),
                (
                    "list with some elements marked",
                    Value::list([Value::number(1).mark("a"), Value::number(2)]),
                    Value::list([Value::number(1), Value::number(2)]),
                    ValueMarks::from_marks(["a"]),
                ),
                (
                    "marked list with all elements marked",
                    Value::list([Value::number(1).mark("a"), Value::number(2).mark("b")]).mark("c"),
                    Value::list([Value::number(1), Value::number(2)]),
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
                    Value::tuple([Value::number(1).mark("a"), Value::string("y").mark("z")]),
                    Value::tuple([Value::number(1), Value::string("y")]),
                    ValueMarks::from_marks(["a", "z"]),
                ),
                (
                    "set with elements marked",
                    Value::set([Value::number(1).mark("a"), Value::number(2).mark("z")]),
                    Value::set([Value::number(1), Value::number(2)]),
                    ValueMarks::from_marks(["a", "z"]),
                ),
                (
                    "complex marked object with lots of marks",
                    Value::object([
                        (
                            "x",
                            Value::list([Value::number(3).mark("a"), Value::number(5).mark("b")])
                                .with_marks([ValueMarks::from_marks(["c", "d"])]),
                        ),
                        ("y", Value::string("y").mark("e")),
                        ("z", Value::bool(true).mark("f")),
                    ])
                    .mark("g"),
                    Value::object([
                        ("x", Value::list([Value::number(3), Value::number(5)])),
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
                    Value::number(1).mark("a"),
                    Value::number(1),
                    vec![PathValueMarks {
                        path: Path::new(),
                        marks: ValueMarks::from_marks(["a"]),
                    }],
                ),
                (
                    "list with some elements marked",
                    Value::list([Value::number(1).mark("a"), Value::number(2)]),
                    Value::list([Value::number(1), Value::number(2)]),
                    vec![PathValueMarks {
                        path: Path::new().index_int(0),
                        marks: ValueMarks::from_marks(["a"]),
                    }],
                ),
                (
                    "marked list with all elements marked",
                    Value::list([Value::number(1).mark("a"), Value::number(2).mark("b")]).mark("c"),
                    Value::list([Value::number(1), Value::number(2)]),
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
                        Value::number(1).mark("a"),
                        Value::string("y").mark("z"),
                        Value::object([("x", Value::bool(true))]).mark("o"),
                    ]),
                    Value::tuple([
                        Value::number(1),
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
                    Value::set([Value::number(1).mark("a"), Value::number(2).mark("z")]),
                    Value::set([Value::number(1), Value::number(2)]),
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
                            Value::list([Value::number(3).mark("a"), Value::number(5).mark("b")])
                                .with_marks([ValueMarks::from_marks(["c", "d"])]),
                        ),
                        ("y", Value::string("y").mark("e")),
                        ("z", Value::bool(true).mark("f")),
                    ])
                    .mark("g"),
                    Value::object([
                        ("x", Value::list([Value::number(3), Value::number(5)])),
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
    }

    mod wrangle {
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
        //! wranglers return `(Option<WrangleAction>, Option<CtyCtyError>)` and
        //! `wrangle_marks_deep` returns `(Value, Option<CtyCtyError>)`.

        use crate::{CtyError, Mark, Path, PathStep, Type, Value, WrangleAction, WrangleFunc};

        /// A boxed wrangler callback, so heterogeneous per-case closures can live in
        /// one table.
        type Wrangler = Box<dyn FnMut(&Mark, &Path) -> (Option<WrangleAction>, Option<CtyError>)>;

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
        fn found_mark_err(mark: &Mark, path: &Path) -> CtyError {
            let mark_str = mark
                .downcast_ref::<String>()
                .map(String::as_str)
                .unwrap_or("<non-string mark>");
            CtyError::new(format!(
                "found mark \"{mark_str}\" at path {}",
                path_go_string(path)
            ))
        }

        /// The upstream closures' placeholder error for calls that must not decide the
        /// outcome: `fmt.Errorf("this error should not be observed")`.
        fn unobserved_err() -> CtyError {
            CtyError::new("this error should not be observed")
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
        #[ignore = "not yet implemented"]
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
                        ("age", Value::number(84)),
                        (
                            "friends",
                            Value::list([Value::string("Harpreet"), Value::string("Amanda")]),
                        ),
                    ]),
                    funcs: vec![Box::new(|_mark, _path| (None, Some(unobserved_err())))],
                    want: Value::object([
                        ("name", Value::string("Bob")),
                        ("age", Value::number(84)),
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
                        ("age", Value::number(84).mark("drop").mark("keep")),
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
                        ("age", Value::number(84).mark("keep")),
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
                        ("age", Value::number(84).mark("keep")),
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
                        ("age", Value::number(84).mark("keep")),
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
    }
}
