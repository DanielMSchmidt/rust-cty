//! Paths through nested value structures: [`Path`], [`PathStep`], [`PathSet`].
//!
//! Mirrors go-cty's `path.go` and `path_set.go`. Where go-cty builds paths with
//! free functions (`cty.GetAttrPath("a").Index(...)`), the Rust API starts from
//! `Path::new()` and chains: `Path::new().attr("a").index_int(0)`.

use crate::error::CtyError;
use crate::value::Value;

/// One step in a [`Path`] (go-cty: `cty.PathStep`, i.e. `GetAttrStep` or
/// `IndexStep`).
#[derive(Debug, Clone)]
pub enum PathStep {
    /// Accessing an attribute of an object value (go-cty: `cty.GetAttrStep`).
    GetAttr(String),
    /// Indexing into a list, map, or set value (go-cty: `cty.IndexStep`).
    Index(Value),
}

impl PathStep {
    /// Applies this single step to a value (go-cty: `PathStep.Apply`).
    pub fn apply(&self, value: &Value) -> Result<Value, CtyError> {
        let _ = value;
        todo!()
    }

    /// The Go-syntax representation, identical to go-cty's `GoString` for the
    /// step, e.g. `cty.GetAttrStep{Name:"foo"}`.
    pub fn go_string(&self) -> String {
        todo!()
    }
}

impl PartialEq for PathStep {
    fn eq(&self, other: &Self) -> bool {
        let _ = other;
        todo!()
    }
}

impl Eq for PathStep {}

/// A sequence of [`PathStep`]s describing a route to a nested value
/// (go-cty: `cty.Path`).
#[derive(Debug, Clone)]
pub struct Path {
    _priv: (),
}

/// Step-wise path equality (go-cty: `Path.Equals`).
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        let _ = other;
        todo!()
    }
}

impl Eq for Path {}

impl Default for Path {
    fn default() -> Path {
        Path::new()
    }
}

impl Path {
    /// The empty path, addressing the root value itself (go-cty: `cty.Path{}`).
    pub fn new() -> Path {
        todo!()
    }

    /// A path built from explicit steps.
    pub fn from_steps(steps: impl IntoIterator<Item = PathStep>) -> Path {
        let _ = steps.into_iter().collect::<Vec<_>>();
        todo!()
    }

    /// The steps of this path, in order.
    pub fn steps(&self) -> &[PathStep] {
        todo!()
    }

    /// Whether this is the empty path.
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// The number of steps.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Returns a new path with a get-attribute step appended
    /// (go-cty: `Path.GetAttr` / `cty.GetAttrPath`).
    pub fn attr(&self, name: impl Into<String>) -> Path {
        let _ = name.into();
        todo!()
    }

    /// Returns a new path with an index step appended
    /// (go-cty: `Path.Index` / `cty.IndexPath`).
    pub fn index(&self, key: Value) -> Path {
        let _ = key;
        todo!()
    }

    /// Shorthand for [`Path::index`] with a number key
    /// (go-cty: `Path.IndexInt` / `cty.IndexIntPath`).
    pub fn index_int(&self, key: i64) -> Path {
        let _ = key;
        todo!()
    }

    /// Shorthand for [`Path::index`] with a string key
    /// (go-cty: `Path.IndexString` / `cty.IndexStringPath`).
    pub fn index_string(&self, key: impl Into<String>) -> Path {
        let _ = key.into();
        todo!()
    }

    /// Follows the path down into the given value (go-cty: `Path.Apply`).
    pub fn apply(&self, value: &Value) -> Result<Value, CtyError> {
        let _ = value;
        todo!()
    }

    /// Applies all but the final step, returning the penultimate value and the
    /// final step (go-cty: `Path.LastStep`).
    pub fn last_step(&self, value: &Value) -> Result<(Value, PathStep), CtyError> {
        let _ = value;
        todo!()
    }

    /// Whether this path equals another (go-cty: `Path.Equals`).
    /// Also available via `==` through `PartialEq`.
    pub fn equals(&self, other: &Path) -> bool {
        self == other
    }

    /// Whether this path starts with all the steps of `prefix`
    /// (go-cty: `Path.HasPrefix`).
    pub fn has_prefix(&self, prefix: &Path) -> bool {
        let _ = prefix;
        todo!()
    }

    /// Creates an error carrying this path as context (go-cty: `Path.NewErrorf`).
    pub fn error(&self, message: impl Into<String>) -> CtyError {
        CtyError::new_at_path(self.clone(), message)
    }

    /// The Go-syntax representation of this path, byte-for-byte identical to
    /// go-cty's `GoString` rendering, e.g.
    /// `cty.Path{cty.GetAttrStep{Name:"foo"}}` (`cty.Path(nil)` when empty).
    pub fn go_string(&self) -> String {
        todo!()
    }
}

