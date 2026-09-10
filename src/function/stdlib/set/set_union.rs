//! `set_union` (go-cty: `cty/function/stdlib/set.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`set_union`] (go-cty: `stdlib.SetUnionFunc`).
pub fn set_union_func() -> Function {
    todo!()
}

/// The union of the given sets (go-cty: `stdlib.SetUnion`).
pub fn set_union(sets: &[Value]) -> Result<Value, CtyError> {
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

    use crate::function::stdlib::set_union;
    use crate::{Type, Value};

    // Ported from TestSetUnion:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/set_test.go#L10
    #[test]
    #[ignore = "not yet implemented"]
    fn set_union_test() {
        let tests: Vec<(Vec<Value>, Value)> = vec![
            (
                vec![Value::set_empty(Type::string())],
                Value::set_empty(Type::string()),
            ),
            (
                vec![
                    Value::set_empty(Type::string()),
                    Value::set_empty(Type::string()),
                ],
                Value::set_empty(Type::string()),
            ),
            (
                vec![
                    Value::set([Value::bool(true)]),
                    Value::set_empty(Type::string()),
                ],
                Value::set([Value::string("true")]),
            ),
            (
                vec![
                    Value::set([Value::bool(true)]),
                    Value::set([Value::bool(true)]),
                    Value::set([Value::bool(false)]),
                ],
                Value::set([Value::bool(true), Value::bool(false)]),
            ),
            (
                vec![
                    Value::set([Value::string("a")]),
                    Value::set([Value::string("b")]),
                    Value::set([Value::string("b"), Value::string("c")]),
                ],
                Value::set([Value::string("a"), Value::string("b"), Value::string("c")]),
            ),
            (
                vec![
                    Value::set([Value::bool(true)]),
                    Value::set_empty(Type::dynamic()),
                ],
                Value::set([Value::bool(true)]),
            ),
            (
                vec![
                    Value::set([Value::empty_object()]),
                    Value::set_empty(Type::dynamic()),
                ],
                Value::set([Value::empty_object()]),
            ),
            (
                vec![
                    Value::set_empty(Type::dynamic()),
                    Value::set_empty(Type::dynamic()),
                ],
                Value::set_empty(Type::dynamic()),
            ),
            (
                vec![
                    Value::set([Value::string("5")]),
                    Value::unknown(Type::set(Type::number())),
                ],
                Value::unknown(Type::set(Type::string())).refine_not_null(),
            ),
            (
                vec![
                    Value::set([Value::string("5")]),
                    Value::set([Value::unknown(Type::string())]),
                ],
                Value::set([Value::string("5"), Value::unknown(Type::string())]),
            ),
        ];

        for (i, (input, want)) in tests.iter().enumerate() {
            let got = match set_union(input) {
                Ok(got) => got,
                Err(err) => panic!("case {i}: unexpected error: {err}"),
            };
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
