//! `regex` (go-cty: `cty/function/stdlib/regexp.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`regex`] (go-cty: `stdlib.RegexFunc`).
pub fn regex_func() -> Function {
    todo!()
}

/// The captures of the first match of the pattern in the string
/// (go-cty: `stdlib.Regex`).
pub fn regex(pattern: &Value, str_val: &Value) -> Result<Value, CtyError> {
    let _ = (pattern, str_val);
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

    // Ported from TestRegex:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/regexp_test.go#L10
    #[test]
    #[ignore = "not yet implemented"]
    fn regex() {
        let tests: Vec<(Value, Value, Value)> = vec![
            (
                Value::string("[a-z]+"),
                Value::string("135abc456def789"),
                Value::string("abc"),
            ),
            (
                Value::string("([0-9]*)([a-z]*)"),
                Value::string("135abc456def"),
                Value::tuple([Value::string("135"), Value::string("abc")]),
            ),
            (
                Value::string(
                    r"^(?:(?P<scheme>[^:/?#]+):)?(?://(?P<authority>[^/?#]*))?(?P<path>[^?#]*)(?:\?(?P<query>[^#]*))?(?:#(?P<fragment>.*))?",
                ),
                Value::string("http://www.ics.uci.edu/pub/ietf/uri/#Related"),
                Value::object([
                    ("scheme", Value::string("http")),
                    ("authority", Value::string("www.ics.uci.edu")),
                    ("path", Value::string("/pub/ietf/uri/")),
                    // query portion isn't present at all, because there's no ?
                    ("query", Value::null(Type::string())),
                    ("fragment", Value::string("Related")),
                ]),
            ),
            (
                Value::string("([0-9]*)([a-z]*)"),
                Value::unknown(Type::string()),
                Value::unknown(Type::tuple([Type::string(), Type::string()])).refine_not_null(),
            ),
            (
                Value::string("(?P<num>[0-9]*)"),
                Value::unknown(Type::string()),
                Value::unknown(Type::object([("num", Type::string())])).refine_not_null(),
            ),
            (
                Value::unknown(Type::string()),
                Value::string("135abc456def"),
                Value::dynamic(),
            ),
            (
                Value::string("[a-z]+").mark(1),
                Value::string("135abc456def789"),
                Value::string("abc").mark(1),
            ),
            (
                Value::string("[a-z]+"),
                Value::string("135abc456def789").mark(2),
                Value::string("abc").mark(2),
            ),
        ];

        for (i, (pattern, string, want)) in tests.iter().enumerate() {
            let got = stdlib::regex(pattern, string)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(
                got, *want,
                "case {i}: wrong result for pattern {pattern:?}, string {string:?}"
            );
        }
    }
}
