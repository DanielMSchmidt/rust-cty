//! Type-directed JSON serialization: the analogue of go-cty's `cty/json`
//! package.
//!
//! JSON documents are handled as `String`s rather than byte slices; the
//! upstream tests' expected outputs port over verbatim.

use crate::error::CtyError;
use crate::types::Type;
use crate::value::Value;

/// Serializes a value as JSON, guided by the given type (which may include
/// dynamic pseudo-types, encoded as type/value pairs)
/// (go-cty: `json.Marshal`).
pub fn marshal(value: &Value, ty: &Type) -> Result<String, CtyError> {
    let _ = (value, ty);
    todo!()
}

/// Decodes a JSON document into a value of the given type
/// (go-cty: `json.Unmarshal`).
pub fn unmarshal(json: &str, ty: &Type) -> Result<Value, CtyError> {
    let _ = (json, ty);
    todo!()
}

/// Serializes a type itself as JSON (go-cty: `json.MarshalType` and
/// `cty.Type`'s `MarshalJSON`).
pub fn marshal_type(ty: &Type) -> Result<String, CtyError> {
    let _ = ty;
    todo!()
}

/// Decodes a JSON type description back into a type
/// (go-cty: `json.UnmarshalType` and `cty.Type`'s `UnmarshalJSON`).
pub fn unmarshal_type(json: &str) -> Result<Type, CtyError> {
    let _ = json;
    todo!()
}

/// The most suitable cty type to represent the given JSON document, such that
/// decoding into it loses no information (go-cty: `json.ImpliedType`).
pub fn implied_type(json: &str) -> Result<Type, CtyError> {
    let _ = json;
    todo!()
}

/// A value wrapper that serializes with an implied type rather than an
/// explicit one, for embedding cty values in ordinary JSON structures
/// (go-cty: `json.SimpleJSONValue`).
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleValue(pub Value);

impl SimpleValue {
    /// Serializes the wrapped value as JSON using its implied representation
    /// (go-cty: `SimpleJSONValue.MarshalJSON`).
    pub fn to_json(&self) -> Result<String, CtyError> {
        todo!()
    }

