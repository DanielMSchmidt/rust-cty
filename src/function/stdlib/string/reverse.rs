//! `reverse` (go-cty: `cty/function/stdlib/string.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`reverse`] (go-cty: `stdlib.ReverseFunc`).
pub fn reverse_func() -> Function {
    todo!()
}

/// The string with its grapheme clusters in reverse order
/// (go-cty: `stdlib.Reverse`).
pub fn reverse(str_val: &Value) -> Result<Value, CtyError> {
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

    // Ported from TestReverse:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/string_test.go#L130
    #[test]
    #[ignore = "not yet implemented"]
    fn reverse() {
        let tests: Vec<(Value, Value)> = vec![
            (Value::string("hello"), Value::string("olleh")),
            (Value::string(""), Value::string("")),
            (Value::string("1"), Value::string("1")),
            (
                Value::string(
                    "\u{416}\u{438}\u{432}\u{43e}\u{439} \u{416}\u{443}\u{440}\u{43d}\u{430}\u{43b}",
                ),
                Value::string(
                    "\u{43b}\u{430}\u{43d}\u{440}\u{443}\u{416} \u{439}\u{43e}\u{432}\u{438}\u{416}",
                ),
            ),
            (
                // note that the dieresis here is intentionally a combining
                // ligature.
                Value::string("noe\u{308}l"),
                Value::string("le\u{308}on"),
            ),
            (
                // The Es in this string has three combining acute accents.
                // This tests something that NFC-normalization cannot collapse
                // into a single precombined codepoint, since otherwise we might
                // be cheating and relying on the single-codepoint forms.
                Value::string(
                    "we\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}!",
                ),
                Value::string(
                    "!e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}w",
                ),
            ),
            (
                // Go's normalization forms don't handle this ligature, so we
                // will produce the wrong result but this is now a compatibility
                // constraint and so we'll test it.
                Value::string("ba\u{fb04}e"),
                Value::string("e\u{fb04}ab"),
            ),
            (
                Value::string("\u{1f638}\u{1f63e}"),
                Value::string("\u{1f63e}\u{1f638}"),
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
            let got = stdlib::reverse(input)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
