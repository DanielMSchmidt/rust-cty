//! `keys` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`keys`] (go-cty: `stdlib.KeysFunc`).
pub fn keys_func() -> Function {
    todo!()
}

/// The keys of a map or object, sorted (go-cty: `stdlib.Keys`).
pub fn keys(input_map: &Value) -> Result<Value, CtyError> {
    let _ = input_map;
    todo!()
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/function/stdlib/collection_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::function::stdlib;
    use crate::{Type, Value};

    // Ported from TestKeys:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L1900
    #[test]
    #[ignore = "not yet implemented"]
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
}