/// Renders the path as the Rust expression that constructs it, e.g.
/// `Path::new().attr("foo")` — the Rust analogue of [`Path::go_string`].
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        todo!()
    }
}

/// A mutable set of [`Path`]s (go-cty: `cty.PathSet`).
#[derive(Debug, Clone)]
pub struct PathSet {
    _priv: (),
}

impl Default for PathSet {
    fn default() -> PathSet {
        PathSet::new([])
    }
}

impl PathSet {
    /// A new path set containing the given paths (go-cty: `cty.NewPathSet`).
    pub fn new(paths: impl IntoIterator<Item = Path>) -> PathSet {
        let _ = paths.into_iter().collect::<Vec<_>>();
        todo!()
    }

    /// Adds a path to the set (go-cty: `PathSet.Add`).
    pub fn add(&mut self, path: Path) {
        let _ = path;
        todo!()
    }

    /// Adds the path along with every one of its ancestor prefixes
    /// (go-cty: `PathSet.AddAllSteps`).
    pub fn add_all_steps(&mut self, path: Path) {
        let _ = path;
        todo!()
    }

    /// Whether the set contains the given path (go-cty: `PathSet.Has`).
    pub fn has(&self, path: &Path) -> bool {
        let _ = path;
        todo!()
    }

    /// Removes a path from the set (go-cty: `PathSet.Remove`).
    pub fn remove(&mut self, path: &Path) {
        let _ = path;
        todo!()
    }

    /// All paths in the set, in an unspecified but consistent order
    /// (go-cty: `PathSet.List`).
    pub fn list(&self) -> Vec<Path> {
        todo!()
    }

    /// Whether the set is empty (go-cty: `PathSet.Empty`).
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// The union of this set and another (go-cty: `PathSet.Union`).
    pub fn union(&self, other: &PathSet) -> PathSet {
        let _ = other;
        todo!()
    }

    /// The intersection of this set and another (go-cty: `PathSet.Intersection`).
    pub fn intersection(&self, other: &PathSet) -> PathSet {
        let _ = other;
        todo!()
    }

    /// The paths in this set that are not in the other
    /// (go-cty: `PathSet.Subtract`).
    pub fn subtract(&self, other: &PathSet) -> PathSet {
        let _ = other;
        todo!()
    }

    /// The paths in exactly one of the two sets
    /// (go-cty: `PathSet.SymmetricDifference`).
    pub fn symmetric_difference(&self, other: &PathSet) -> PathSet {
        let _ = other;
        todo!()
    }
}

/// Set equality (go-cty: `PathSet.Equal`).
impl PartialEq for PathSet {
    fn eq(&self, other: &Self) -> bool {
        let _ = other;
        todo!()
    }
}

