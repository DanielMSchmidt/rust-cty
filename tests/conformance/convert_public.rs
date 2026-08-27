//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/convert/public_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::convert::convert;
use cty::{Type, Value, ValueMarks};

// Ported from TestConvert:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/convert/public_test.go#L12
//
// Each case is (input value, target type, expectation): `Ok(want)` ports an
// upstream `Want` value (asserted with `==`, i.e. RawEquals), `Err(msg)`
// ports an upstream `WantError` string (asserted against `err.to_string()`).
// NOTE(port): upstream compares errors via its `errorStrForTesting` helper,
// which prefixes a rendered path for `cty.PathError`; every expected error in
// this table has an empty path, so the helper reduces to `err.Error()` and
// `err.to_string()` is the faithful analogue.
#[test]
#[ignore = "not yet implemented"]
#[allow(clippy::approx_constant)] // upstream uses the literal 3.14159265359
fn convert_public() {
    let tests: Vec<(Value, Type, Result<Value, &str>)> = vec![
        (
            Value::string("hello"),
            Type::string(),
            Ok(Value::string("hello")),
        ),
        (Value::string("1"), Type::number(), Ok(Value::number_int(1))),
        (
            Value::string("1.5"),
            Type::number(),
            Ok(Value::number_float(1.5)),
        ),
        (
            Value::string("hello"),
            Type::number(),
            Err("a number is required"),
        ),
        (Value::string("true"), Type::bool(), Ok(Value::bool(true))),
        (Value::string("1"), Type::bool(), Ok(Value::bool(true))),
        (Value::string("false"), Type::bool(), Ok(Value::bool(false))),
        (Value::string("0"), Type::bool(), Ok(Value::bool(false))),
        (
            Value::string("hello"),
            Type::bool(),
            Err("a bool is required"),
        ),
        (Value::number_int(4), Type::string(), Ok(Value::string("4"))),
        (
            Value::number_float(3.14159265359),
            Type::string(),
            Ok(Value::string("3.14159265359")),
        ),
        (Value::bool(true), Type::string(), Ok(Value::string("true"))),
        (
            Value::bool(false),
            Type::string(),
            Ok(Value::string("false")),
        ),
        (
            Value::unknown(Type::string()),
            Type::number(),
            Ok(Value::unknown(Type::number())),
        ),
        (
            Value::unknown(Type::number()),
            Type::string(),
            Ok(Value::unknown(Type::string())),
        ),
        (
            Value::dynamic(),
            Type::string(),
            Ok(Value::unknown(Type::string())),
        ),
        (
            Value::string("hello"),
            Type::dynamic(),
            Ok(Value::string("hello")),
        ),
        (
            Value::list([Value::number_int(5), Value::number_int(10)]),
            Type::list(Type::string()),
            Ok(Value::list([Value::string("5"), Value::string("10")])),
        ),
        (
            Value::list([Value::number_int(5), Value::number_int(10)]),
            Type::list(Type::dynamic()),
            Ok(Value::list([Value::number_int(5), Value::number_int(10)])),
        ),
        (
            Value::tuple([
                Value::object([
                    ("type", Value::string("ingress")),
                    ("from_port", Value::number_int(-1)),
                    ("to_port", Value::number_int(-1)),
                    ("protocol", Value::string("icmp")),
                    ("description", Value::string("ICMP in")),
                    ("cidr", Value::tuple([Value::string("0.0.0.0/0")])),
                ]),
                Value::object([
                    ("type", Value::string("ingress")),
                    ("from_port", Value::number_int(22)),
                    ("to_port", Value::number_int(22)),
                    ("protocol", Value::string("tcp")),
                    ("description", Value::string("SSH from Bastion")),
                    ("source_sg", Value::string("sg-abc123")),
                ]),
            ]),
            Type::list(Type::dynamic()),
            Err("all list elements must have the same type"),
        ),
        (
            Value::set([Value::string("5"), Value::unknown(Type::string())]),
            Type::set(Type::number()),
            Ok(Value::set([
                Value::number_int(5),
                Value::unknown(Type::number()),
            ])),
        ),
        (
            Value::set([Value::string("5"), Value::string("10")]),
            Type::list(Type::string()),
            Ok(Value::list([
                // NOTE: This results depends on the traversal order of the
                // set, which may change if the set implementation changes.
                Value::string("10"),
                Value::string("5"),
            ])),
        ),
        (
            Value::set([Value::string("5"), Value::string("10")]),
            Type::list(Type::dynamic()),
            Ok(Value::list([
                // NOTE: This results depends on the traversal order of the
                // set, which may change if the set implementation changes.
                Value::string("10"),
                Value::string("5"),
            ])),
        ),
        (
            Value::set([Value::number_int(5), Value::number_int(10)]),
            Type::list(Type::string()),
            Ok(Value::list([
                // NOTE: This results depends on the traversal order of the
                // set, which may change if the set implementation changes.
                Value::string("5"),
                Value::string("10"),
            ])),
        ),
        (
            Value::set([Value::string("5"), Value::unknown(Type::string())]),
            Type::list(Type::string()),
            Ok(Value::unknown(Type::list(Type::string()))),
        ),
        (
            Value::set([Value::unknown(Type::string())]),
            Type::list(Type::string()),
            // We get a known list value this time because even though we
            // don't know the single value that's in the list, we _do_ know
            // that there are no other values in the set for it to coalesce
            // with.
            Ok(Value::list([Value::unknown(Type::string())])),
        ),
        (
            Value::list([
                Value::number_int(5),
                Value::number_int(10),
                Value::number_int(10),
            ]),
            Type::set(Type::string()),
            Ok(Value::set([Value::string("5"), Value::string("10")])),
        ),
        (
            Value::tuple([Value::number_int(5), Value::string("hello")]),
            Type::list(Type::string()),
            Ok(Value::list([Value::string("5"), Value::string("hello")])),
        ),
        (
            Value::tuple([Value::number_int(5), Value::string("12")]),
            Type::list(Type::number()),
            Ok(Value::list([Value::number_int(5), Value::number_int(12)])),
        ),
        (
            Value::tuple([Value::number_int(5), Value::number_int(10)]),
            Type::list(Type::dynamic()),
            Ok(Value::list([Value::number_int(5), Value::number_int(10)])),
        ),
        (
            Value::tuple([Value::number_int(5), Value::string("hello")]),
            Type::list(Type::dynamic()),
            Ok(Value::list([Value::string("5"), Value::string("hello")])),
        ),
        (
            Value::tuple([Value::number_int(5), Value::string("hello")]),
            Type::set(Type::dynamic()),
            Ok(Value::set([Value::string("5"), Value::string("hello")])),
        ),
        (
            Value::list_empty(Type::string()),
            Type::set(Type::dynamic()),
            Ok(Value::set_empty(Type::string())),
        ),
        (
            Value::set_empty(Type::string()),
            Type::list(Type::dynamic()),
            Ok(Value::list_empty(Type::string())),
        ),
        (
            Value::object([
                ("num", Value::number_int(5)),
                ("str", Value::string("hello")),
            ]),
            Type::map(Type::string()),
            Ok(Value::map([
                ("num", Value::string("5")),
                ("str", Value::string("hello")),
            ])),
        ),
        (
            Value::object([("num", Value::number_int(5)), ("str", Value::string("12"))]),
            Type::map(Type::number()),
            Ok(Value::map([
                ("num", Value::number_int(5)),
                ("str", Value::number_int(12)),
            ])),
        ),
        (
            Value::object([
                ("num1", Value::number_int(5)),
                ("num2", Value::number_int(10)),
            ]),
            Type::map(Type::dynamic()),
            Ok(Value::map([
                ("num1", Value::number_int(5)),
                ("num2", Value::number_int(10)),
            ])),
        ),
        (
            Value::object([
                ("num", Value::number_int(5)),
                ("str", Value::string("hello")),
            ]),
            Type::map(Type::dynamic()),
            Ok(Value::map([
                ("num", Value::string("5")),
                ("str", Value::string("hello")),
            ])),
        ),
        (
            Value::object([
                ("list", Value::list_empty(Type::bool())),
                ("tuple", Value::empty_tuple()),
            ]),
            Type::map(Type::dynamic()),
            Ok(Value::map([
                ("list", Value::list_empty(Type::bool())),
                ("tuple", Value::list_empty(Type::bool())),
            ])),
        ),
        (
            Value::object([
                ("map", Value::map_empty(Type::string())),
                ("obj", Value::empty_object()),
            ]),
            Type::map(Type::dynamic()),
            Ok(Value::map([
                ("map", Value::map_empty(Type::string())),
                ("obj", Value::map_empty(Type::string())),
            ])),
        ),
        (
            Value::object([("num", Value::number_int(5)), ("bool", Value::bool(true))]),
            Type::map(Type::dynamic()),
            Err("all map elements must have the same type"),
        ),
        (
            Value::map([
                ("greeting", Value::string("Hello")),
                ("name", Value::string("John")),
            ]),
            Type::map(Type::dynamic()),
            Ok(Value::map([
                ("greeting", Value::string("Hello")),
                ("name", Value::string("John")),
            ])),
        ),
        (
            Value::map([
                ("greeting", Value::string("Hello")),
                ("name", Value::string("John")),
            ]),
            Type::object([("greeting", Type::string()), ("name", Type::string())]),
            Ok(Value::object([
                ("greeting", Value::string("Hello")),
                ("name", Value::string("John")),
            ])),
        ),
        (
            Value::map([
                ("greeting", Value::string("Hello")),
                ("name", Value::string("John")),
            ]),
            Type::object([
                ("greeting", Type::list(Type::string())),
                ("name", Type::string()),
            ]),
            Err("object required"), // FIXME: should be something like "attribute greeting: must be a list"
        ),
        (
            Value::map([
                ("greeting", Value::string("Hello")),
                ("name", Value::string("John")),
            ]),
            Type::object([("name", Type::string())]),
            Ok(Value::object([("name", Value::string("John"))])),
        ),
        (
            Value::map([("name", Value::string("John"))]),
            Type::object([("name", Type::string()), ("greeting", Type::string())]),
            Err(r#"map has no element for required attribute "greeting""#),
        ),
        (
            Value::map([("name", Value::string("John"))]),
            Type::object_with_optional_attrs(
                [("name", Type::string()), ("greeting", Type::string())],
                &["greeting"],
            ),
            Ok(Value::object([
                ("greeting", Value::null(Type::string())),
                ("name", Value::string("John")),
            ])),
        ),
        (
            Value::map([("a", Value::number_int(2)), ("b", Value::number_int(5))]),
            Type::map(Type::string()),
            Ok(Value::map([
                ("a", Value::string("2")),
                ("b", Value::string("5")),
            ])),
        ),
        (
            Value::object([
                ("foo", Value::string("foo value")),
                ("bar", Value::string("bar value")),
            ]),
            Type::object([("foo", Type::string())]),
            Ok(Value::object([("foo", Value::string("foo value"))])),
        ),
        (
            Value::object([("foo", Value::bool(true))]),
            Type::object([("foo", Type::string())]),
            Ok(Value::object([("foo", Value::string("true"))])),
        ),
        (
            Value::object([("foo", Value::dynamic())]),
            Type::object([("foo", Type::string())]),
            Ok(Value::object([("foo", Value::unknown(Type::string()))])),
        ),
        (
            Value::object([("foo", Value::null(Type::string()))]),
            Type::object([("foo", Type::string())]),
            Ok(Value::object([("foo", Value::null(Type::string()))])),
        ),
        (
            Value::object([("foo", Value::bool(true))]),
            Type::object([("foo", Type::dynamic())]),
            Ok(Value::object([("foo", Value::bool(true))])),
        ),
        (
            Value::object([("bar", Value::string("bar value"))]),
            Type::object([("foo", Type::string())]),
            Err(r#"attribute "foo" is required"#),
        ),
        (
            Value::object([("bar", Value::string("bar value"))]),
            Type::object([("foo", Type::string()), ("baz", Type::string())]),
            Err(r#"attributes "baz" and "foo" are required"#),
        ),
        (
            Value::empty_object(),
            Type::object([
                ("foo", Type::string()),
                ("bar", Type::string()),
                ("baz", Type::string()),
            ]),
            Err(r#"attributes "bar", "baz", and "foo" are required"#),
        ),
        (
            Value::object([("bar", Value::string("bar value"))]),
            Type::object_with_optional_attrs(
                [("foo", Type::string()), ("bar", Type::string())],
                &["foo"],
            ),
            Ok(Value::object([
                ("foo", Value::null(Type::string())),
                ("bar", Value::string("bar value")),
            ])),
        ),
        (
            Value::object([
                ("foo", Value::string("foo value")),
                ("bar", Value::string("bar value")),
            ]),
            Type::object_with_optional_attrs(
                [("foo", Type::string()), ("bar", Type::string())],
                &["foo"],
            ),
            Ok(Value::object([
                ("foo", Value::string("foo value")),
                ("bar", Value::string("bar value")),
            ])),
        ),
        (
            Value::empty_object(),
            Type::object_with_optional_attrs(
                [("foo", Type::string()), ("bar", Type::string())],
                &["foo"],
            ),
            Err(r#"attribute "bar" is required"#),
        ),
        (
            Value::null(Type::dynamic()),
            Type::object_with_optional_attrs(
                [("foo", Type::string()), ("bar", Type::string())],
                &["foo"],
            ),
            Ok(Value::null(Type::object([
                ("foo", Type::string()),
                ("bar", Type::string()),
            ]))),
        ),
        (
            Value::list([
                Value::null(Type::dynamic()),
                Value::object([("bar", Value::string("bar value"))]),
            ]),
            Type::list(Type::object_with_optional_attrs(
                [("foo", Type::string()), ("bar", Type::string())],
                &["foo"],
            )),
            Ok(Value::list([
                Value::null(Type::object([
                    ("foo", Type::string()),
                    ("bar", Type::string()),
                ])),
                Value::object([
                    ("foo", Value::null(Type::string())),
                    ("bar", Value::string("bar value")),
                ]),
            ])),
        ),
        (
            Value::object([("foo", Value::bool(true))]),
            Type::object([("foo", Type::number())]),
            Err(r#"attribute "foo": number required, but have bool"#),
        ),
        (
            Value::object([("foo", Value::unknown(Type::bool()))]),
            Type::object([("foo", Type::number())]),
            Err(r#"attribute "foo": number required, but have bool"#),
        ),
        (
            Value::null(Type::string()),
            Type::dynamic(),
            Ok(Value::null(Type::string())),
        ),
        (
            Value::unknown(Type::string()),
            Type::dynamic(),
            Ok(Value::unknown(Type::string())),
        ),
        (
            Value::tuple([Value::string("hello")]),
            Type::tuple([Type::string()]),
            Ok(Value::tuple([Value::string("hello")])),
        ),
        (
            Value::tuple([Value::bool(true)]),
            Type::tuple([Type::string()]),
            Ok(Value::tuple([Value::string("true")])),
        ),
        (
            Value::tuple([Value::bool(true)]),
            Type::empty_tuple(),
            Err("tuple required"), // FIXME: this error is not descriptive enough
        ),
        (
            Value::empty_tuple(),
            Type::tuple([Type::string()]),
            Err("tuple required"), // FIXME: this error is not descriptive enough
        ),
        (
            Value::empty_tuple(),
            Type::set(Type::string()),
            Ok(Value::set_empty(Type::string())),
        ),
        // Marks on values should propagate, even deeply.
        (
            Value::string("hello").mark(1),
            Type::string(),
            Ok(Value::string("hello").mark(1)),
        ),
        (
            Value::string("true").mark(1),
            Type::bool(),
            Ok(Value::bool(true).mark(1)),
        ),
        (
            Value::tuple([Value::string("hello").mark(1)]),
            Type::list(Type::string()),
            Ok(Value::list([Value::string("hello").mark(1)])),
        ),
        (
            Value::set([
                Value::string("hello").mark(1),
                Value::string("hello").mark(2),
            ]),
            Type::set(Type::string()),
            Ok(Value::set([Value::string("hello")]).with_marks([ValueMarks::from_marks([1, 2])])),
        ),
        (
            Value::object([("foo", Value::string("hello").mark(1))]),
            Type::map(Type::string()),
            Ok(Value::map([("foo", Value::string("hello").mark(1))])),
        ),
        (
            Value::object([
                ("foo", Value::string("hello").mark(1)),
                ("bar", Value::string("world").mark(1)),
            ]),
            Type::object([("foo", Type::string())]),
            Ok(Value::object([("foo", Value::string("hello").mark(1))])),
        ),
        (
            Value::object([
                ("foo", Value::string("hello")),
                ("bar", Value::string("world").mark(1)),
            ]),
            Type::object([("foo", Type::string())]),
            Ok(Value::object([("foo", Value::string("hello"))])),
        ),
        // reduction of https://github.com/hashicorp/terraform/issues/23804
        (
            Value::object([
                (
                    "a",
                    Value::object([("x", Value::tuple([Value::string("foo")]))]),
                ),
                (
                    "b",
                    Value::object([("x", Value::tuple([Value::string("bar")]))]),
                ),
                (
                    "c",
                    Value::object([(
                        "x",
                        Value::tuple([Value::string("foo"), Value::string("bar")]),
                    )]),
                ),
            ]),
            Type::map(Type::map(Type::dynamic())),
            Ok(Value::map([
                (
                    "a",
                    Value::map([("x", Value::list([Value::string("foo")]))]),
                ),
                (
                    "b",
                    Value::map([("x", Value::list([Value::string("bar")]))]),
                ),
                (
                    "c",
                    Value::map([(
                        "x",
                        Value::list([Value::string("foo"), Value::string("bar")]),
                    )]),
                ),
            ])),
        ),
        // reduction of https://github.com/hashicorp/terraform/issues/24167
        (
            Value::object([
                ("a", Value::object([("x", Value::null(Type::dynamic()))])),
                (
                    "b",
                    Value::object([(
                        "x",
                        Value::object([("c", Value::number_int(1)), ("d", Value::number_int(2))]),
                    )]),
                ),
            ]),
            Type::map(Type::map(Type::object([("x", Type::map(Type::dynamic()))]))),
            Err(r#"element "b": element "x": attribute "x" is required"#),
        ),
        // reduction of https://github.com/hashicorp/terraform/issues/23431
        (
            Value::object([
                ("a", Value::object([("x", Value::string("foo"))])),
                ("b", Value::map_empty(Type::dynamic())),
            ]),
            Type::map(Type::map(Type::dynamic())),
            Ok(Value::map([
                ("a", Value::map([("x", Value::string("foo"))])),
                ("b", Value::map_empty(Type::string())),
            ])),
        ),
        // reduction of https://github.com/hashicorp/terraform/issues/27269
        (
            Value::tuple([
                Value::object([("a", Value::null(Type::dynamic()))]),
                Value::object([(
                    "a",
                    Value::object([(
                        "b",
                        Value::list([Value::object([("c", Value::string("d"))])]),
                    )]),
                )]),
            ]),
            Type::list(Type::object([(
                "a",
                Type::object([(
                    "b",
                    Type::list(Type::object_with_optional_attrs(
                        [("c", Type::string()), ("d", Type::string())],
                        &["d"],
                    )),
                )]),
            )])),
            Ok(Value::list([
                Value::object([(
                    "a",
                    Value::null(Type::object([(
                        "b",
                        Type::list(Type::object([("c", Type::string()), ("d", Type::string())])),
                    )])),
                )]),
                Value::object([(
                    "a",
                    Value::object([(
                        "b",
                        Value::list([Value::object([
                            ("c", Value::string("d")),
                            ("d", Value::null(Type::string())),
                        ])]),
                    )]),
                )]),
            ])),
        ),
        // When converting null values into nested types which include objects
        // with optional attributes, we expect the resulting value to be of a
        // recursively concretized type.
        (
            Value::null(Type::dynamic()),
            Type::object([(
                "foo",
                Type::object_with_optional_attrs([("bar", Type::string())], &["bar"]),
            )]),
            Ok(Value::null(Type::object([(
                "foo",
                Type::object([("bar", Type::string())]),
            )]))),
        ),
        // The same nested optional attributes flattening should happen for
        // unknown values, too.
        (
            Value::unknown(Type::dynamic()),
            Type::object([(
                "foo",
                Type::object_with_optional_attrs([("bar", Type::string())], &["bar"]),
            )]),
            Ok(Value::unknown(Type::object([(
                "foo",
                Type::object([("bar", Type::string())]),
            )]))),
        ),
        // https://github.com/hashicorp/terraform/issues/21588:
        (
            Value::tuple([
                Value::object([("a", Value::empty_object()), ("b", Value::number_int(2))]),
                Value::object([
                    ("a", Value::object([("var1", Value::string("val1"))])),
                    ("b", Value::string("2")),
                ]),
            ]),
            Type::list(Type::object([
                ("a", Type::dynamic()),
                ("b", Type::string()),
            ])),
            Ok(Value::list([
                Value::object([
                    ("a", Value::map_empty(Type::string())),
                    ("b", Value::string("2")),
                ]),
                Value::object([
                    ("a", Value::map([("var1", Value::string("val1"))])),
                    ("b", Value::string("2")),
                ]),
            ])),
        ),
        // https://github.com/hashicorp/terraform/issues/24377:
        (
            Value::tuple([
                Value::list([Value::string("a")]),
                Value::string("b"),
                Value::null(Type::dynamic()),
            ]),
            Type::set(Type::dynamic()),
            Err("all set elements must have the same type"),
        ),
        (
            Value::tuple([
                Value::list([Value::string("a")]),
                Value::string("b"),
                Value::null(Type::dynamic()),
            ]),
            Type::list(Type::dynamic()),
            Err("all list elements must have the same type"),
        ),
        (
            Value::tuple([Value::list([Value::string("a")]), Value::string("b")]),
            Type::set(Type::dynamic()),
            Err("all set elements must have the same type"),
        ),
        (
            Value::tuple([Value::list([Value::string("a")]), Value::string("b")]),
            Type::list(Type::dynamic()),
            Err("all list elements must have the same type"),
        ),
        (
            Value::tuple([
                Value::string("a"),
                Value::number_int(9),
                Value::null(Type::dynamic()),
            ]),
            Type::set(Type::dynamic()),
            Ok(Value::set([
                Value::string("a"),
                Value::string("9"),
                Value::null(Type::dynamic()),
            ])),
        ),
        (
            Value::tuple([
                Value::string("a"),
                Value::number_int(9),
                Value::null(Type::dynamic()),
            ]),
            Type::list(Type::dynamic()),
            Ok(Value::list([
                Value::string("a"),
                Value::string("9"),
                Value::null(Type::dynamic()),
            ])),
        ),
        (
            Value::tuple([
                Value::null(Type::dynamic()),
                Value::null(Type::dynamic()),
                Value::null(Type::dynamic()),
            ]),
            Type::set(Type::dynamic()),
            Ok(Value::set([Value::null(Type::dynamic())])),
        ),
        (
            Value::tuple([
                Value::null(Type::dynamic()),
                Value::null(Type::dynamic()),
                Value::null(Type::dynamic()),
            ]),
            Type::list(Type::dynamic()),
            Ok(Value::list([
                Value::null(Type::dynamic()),
                Value::null(Type::dynamic()),
                Value::null(Type::dynamic()),
            ])),
        ),
        (
            Value::map([
                ("a", Value::string("boop")),
                // It's okay to use a map of string to convert to this
                // target type as long as the source map does not include
                // any of the optional attributes that cannot be assigned
                // from a string.
            ]),
            Type::object_with_optional_attrs(
                [
                    ("a", Type::string()),
                    ("b", Type::string()),
                    ("c", Type::object([("d", Type::string())])),
                ],
                &["b", "c"],
            ),
            Ok(Value::object([
                ("a", Value::string("boop")),
                ("b", Value::null(Type::string())),
                ("c", Value::null(Type::object([("d", Type::string())]))),
            ])),
        ),
        (
            Value::map([("a", Value::string("boop"))]),
            Type::object_with_optional_attrs(
                [
                    ("a", Type::string()),
                    ("b", Type::string()),
                    ("c", Type::object([("d", Type::dynamic())])),
                ],
                &["b", "c"],
            ),
            Ok(Value::object([
                ("a", Value::string("boop")),
                ("b", Value::null(Type::string())),
                ("c", Value::null(Type::object([("d", Type::dynamic())]))),
            ])),
        ),
        (
            Value::map([("a", Value::string("boop"))]),
            Type::object_with_optional_attrs(
                [
                    ("a", Type::string()),
                    ("b", Type::string()),
                    ("c", Type::dynamic()),
                ],
                &["b", "c"],
            ),
            Ok(Value::object([
                ("a", Value::string("boop")),
                ("b", Value::null(Type::string())),
                ("c", Value::null(Type::dynamic())),
            ])),
        ),
        (
            Value::map([
                ("a", Value::string("boop")),
                // This case is invalid, because an element of a map of
                // string cannot be assigned to an object-typed attribute.
                ("c", Value::string("foobar")),
            ]),
            Type::object_with_optional_attrs(
                [
                    ("a", Type::string()),
                    ("b", Type::string()),
                    ("c", Type::object([("d", Type::string())])),
                ],
                &["b", "c"],
            ),
            Err(
                r#"map element type is incompatible with attribute "c": object required, but have string"#,
            ),
        ),
        (
            Value::tuple([
                Value::object([
                    ("d", Value::number_float(10.0)),
                    (
                        "c",
                        Value::object([("a", Value::string("foo")), ("b", Value::bool(true))]),
                    ),
                ]),
                Value::object([
                    ("d", Value::number_float(5.0)),
                    (
                        "c",
                        Value::null(Type::object_with_optional_attrs(
                            [("a", Type::string()), ("b", Type::bool())],
                            &["b"],
                        )),
                    ),
                ]),
            ]),
            Type::set(Type::object_with_optional_attrs(
                [
                    (
                        "c",
                        Type::object_with_optional_attrs(
                            [("a", Type::string()), ("b", Type::bool())],
                            &["b"],
                        ),
                    ),
                    ("d", Type::number()),
                ],
                &["c"],
            )),
            Ok(Value::set([
                Value::object([
                    ("d", Value::number_float(10.0)),
                    (
                        "c",
                        Value::object([("a", Value::string("foo")), ("b", Value::bool(true))]),
                    ),
                ]),
                Value::object([
                    ("d", Value::number_float(5.0)),
                    (
                        "c",
                        Value::null(Type::object([("a", Type::string()), ("b", Type::bool())])),
                    ),
                ]),
            ])),
        ),
        (
            Value::tuple([
                Value::object([
                    ("d", Value::number_float(10.0)),
                    (
                        "c",
                        Value::object([("a", Value::string("foo")), ("b", Value::bool(true))]),
                    ),
                ]),
                Value::object([("d", Value::number_float(5.0))]),
            ]),
            Type::set(Type::object_with_optional_attrs(
                [
                    (
                        "c",
                        Type::object_with_optional_attrs(
                            [("a", Type::string()), ("b", Type::bool())],
                            &["b"],
                        ),
                    ),
                    ("d", Type::number()),
                ],
                &["c"],
            )),
            Ok(Value::set([
                Value::object([
                    ("d", Value::number_float(10.0)),
                    (
                        "c",
                        Value::object([("a", Value::string("foo")), ("b", Value::bool(true))]),
                    ),
                ]),
                Value::object([
                    ("d", Value::number_float(5.0)),
                    (
                        "c",
                        Value::null(Type::object([("a", Type::string()), ("b", Type::bool())])),
                    ),
                ]),
            ])),
        ),
        (
            Value::map([("a", Value::string("boop"))]),
            Type::object_with_optional_attrs(
                [
                    ("a", Type::string()),
                    ("b", Type::string()),
                    ("c", Type::object([("d", Type::string())])),
                ],
                &["b", "c"],
            ),
            Ok(Value::object([
                ("a", Value::string("boop")),
                ("b", Value::null(Type::string())),
                ("c", Value::null(Type::object([("d", Type::string())]))),
            ])),
        ),
        (
            Value::list([
                Value::object([(
                    "xs",
                    Value::list([Value::object([("x", Value::number_float(1234.0))])]),
                )]),
                Value::object([(
                    "xs",
                    Value::list_empty(Type::object([("x", Type::number())])),
                )]),
            ]),
            Type::list(Type::object([(
                "xs",
                Type::list(Type::object_with_optional_attrs(
                    [("x", Type::number())],
                    &["x"],
                )),
            )])),
            Ok(Value::list([
                Value::object([(
                    "xs",
                    Value::list([Value::object([("x", Value::number_float(1234.0))])]),
                )]),
                Value::object([(
                    "xs",
                    Value::list_empty(Type::object([("x", Type::number())])),
                )]),
            ])),
        ),
        (
            Value::set([
                Value::object([(
                    "xs",
                    Value::set([Value::object([("x", Value::number_float(1234.0))])]),
                )]),
                Value::object([(
                    "xs",
                    Value::set_empty(Type::object([("x", Type::number())])),
                )]),
            ]),
            Type::set(Type::object([(
                "xs",
                Type::set(Type::object_with_optional_attrs(
                    [("x", Type::number())],
                    &["x"],
                )),
            )])),
            Ok(Value::set([
                Value::object([(
                    "xs",
                    Value::set([Value::object([("x", Value::number_float(1234.0))])]),
                )]),
                Value::object([(
                    "xs",
                    Value::set_empty(Type::object([("x", Type::number())])),
                )]),
            ])),
        ),
        (
            Value::map([
                (
                    "foo",
                    Value::object([(
                        "xs",
                        Value::map([(
                            "nested_foo",
                            Value::object([("x", Value::number_float(1234.0))]),
                        )]),
                    )]),
                ),
                (
                    "bar",
                    Value::object([(
                        "xs",
                        Value::map_empty(Type::object([("x", Type::number())])),
                    )]),
                ),
            ]),
            Type::map(Type::object([(
                "xs",
                Type::map(Type::object_with_optional_attrs(
                    [("x", Type::number())],
                    &["x"],
                )),
            )])),
            Ok(Value::map([
                (
                    "foo",
                    Value::object([(
                        "xs",
                        Value::map([(
                            "nested_foo",
                            Value::object([("x", Value::number_float(1234.0))]),
                        )]),
                    )]),
                ),
                (
                    "bar",
                    Value::object([(
                        "xs",
                        Value::map_empty(Type::object([("x", Type::number())])),
                    )]),
                ),
            ])),
        ),
        // We should strip optional attributes out of empty sets, maps, lists,
        // and tuples.
        (
            Value::list_empty(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Type::set(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::set_empty(Type::object([("a", Type::string())]))),
        ),
        (
            Value::empty_tuple(),
            Type::set(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::set_empty(Type::object([("a", Type::string())]))),
        ),
        (
            Value::set_empty(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Type::list(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::list_empty(Type::object([("a", Type::string())]))),
        ),
        (
            Value::empty_tuple(),
            Type::list(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::list_empty(Type::object([("a", Type::string())]))),
        ),
        (
            Value::empty_object(),
            Type::map(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::map_empty(Type::object([("a", Type::string())]))),
        ),
        (
            Value::map_empty(Type::string()),
            Type::object_with_optional_attrs([("a", Type::string())], &["a"]),
            Ok(Value::object([("a", Value::null(Type::string()))])),
        ),
        // We should strip optional attributes out of null sets, maps, lists,
        // and tuples.
        (
            Value::null(Type::list(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            ))),
            Type::set(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::null(Type::set(Type::object([(
                "a",
                Type::string(),
            )])))),
        ),
        (
            Value::null(Type::empty_tuple()),
            Type::set(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::null(Type::set(Type::object([(
                "a",
                Type::string(),
            )])))),
        ),
        (
            Value::null(Type::set(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            ))),
            Type::list(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::null(Type::list(Type::object([(
                "a",
                Type::string(),
            )])))),
        ),
        (
            Value::null(Type::empty_tuple()),
            Type::list(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::null(Type::list(Type::object([(
                "a",
                Type::string(),
            )])))),
        ),
        (
            Value::null(Type::empty_object()),
            Type::map(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::null(Type::map(Type::object([(
                "a",
                Type::string(),
            )])))),
        ),
        (
            Value::null(Type::map(Type::string())),
            Type::object_with_optional_attrs([("a", Type::string())], &["a"]),
            Ok(Value::null(Type::object([("a", Type::string())]))),
        ),
        // We should strip optional attributes out of null values in sets, maps,
        // lists and tuples.
        (
            Value::list([Value::null(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            ))]),
            Type::set(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::set([Value::null(Type::object([(
                "a",
                Type::string(),
            )]))])),
        ),
        (
            Value::tuple([Value::null(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            ))]),
            Type::set(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::set([Value::null(Type::object([(
                "a",
                Type::string(),
            )]))])),
        ),
        (
            Value::set([Value::null(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            ))]),
            Type::list(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::list([Value::null(Type::object([(
                "a",
                Type::string(),
            )]))])),
        ),
        (
            Value::tuple([Value::null(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            ))]),
            Type::list(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::list([Value::null(Type::object([(
                "a",
                Type::string(),
            )]))])),
        ),
        (
            Value::object([(
                "object",
                Value::null(Type::object_with_optional_attrs(
                    [("a", Type::string())],
                    &["a"],
                )),
            )]),
            Type::map(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::map([(
                "object",
                Value::null(Type::object([("a", Type::string())])),
            )])),
        ),
        (
            Value::map([(
                "object",
                Value::null(Type::object_with_optional_attrs(
                    [("a", Type::string())],
                    &["a"],
                )),
            )]),
            Type::object([(
                "object",
                Type::object_with_optional_attrs([("a", Type::string())], &["a"]),
            )]),
            Ok(Value::object([(
                "object",
                Value::null(Type::object([("a", Type::string())])),
            )])),
        ),
        (
            Value::map([(
                "object",
                Value::null(Type::object_with_optional_attrs(
                    [("a", Type::number())],
                    &["a"],
                )),
            )]),
            Type::map(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::map([(
                "object",
                Value::null(Type::object([("a", Type::string())])),
            )])),
        ),
        (
            Value::tuple([Value::null(Type::object_with_optional_attrs(
                [("a", Type::number())],
                &["a"],
            ))]),
            Type::tuple([Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )]),
            Ok(Value::tuple([Value::null(Type::object([(
                "a",
                Type::string(),
            )]))])),
        ),
        // Collections should prefer concrete types over dynamic types.
        (
            Value::list_empty(Type::number()),
            Type::list(Type::dynamic()),
            Ok(Value::list_empty(Type::number())),
        ),
        (
            Value::null(Type::list(Type::number())),
            Type::list(Type::dynamic()),
            Ok(Value::null(Type::list(Type::number()))),
        ),
        (
            Value::null(Type::list(Type::number())),
            Type::set(Type::dynamic()),
            Ok(Value::null(Type::set(Type::number()))),
        ),
        (
            Value::map_empty(Type::number()),
            Type::map(Type::dynamic()),
            Ok(Value::map_empty(Type::number())),
        ),
        (
            Value::null(Type::map(Type::number())),
            Type::map(Type::dynamic()),
            Ok(Value::null(Type::map(Type::number()))),
        ),
        (
            Value::null(Type::map(Type::number())),
            Type::object([("a", Type::dynamic())]),
            Ok(Value::null(Type::object([("a", Type::number())]))),
        ),
        (
            Value::set_empty(Type::number()),
            Type::set(Type::dynamic()),
            Ok(Value::set_empty(Type::number())),
        ),
        (
            Value::null(Type::set(Type::number())),
            Type::set(Type::dynamic()),
            Ok(Value::null(Type::set(Type::number()))),
        ),
        (
            Value::null(Type::set(Type::number())),
            Type::list(Type::dynamic()),
            Ok(Value::null(Type::list(Type::number()))),
        ),
        (
            Value::null(Type::object([("a", Type::string())])),
            Type::map(Type::dynamic()),
            Ok(Value::null(Type::map(Type::string()))),
        ),
        (
            Value::null(Type::object([("a", Type::object([("b", Type::string())]))])),
            Type::object([("a", Type::object([("b", Type::dynamic())]))]),
            Ok(Value::null(Type::object([(
                "a",
                Type::object([("b", Type::string())]),
            )]))),
        ),
        (
            Value::null(Type::tuple([Type::string()])),
            Type::tuple([Type::dynamic()]),
            Ok(Value::null(Type::tuple([Type::string()]))),
        ),
        // We should strip optional attributes out of types even if they match.
        (
            Value::map([(
                "object",
                Value::null(Type::object_with_optional_attrs(
                    [("a", Type::string())],
                    &["a"],
                )),
            )]),
            Type::map(Type::object_with_optional_attrs(
                [("a", Type::string())],
                &["a"],
            )),
            Ok(Value::map([(
                "object",
                Value::null(Type::object([("a", Type::string())])),
            )])),
        ),
        // Object to map refinements
        (
            Value::unknown(Type::empty_object()),
            Type::map(Type::string()),
            Ok(Value::unknown(Type::map(Type::string()))
                .refine()
                .collection_length(0)
                .new_value()),
        ),
        (
            Value::unknown(Type::empty_object()).refine_not_null(),
            Type::map(Type::string()),
            Ok(Value::map_empty(Type::string())),
        ),
        (
            Value::unknown(Type::object([("a", Type::string())])),
            Type::map(Type::string()),
            Ok(Value::unknown(Type::map(Type::string()))
                .refine()
                .collection_length(1)
                .new_value()),
        ),
        (
            Value::unknown(Type::object([("a", Type::string())])).refine_not_null(),
            Type::map(Type::string()),
            Ok(Value::unknown(Type::map(Type::string()))
                .refine()
                .not_null()
                .collection_length(1)
                .new_value()),
        ),
        // Tuple to list refinements
        (
            Value::unknown(Type::empty_tuple()),
            Type::list(Type::string()),
            Ok(Value::unknown(Type::list(Type::string()))
                .refine()
                .collection_length(0)
                .new_value()),
        ),
        (
            Value::unknown(Type::empty_tuple()).refine_not_null(),
            Type::list(Type::string()),
            Ok(Value::list_empty(Type::string())),
        ),
        (
            Value::unknown(Type::tuple([Type::string()])),
            Type::list(Type::string()),
            Ok(Value::unknown(Type::list(Type::string()))
                .refine()
                .collection_length(1)
                .new_value()),
        ),
        (
            Value::unknown(Type::tuple([Type::string()])).refine_not_null(),
            Type::list(Type::string()),
            Ok(Value::list([Value::unknown(Type::string())])),
        ),
        // Tuple to set refinements
        (
            Value::unknown(Type::empty_tuple()),
            Type::set(Type::string()),
            Ok(Value::unknown(Type::set(Type::string()))
                .refine()
                .collection_length(0)
                .new_value()),
        ),
        (
            Value::unknown(Type::empty_tuple()).refine_not_null(),
            Type::set(Type::string()),
            Ok(Value::set_empty(Type::string())),
        ),
        (
            Value::unknown(Type::tuple([Type::string()])),
            Type::set(Type::string()),
            Ok(Value::unknown(Type::set(Type::string()))
                .refine()
                .collection_length(1)
                .new_value()),
        ),
        (
            Value::unknown(Type::tuple([Type::string()])).refine_not_null(),
            Type::set(Type::string()),
            Ok(Value::set([Value::unknown(Type::string())])),
        ),
        (
            Value::unknown(Type::tuple([Type::string(), Type::string()])),
            Type::set(Type::string()),
            Ok(Value::unknown(Type::set(Type::string()))
                .refine()
                .collection_length_lower_bound(1)
                .collection_length_upper_bound(2)
                .new_value()),
        ),
        (
            Value::unknown(Type::tuple([Type::string(), Type::string()])).refine_not_null(),
            Type::set(Type::string()),
            Ok(Value::unknown(Type::set(Type::string()))
                .refine()
                .not_null()
                .collection_length_lower_bound(1)
                .collection_length_upper_bound(2)
                .new_value()),
        ),
        // Collection to collection refinements
        (
            Value::unknown(Type::list(Type::string()))
                .refine()
                .collection_length_lower_bound(2)
                .collection_length_upper_bound(4)
                .new_value(),
            Type::set(Type::string()),
            Ok(Value::unknown(Type::set(Type::string()))
                .refine()
                .collection_length_lower_bound(1)
                .collection_length_upper_bound(4)
                .new_value()),
        ),
        (
            Value::unknown(Type::list(Type::string()))
                .refine()
                .not_null()
                .collection_length_lower_bound(2)
                .collection_length_upper_bound(4)
                .new_value(),
            Type::set(Type::string()),
            Ok(Value::unknown(Type::set(Type::string()))
                .refine()
                .not_null()
                .collection_length_lower_bound(1)
                .collection_length_upper_bound(4)
                .new_value()),
        ),
        (
            Value::unknown(Type::set(Type::string()))
                .refine()
                .collection_length_lower_bound(2)
                .collection_length_upper_bound(4)
                .new_value(),
            Type::list(Type::string()),
            Ok(Value::unknown(Type::list(Type::string()))
                .refine()
                .collection_length_lower_bound(2)
                .collection_length_upper_bound(4)
                .new_value()),
        ),
        (
            Value::unknown(Type::set(Type::string()))
                .refine()
                .not_null()
                .collection_length_lower_bound(2)
                .collection_length_upper_bound(4)
                .new_value(),
            Type::list(Type::string()),
            Ok(Value::unknown(Type::list(Type::string()))
                .refine()
                .not_null()
                .collection_length_lower_bound(2)
                .collection_length_upper_bound(4)
                .new_value()),
        ),
        // General unknown value refinements
        (
            Value::unknown(Type::bool()).refine_not_null(),
            Type::string(),
            Ok(Value::unknown(Type::string()).refine_not_null()),
        ),
        // Make sure we get valid unknown attribute types when converting from
        // a map to an object with optional attributes.
        (
            Value::object([("TTTattr", Value::unknown(Type::map(Type::string())))]),
            Type::object([(
                "TTTattr",
                Type::object_with_optional_attrs(
                    [
                        ("string", Type::string()),
                        ("set", Type::set(Type::string())),
                        ("list", Type::list(Type::string())),
                        ("map", Type::map(Type::string())),
                    ],
                    &["set", "list", "map"],
                ),
            )]),
            Ok(Value::object([(
                "TTTattr",
                Value::unknown(Type::object([
                    ("list", Type::list(Type::string())),
                    ("map", Type::map(Type::string())),
                    ("set", Type::set(Type::string())),
                    ("string", Type::string()),
                ])),
            )])),
        ),
        (
            Value::tuple([
                Value::object([("optional_map", Value::empty_object())]),
                Value::object([(
                    "optional_map",
                    Value::map_empty(Type::object([("asdf", Type::string())])),
                )]),
            ]),
            Type::set(Type::object_with_optional_attrs(
                [(
                    "optional_map",
                    Type::map(Type::object_with_optional_attrs(
                        [("asdf", Type::string())],
                        &["asdf"],
                    )),
                )],
                &["optional_map"],
            )),
            Ok(Value::set([
                Value::object([(
                    "optional_map",
                    Value::map_empty(Type::object([("asdf", Type::string())])),
                )]),
                Value::object([(
                    "optional_map",
                    Value::map_empty(Type::object([("asdf", Type::string())])),
                )]),
            ])),
        ),
    ];

    for (i, (value, ty, expect)) in tests.iter().enumerate() {
        let got = convert(value, ty);
        match expect {
            Err(want_err) => match got {
                Ok(got) => {
                    panic!("case {i}: conversion succeeded with {got:?}; want error {want_err:?}")
                }
                Err(err) => assert_eq!(
                    err.to_string(),
                    *want_err,
                    "case {i}: wrong error converting {value:?} to {ty:?}"
                ),
            },
            Ok(want) => match got {
                Ok(got) => assert_eq!(
                    &got, want,
                    "case {i}: wrong result converting {value:?} to {ty:?}"
                ),
                Err(err) => panic!("case {i}: conversion failed: {err}"),
            },
        }
    }
}
