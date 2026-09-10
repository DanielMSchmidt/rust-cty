//! `set_symmetric_difference` (go-cty: `cty/function/stdlib/set.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`set_symmetric_difference`]
/// (go-cty: `stdlib.SetSymmetricDifferenceFunc`).
pub fn set_symmetric_difference_func() -> Function {
    todo!()
}

/// The elements present in exactly one of the given sets
/// (go-cty: `stdlib.SetSymmetricDifference`).
pub fn set_symmetric_difference(sets: &[Value]) -> Result<Value, CtyError> {
    let _ = sets;
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

    use crate::function::stdlib::set_symmetric_difference;
    use crate::{Type, Value};

    // Ported from TestSetSymmetricDifference:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/set_test.go#L284
    #[test]
    #[ignore = "not yet implemented"]
    fn set_symmetric_difference_test() {
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
                Value::set([Value::bool(true), Value::bool(false)]),
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
                Value::set([Value::unknown(Type::number())]),
                Value::unknown(Type::set(Type::string())).refine_not_null(),
            ),
        ];

        for (i, (input_a, input_b, want)) in tests.iter().enumerate() {
            let got = match set_symmetric_difference(&[input_a.clone(), input_b.clone()]) {
                Ok(got) => got,
                Err(err) => panic!("case {i}: unexpected error: {err}"),
            };
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
