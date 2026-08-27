//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/walk_test.go
//!   cty/unknown_as_null_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::collections::HashSet;

use cty::{
    Error, Path, Transformer, Type, Value, deep_values, transform, transform_with_transformer,
    unknown_as_null, walk,
};

/// The input value shared by the upstream `TestWalk` assertions.
fn walk_test_value() -> Value {
    Value::object([
        ("string", Value::string("hello")),
        ("number", Value::number_int(10)),
        ("bool", Value::bool(true)),
        ("list", Value::list([Value::bool(true)])),
        ("list_empty", Value::list_empty(Type::bool())),
        ("set", Value::set([Value::bool(true)])),
        ("set_empty", Value::list_empty(Type::bool())),
        ("tuple", Value::tuple([Value::bool(true)])),
        ("tuple_empty", Value::empty_tuple()),
        ("map", Value::map([("true", Value::bool(true))])),
        ("map_empty", Value::map_empty(Type::bool())),
        ("object", Value::object([("true", Value::bool(true))])),
        ("object_empty", Value::empty_object()),
        ("null", Value::null(Type::list(Type::string()))),
        ("unknown", Value::unknown(Type::map(Type::bool()))),
        ("marked_string", Value::string("boop").mark("blorp")),
        (
            "marked_list",
            Value::list([Value::bool(true)]).mark("blorp"),
        ),
        (
            "marked_tuple",
            Value::tuple([Value::bool(true)]).mark("blorp"),
        ),
        ("marked_set", Value::set([Value::bool(true)]).mark("blorp")),
        (
            "marked_object",
            Value::object([("true", Value::bool(true))]).mark("blorp"),
        ),
        ("marked_map", Value::map([("true", Value::bool(true))])),
    ])
}

fn assert_walk_calls(
    want_calls: &[(&str, &str)],
    got_calls: &HashSet<(String, String)>,
    what: &str,
) {
    assert_eq!(
        got_calls.len(),
        want_calls.len(),
        "wrong number of calls from {what} {}; want {}",
        got_calls.len(),
        want_calls.len()
    );
    for (i, (want_path, want_ty)) in want_calls.iter().enumerate() {
        assert!(
            got_calls.contains(&((*want_path).to_string(), (*want_ty).to_string())),
            "case {i}: {what} did not produce ({want_path:?}, {want_ty:?})"
        );
    }
}

