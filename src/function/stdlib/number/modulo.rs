//! `modulo` (go-cty: `cty/function/stdlib/number.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`modulo`] (go-cty: `stdlib.ModuloFunc`).
pub fn modulo_func() -> Function {
    todo!()
}

/// The remainder of dividing two numbers (go-cty: `stdlib.Modulo`).
pub fn modulo(a: &Value, b: &Value) -> Result<Value, CtyError> {
    let _ = (a, b);
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

    // Ported from TestModulo:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/number_test.go#L274
    #[test]
    #[ignore = "not yet implemented"]
    fn modulo() {
        let tests: Vec<(Value, Value, Value)> = vec![
            (Value::number(15), Value::number(10), Value::number(5)),
            (Value::number(0), Value::number(0), Value::number(0)),
            (
                Value::positive_infinity(),
                Value::number(1),
                Value::positive_infinity(),
            ),
            (
                Value::negative_infinity(),
                Value::number(1),
                Value::negative_infinity(),
            ),
            (
                Value::number(1),
                Value::positive_infinity(),
                Value::positive_infinity(),
            ),
            (
                Value::number(1),
                Value::unknown(Type::number()),
                Value::unknown(Type::number()).refine_not_null(),
            ),
            (
                Value::unknown(Type::number()),
                Value::unknown(Type::number()),
                Value::unknown(Type::number()).refine_not_null(),
            ),
            (
                Value::number(1),
                Value::dynamic(),
                Value::unknown(Type::number()).refine_not_null(),
            ),
            (
                Value::dynamic(),
                Value::dynamic(),
                Value::unknown(Type::number()).refine_not_null(),
            ),
        ];

        for (i, (a, b, want)) in tests.iter().enumerate() {
            let got = stdlib::modulo(a, b)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
