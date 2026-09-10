//! `json_encode` (go-cty: `cty/function/stdlib/json.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`json_encode`] (go-cty: `stdlib.JSONEncodeFunc`).
pub fn json_encode_func() -> Function {
    todo!()
}

/// Encodes a value as a JSON string (go-cty: `stdlib.JSONEncode`).
pub fn json_encode(val: &Value) -> Result<Value, CtyError> {
    let _ = val;
    todo!()
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/function/stdlib/bool_test.go
    //!   cty/function/stdlib/bytes_test.go
    //!   cty/function/stdlib/csv_test.go
    //!   cty/function/stdlib/conversion_test.go
    //!   cty/function/stdlib/datetime_test.go
    //!   cty/function/stdlib/general_test.go
    //!   cty/function/stdlib/json_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::function::stdlib;
    use crate::{Type, Value};

    // Ported from TestJSONEncode:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/json_test.go#L10
    #[test]
    #[ignore = "not yet implemented"]
    fn json_encode() {
        let tests: Vec<(Value, Value)> = vec![
            // This does not comprehensively test all possible inputs because
            // the underlying functions in package json already have tests of
            // their own. Here we are mainly concerned with seeing that the
            // function's definition accepts all reasonable values.
            (Value::number(15), Value::string("15")),
            (Value::string("hello"), Value::string(r#""hello""#)),
            (Value::bool(true), Value::string("true")),
            (Value::list_empty(Type::number()), Value::string("[]")),
            (
                Value::list([Value::bool(true), Value::bool(false)]),
                Value::string("[true,false]"),
            ),
            (
                Value::object([("true", Value::bool(true)), ("false", Value::bool(false))]),
                Value::string(r#"{"false":false,"true":true}"#),
            ),
            (
                Value::unknown(Type::number()),
                Value::unknown(Type::string()).refine_not_null(),
            ),
            (
                Value::object([
                    ("dunno", Value::unknown(Type::bool())),
                    ("false", Value::bool(false)),
                ]),
                Value::unknown(Type::string())
                    .refine()
                    .not_null()
                    .string_prefix_full("{")
                    .new_value(),
            ),
            (
                Value::list([Value::unknown(Type::string())]),
                Value::unknown(Type::string())
                    .refine()
                    .not_null()
                    .string_prefix_full("[")
                    .new_value(),
            ),
            (
                Value::unknown(Type::string()),
                Value::unknown(Type::string()).refine_not_null(), // Can't refine the prefix because the input might be null
            ),
            (
                Value::unknown(Type::string()).refine_not_null(),
                Value::unknown(Type::string())
                    .refine()
                    .not_null()
                    .string_prefix_full("\"")
                    .new_value(),
            ),
            (
                Value::unknown(Type::number()),
                Value::unknown(Type::string()).refine_not_null(),
            ),
            (
                Value::unknown(Type::bool()),
                Value::unknown(Type::string()).refine_not_null(),
            ),
            (
                Value::dynamic(),
                Value::unknown(Type::string()).refine_not_null(),
            ),
            (Value::null(Type::string()), Value::string("null")),
        ];

        for (i, (input, want)) in tests.iter().enumerate() {
            let got = stdlib::json_encode(input)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }

    // Ported from TestJSONDecode:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/json_test.go#L96
    #[test]
    #[ignore = "not yet implemented"]
    fn json_decode() {
        let tests: Vec<(Value, Value)> = vec![
            (Value::string("15"), Value::number(15)),
            (Value::string(r#""hello""#), Value::string("hello")),
            (Value::string("true"), Value::bool(true)),
            (Value::string("[]"), Value::empty_tuple()),
            (
                Value::string("[true,false]"),
                Value::tuple([Value::bool(true), Value::bool(false)]),
            ),
            (
                Value::string(r#"{"false":false,"true":true}"#),
                Value::object([("true", Value::bool(true)), ("false", Value::bool(false))]),
            ),
            (
                Value::unknown(Type::string()),
                Value::dynamic(), // need to know the value to determine the type
            ),
            (
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("1")
                    .new_value(),
                Value::unknown(Type::number()), // deduced from refinement
            ),
            (
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("-")
                    .new_value(),
                Value::unknown(Type::number()), // deduced from refinement
            ),
            (
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full(".")
                    .new_value(),
                Value::unknown(Type::number()), // deduced from refinement
            ),
            (
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("t")
                    .new_value(),
                Value::unknown(Type::bool()), // deduced from refinement
            ),
            (
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("f")
                    .new_value(),
                Value::unknown(Type::bool()), // deduced from refinement
            ),
            (
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("\"blurt")
                    .new_value(),
                Value::unknown(Type::string()), // deduced from refinement
            ),
            (
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("{")
                    .new_value(),
                Value::dynamic(), // can't deduce the result type, but potentially valid syntax
            ),
            (
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("[")
                    .new_value(),
                Value::dynamic(), // can't deduce the result type, but potentially valid syntax
            ),
            (Value::dynamic(), Value::dynamic()),
            (Value::string("true").mark(1), Value::bool(true).mark(1)),
        ];

        for (i, (input, want)) in tests.iter().enumerate() {
            let got = stdlib::json_decode(input)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }

        let error_tests: Vec<(Value, &str)> = vec![
            (
                Value::string("aaaa"),
                "invalid character 'a' looking for beginning of value",
            ),
            (
                Value::string("nope"),
                "invalid character 'o' in literal null (expecting 'u')", // (the 'n' looked like the beginning of 'null')
            ),
            (
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("a")
                    .new_value(),
                "a JSON document cannot begin with the character 'a'", // error deduced from refinement, despite full value being unknown
            ),
        ];

        for (i, (input, want_err)) in error_tests.iter().enumerate() {
            let err = match stdlib::json_decode(input) {
                Ok(_) => panic!("error case {i}: unexpected success"),
                Err(err) => err,
            };
            assert_eq!(
                err.to_string(),
                *want_err,
                "error case {i}: wrong error\ngot:  {err}\nwant: {want_err}"
            );
        }
    }
}
