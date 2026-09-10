//! `replace` (go-cty: `cty/function/stdlib/string_replace.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`replace`] (go-cty: `stdlib.ReplaceFunc`).
pub fn replace_func() -> Function {
    todo!()
}

/// The string with all occurrences of a substring replaced
/// (go-cty: `stdlib.Replace`).
pub fn replace(str_val: &Value, substr: &Value, replace: &Value) -> Result<Value, CtyError> {
    let _ = (str_val, substr, replace);
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

    // Ported from TestReplace:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/string_replace_test.go#L9
    #[test]
    #[ignore = "not yet implemented"]
    fn replace() {
        let tests: Vec<(Value, Value, Value, Value)> = vec![
            (
                Value::string("hello"),
                Value::string("l"),
                Value::string(""),
                Value::string("heo"),
            ),
            (
                Value::string("\u{1f638}\u{1f638}\u{1f638}\u{1f63e}\u{1f63e}\u{1f63e}"),
                Value::string("\u{1f63e}"),
                Value::string("\u{1f638}"),
                Value::string("\u{1f638}\u{1f638}\u{1f638}\u{1f638}\u{1f638}\u{1f638}"),
            ),
            (
                Value::string("\u{1f638}\u{1f638}\u{1f638}\u{1f638}\u{1f638}\u{1f63e}"),
                Value::string("\u{1f63e}"),
                Value::string("\u{1f638}"),
                Value::string("\u{1f638}\u{1f638}\u{1f638}\u{1f638}\u{1f638}\u{1f638}"),
            ),
        ];

        for (i, (input, substr, replace, want)) in tests.iter().enumerate() {
            // Upstream runs each case as two subtests ("_replace" and
            // "_regex_replace"), both of which call Replace; ported faithfully.
            {
                let got = stdlib::replace(input, substr, replace)
                    .unwrap_or_else(|err| panic!("case {i} (replace): unexpected error: {err}"));
                assert_eq!(got, *want, "case {i} (replace): wrong result");
            }
            {
                let got = stdlib::replace(input, substr, replace).unwrap_or_else(|err| {
                    panic!("case {i} (regex_replace): unexpected error: {err}")
                });
                assert_eq!(got, *want, "case {i} (regex_replace): wrong result");
            }
        }
    }
}
