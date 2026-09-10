//! `set_subtract` (go-cty: `cty/function/stdlib/set.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`set_subtract`] (go-cty: `stdlib.SetSubtractFunc`).
pub fn set_subtract_func() -> Function {
    todo!()
}

/// The elements of `a` not present in `b` (go-cty: `stdlib.SetSubtract`).
pub fn set_subtract(a: &Value, b: &Value) -> Result<Value, CtyError> {
    let _ = (a, b);
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

    use crate::function::stdlib::set_subtract;
    use crate::{Type, Value};

    // Ported from TestSetSubtract:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/set_test.go#L207
    #[test]
    #[ignore = "not yet implemented"]
    fn set_subtract_test() {
        let tests: Vec<(Value, Value, Value)> = vec![
            (
                Value::set_empty(Type::string()),
                Value::set_empty(Type::string()),
                Value::set_empty(Type::string()),
            ),
            (
                Value::set([Value::bool(true)]),
                Value::set_empty(Type::string()),
                Value::set([Value::string("true")]),
            ),
            (
                Value::set([Value::bool(true)]),
                Value::set([Value::bool(false)]),
                Value::set([Value::bool(true)]),
            ),
            (
                Value::set([Value::string("a"), Value::string("b"), Value::string("c")]),
                Value::set([Value::string("a"), Value::string("c")]),
                Value::set([Value::string("b")]),
            ),
            (
                Value::set([Value::string("a")]),
                Value::set_empty(Type::dynamic()),
                Value::set([Value::string("a")]),
            ),
            (
                Value::set([Value::empty_object()]),
                Value::set_empty(Type::dynamic()),
                Value::set([Value::empty_object()]),
            ),
            (
                Value::set_empty(Type::dynamic()),
                Value::set_empty(Type::dynamic()),
                Value::set_empty(Type::dynamic()),
            ),
            (
                Value::set([Value::string("5")]),
                Value::unknown(Type::set(Type::number())),
                Value::unknown(Type::set(Type::string())).refine_not_null(),
            ),
            (
                Value::set([Value::string("5")]),
                Value::set([Value::unknown(Type::string())]),
                Value::unknown(Type::set(Type::string())).refine_not_null(),
            ),
        ];

        for (i, (input_a, input_b, want)) in tests.iter().enumerate() {
            let got = match set_subtract(input_a, input_b) {
                Ok(got) => got,
                Err(err) => panic!("case {i}: unexpected error: {err}"),
            };
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