impl Eq for PathSet {}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/path_test.go
    //!   cty/path_set_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::{Path, PathSet, PathStep, Type, Value};

    // Ported from TestPathApply:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/path_test.go#L10
    //
    // Upstream TestPathApply is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod path_apply {
        use super::*;

        // NOTE(port): upstream's `Want` field is `cty.NilVal` in every case where
        // `WantErr` is set; `Path.Apply` returns `Result` here, so those cases
        // carry `None` for the want value rather than a nil placeholder.
        struct Case {
            start: Value,
            path: Path,
            want: Option<Value>,
            want_err: Option<&'static str>,
        }

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[Case]) {
            for (i, test) in tests.iter().enumerate() {
                let got = test.path.apply(&test.start);

                if let Some(want_err) = test.want_err {
                    match &got {
                        Ok(_) => {
                            panic!("case {i}: succeeded, but want error\nwant error: {want_err}")
                        }
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

        // TestPathApply, primitives:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/path_test.go#L10
        #[test]
        #[ignore = "not yet implemented"]
        fn primitives() {
            let tests: Vec<Case> = vec![
                Case {
                    start: Value::string("hello"),
                    path: Path::new(),
                    want: Some(Value::string("hello")),
                    want_err: None,
                },
                Case {
                    start: Value::list_empty(Type::string()),
                    path: Path::new().index(Value::number(0)),
                    want: None,
                    want_err: Some(r#"at step 0: value does not have given index key"#),
                },
                Case {
                    start: Value::null(Type::empty_object()),
                    path: Path::new().attr("foo"),
                    want: None,
                    want_err: Some(r#"at step 0: cannot access attributes on a null value"#),
                },
            ];

            check(&tests);
        }

        // TestPathApply, collections:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/path_test.go#L10
        #[test]
        #[ignore = "not yet implemented"]
        fn collections() {
            let tests: Vec<Case> = vec![
                Case {
                    start: Value::string("hello"),
                    path: Path::new().index(Value::string("boop")),
                    want: None,
                    want_err: Some(r#"at step 0: not a map type"#),
                },
                Case {
                    start: Value::string("hello"),
                    path: Path::new().index(Value::number(0)),
                    want: None,
                    want_err: Some(r#"at step 0: not a list type"#),
                },
                Case {
                    start: Value::list([Value::string("hello")]),
                    path: Path::new().index(Value::number(0)),
                    want: Some(Value::string("hello")),
                    want_err: None,
                },
                Case {
                    start: Value::tuple([Value::string("hello")]),
                    path: Path::new().index(Value::number(0)),
                    want: Some(Value::string("hello")),
                    want_err: None,
                },
                Case {
                    start: Value::list([Value::string("hello")]),
                    path: Path::new().index(Value::number(1)),
                    want: None,
                    want_err: Some(r#"at step 0: value does not have given index key"#),
                },
                Case {
                    start: Value::list([Value::string("hello")]),
                    path: Path::new().index(Value::number(0)).attr("foo"),
                    want: None,
                    want_err: Some(r#"at step 1: not an object type"#),
                },
                Case {
                    start: Value::list([Value::empty_object()]),
                    path: Path::new().index(Value::number(0)).attr("foo"),
                    want: None,
                    want_err: Some(r#"at step 1: object has no attribute "foo""#),
                },
                Case {
                    start: Value::null(Type::list(Type::string())),
                    path: Path::new().index(Value::number(0)),
                    want: None,
                    want_err: Some(r#"at step 0: cannot index a null value"#),
                },
                Case {
                    start: Value::null(Type::map(Type::string())),
                    path: Path::new().index(Value::number(0)),
                    want: None,
                    want_err: Some(r#"at step 0: cannot index a null value"#),
                },
            ];

            check(&tests);
        }

        // TestPathApply, marks:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/path_test.go#L10
        #[test]
        #[ignore = "not yet implemented"]
        fn marks() {
            let tests: Vec<Case> = vec![
                Case {
                    start: Value::list([Value::list([Value::string("hello")]).mark(2)]).mark(1),
                    path: Path::new().index(Value::number(0)).index(Value::number(0)),
                    want: Some(Value::string("hello").mark(1).mark(2)),
                    want_err: None,
                },
                Case {
                    start: Value::tuple([Value::list([Value::string("hello")]).mark(2)]).mark(1),
                    path: Path::new().index(Value::number(0)).index(Value::number(0)),
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
                    path: Path::new().index(Value::number(0)),
                    want: Some(Value::string("hello").mark(1)),
                    want_err: None,
                },
                Case {
                    start: Value::tuple([Value::string("hello").mark(1)]),
                    path: Path::new().index(Value::number(0)),
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

            check(&tests);
        }
    }

    // Ported from TestPathEquals:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/path_test.go#L260
    //
    // Upstream TestPathEquals is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod path_equals {
        use super::*;

        struct Case {
            a: Path,
            b: Path,
            equal: bool,
            prefix: bool,
        }

        fn steps(steps: impl IntoIterator<Item = PathStep>) -> Path {
            Path::from_steps(steps)
        }

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[Case]) {
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

        // TestPathEquals, primitives:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/path_test.go#L260
        #[test]
        #[ignore = "not yet implemented"]
        fn primitives() {
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
                        PathStep::Index(Value::number(0.0)),
                        PathStep::GetAttr("attr".into()),
                    ]),
                    b: steps([
                        PathStep::GetAttr("attr".into()),
                        PathStep::Index(Value::number(0)),
                        PathStep::GetAttr("attr".into()),
                    ]),
                    equal: true,
                    prefix: true,
                },
                Case {
                    a: steps([
                        PathStep::GetAttr("attr".into()),
                        PathStep::Index(Value::number(1)),
                        PathStep::GetAttr("attr".into()),
                    ]),
                    b: steps([
                        PathStep::GetAttr("attr".into()),
                        PathStep::Index(Value::number(0)),
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
                    a: steps([PathStep::Index(Value::number(0))]),
                    b: Path::new().index(Value::number(0)),
                    equal: true,
                    prefix: true,
                },
                Case {
                    a: steps([PathStep::Index(Value::number(0))]),
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
                        PathStep::Index(Value::number(0)),
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

            check(&tests);
        }

        // TestPathEquals, unknowns:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/path_test.go#L260
        #[test]
        #[ignore = "not yet implemented"]
        fn unknowns() {
            let tests: Vec<Case> = vec![
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
                        PathStep::Index(Value::unknown(Type::string())),
                    ]),
                    b: steps([
                        PathStep::GetAttr("attr".into()),
                        PathStep::Index(Value::unknown(Type::string())),
                    ]),
                    equal: true,
                    prefix: true,
                },
            ];

            check(&tests);
        }
    }

    // Ported from TestPathSet:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/path_set_test.go#L8
    #[test]
    #[ignore = "not yet implemented"]
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
}
