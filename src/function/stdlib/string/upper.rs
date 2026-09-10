//! `upper` (go-cty: `cty/function/stdlib/string.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`upper`] (go-cty: `stdlib.UpperFunc`).
pub fn upper_func() -> Function {
    todo!()
}

/// The string converted to uppercase (go-cty: `stdlib.Upper`).
pub fn upper(str_val: &Value) -> Result<Value, CtyError> {
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

    // Ported from TestUpper:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/string_test.go#L10
    #[test]
    #[ignore = "not yet implemented"]
    fn upper() {
        let tests: Vec<(Value, Value)> = vec![
            (Value::string("hello"), Value::string("HELLO")),
            (Value::string("HELLO"), Value::string("HELLO")),
            (Value::string(""), Value::string("")),
            (Value::string("1"), Value::string("1")),
            (
                Value::string("\u{436}\u{436}"),
                Value::string("\u{416}\u{416}"),
            ),
            (Value::string("noe\u{308}l"), Value::string("NO\u{cb}L")),
            (
                // Go's case conversions don't handle this ligature, which is
                // unfortunate but is now a compatibility constraint since it
                // would be potentially-breaking to behave differently here in
                // future.
                Value::string("ba\u{fb04}e"),
                Value::string("BA\u{fb04}E"),
            ),
            (
                Value::string("\u{1f638}\u{1f63e}"),
                Value::string("\u{1f638}\u{1f63e}"),
            ),
            (
                Value::unknown(Type::string()),
                Value::unknown(Type::string()).refine_not_null(),
            ),
            (
                Value::dynamic(),
                Value::unknown(Type::string()).refine_not_null(),
            ),
            (
                Value::string("hello").mark(1),
                Value::string("HELLO").mark(1),
            ),
        ];

        for (i, (input, want)) in tests.iter().enumerate() {
            let got = stdlib::upper(input)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
