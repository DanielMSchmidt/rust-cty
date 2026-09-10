//! `make_to` (go-cty: `cty/function/stdlib/conversion.go`).

use crate::function::Function;
use crate::types::Type;

/// A function converting its argument to the given type constraint
/// (go-cty: `stdlib.MakeToFunc`).
pub fn make_to_func(want_ty: Type) -> Function {
    let _ = want_ty;
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

    // Ported from TestTo:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/conversion_test.go#L10
    #[test]
    #[ignore = "not yet implemented"]
    fn to() {
        let tests: Vec<(Value, Type, Value, &str)> = vec![
            (Value::string("a"), Type::string(), Value::string("a"), ""),
            (
                Value::unknown(Type::string()),
                Type::string(),
                Value::unknown(Type::string()),
                "",
            ),
            (
                Value::null(Type::string()),
                Type::string(),
                Value::null(Type::string()),
                "",
            ),
            (Value::bool(true), Type::string(), Value::string("true"), ""),
            (
                Value::string("a"),
                Type::bool(),
                Value::dynamic(),
                r#"cannot convert "a" to bool; only the strings "true" or "false" are allowed"#,
            ),
            (
                Value::string("a"),
                Type::number(),
                Value::dynamic(),
                r#"cannot convert "a" to number; given string must be a decimal representation of a number"#,
            ),
            (
                Value::null(Type::string()),
                Type::number(),
                Value::null(Type::number()),
                "",
            ),
            (
                Value::null(Type::dynamic()),
                Type::number(),
                Value::null(Type::number()),
                "",
            ),
            (
                Value::unknown(Type::bool()),
                Type::string(),
                Value::unknown(Type::string()),
                "",
            ),
            (
                Value::unknown(Type::string()),
                Type::bool(),
                Value::unknown(Type::bool()), // conversion is optimistic
                "",
            ),
            (
                Value::tuple([Value::string("hello"), Value::bool(true)]),
                Type::list(Type::string()),
                Value::list([Value::string("hello"), Value::string("true")]),
                "",
            ),
            (
                Value::tuple([Value::string("hello"), Value::bool(true)]),
                Type::set(Type::string()),
                Value::set([Value::string("hello"), Value::string("true")]),
                "",
            ),
            (
                Value::object([("foo", Value::string("hello")), ("bar", Value::bool(true))]),
                Type::map(Type::string()),
                Value::map([
                    ("foo", Value::string("hello")),
                    ("bar", Value::string("true")),
                ]),
                "",
            ),
            (
                Value::empty_tuple(),
                Type::string(),
                Value::dynamic(),
                "cannot convert tuple to string",
            ),
            (
                Value::unknown(Type::empty_tuple()),
                Type::string(),
                Value::dynamic(),
                "cannot convert tuple to string",
            ),
            (
                Value::empty_object(),
                Type::object([("foo", Type::string())]),
                Value::dynamic(),
                r#"incompatible object type for conversion: attribute "foo" is required"#,
            ),
        ];

        for (i, (value, target_ty, want, want_err)) in tests.iter().enumerate() {
            let f = stdlib::make_to_func(target_ty.clone());
            let result = f.call(std::slice::from_ref(value));

            if !want_err.is_empty() {
                let err = match result {
                    Ok(_) => panic!("case {i}: succeeded; want error"),
                    Err(err) => err,
                };
                assert_eq!(
                    err.to_string(),
                    *want_err,
                    "case {i}: wrong error\ngot:  {err}\nwant: {want_err}"
                );
                continue;
            }

            let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
