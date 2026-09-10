//! `range` (go-cty: `cty/function/stdlib/sequence.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`range`] (go-cty: `stdlib.RangeFunc`).
pub fn range_func() -> Function {
    todo!()
}

/// A list of numbers counted from a start to a limit by a step; accepts one,
/// two, or three arguments (go-cty: `stdlib.Range`).
pub fn range(params: &[Value]) -> Result<Value, CtyError> {
    let _ = params;
    todo!()
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/function/stdlib/sequence_test.go
    //!   cty/function/stdlib/set_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::function::stdlib::range;
    use crate::{Type, Value};

    // Ported from TestRange:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/sequence_test.go#L259
    //
    // Upstream TestRange is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod range_test {
        use super::*;

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[(Vec<Value>, Value)]) {
            for (i, (args, want)) in tests.iter().enumerate() {
                let got = match range(args) {
                    Ok(got) => got,
                    Err(err) => panic!("case {i}: unexpected error: {err}"),
                };
                assert_eq!(got, *want, "case {i}: wrong result\nargs: {args:?}");
            }
        }

        // TestRange, one argument:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/sequence_test.go#L259
        #[test]
        #[ignore = "not yet implemented"]
        fn one_argument() {
            let tests: Vec<(Vec<Value>, Value)> = vec![
                // One argument
                (
                    vec![Value::number(5)],
                    Value::list([
                        Value::number(0),
                        Value::number(1),
                        Value::number(2),
                        Value::number(3),
                        Value::number(4),
                    ]),
                ),
                (
                    vec![Value::number(-5)],
                    Value::list([
                        Value::number(0),
                        Value::number(-1),
                        Value::number(-2),
                        Value::number(-3),
                        Value::number(-4),
                    ]),
                ),
                (vec![Value::number(1)], Value::list([Value::number(0)])),
                (vec![Value::number(0)], Value::list_empty(Type::number())),
                (
                    vec![Value::parse_number("5.5")],
                    Value::list([
                        Value::number(0),
                        Value::number(1),
                        Value::number(2),
                        Value::number(3),
                        Value::number(4),
                        Value::number(5), // because 5 < 5.5
                    ]),
                ),
            ];

            check(&tests);
        }

        // TestRange, two arguments:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/sequence_test.go#L259
        #[test]
        #[ignore = "not yet implemented"]
        fn two_arguments() {
            let tests: Vec<(Vec<Value>, Value)> = vec![
                // Two arguments
                (
                    vec![Value::number(1), Value::number(5)],
                    Value::list([
                        Value::number(1),
                        Value::number(2),
                        Value::number(3),
                        Value::number(4),
                    ]),
                ),
                (
                    vec![Value::number(5), Value::number(1)],
                    Value::list([
                        Value::number(5),
                        Value::number(4),
                        Value::number(3),
                        Value::number(2),
                    ]),
                ),
                (
                    vec![Value::number(1.5), Value::number(5)],
                    Value::list([
                        Value::number(1.5),
                        Value::number(2.5),
                        Value::number(3.5),
                        Value::number(4.5),
                    ]),
                ),
                (
                    vec![Value::number(1), Value::number(2)],
                    Value::list([Value::number(1)]),
                ),
                (
                    vec![Value::number(1), Value::number(1)],
                    Value::list_empty(Type::number()),
                ),
            ];

            check(&tests);
        }

        // TestRange, three arguments:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/sequence_test.go#L259
        #[test]
        #[ignore = "not yet implemented"]
        fn three_arguments() {
            let tests: Vec<(Vec<Value>, Value)> = vec![
                // Three arguments
                (
                    vec![Value::number(0), Value::number(5), Value::number(2)],
                    Value::list([Value::number(0), Value::number(2), Value::number(4)]),
                ),
                (
                    vec![Value::number(0), Value::number(5), Value::number(1)],
                    Value::list([
                        Value::number(0),
                        Value::number(1),
                        Value::number(2),
                        Value::number(3),
                        Value::number(4),
                    ]),
                ),
                (
                    vec![Value::number(0), Value::number(1), Value::number(1)],
                    Value::list([Value::number(0)]),
                ),
                (
                    vec![Value::number(0), Value::number(0), Value::number(1)],
                    Value::list_empty(Type::number()),
                ),
                (
                    vec![Value::number(5), Value::number(0), Value::number(-1)],
                    Value::list([
                        Value::number(5),
                        Value::number(4),
                        Value::number(3),
                        Value::number(2),
                        Value::number(1),
                    ]),
                ),
                (
                    vec![Value::number(0), Value::number(5), Value::number(0.5)],
                    Value::list([
                        Value::number(0),
                        Value::number(0.5),
                        Value::number(1),
                        Value::number(1.5),
                        Value::number(2),
                        Value::number(2.5),
                        Value::number(3),
                        Value::number(3.5),
                        Value::number(4),
                        Value::number(4.5),
                    ]),
                ),
            ];

            check(&tests);
        }
    }
}