// upstream: cty/walk_test.go TestWalk
#[test]
fn walk_and_deep_values() {
    let val = walk_test_value();

    let want_calls: Vec<(&str, &str)> = vec![
        (r#"cty.Path(nil)"#, "object"),
        (r#"cty.Path{cty.GetAttrStep{Name:"string"}}"#, "string"),
        (r#"cty.Path{cty.GetAttrStep{Name:"number"}}"#, "number"),
        (r#"cty.Path{cty.GetAttrStep{Name:"bool"}}"#, "bool"),
        (r#"cty.Path{cty.GetAttrStep{Name:"list"}}"#, "list of bool"),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"list"}, cty.IndexStep{Key:cty.NumberIntVal(0)}}"#,
            "bool",
        ),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"list_empty"}}"#,
            "list of bool",
        ),
        (r#"cty.Path{cty.GetAttrStep{Name:"set"}}"#, "set of bool"),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"set"}, cty.IndexStep{Key:cty.True}}"#,
            "bool",
        ),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"set_empty"}}"#,
            "list of bool",
        ),
        (r#"cty.Path{cty.GetAttrStep{Name:"tuple"}}"#, "tuple"),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"tuple"}, cty.IndexStep{Key:cty.NumberIntVal(0)}}"#,
            "bool",
        ),
        (r#"cty.Path{cty.GetAttrStep{Name:"tuple_empty"}}"#, "tuple"),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"map"}, cty.IndexStep{Key:cty.StringVal("true")}}"#,
            "bool",
        ),
        (r#"cty.Path{cty.GetAttrStep{Name:"map"}}"#, "map of bool"),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"map_empty"}}"#,
            "map of bool",
        ),
        (r#"cty.Path{cty.GetAttrStep{Name:"object"}}"#, "object"),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"object"}, cty.GetAttrStep{Name:"true"}}"#,
            "bool",
        ),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"object_empty"}}"#,
            "object",
        ),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"null"}}"#,
            "list of string",
        ),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"unknown"}}"#,
            "map of bool",
        ),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"marked_string"}}"#,
            "string",
        ),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"marked_list"}}"#,
            "list of bool",
        ),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"marked_list"}, cty.IndexStep{Key:cty.NumberIntVal(0)}}"#,
            "bool",
        ),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"marked_set"}}"#,
            "set of bool",
        ),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"marked_set"}, cty.IndexStep{Key:cty.True}}"#,
            "bool",
        ),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"marked_object"}}"#,
            "object",
        ),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"marked_object"}, cty.GetAttrStep{Name:"true"}}"#,
            "bool",
        ),
        (r#"cty.Path{cty.GetAttrStep{Name:"marked_tuple"}}"#, "tuple"),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"marked_tuple"}, cty.IndexStep{Key:cty.NumberIntVal(0)}}"#,
            "bool",
        ),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"marked_map"}}"#,
            "map of bool",
        ),
        (
            r#"cty.Path{cty.GetAttrStep{Name:"marked_map"}, cty.IndexStep{Key:cty.StringVal("true")}}"#,
            "bool",
        ),
    ];

    let mut got_calls_walk: HashSet<(String, String)> = HashSet::new();
    walk(&val, |path, val| {
        got_calls_walk.insert((path.go_string(), val.ty().friendly_name()));
        Ok(true)
    })
    .unwrap();

    let mut got_calls_deep_values: HashSet<(String, String)> = HashSet::new();
    for (path, val) in deep_values(&val) {
        got_calls_deep_values.insert((path.go_string(), val.ty().friendly_name()));
    }

    assert_walk_calls(&want_calls, &got_calls_walk, "walk");
    assert_walk_calls(&want_calls, &got_calls_deep_values, "deep_values");
}

