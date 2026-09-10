//! `csv_decode` (go-cty: `cty/function/stdlib/csv.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`csv_decode`] (go-cty: `stdlib.CSVDecodeFunc`).
pub fn csv_decode_func() -> Function {
    todo!()
}

/// Parses a CSV document into a list of objects, one per row
/// (go-cty: `stdlib.CSVDecode`).
pub fn csv_decode(str_val: &Value) -> Result<Value, CtyError> {
    let _ = str_val;
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

    const CSV_TEST: &str = r#""name","size","type"
    "foo","100","tiny"
    "bar","","huge"
    "baz","50","weedy"
    "#;

    // Ported from TestCSVDecode:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/csv_test.go#L10
    #[test]
    #[ignore = "not yet implemented"]
    fn csv_decode() {
        let tests: Vec<(Value, Value, &str)> = vec![
            (
                Value::string(CSV_TEST),
                Value::list([
                    Value::object([
                        ("name", Value::string("foo")),
                        ("size", Value::string("100")),
                        ("type", Value::string("tiny")),
                    ]),
                    Value::object([
                        ("name", Value::string("bar")),
                        ("size", Value::string("")),
                        ("type", Value::string("huge")),
                    ]),
                    Value::object([
                        ("name", Value::string("baz")),
                        ("size", Value::string("50")),
                        ("type", Value::string("weedy")),
                    ]),
                ]),
                "",
            ),
            (
                Value::string(r#""just","header","line""#),
                Value::list_empty(Type::object([
                    ("just", Type::string()),
                    ("header", Type::string()),
                    ("line", Type::string()),
                ])),
                "",
            ),
            (Value::string(""), Value::dynamic(), "missing header line"),
            (
                Value::string("not csv at all"),
                Value::list_empty(Type::object([("not csv at all", Type::string())])),
                "",
            ),
            (
                Value::string(r#"invalid"thing""#),
                Value::dynamic(),
                r#"CSV parse error on line 1: bare " in non-quoted-field"#,
            ),
            (
                Value::unknown(Type::string()),
                Value::dynamic(), // need to know the value to determine the type
                "",
            ),
            (Value::dynamic(), Value::dynamic(), ""),
            (
                Value::bool(true),
                Value::dynamic(),
                "string required, but received bool",
            ),
            (
                Value::null(Type::string()),
                Value::dynamic(),
                "argument must not be null",
            ),
        ];

        for (i, (input, want, want_err)) in tests.iter().enumerate() {
            let result = stdlib::csv_decode(input);
            match result {
                Err(err) => {
                    assert_eq!(
                        err.to_string(),
                        *want_err,
                        "case {i}: wrong error\ngot:  {err}\nwant: {want_err}"
                    );
                }
                Ok(got) => {
                    assert_eq!(
                        *want_err, "",
                        "case {i}: succeeded; want error {want_err:?}"
                    );
                    assert_eq!(got, *want, "case {i}: wrong result");
                }
            }
        }
    }
}
