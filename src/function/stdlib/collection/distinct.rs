//! `distinct` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`distinct`] (go-cty: `stdlib.DistinctFunc`).
pub fn distinct_func() -> Function {
    todo!()
}

/// The list with duplicate elements removed, keeping first occurrences
/// (go-cty: `stdlib.Distinct`).
pub fn distinct(list: &Value) -> Result<Value, CtyError> {
    let _ = list;
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

    // Ported from TestDistinct:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L2985
    #[test]
    #[ignore = "not yet implemented"]
    fn distinct() {
        // NOTE(port): upstream's `Want` field is `cty.NilVal` in the error case;
        // that case carries `None` here.
        struct Case {
            list: Value,
            want: Option<Value>,
            err: &'static str,
        }

        let tests: Vec<Case> = vec![
            // Empty list (string type)
            Case {
                list: Value::list_empty(Type::string()),
                want: Some(Value::list_empty(Type::string())),
                err: "",
            },
            // Empty list (number type)
            Case {
                list: Value::list_empty(Type::number()),
                want: Some(Value::list_empty(Type::number())),
                err: "",
            },
            // Empty list (unknown element type)
            Case {
                list: Value::list_empty(Type::dynamic()),
                want: Some(Value::list_empty(Type::dynamic())),
                err: "",
            },
            // List with single element
            Case {
                list: Value::list([Value::string("single")]),
                want: Some(Value::list([Value::string("single")])),
                err: "",
            },
            // List where all elements are identical
            Case {
                list: Value::list([Value::number(42), Value::number(42), Value::number(42)]),
                want: Some(Value::list([Value::number(42)])),
                err: "",
            },
            // List that is already distinct
            Case {
                list: Value::list([Value::string("a"), Value::string("b"), Value::string("c")]),
                want: Some(Value::list([
                    Value::string("a"),
                    Value::string("b"),
                    Value::string("c"),
                ])),
                err: "",
            },
            // List with nested lists
            Case {
                list: Value::list([
                    Value::list([Value::string("a"), Value::string("a")]),
                    Value::list([Value::string("b")]),
                    Value::list([Value::string("a"), Value::string("a")]),
                ]),
                want: Some(Value::list([
                    Value::list([Value::string("a"), Value::string("a")]),
                    Value::list([Value::string("b")]),
                ])),
                err: "",
            },
            // Wholly-unknown list
            Case {
                list: Value::unknown(Type::list(Type::string())),
                want: Some(Value::unknown(Type::list(Type::string())).refine_not_null()),
                err: "",
            },
            // List with unknown values
            Case {
                list: Value::list([
                    Value::unknown(Type::string()),
                    Value::string("a"),
                    Value::string("b"),
                    Value::unknown(Type::string()),
                ]),
                want: Some(Value::unknown(Type::list(Type::string())).refine_not_null()),
                err: "",
            },
            // null list
            Case {
                list: Value::null(Type::list(Type::string())),
                want: None,
                err: "argument must not be null",
            },
            // List with null values
            Case {
                list: Value::list([
                    Value::null(Type::string()),
                    Value::string("a"),
                    Value::null(Type::string()),
                    Value::string("b"),
                ]),
                want: Some(Value::list([
                    Value::null(Type::string()),
                    Value::string("a"),
                    Value::string("b"),
                ])),
                err: "",
            },
        ];

        for (i, case) in tests.iter().enumerate() {
            let got = stdlib::distinct(&case.list);
            if !case.err.is_empty() {
                let err = got.err().unwrap_or_else(|| {
                    panic!("case {i}: succeeded; want error");
                });
                assert_eq!(err.to_string(), case.err, "case {i}: wrong error");
                continue;
            }
            let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *case.want.as_ref().unwrap(), "case {i}: wrong result");
        }
    }
}