// Rust-syntax twin of walk_and_deep_values: the same table with the path
// expectations translated into this crate's constructor syntax, pinning
// `Path`'s `Display`.
#[test]
fn walk_and_deep_values_display() {
    let val = walk_test_value();

    let want_calls: Vec<(&str, &str)> = vec![
        ("Path::new()", "object"),
        (r#"Path::new().attr("string")"#, "string"),
        (r#"Path::new().attr("number")"#, "number"),
        (r#"Path::new().attr("bool")"#, "bool"),
        (r#"Path::new().attr("list")"#, "list of bool"),
        (
            r#"Path::new().attr("list").index(Value::number_int(0))"#,
            "bool",
        ),
        (r#"Path::new().attr("list_empty")"#, "list of bool"),
        (r#"Path::new().attr("set")"#, "set of bool"),
        (
            r#"Path::new().attr("set").index(Value::bool(true))"#,
            "bool",
        ),
        (r#"Path::new().attr("set_empty")"#, "list of bool"),
        (r#"Path::new().attr("tuple")"#, "tuple"),
        (
            r#"Path::new().attr("tuple").index(Value::number_int(0))"#,
            "bool",
        ),
        (r#"Path::new().attr("tuple_empty")"#, "tuple"),
        (
            r#"Path::new().attr("map").index(Value::string("true"))"#,
            "bool",
        ),
        (r#"Path::new().attr("map")"#, "map of bool"),
        (r#"Path::new().attr("map_empty")"#, "map of bool"),
        (r#"Path::new().attr("object")"#, "object"),
        (r#"Path::new().attr("object").attr("true")"#, "bool"),
        (r#"Path::new().attr("object_empty")"#, "object"),
        (r#"Path::new().attr("null")"#, "list of string"),
        (r#"Path::new().attr("unknown")"#, "map of bool"),
        (r#"Path::new().attr("marked_string")"#, "string"),
        (r#"Path::new().attr("marked_list")"#, "list of bool"),
        (
            r#"Path::new().attr("marked_list").index(Value::number_int(0))"#,
            "bool",
        ),
        (r#"Path::new().attr("marked_set")"#, "set of bool"),
        (
            r#"Path::new().attr("marked_set").index(Value::bool(true))"#,
            "bool",
        ),
        (r#"Path::new().attr("marked_object")"#, "object"),
        (r#"Path::new().attr("marked_object").attr("true")"#, "bool"),
        (r#"Path::new().attr("marked_tuple")"#, "tuple"),
        (
            r#"Path::new().attr("marked_tuple").index(Value::number_int(0))"#,
            "bool",
        ),
        (r#"Path::new().attr("marked_map")"#, "map of bool"),
        (
            r#"Path::new().attr("marked_map").index(Value::string("true"))"#,
            "bool",
        ),
    ];

    let mut got_calls_walk: HashSet<(String, String)> = HashSet::new();
    walk(&val, |path, val| {
        got_calls_walk.insert((path.to_string(), val.ty().friendly_name()));
        Ok(true)
    })
    .unwrap();

    let mut got_calls_deep_values: HashSet<(String, String)> = HashSet::new();
    for (path, val) in deep_values(&val) {
        got_calls_deep_values.insert((path.to_string(), val.ty().friendly_name()));
    }

    assert_walk_calls(&want_calls, &got_calls_walk, "walk");
    assert_walk_calls(&want_calls, &got_calls_deep_values, "deep_values");
}

/// The input value shared by the upstream `TestTransformWithTransformer`
/// assertions.
fn transform_with_transformer_test_value() -> Value {
    Value::object([
        ("string", Value::string("hello")),
        ("number", Value::number_int(10)),
        ("bool", Value::bool(true)),
        ("list", Value::list([Value::bool(true)])),
        ("list_empty", Value::list_empty(Type::bool())),
        ("set", Value::set([Value::bool(true)])),
        ("set_empty", Value::list_empty(Type::bool())),
        ("tuple", Value::tuple([Value::bool(true)])),
        ("tuple_empty", Value::empty_tuple()),
        ("map", Value::map([("true", Value::bool(true))])),
        ("map_empty", Value::map_empty(Type::bool())),
        ("object", Value::object([("true", Value::bool(true))])),
        ("object_empty", Value::empty_object()),
        ("null", Value::null(Type::string())),
        ("unknown", Value::unknown(Type::bool())),
        ("null_list", Value::null(Type::list(Type::string()))),
        ("unknown_map", Value::unknown(Type::map(Type::bool()))),
        ("marked_string", Value::string("hello").mark("blorp")),
        (
            "marked_list",
            Value::list([Value::bool(true)]).mark("blorp"),
        ),
        ("marked_set", Value::set([Value::bool(true)]).mark("blorp")),
        (
            "marked_tuple",
            Value::tuple([Value::bool(true)]).mark("blorp"),
        ),
        (
            "marked_map",
            Value::map([("true", Value::bool(true))]).mark("blorp"),
        ),
        (
            "marked_object",
            Value::object([("true", Value::bool(true))]).mark("blorp"),
        ),
    ])
}

// upstream: cty/walk_test.go pathTransformer
struct PathTransformer;

impl Transformer for PathTransformer {
    fn enter(&mut self, _path: &Path, value: &Value) -> Result<Value, Error> {
        Ok(value.clone())
    }

    fn exit(&mut self, path: &Path, value: &Value) -> Result<Value, Error> {
        if value.ty().is_primitive_type() {
            return Ok(Value::string(path.go_string()));
        }
        Ok(value.clone())
    }
}

// upstream: cty/walk_test.go TestTransformWithTransformer
#[test]
fn transform_with_transformer_paths() {
    let val = transform_with_transformer_test_value();

    let got_val = transform_with_transformer(&val, &mut PathTransformer)
        .unwrap_or_else(|err| panic!("unexpected error: {err}"));

    let want_val = Value::object([
        (
            "string",
            Value::string(r#"cty.Path{cty.GetAttrStep{Name:"string"}}"#),
        ),
        (
            "number",
            Value::string(r#"cty.Path{cty.GetAttrStep{Name:"number"}}"#),
        ),
        (
            "bool",
            Value::string(r#"cty.Path{cty.GetAttrStep{Name:"bool"}}"#),
        ),
        (
            "list",
            Value::list([Value::string(
                r#"cty.Path{cty.GetAttrStep{Name:"list"}, cty.IndexStep{Key:cty.NumberIntVal(0)}}"#,
            )]),
        ),
        ("list_empty", Value::list_empty(Type::bool())),
        (
            "set",
            Value::set([Value::string(
                r#"cty.Path{cty.GetAttrStep{Name:"set"}, cty.IndexStep{Key:cty.True}}"#,
            )]),
        ),
        ("set_empty", Value::list_empty(Type::bool())),
        (
            "tuple",
            Value::tuple([Value::string(
                r#"cty.Path{cty.GetAttrStep{Name:"tuple"}, cty.IndexStep{Key:cty.NumberIntVal(0)}}"#,
            )]),
        ),
        ("tuple_empty", Value::empty_tuple()),
        (
            "map",
            Value::map([(
                "true",
                Value::string(
                    r#"cty.Path{cty.GetAttrStep{Name:"map"}, cty.IndexStep{Key:cty.StringVal("true")}}"#,
                ),
            )]),
        ),
        ("map_empty", Value::map_empty(Type::bool())),
        (
            "object",
            Value::object([(
                "true",
                Value::string(
                    r#"cty.Path{cty.GetAttrStep{Name:"object"}, cty.GetAttrStep{Name:"true"}}"#,
                ),
            )]),
        ),
        ("object_empty", Value::empty_object()),
        (
            "null",
            Value::string(r#"cty.Path{cty.GetAttrStep{Name:"null"}}"#),
        ),
        (
            "unknown",
            Value::string(r#"cty.Path{cty.GetAttrStep{Name:"unknown"}}"#),
        ),
        ("null_list", Value::null(Type::list(Type::string()))),
        ("unknown_map", Value::unknown(Type::map(Type::bool()))),
        (
            "marked_string",
            Value::string(r#"cty.Path{cty.GetAttrStep{Name:"marked_string"}}"#),
        ),
        (
            "marked_list",
            Value::list([Value::string(
                r#"cty.Path{cty.GetAttrStep{Name:"marked_list"}, cty.IndexStep{Key:cty.NumberIntVal(0)}}"#,
            )])
            .mark("blorp"),
        ),
        (
            "marked_set",
            Value::set([Value::string(
                r#"cty.Path{cty.GetAttrStep{Name:"marked_set"}, cty.IndexStep{Key:cty.True}}"#,
            )])
            .mark("blorp"),
        ),
        (
            "marked_tuple",
            Value::tuple([Value::string(
                r#"cty.Path{cty.GetAttrStep{Name:"marked_tuple"}, cty.IndexStep{Key:cty.NumberIntVal(0)}}"#,
            )])
            .mark("blorp"),
        ),
        (
            "marked_map",
            Value::map([(
                "true",
                Value::string(
                    r#"cty.Path{cty.GetAttrStep{Name:"marked_map"}, cty.IndexStep{Key:cty.StringVal("true")}}"#,
                ),
            )])
            .mark("blorp"),
        ),
        (
            "marked_object",
            Value::object([(
                "true",
                Value::string(
                    r#"cty.Path{cty.GetAttrStep{Name:"marked_object"}, cty.GetAttrStep{Name:"true"}}"#,
                ),
            )])
            .mark("blorp"),
        ),
    ]);

    assert_eq!(got_val, want_val, "wrong result");
}

// Rust-syntax twin of PathTransformer, rendering paths via `Display` instead
// of `go_string`.
struct DisplayPathTransformer;

impl Transformer for DisplayPathTransformer {
    fn enter(&mut self, _path: &Path, value: &Value) -> Result<Value, Error> {
        Ok(value.clone())
    }

    fn exit(&mut self, path: &Path, value: &Value) -> Result<Value, Error> {
        if value.ty().is_primitive_type() {
            return Ok(Value::string(path.to_string()));
        }
        Ok(value.clone())
    }
}

// Rust-syntax twin of transform_with_transformer_paths: the same table with
// the path expectations translated into this crate's constructor syntax,
// pinning `Path`'s `Display`.
#[test]
fn transform_with_transformer_paths_display() {
    let val = transform_with_transformer_test_value();

    let got_val = transform_with_transformer(&val, &mut DisplayPathTransformer)
        .unwrap_or_else(|err| panic!("unexpected error: {err}"));

    let want_val = Value::object([
        ("string", Value::string(r#"Path::new().attr("string")"#)),
        ("number", Value::string(r#"Path::new().attr("number")"#)),
        ("bool", Value::string(r#"Path::new().attr("bool")"#)),
        (
            "list",
            Value::list([Value::string(
                r#"Path::new().attr("list").index(Value::number_int(0))"#,
            )]),
        ),
        ("list_empty", Value::list_empty(Type::bool())),
        (
            "set",
            Value::set([Value::string(
                r#"Path::new().attr("set").index(Value::bool(true))"#,
            )]),
        ),
        ("set_empty", Value::list_empty(Type::bool())),
        (
            "tuple",
            Value::tuple([Value::string(
                r#"Path::new().attr("tuple").index(Value::number_int(0))"#,
            )]),
        ),
        ("tuple_empty", Value::empty_tuple()),
        (
            "map",
            Value::map([(
                "true",
                Value::string(r#"Path::new().attr("map").index(Value::string("true"))"#),
            )]),
        ),
        ("map_empty", Value::map_empty(Type::bool())),
        (
            "object",
            Value::object([(
                "true",
                Value::string(r#"Path::new().attr("object").attr("true")"#),
            )]),
        ),
        ("object_empty", Value::empty_object()),
        ("null", Value::string(r#"Path::new().attr("null")"#)),
        ("unknown", Value::string(r#"Path::new().attr("unknown")"#)),
        ("null_list", Value::null(Type::list(Type::string()))),
        ("unknown_map", Value::unknown(Type::map(Type::bool()))),
        (
            "marked_string",
            Value::string(r#"Path::new().attr("marked_string")"#),
        ),
        (
            "marked_list",
            Value::list([Value::string(
                r#"Path::new().attr("marked_list").index(Value::number_int(0))"#,
            )])
            .mark("blorp"),
        ),
        (
            "marked_set",
            Value::set([Value::string(
                r#"Path::new().attr("marked_set").index(Value::bool(true))"#,
            )])
            .mark("blorp"),
        ),
        (
            "marked_tuple",
            Value::tuple([Value::string(
                r#"Path::new().attr("marked_tuple").index(Value::number_int(0))"#,
            )])
            .mark("blorp"),
        ),
        (
            "marked_map",
            Value::map([(
                "true",
                Value::string(r#"Path::new().attr("marked_map").index(Value::string("true"))"#),
            )])
            .mark("blorp"),
        ),
        (
            "marked_object",
            Value::object([(
                "true",
                Value::string(r#"Path::new().attr("marked_object").attr("true")"#),
            )])
            .mark("blorp"),
        ),
    ]);

    assert_eq!(got_val, want_val, "wrong result");
}

// upstream: cty/walk_test.go errorTransformer
struct ErrorTransformer;

impl Transformer for ErrorTransformer {
    fn enter(&mut self, _path: &Path, value: &Value) -> Result<Value, Error> {
        Ok(value.clone())
    }

    fn exit(&mut self, path: &Path, value: &Value) -> Result<Value, Error> {
        let ty = value.ty();
        if ty.is_primitive_type() {
            return Ok(value.clone());
        }
        Err(path.error(format!("expected primitive type, was {}", ty.go_string())))
    }
}

// upstream: cty/walk_test.go TestTransformWithTransformer_error
#[test]
fn transform_with_transformer_error() {
    let val = Value::object([
        ("string", Value::string("hello")),
        ("number", Value::number_int(10)),
        ("bool", Value::bool(true)),
        ("list", Value::list([Value::bool(true)])),
    ]);

    // NOTE(port): upstream also asserts that the value returned alongside the
    // error is cty.DynamicVal; Rust's `Result` carries no value in the `Err`
    // case, so that check has no analogue here.
    let err = match transform_with_transformer(&val, &mut ErrorTransformer) {
        Ok(got_val) => panic!("expected error, got {got_val:?}"),
        Err(err) => err,
    };

    // NOTE(port): upstream asserts the error is a cty.PathError via a type
    // assertion; the analogue here is `Error::path` returning the path.
    let want = Path::new().attr("list");
    let got = err.path().expect("expected path-carrying error, got none");
    assert!(
        got.equals(&want),
        "wrong path\n got: {got:?}\nwant: {want:?}"
    );
}

// upstream: cty/walk_test.go TestTransform
#[test]
fn transform_values() {
    let val = Value::object([
        (
            "list",
            Value::list([Value::bool(true), Value::bool(true), Value::bool(false)]),
        ),
        ("set", Value::set([Value::bool(true), Value::bool(false)])),
        (
            "map",
            Value::map([("a", Value::bool(true)), ("b", Value::bool(false))]),
        ),
        (
            "object",
            Value::object([
                ("a", Value::bool(true)),
                (
                    "b",
                    Value::list([Value::bool(false), Value::bool(false), Value::bool(false)]),
                ),
            ]),
        ),
    ]);
    let want_val = Value::object([
        (
            "list",
            Value::list([Value::bool(false), Value::bool(false), Value::bool(true)]),
        ),
        ("set", Value::set([Value::bool(true), Value::bool(false)])),
        (
            "map",
            Value::map([("a", Value::bool(false)), ("b", Value::bool(true))]),
        ),
        (
            "object",
            Value::object([
                ("a", Value::bool(false)),
                (
                    "b",
                    Value::list([Value::bool(true), Value::bool(true), Value::bool(true)]),
                ),
            ]),
        ),
    ]);

    let got_val = transform(&val, |_p, v| {
        if v.ty().equals(&Type::bool()) {
            return Ok(v.not());
        }
        Ok(v.clone())
    })
    .unwrap_or_else(|err| panic!("unexpected error: {err}"));

    assert_eq!(got_val, want_val, "wrong value");
}

// upstream: cty/walk_test.go TestTransformMarked
#[test]
fn transform_marked() {
    let val = Value::object([
        (
            "list",
            Value::list([Value::bool(true), Value::bool(true), Value::bool(false)]).mark("mark"),
        ),
        (
            "set",
            Value::set([Value::bool(true), Value::bool(false)]).mark("mark"),
        ),
        (
            "map",
            Value::map([("a", Value::bool(true)), ("b", Value::bool(false))]).mark("mark"),
        ),
        (
            "object",
            Value::object([
                ("a", Value::bool(true)),
                (
                    "b",
                    Value::list([Value::bool(false), Value::bool(false), Value::bool(false)]),
                ),
            ])
            .mark("mark"),
        ),
    ]);

    // This noop transform should not change any values or marks.
    let got_val = transform(&val, |_p, v| Ok(v.clone()))
        .unwrap_or_else(|err| panic!("unexpected error: {err}"));

    assert_eq!(got_val, val, "wrong value");
}

// upstream: cty/unknown_as_null_test.go TestUnknownAsNull
#[test]
fn unknown_as_null_conformance() {
    let tests: Vec<(Value, Value)> = vec![
        (Value::string("hello"), Value::string("hello")),
        (Value::null(Type::string()), Value::null(Type::string())),
        (Value::unknown(Type::string()), Value::null(Type::string())),
        (Value::null(Type::dynamic()), Value::null(Type::dynamic())),
        (
            Value::null(Type::object([("test", Type::string())])),
            Value::null(Type::object([("test", Type::string())])),
        ),
        (Value::dynamic(), Value::null(Type::dynamic())),
        (
            Value::list_empty(Type::string()),
            Value::list_empty(Type::string()),
        ),
        (
            Value::list([Value::string("hello")]),
            Value::list([Value::string("hello")]),
        ),
        (
            Value::list([Value::null(Type::string())]),
            Value::list([Value::null(Type::string())]),
        ),
        (
            Value::list([Value::unknown(Type::string())]),
            Value::list([Value::null(Type::string())]),
        ),
        (
            Value::set_empty(Type::string()),
            Value::set_empty(Type::string()),
        ),
        (
            Value::set([Value::string("hello")]),
            Value::set([Value::string("hello")]),
        ),
        (
            Value::set([Value::null(Type::string())]),
            Value::set([Value::null(Type::string())]),
        ),
        (
            Value::set([Value::unknown(Type::string())]),
            Value::set([Value::null(Type::string())]),
        ),
        (Value::empty_tuple(), Value::empty_tuple()),
        (
            Value::tuple([Value::string("hello")]),
            Value::tuple([Value::string("hello")]),
        ),
        (
            Value::tuple([Value::null(Type::string())]),
            Value::tuple([Value::null(Type::string())]),
        ),
        (
            Value::tuple([Value::unknown(Type::string())]),
            Value::tuple([Value::null(Type::string())]),
        ),
        (
            Value::map_empty(Type::string()),
            Value::map_empty(Type::string()),
        ),
        (
            Value::map([("greeting", Value::string("hello"))]),
            Value::map([("greeting", Value::string("hello"))]),
        ),
        (
            Value::map([("greeting", Value::null(Type::string()))]),
            Value::map([("greeting", Value::null(Type::string()))]),
        ),
        (
            Value::map([("greeting", Value::unknown(Type::string()))]),
            Value::map([("greeting", Value::null(Type::string()))]),
        ),
        (Value::empty_object(), Value::empty_object()),
        (
            Value::object([("greeting", Value::string("hello"))]),
            Value::object([("greeting", Value::string("hello"))]),
        ),
        (
            Value::object([("greeting", Value::null(Type::string()))]),
            Value::object([("greeting", Value::null(Type::string()))]),
        ),
        (
            Value::object([("greeting", Value::unknown(Type::string()))]),
            Value::object([("greeting", Value::null(Type::string()))]),
        ),
        // Marks should be accepted and preserved verbatim
        (
            Value::string("hello").mark("..."),
            Value::string("hello").mark("..."),
        ),
        (
            Value::unknown(Type::string()).mark("..."),
            Value::null(Type::string()).mark("..."),
        ),
        (
            Value::dynamic().mark("..."),
            Value::null(Type::dynamic()).mark("..."),
        ),
        (
            Value::list([Value::unknown(Type::string())]).mark("..."),
            Value::list([Value::null(Type::string())]).mark("..."),
        ),
        (
            Value::list([Value::unknown(Type::string()).mark("...")]),
            Value::list([Value::null(Type::string()).mark("...")]),
        ),
        (
            Value::set([Value::unknown(Type::string())]).mark("..."),
            Value::set([Value::null(Type::string())]).mark("..."),
        ),
        (
            Value::set([Value::unknown(Type::string()).mark("...")]),
            Value::set([Value::null(Type::string()).mark("...")]),
        ),
        (
            Value::tuple([Value::unknown(Type::string())]).mark("..."),
            Value::tuple([Value::null(Type::string())]).mark("..."),
        ),
        (
            Value::tuple([Value::unknown(Type::string()).mark("...")]),
            Value::tuple([Value::null(Type::string()).mark("...")]),
        ),
        (
            Value::map([("greeting", Value::unknown(Type::string()))]).mark("..."),
            Value::map([("greeting", Value::null(Type::string()))]).mark("..."),
        ),
        (
            Value::map([("greeting", Value::unknown(Type::string()).mark("..."))]),
            Value::map([("greeting", Value::null(Type::string()).mark("..."))]),
        ),
        (
            Value::object([("greeting", Value::unknown(Type::string()))]).mark("..."),
            Value::object([("greeting", Value::null(Type::string()))]).mark("..."),
        ),
        (
            Value::object([("greeting", Value::unknown(Type::string()).mark("..."))]),
            Value::object([("greeting", Value::null(Type::string()).mark("..."))]),
        ),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got = unknown_as_null(input);
        assert_eq!(
            got, *want,
            "case {i}: wrong result\ninput: {input:?}\ngot:   {got:?}\nwant:  {want:?}"
        );
    }
}
