//! `has_index` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`has_index`] (go-cty: `stdlib.HasIndexFunc`).
pub fn has_index_func() -> Function {
    todo!()
}

/// Whether the collection has an element at the given key
/// (go-cty: `stdlib.HasIndex`).
pub fn has_index(collection: &Value, key: &Value) -> Result<Value, CtyError> {
    let _ = (collection, key);
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

    // Ported from TestHasIndex:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L11
    #[test]
    #[ignore = "not yet implemented"]
    fn has_index() {
        let tests: Vec<(Value, Value, Value)> = vec![
            (
                Value::list_empty(Type::number()),
                Value::number(2),
                Value::bool(false),
            ),
            (
                Value::list([Value::bool(true)]),
                Value::number(0),
                Value::bool(true),
            ),
            (
                Value::list([Value::bool(true)]),
                Value::string("hello"),
                Value::bool(false),
            ),
            (
                Value::map_empty(Type::bool()),
                Value::string("hello"),
                Value::bool(false),
            ),
            (
                Value::map([("hello", Value::bool(true))]),
                Value::string("hello"),
                Value::bool(true),
            ),
            (
                Value::empty_tuple(),
                Value::string("hello"),
                Value::bool(false),
            ),
            (Value::empty_tuple(), Value::number(0), Value::bool(false)),
            (
                Value::tuple([Value::bool(true)]),
                Value::number(0),
                Value::bool(true),
            ),
            (
                Value::list_empty(Type::number()),
                Value::unknown(Type::number()),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::unknown(Type::list(Type::bool())),
                Value::unknown(Type::number()),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::list_empty(Type::number()),
                Value::dynamic(),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
            (
                Value::dynamic(),
                Value::dynamic(),
                Value::unknown(Type::bool()).refine_not_null(),
            ),
        ];

        for (i, (collection, key, want)) in tests.iter().enumerate() {
            let got = stdlib::has_index(collection, key)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
