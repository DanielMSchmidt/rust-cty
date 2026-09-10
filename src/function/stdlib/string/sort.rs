//! `sort` (go-cty: `cty/function/stdlib/string.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`sort`] (go-cty: `stdlib.SortFunc`).
pub fn sort_func() -> Function {
    todo!()
}

/// The list of strings sorted lexically (go-cty: `stdlib.Sort`).
pub fn sort(list: &Value) -> Result<Value, CtyError> {
    let _ = list;
    todo!()
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/function/stdlib/string_test.go
    //!   cty/function/stdlib/string_replace_test.go
    //!   cty/function/stdlib/regexp_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::function::stdlib;
    use crate::{Type, Value};

    // Ported from TestSort:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/string_test.go#L482
    #[test]
    #[ignore = "not yet implemented"]
    fn sort() {
        let tests: Vec<(Value, Value, &str)> = vec![
            (
                Value::list_empty(Type::string()),
                Value::list_empty(Type::string()),
                "",
            ),
            (
                Value::list([Value::string("a")]),
                Value::list([Value::string("a")]),
                "",
            ),
            (
                Value::list([Value::string("b"), Value::string("a")]),
                Value::list([Value::string("a"), Value::string("b")]),
                "",
            ),
            (
                Value::list([Value::string("b"), Value::string("a"), Value::string("c")]),
                Value::list([Value::string("a"), Value::string("b"), Value::string("c")]),
                "",
            ),
            (
                Value::unknown(Type::list(Type::string())),
                Value::unknown(Type::list(Type::string())).refine_not_null(),
                "",
            ),
            (
                // If the list contains any unknown values then we can still
                // preserve the length of the list by generating a known list
                // with unknown elements, because sort can never change the length.
                Value::list([Value::string("b"), Value::unknown(Type::string())]),
                Value::list([
                    Value::unknown(Type::string()),
                    Value::unknown(Type::string()),
                ]),
                "",
            ),
            (
                // For a completely unknown list we can still preserve any
                // refinements it had for its length, because sorting can never
                // change the length.
                Value::unknown(Type::list(Type::string()))
                    .refine()
                    .collection_length_lower_bound(1)
                    .collection_length_upper_bound(2)
                    .new_value(),
                Value::unknown(Type::list(Type::string()))
                    .refine()
                    .not_null()
                    .collection_length_lower_bound(1)
                    .collection_length_upper_bound(2)
                    .new_value(),
                "",
            ),
        ];

        for (i, (input, want, want_err)) in tests.iter().enumerate() {
            let result = stdlib::sort(input);

            if !want_err.is_empty() {
                match result {
                    Err(err) => {
                        assert_eq!(err.to_string(), *want_err, "case {i}: wrong error");
                    }
                    Ok(_) => panic!("case {i}: expected error, got success"),
                }
                continue;
            }

            let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
