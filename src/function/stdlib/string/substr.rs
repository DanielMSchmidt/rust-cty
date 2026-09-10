//! `substr` (go-cty: `cty/function/stdlib/string.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`substr`] (go-cty: `stdlib.SubstrFunc`).
pub fn substr_func() -> Function {
    todo!()
}

/// A substring by grapheme-cluster offset and length
/// (go-cty: `stdlib.Substr`).
pub fn substr(str_val: &Value, offset: &Value, length: &Value) -> Result<Value, CtyError> {
    let _ = (str_val, offset, length);
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

    // Ported from TestSubstr:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/string_test.go#L276
    #[test]
    #[ignore = "not yet implemented"]
    fn substr() {
        let tests: Vec<(Value, Value, Value, Value)> = vec![
            (
                Value::string("hello"),
                Value::number(0),
                Value::number(2),
                Value::string("he"),
            ),
            (
                Value::string("hello"),
                Value::number(1),
                Value::number(3),
                Value::string("ell"),
            ),
            (
                Value::string("hello"),
                Value::number(1),
                Value::number(-1),
                Value::string("ello"),
            ),
            (
                Value::string("hello"),
                Value::number(1),
                Value::number(-10), // not documented, but <0 is the same as -1
                Value::string("ello"),
            ),
            (
                Value::string("hello"),
                Value::number(1),
                Value::number(10),
                Value::string("ello"),
            ),
            (
                Value::string("hello"),
                Value::number(-3),
                Value::number(-1),
                Value::string("llo"),
            ),
            (
                Value::string("hello"),
                Value::number(-3),
                Value::number(2),
                Value::string("ll"),
            ),
            (
                Value::string("hello"),
                Value::number(10),
                Value::number(10),
                Value::string(""),
            ),
            (
                Value::string("hello"),
                Value::number(0),
                Value::number(0),
                Value::string(""),
            ),
            (
                Value::string("noe\u{308}l"),
                Value::number(0),
                Value::number(3),
                Value::string("noe\u{308}"),
            ),
            (
                Value::string("noe\u{308}l"),
                Value::number(3),
                Value::number(-1),
                Value::string("l"),
            ),
            (
                Value::string(
                    "we\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}!",
                ),
                Value::number(2),
                Value::number(2),
                Value::string("e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}"),
            ),
            (
                Value::string(
                    "we\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}!",
                ),
                Value::number(3),
                Value::number(2),
                Value::string("e\u{301}\u{301}\u{301}!"),
            ),
            (
                Value::string(
                    "we\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}e\u{301}\u{301}\u{301}!",
                ),
                Value::number(-2),
                Value::number(-1),
                Value::string("e\u{301}\u{301}\u{301}!"),
            ),
            (
                Value::string("noe\u{308}l"),
                Value::number(-2),
                Value::number(-1),
                Value::string("e\u{308}l"),
            ),
            (
                Value::string("\u{1f638}\u{1f63e}"),
                Value::number(0),
                Value::number(1),
                Value::string("\u{1f638}"),
            ),
            (
                Value::string("\u{1f638}\u{1f63e}"),
                Value::number(1),
                Value::number(1),
                Value::string("\u{1f63e}"),
            ),
        ];

        for (i, (input, offset, length, want)) in tests.iter().enumerate() {
            let got = stdlib::substr(input, offset, length)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
