//! `coalesce` (go-cty: `cty/function/stdlib/general.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`coalesce`] (go-cty: `stdlib.CoalesceFunc`).
pub fn coalesce_func() -> Function {
    todo!()
}

/// The first non-null argument (go-cty: `stdlib.Coalesce`).
pub fn coalesce(vals: &[Value]) -> Result<Value, CtyError> {
    let _ = vals;
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

    // Ported from TestCoalesce:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/general_test.go#L73
    #[test]
    #[ignore = "not yet implemented"]
    fn coalesce() {
        let tests: Vec<(Vec<Value>, Value)> = vec![
            (vec![Value::bool(true)], Value::bool(true)),
            (
                vec![Value::null(Type::bool()), Value::bool(true)],
                Value::bool(true),
            ),
            (
                vec![Value::null(Type::bool()), Value::bool(false)],
                Value::bool(false),
            ),
            (
                vec![
                    Value::null(Type::bool()),
                    Value::bool(false),
                    Value::string("hello"),
                ],
                Value::string("false"),
            ),
            (
                vec![Value::bool(true), Value::unknown(Type::bool())],
                Value::bool(true),
            ),
            (
                vec![Value::unknown(Type::bool()), Value::bool(true)],
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                vec![Value::unknown(Type::bool()), Value::string("hello")],
                Value::unknown(Type::string()).refine_not_null(),
            ),
            (
                vec![Value::dynamic(), Value::bool(true)],
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (vec![Value::dynamic()], Value::dynamic()),
        ];

        for (i, (values, want)) in tests.iter().enumerate() {
            let got = stdlib::coalesce(values)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
