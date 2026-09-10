//! `coalesce_list` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`coalesce_list`] (go-cty: `stdlib.CoalesceListFunc`).
pub fn coalesce_list_func() -> Function {
    todo!()
}

/// The first non-empty list argument (go-cty: `stdlib.CoalesceList`).
pub fn coalesce_list(args: &[Value]) -> Result<Value, CtyError> {
    let _ = args;
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

    // Ported from TestCoalesceList:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L1423
    #[test]
    #[ignore = "not yet implemented"]
    fn coalesce_list() {
        // NOTE(port): upstream's `Want` field is `cty.NilVal` in the error cases;
        // those cases carry `None` here.
        struct Case {
            name: &'static str,
            values: Vec<Value>,
            want: Option<Value>,
            err: bool,
        }

        let tests: Vec<Case> = vec![
            Case {
                name: "returns first list if non-empty",
                values: vec![
                    Value::list([Value::string("a"), Value::string("b")]),
                    Value::list([Value::string("c"), Value::string("d")]),
                ],
                want: Some(Value::list([Value::string("a"), Value::string("b")])),
                err: false,
            },
            Case {
                name: "returns second list if first is empty",
                values: vec![
                    Value::list_empty(Type::string()),
                    Value::list([Value::string("c"), Value::string("d")]),
                ],
                want: Some(Value::list([Value::string("c"), Value::string("d")])),
                err: false,
            },
            Case {
                name: "return type is dynamic, not unified",
                values: vec![
                    Value::list_empty(Type::string()),
                    Value::list([Value::number(3), Value::number(4)]),
                ],
                want: Some(Value::list([Value::number(3), Value::number(4)])),
                err: false,
            },
            Case {
                name: "works with tuples",
                values: vec![
                    Value::empty_tuple(),
                    Value::tuple([Value::string("c"), Value::string("d")]),
                ],
                want: Some(Value::tuple([Value::string("c"), Value::string("d")])),
                err: false,
            },
            Case {
                name: "unknown arguments",
                values: vec![
                    Value::unknown(Type::list(Type::string())),
                    Value::list([Value::string("c"), Value::string("d")]),
                ],
                want: Some(Value::dynamic()),
                err: false,
            },
            Case {
                name: "null arguments",
                values: vec![
                    Value::null(Type::list(Type::string())),
                    Value::list([Value::string("c"), Value::string("d")]),
                ],
                want: Some(Value::list([Value::string("c"), Value::string("d")])),
                err: false,
            },
            Case {
                name: "all null arguments",
                values: vec![
                    Value::null(Type::list(Type::string())),
                    Value::null(Type::list(Type::string())),
                ],
                want: None,
                err: true,
            },
            Case {
                name: "invalid arguments",
                values: vec![
                    Value::map([("a", Value::bool(true))]),
                    Value::object([("b", Value::bool(false))]),
                ],
                want: None,
                err: true,
            },
            Case {
                name: "no arguments",
                values: vec![],
                want: None,
                err: true,
            },
        ];

        for case in tests.iter() {
            let name = case.name;
            let got = stdlib::coalesce_list(&case.values);
            if case.err {
                assert!(got.is_err(), "{name}: succeeded; want error");
                continue;
            }
            let got = got.unwrap_or_else(|err| panic!("{name}: unexpected error: {err}"));
            assert_eq!(got, *case.want.as_ref().unwrap(), "{name}: wrong result");
        }
    }
}
