//! `parse_int` (go-cty: `cty/function/stdlib/number.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`parse_int`] (go-cty: `stdlib.ParseIntFunc`).
pub fn parse_int_func() -> Function {
    todo!()
}

/// Parses an integer from a string in the given base
/// (go-cty: `stdlib.ParseInt`).
pub fn parse_int(num: &Value, base: &Value) -> Result<Value, CtyError> {
    let _ = (num, base);
    todo!()
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/function/stdlib/number_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::function::stdlib;
    use crate::{Type, Value};

    // Ported from TestParseInt:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L1125
    //
    // Upstream TestParseInt is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod parse_int {
        use super::*;

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[(Value, Value, Value, bool)]) {
            for (i, (num, base, want, want_err)) in tests.iter().enumerate() {
                let result = stdlib::parse_int(num, base);
                if *want_err {
                    assert!(result.is_err(), "case {i}: succeeded; want error");
                    continue;
                }
                let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
                assert_eq!(got, *want, "case {i}: wrong result");
            }
        }

        // TestParseInt, primitives:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L1125
        #[test]
        #[ignore = "not yet implemented"]
        fn primitives() {
            let tests: Vec<(Value, Value, Value, bool)> = vec![
                (
                    Value::string("128"),
                    Value::number(10),
                    Value::number(128),
                    false,
                ),
                (
                    Value::string("-128"),
                    Value::number(10),
                    Value::number(-128),
                    false,
                ),
                (
                    Value::string("00128"),
                    Value::number(10),
                    Value::number(128),
                    false,
                ),
                (
                    Value::string("-00128"),
                    Value::number(10),
                    Value::number(-128),
                    false,
                ),
                (
                    Value::string("FF00"),
                    Value::number(16),
                    Value::number(65280),
                    false,
                ),
                (
                    Value::string("ff00"),
                    Value::number(16),
                    Value::number(65280),
                    false,
                ),
                (
                    Value::string("-FF00"),
                    Value::number(16),
                    Value::number(-65280),
                    false,
                ),
                (
                    Value::string("00FF00"),
                    Value::number(16),
                    Value::number(65280),
                    false,
                ),
                (
                    Value::string("-00FF00"),
                    Value::number(16),
                    Value::number(-65280),
                    false,
                ),
                (
                    Value::string("1011111011101111"),
                    Value::number(2),
                    Value::number(48879),
                    false,
                ),
                (
                    Value::string("aA"),
                    Value::number(62),
                    Value::number(656),
                    false,
                ),
                (
                    Value::string("Aa"),
                    Value::number(62),
                    Value::number(2242),
                    false,
                ),
                (
                    Value::string("999999999999999999999999999999999999999999999999999999999999"),
                    Value::number(10),
                    Value::parse_number(
                        "999999999999999999999999999999999999999999999999999999999999",
                    ),
                    false,
                ),
            ];

            check(&tests);
        }

        // TestParseInt, unknowns:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L1125
        #[test]
        #[ignore = "not yet implemented"]
        fn unknowns() {
            let tests: Vec<(Value, Value, Value, bool)> = vec![
                (
                    Value::string("FF"),
                    Value::number(10),
                    Value::unknown(Type::number()).refine_not_null(),
                    true,
                ),
                (
                    Value::string("00FF"),
                    Value::number(10),
                    Value::unknown(Type::number()).refine_not_null(),
                    true,
                ),
                (
                    Value::string("-00FF"),
                    Value::number(10),
                    Value::unknown(Type::number()).refine_not_null(),
                    true,
                ),
                (
                    Value::number(2),
                    Value::number(10),
                    Value::unknown(Type::number()).refine_not_null(),
                    true,
                ),
                (
                    Value::string("1"),
                    Value::number(63),
                    Value::unknown(Type::number()).refine_not_null(),
                    true,
                ),
                (
                    Value::string("1"),
                    Value::number(-1),
                    Value::unknown(Type::number()).refine_not_null(),
                    true,
                ),
                (
                    Value::string("1"),
                    Value::number(1),
                    Value::unknown(Type::number()).refine_not_null(),
                    true,
                ),
                (
                    Value::string("1"),
                    Value::number(0),
                    Value::unknown(Type::number()).refine_not_null(),
                    true,
                ),
                (
                    Value::string("1.2"),
                    Value::number(10),
                    Value::unknown(Type::number()).refine_not_null(),
                    true,
                ),
            ];

            check(&tests);
        }
    }
}
