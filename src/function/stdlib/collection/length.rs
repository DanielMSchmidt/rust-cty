//! `length` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`length`] (go-cty: `stdlib.LengthFunc`).
pub fn length_func() -> Function {
    todo!()
}

/// The number of elements of the collection (go-cty: `stdlib.Length`).
pub fn length(collection: &Value) -> Result<Value, CtyError> {
    let _ = collection;
    todo!()
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/function/stdlib/collection_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::function::stdlib;
    use crate::{Type, Value};

    // Ported from TestLength:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L970
    #[test]
    #[ignore = "not yet implemented"]
    fn length() {
        let tests: Vec<(Value, Value)> = vec![
            (Value::list_empty(Type::number()), Value::number(0)),
            (Value::list([Value::bool(true)]), Value::number(1)),
            (Value::set_empty(Type::number()), Value::number(0)),
            (Value::set([Value::bool(true)]), Value::number(1)),
            (
                Value::set([Value::bool(true), Value::bool(false)]),
                Value::number(2),
            ),
            (
                Value::set([Value::bool(true), Value::unknown(Type::bool())]),
                // Don't know if the unknown in the input represents cty.True or cty.False,
                // so it may or may not coalesce with the one known value.
                Value::unknown(Type::number())
                    .refine()
                    .not_null()
                    .number_range_inclusive(Value::number(1), Value::number(2))
                    .new_value(),
            ),
            (
                Value::set([Value::unknown(Type::bool())]),
                Value::number(1), // Will be one regardless of what value the unknown in the input is representing
            ),
            (Value::map_empty(Type::bool()), Value::number(0)),
            (Value::map([("hello", Value::bool(true))]), Value::number(1)),
            (Value::empty_tuple(), Value::number(0)),
            (Value::tuple([Value::bool(true)]), Value::number(1)),
            (
                Value::unknown(Type::list(Type::bool())),
                Value::unknown(Type::number())
                    .refine()
                    .not_null()
                    .number_range_inclusive(Value::zero(), Value::number(f64::MAX))
                    .new_value(),
            ),
            (
                Value::dynamic(),
                Value::unknown(Type::number())
                    .refine()
                    .not_null()
                    .number_range_inclusive(Value::zero(), Value::number(f64::MAX))
                    .new_value(),
            ),
            (
                Value::unknown(Type::list(Type::bool()))
                    .refine()
                    .collection_length_upper_bound(2)
                    .new_value(),
                Value::unknown(Type::number())
                    .refine()
                    .not_null()
                    .number_range_inclusive(Value::zero(), Value::number(2))
                    .new_value(),
            ),
            // Marked collections return a marked length
            (
                Value::list([Value::string("hello"), Value::string("world")]).mark("secret"),
                Value::number(2).mark("secret"),
            ),
            // Marks on values in unmarked collections do not propagate
            (
                Value::list([
                    Value::string("hello").mark("a"),
                    Value::string("world").mark("b"),
                ]),
                Value::number(2),
            ),
        ];

        for (i, (collection, want)) in tests.iter().enumerate() {
            let got = stdlib::length(collection)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