    /// Decodes JSON into a value of the implied type
    /// (go-cty: `SimpleJSONValue.UnmarshalJSON`).
    pub fn from_json(json: &str) -> Result<SimpleValue, CtyError> {
        let _ = json;
        todo!()
    }
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/json_test.go
    //!   cty/json/simple_test.go
    //!   cty/json/value_test.go
    //!   cty/json/type_implied_test.go
    //!   cty/json/type_implied_json1_test.go
    //!   cty/json/type_implied_json2_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::json::{
        SimpleValue, implied_type, marshal, marshal_type, unmarshal, unmarshal_type,
    };
    use crate::{Type, Value};

    // Ported from TestTypeJSONable:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/json_test.go#L8
    #[test]
    #[ignore = "not yet implemented"]
    fn type_jsonable() {
        let tests: Vec<(Type, &str)> = vec![
            (Type::string(), r#""string""#),
            (Type::number(), r#""number""#),
            (Type::bool(), r#""bool""#),
            (Type::list(Type::bool()), r#"["list","bool"]"#),
            (Type::map(Type::bool()), r#"["map","bool"]"#),
            (Type::set(Type::bool()), r#"["set","bool"]"#),
            (
                Type::list(Type::map(Type::bool())),
                r#"["list",["map","bool"]]"#,
            ),
            (
                Type::tuple([Type::bool(), Type::string()]),
                r#"["tuple",["bool","string"]]"#,
            ),
            (
                Type::object([("bool", Type::bool()), ("string", Type::string())]),
                r#"["object",{"bool":"bool","string":"string"}]"#,
            ),
            (
                Type::object_with_optional_attrs(
                    [("bool", Type::bool()), ("string", Type::string())],
                    &["string", "bool"],
                ),
                r#"["object",{"bool":"bool","string":"string"},["bool","string"]]"#,
            ),
            (Type::dynamic(), r#""dynamic""#),
        ];

        for (i, (ty, want)) in tests.iter().enumerate() {
            let result = marshal_type(ty)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error from Marshal: {err}"));

            assert_eq!(
                result, *want,
                "case {i}: wrong result\ntype: {ty:?}\ngot:  {result}\nwant: {want}"
            );

            let got_ty = unmarshal_type(&result)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error from Unmarshal: {err}"));

            assert!(
                got_ty.equals(ty),
                "case {i}: type did not unmarshal correctly\njson: {result}\ngot:  {got_ty:?}\nwant: {ty:?}"
            );
        }
    }

    // Ported from TestSimpleJSONValue:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/json/simple_test.go#L10
    #[test]
    #[ignore = "not yet implemented"]
    fn simple_json_value() {
        let tests: Vec<(Value, &str, Value)> = vec![
            (Value::number(5), r#"5"#, Value::number(5)),
            (Value::bool(true), r#"true"#, Value::bool(true)),
            (Value::string("hello"), r#""hello""#, Value::string("hello")),
            (
                Value::tuple([Value::string("hello"), Value::bool(true)]),
                r#"["hello",true]"#,
                Value::tuple([Value::string("hello"), Value::bool(true)]),
            ),
            (
                Value::list([Value::bool(false), Value::bool(true)]),
                r#"[false,true]"#,
                Value::tuple([Value::bool(false), Value::bool(true)]),
            ),
            (
                Value::set([Value::bool(false), Value::bool(true)]),
                r#"[false,true]"#,
                Value::tuple([Value::bool(false), Value::bool(true)]),
            ),
            (
                Value::object([
                    ("true", Value::bool(true)),
                    ("greet", Value::string("hello")),
                ]),
                r#"{"greet":"hello","true":true}"#,
                Value::object([
                    ("true", Value::bool(true)),
                    ("greet", Value::string("hello")),
                ]),
            ),
            (
                Value::map([("true", Value::bool(true)), ("false", Value::bool(false))]),
                r#"{"false":false,"true":true}"#,
                Value::object([("true", Value::bool(true)), ("false", Value::bool(false))]),
            ),
            (
                Value::null(Type::bool()),
                r#"null"#,
                Value::null(Type::dynamic()), // type is lost in the round-trip
            ),
        ];

        for (i, (input, json, want)) in tests.iter().enumerate() {
            let wrapped_input = SimpleValue(input.clone());
            let buf = wrapped_input.to_json().unwrap_or_else(|err| {
                panic!("case {i}: unexpected error from json.Marshal: {err}")
            });
            assert_eq!(
                buf, *json,
                "case {i}: incorrect JSON\ninput: {input:?}\ngot:   {buf}\nwant:  {json}"
            );

            let wrapped_output = SimpleValue::from_json(&buf).unwrap_or_else(|err| {
                panic!("case {i}: unexpected error from json.Unmarshal: {err}")
            });

            assert_eq!(
                wrapped_output.0, *want,
                "case {i}: incorrect result\nJSON:  {buf}\ngot:   {:?}\nwant:  {want:?}",
                wrapped_output.0
            );
        }
    }

    /// The capsule type used by the upstream table's `bytes` cases
    /// (`cty/json/value_test.go` encapsulates a `*[]byte`).
    fn bytes_type() -> Type {
        Type::capsule::<Vec<u8>>("bytes")
    }

    // Ported from TestValueJSONable:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/json/value_test.go#L11
    //
    // Upstream TestValueJSONable is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod value_jsonable {
        use super::*;

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[(Value, Type, &str, Value)]) {
            for (i, (value, ty, want, dec_val)) in tests.iter().enumerate() {
                let got = marshal(value, ty)
                    .unwrap_or_else(|err| panic!("case {i}: unexpected error from Marshal: {err}"));

                assert_eq!(
                    got, *want,
                    "case {i}: wrong serialization\nvalue: {value:?}\ntype:  {ty:?}\ngot:   {got}\nwant:  {want}"
                );

                let new_val = unmarshal(&got, ty).unwrap_or_else(|err| {
                    panic!("case {i}: unexpected error from Unmarshal: {err}")
                });

                // If we're dealing with our capsule type then we need to do some
                // more manual comparison because capsule values compare by
                // pointer identity but pointers don't survive marshalling.
                if new_val.ty().equals(&bytes_type()) {
                    let got_buf = new_val
                        .encapsulated_value()
                        .downcast_ref::<Vec<u8>>()
                        .unwrap();
                    let want_buf = dec_val
                        .encapsulated_value()
                        .downcast_ref::<Vec<u8>>()
                        .unwrap();
                    assert_eq!(
                        got_buf, want_buf,
                        "case {i}: mismatch after Unmarshal\njson: {got}\ntype: {ty:?}\ngot:  {new_val:?}\nwant: {value:?}"
                    );
                } else {
                    assert_eq!(
                        new_val, *dec_val,
                        "case {i}: mismatch after Unmarshal\njson: {got}\ntype: {ty:?}\ngot:  {new_val:?}\nwant: {value:?}"
                    );
                }
            }
        }

        // TestValueJSONable, primitives:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/json/value_test.go#L11
        #[test]
        #[ignore = "not yet implemented"]
        fn primitives() {
            let tests: Vec<(Value, Type, &str, Value)> = vec![
                // Primitives
                (
                    Value::string("hello"),
                    Type::string(),
                    r#""hello""#,
                    Value::string("hello"),
                ),
                (
                    Value::string(""),
                    Type::string(),
                    r#""""#,
                    Value::string(""),
                ),
                (
                    Value::string("15"),
                    Type::number(),
                    r#"15"#,
                    Value::number(15),
                ),
                (
                    Value::string("true"),
                    Type::bool(),
                    r#"true"#,
                    Value::bool(true),
                ),
                (
                    Value::string("1"),
                    Type::bool(),
                    r#"true"#,
                    Value::bool(true),
                ),
                (
                    Value::null(Type::string()),
                    Type::string(),
                    r#"null"#,
                    Value::null(Type::string()),
                ),
                (Value::number(2), Type::number(), r#"2"#, Value::number(2)),
                (
                    Value::number(2.5),
                    Type::number(),
                    r#"2.5"#,
                    Value::number(2.5),
                ),
                (
                    Value::number(5),
                    Type::string(),
                    r#""5""#,
                    Value::string("5"),
                ),
                (
                    Value::bool(true),
                    Type::bool(),
                    r#"true"#,
                    Value::bool(true),
                ),
                (
                    Value::bool(false),
                    Type::bool(),
                    r#"false"#,
                    Value::bool(false),
                ),
                (
                    Value::bool(true),
                    Type::string(),
                    r#""true""#,
                    Value::string("true"),
                ),
            ];

            check(&tests);
        }

        // TestValueJSONable, lists:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/json/value_test.go#L11
        #[test]
        #[ignore = "not yet implemented"]
        fn lists() {
            let tests: Vec<(Value, Type, &str, Value)> = vec![
                // Lists
                (
                    Value::list([Value::bool(true), Value::bool(false)]),
                    Type::list(Type::bool()),
                    r#"[true,false]"#,
                    Value::list([Value::bool(true), Value::bool(false)]),
                ),
                (
                    Value::list_empty(Type::bool()),
                    Type::list(Type::bool()),
                    r#"[]"#,
                    Value::list_empty(Type::bool()),
                ),
                (
                    Value::list([Value::bool(true), Value::bool(false)]),
                    Type::list(Type::string()),
                    r#"["true","false"]"#,
                    Value::list([Value::string("true"), Value::string("false")]),
                ),
            ];

            check(&tests);
        }

        // TestValueJSONable, sets:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/json/value_test.go#L11
        #[test]
        #[ignore = "not yet implemented"]
        fn sets() {
            let tests: Vec<(Value, Type, &str, Value)> = vec![
                // Sets
                (
                    Value::set([Value::bool(true), Value::bool(false)]),
                    Type::set(Type::bool()),
                    r#"[false,true]"#,
                    Value::set([Value::bool(true), Value::bool(false)]),
                ),
                (
                    Value::set_empty(Type::bool()),
                    Type::set(Type::bool()),
                    r#"[]"#,
                    Value::set_empty(Type::bool()),
                ),
            ];

            check(&tests);
        }

        // TestValueJSONable, tuples:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/json/value_test.go#L11
        #[test]
        #[ignore = "not yet implemented"]
        fn tuples() {
            let tests: Vec<(Value, Type, &str, Value)> = vec![
                // Tuples
                (
                    Value::tuple([Value::bool(true), Value::number(5)]),
                    Type::tuple([Type::bool(), Type::number()]),
                    r#"[true,5]"#,
                    Value::tuple([Value::bool(true), Value::number(5)]),
                ),
                (
                    Value::empty_tuple(),
                    Type::empty_tuple(),
                    r#"[]"#,
                    Value::empty_tuple(),
                ),
            ];

            check(&tests);
        }

        // TestValueJSONable, maps:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/json/value_test.go#L11
        #[test]
        #[ignore = "not yet implemented"]
        fn maps() {
            let tests: Vec<(Value, Type, &str, Value)> = vec![
                // Maps
                (
                    Value::map_empty(Type::bool()),
                    Type::map(Type::bool()),
                    r#"{}"#,
                    Value::map_empty(Type::bool()),
                ),
                (
                    Value::map([("yes", Value::bool(true)), ("no", Value::bool(false))]),
                    Type::map(Type::bool()),
                    r#"{"no":false,"yes":true}"#,
                    Value::map([("yes", Value::bool(true)), ("no", Value::bool(false))]),
                ),
                (
                    Value::null(Type::map(Type::bool())),
                    Type::map(Type::bool()),
                    r#"null"#,
                    Value::null(Type::map(Type::bool())),
                ),
            ];

            check(&tests);
        }

        // TestValueJSONable, objects:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/json/value_test.go#L11
        #[test]
        #[ignore = "not yet implemented"]
        fn objects() {
            let tests: Vec<(Value, Type, &str, Value)> = vec![
                // Objects
                (
                    Value::empty_object(),
                    Type::empty_object(),
                    r#"{}"#,
                    Value::empty_object(),
                ),
                (
                    Value::object([("bool", Value::bool(true)), ("number", Value::zero())]),
                    Type::object([("bool", Type::bool()), ("number", Type::number())]),
                    r#"{"bool":true,"number":0}"#,
                    Value::object([("bool", Value::bool(true)), ("number", Value::zero())]),
                ),
            ];

            check(&tests);
        }

        // TestValueJSONable, capsules:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/json/value_test.go#L11
        #[test]
        #[ignore = "not yet implemented"]
        fn capsules() {
            // NOTE(port): upstream encapsulates a *[]byte pointing at []byte("hello");
            // the Rust analogue encapsulates a Vec<u8>.
            let buf: Vec<u8> = b"hello".to_vec();
            let bytes_val = Value::capsule(bytes_type(), buf.clone());
            // Second identical capsule for the DecVal column, since the marshalled
            // pointer identity does not survive the round-trip anyway.
            let bytes_dec_val = Value::capsule(bytes_type(), buf.clone());
            let tests: Vec<(Value, Type, &str, Value)> = vec![
                // Capsules
                (bytes_val, bytes_type(), r#""aGVsbG8=""#, bytes_dec_val),
                // Encoding into dynamic produces type information wrapper
                (
                    Value::bool(true),
                    Type::dynamic(),
                    r#"{"value":true,"type":"bool"}"#,
                    Value::bool(true),
                ),
                (
                    Value::string("hello"),
                    Type::dynamic(),
                    r#"{"value":"hello","type":"string"}"#,
                    Value::string("hello"),
                ),
                (
                    Value::number(5),
                    Type::dynamic(),
                    r#"{"value":5,"type":"number"}"#,
                    Value::number(5),
                ),
                (
                    Value::list([Value::bool(true), Value::bool(false)]),
                    Type::dynamic(),
                    r#"{"value":[true,false],"type":["list","bool"]}"#,
                    Value::list([Value::bool(true), Value::bool(false)]),
                ),
                (
                    Value::list([Value::bool(true), Value::bool(false)]),
                    Type::list(Type::dynamic()),
                    r#"[{"value":true,"type":"bool"},{"value":false,"type":"bool"}]"#,
                    Value::list([Value::bool(true), Value::bool(false)]),
                ),
                (
                    Value::object([
                        ("static", Value::bool(true)),
                        ("dynamic", Value::bool(true)),
                    ]),
                    Type::object([("static", Type::bool()), ("dynamic", Type::dynamic())]),
                    r#"{"dynamic":{"value":true,"type":"bool"},"static":true}"#,
                    Value::object([
                        ("static", Value::bool(true)),
                        ("dynamic", Value::bool(true)),
                    ]),
                ),
                (
                    Value::object([
                        ("static", Value::bool(true)),
                        ("dynamic", Value::bool(true)),
                    ]),
                    Type::dynamic(),
                    r#"{"value":{"dynamic":true,"static":true},"type":["object",{"dynamic":"bool","static":"bool"}]}"#,
                    Value::object([
                        ("static", Value::bool(true)),
                        ("dynamic", Value::bool(true)),
                    ]),
                ),
            ];

            check(&tests);
        }
    }

    // Ported from TestImpliedType:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/json/type_implied_test.go#L9
    #[test]
    #[ignore = "not yet implemented"]
    fn implied_type_test() {
        let tests: Vec<(&str, Type)> = vec![
            ("null", Type::dynamic()),
            ("1", Type::number()),
            ("1.2222222222222222222222222222222222", Type::number()),
            (
                "999999999999999999999999999999999999999999999999999999999999",
                Type::number(),
            ),
            (r#""""#, Type::string()),
            (r#""hello""#, Type::string()),
            ("true", Type::bool()),
            ("false", Type::bool()),
            ("{}", Type::empty_object()),
            (r#"{"true": true}"#, Type::object([("true", Type::bool())])),
            (
                r#"{"true": true, "name": "Ermintrude", "null": null}"#,
                Type::object([
                    ("true", Type::bool()),
                    ("name", Type::string()),
                    ("null", Type::dynamic()),
                ]),
            ),
            ("[]", Type::empty_tuple()),
            (
                "[true, 1.2, null]",
                Type::tuple([Type::bool(), Type::number(), Type::dynamic()]),
            ),
            (
                r#"[[true], [1.2], [null]]"#,
                Type::tuple([
                    Type::tuple([Type::bool()]),
                    Type::tuple([Type::number()]),
                    Type::tuple([Type::dynamic()]),
                ]),
            ),
            (
                r#"[{"true": true}, {"name": "Ermintrude"}, {"null": null}]"#,
                Type::tuple([
                    Type::object([("true", Type::bool())]),
                    Type::object([("name", Type::string())]),
                    Type::object([("null", Type::dynamic())]),
                ]),
            ),
            (
                r#"{"a": "hello", "a": "world"}"#,
                Type::object([("a", Type::string())]),
            ),
        ];

        for (i, (input, want)) in tests.iter().enumerate() {
            let got = implied_type(input)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));

            assert!(
                got.equals(want),
                "case {i}: wrong type\ninput: {input}\ngot:   {got:?}\nwant:  {want:?}"
            );
        }
    }

    // upstream: cty/json/type_implied_json1_test.go (build tag !go1.27) and
    // cty/json/type_implied_json2_test.go (build tag go1.27): upstream selects the
    // expected error message for a non-string JSON property name based on which
    // encoding/json the Go toolchain provides.
    // NOTE(port): this port pins the pre-go1.27 message (`invalid character 't'`);
    // the go1.27+ variant is `object member name must be a string`.
    const TRUE_AS_PROPERTY_NAME_ERROR: &str = "invalid character 't'";

    // Ported from TestImpliedTypeErrors:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/json/type_implied_test.go#L120
    #[test]
    #[ignore = "not yet implemented"]
    fn implied_type_errors() {
        let tests: Vec<(&str, &str)> = vec![
            (
                r#"{"a": "hello", "a": true}"#,
                r#"duplicate "a" property in JSON object"#,
            ),
            ("{}boop", "extraneous data after JSON object"),
            (
                "[!]",
                "invalid character '!' looking for beginning of value",
            ),
            ("[}", "invalid character '}' looking for beginning of value"),
            ("{true: null}", TRUE_AS_PROPERTY_NAME_ERROR),
        ];

        for (i, (input, want_error)) in tests.iter().enumerate() {
            let err = implied_type(input);
            assert!(
                err.is_err(),
                "case {i}: unexpected success\nwant error: {want_error}"
            );

            let got = err.unwrap_err().to_string();
            assert_eq!(
                got, *want_error,
                "case {i}: wrong error\ngot:  {got}\nwant: {want_error}"
            );
        }
    }
}
