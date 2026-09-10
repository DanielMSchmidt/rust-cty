//! `lower` (go-cty: `cty/function/stdlib/string.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`lower`] (go-cty: `stdlib.LowerFunc`).
pub fn lower_func() -> Function {
    todo!()
}

/// The string converted to lowercase (go-cty: `stdlib.Lower`).
pub fn lower(str_val: &Value) -> Result<Value, CtyError> {
    let _ = str_val;
    todo!()
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/function/stdlib/string_test.go
    //!   cty/function/stdlib/string_replace_test.go
    //!   cty/function/stdlib/regexp_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::function::stdlib;
    use crate::{Type, Value};

    // Ported from TestLower:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/string_test.go#L80
    #[test]
    #[ignore = "not yet implemented"]
    fn lower() {
        let tests: Vec<(Value, Value)> = vec![
            (Value::string("HELLO"), Value::string("hello")),
            (Value::string("hello"), Value::string("hello")),
            (Value::string(""), Value::string("")),
            (Value::string("1"), Value::string("1")),
            (
                Value::string("\u{416}\u{416}"),
                Value::string("\u{436}\u{436}"),
            ),
            (
                Value::unknown(Type::string()),
                Value::unknown(Type::string()).refine_not_null(),
            ),
            (
                Value::dynamic(),
                Value::unknown(Type::string()).refine_not_null(),
            ),
        ];

        for (i, (input, want)) in tests.iter().enumerate() {
            let got = stdlib::lower(input)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
