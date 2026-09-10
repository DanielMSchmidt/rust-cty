//! `index` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`index`] (go-cty: `stdlib.IndexFunc`).
pub fn index_func() -> Function {
    todo!()
}

/// The element of the collection at the given key (go-cty: `stdlib.Index`).
pub fn index(collection: &Value, key: &Value) -> Result<Value, CtyError> {
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

    // Ported from TestIndex:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L897
    #[test]
    #[ignore = "not yet implemented"]
    fn index() {
        let tests: Vec<(Value, Value, Value)> = vec![
            (
                Value::list([Value::bool(true)]),
                Value::number(0),
                Value::bool(true),
            ),
            (
                Value::map([("hello", Value::bool(true))]),
                Value::string("hello"),
                Value::bool(true),
            ),
            (
                Value::tuple([Value::bool(true), Value::string("hello")]),
                Value::number(0),
                Value::bool(true),
            ),
            (
                Value::tuple([Value::bool(true), Value::string("hello")]),
                Value::number(1),
                Value::string("hello"),
            ),
            (
                Value::list_empty(Type::number()),
                Value::unknown(Type::number()),
                Value::unknown(Type::number()),
            ),
            (
                Value::unknown(Type::list(Type::bool())),
                Value::unknown(Type::number()),
                Value::unknown(Type::bool()),
            ),
            (
                Value::list_empty(Type::number()),
                Value::dynamic(),
                Value::unknown(Type::number()),
            ),
            (
                Value::map_empty(Type::number()),
                Value::dynamic(),
                Value::unknown(Type::number()),
            ),
            (Value::dynamic(), Value::string("hello"), Value::dynamic()),
            (Value::dynamic(), Value::dynamic(), Value::dynamic()),
        ];

        for (i, (collection, key, want)) in tests.iter().enumerate() {
            let got = stdlib::index(collection, key)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
