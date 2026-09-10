//! `regex_replace` (go-cty: `cty/function/stdlib/string_replace.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`regex_replace`] (go-cty: `stdlib.RegexReplaceFunc`).
pub fn regex_replace_func() -> Function {
    todo!()
}

/// The string with all matches of a regular expression pattern replaced
/// (go-cty: `stdlib.RegexReplace`).
pub fn regex_replace(str_val: &Value, pattern: &Value, replace: &Value) -> Result<Value, CtyError> {
    let _ = (str_val, pattern, replace);
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

    use crate::Value;
    use crate::function::stdlib;

    // Ported from TestRegexReplace:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/string_replace_test.go#L61
    #[test]
    #[ignore = "not yet implemented"]
    fn regex_replace() {
        let tests: Vec<(Value, Value, Value, Value)> = vec![
            (
                Value::string("-ab-axxb-"),
                Value::string("a(x*)b"),
                Value::string("T"),
                Value::string("-T-T-"),
            ),
            (
                Value::string("-ab-axxb-"),
                Value::string("a(x*)b"),
                Value::string("${1}W"),
                Value::string("-W-xxW-"),
            ),
        ];

        for (i, (input, substr, replace, want)) in tests.iter().enumerate() {
            let got = stdlib::regex_replace(input, substr, replace)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }

    // Ported from TestRegexReplaceInvalidRegex:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/string_replace_test.go#L96
    #[test]
    #[ignore = "not yet implemented"]
    fn regex_replace_invalid_regex() {
        let result =
            stdlib::regex_replace(&Value::string(""), &Value::string("("), &Value::string(""));
        assert!(result.is_err(), "expected an error");
    }
}
