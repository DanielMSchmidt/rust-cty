//! `zipmap` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`zipmap`] (go-cty: `stdlib.ZipmapFunc`).
pub fn zipmap_func() -> Function {
    todo!()
}

/// A map built by zipping a list of keys with a list of values
/// (go-cty: `stdlib.Zipmap`).
pub fn zipmap(keys: &Value, values: &Value) -> Result<Value, CtyError> {
    let _ = (keys, values);
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

    // Ported from TestZipMap:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L1671
    //
    // Upstream TestZipMap is a single table covering several concepts at once.
    // It is split below into one test per concept so they can be activated one at
    // a time. Rows are transcribed verbatim and keep upstream order within a test;
    // nothing is added, dropped, or rewritten.
    mod zipmap {
        use super::*;

        // NOTE(port): upstream's `Want` field is `cty.NilVal` in the error cases;
        // those cases carry `None` here.
        struct Case {
            keys: Value,
            values: Value,
            want: Option<Value>,
            err: &'static str,
        }

        /// Runs transcribed upstream rows; fixture plumbing only, every
        /// expected value is a literal from the upstream table.
        fn check(tests: &[Case]) {
            for (i, case) in tests.iter().enumerate() {
                let got = stdlib::zipmap(&case.keys, &case.values);
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

        // TestZipMap, collections:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L1671
        #[test]
        #[ignore = "not yet implemented"]
        fn collections() {
            let tests: Vec<Case> = vec![
                // Lists of values (map result)
                Case {
                    keys: Value::list_empty(Type::string()),
                    values: Value::list_empty(Type::string()),
                    want: Some(Value::map_empty(Type::string())),
                    err: "",
                },
                Case {
                    keys: Value::list([Value::string("bleep")]),
                    values: Value::list([Value::string("bloop")]),
                    want: Some(Value::map([("bleep", Value::string("bloop"))])),
                    err: "",
                },
                Case {
                    keys: Value::list([Value::string("bleep"), Value::string("beep")]),
                    values: Value::list([Value::string("bloop"), Value::string("boop")]),
                    want: Some(Value::map([
                        ("beep", Value::string("boop")),
                        ("bleep", Value::string("bloop")),
                    ])),
                    err: "",
                },
                Case {
                    keys: Value::list([Value::string("boop")]),
                    values: Value::list_empty(Type::string()),
                    want: None,
                    err: "number of keys (1) does not match number of values (0)",
                },
                Case {
                    keys: Value::list_empty(Type::string()),
                    values: Value::list([Value::string("boop")]),
                    want: None,
                    err: "number of keys (0) does not match number of values (1)",
                },
                // Tuple of values (object result)
                Case {
                    keys: Value::list_empty(Type::string()),
                    values: Value::empty_tuple(),
                    want: Some(Value::empty_object()),
                    err: "",
                },
                Case {
                    keys: Value::list([Value::string("bleep")]),
                    values: Value::tuple([Value::string("bloop")]),
                    want: Some(Value::object([("bleep", Value::string("bloop"))])),
                    err: "",
                },
                Case {
                    keys: Value::list([Value::string("bleep"), Value::string("beep")]),
                    values: Value::tuple([Value::string("bloop"), Value::string("boop")]),
                    want: Some(Value::object([
                        ("beep", Value::string("boop")),
                        ("bleep", Value::string("bloop")),
                    ])),
                    err: "",
                },
                Case {
                    keys: Value::list([Value::string("boop")]),
                    values: Value::empty_tuple(),
                    want: None,
                    err: "number of keys (1) does not match number of values (0)",
                },
                Case {
                    keys: Value::list_empty(Type::string()),
                    values: Value::tuple([Value::string("boop")]),
                    want: None,
                    err: "number of keys (0) does not match number of values (1)",
                },
            ];

            check(&tests);
        }

        // TestZipMap, unknowns:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L1671
        #[test]
        #[ignore = "not yet implemented"]
        fn unknowns() {
            let tests: Vec<Case> = vec![
                Case {
                    keys: Value::unknown(Type::list(Type::string())),
                    values: Value::unknown(Type::list(Type::string())),
                    want: Some(Value::unknown(Type::map(Type::string())).refine_not_null()),
                    err: "",
                },
                Case {
                    keys: Value::unknown(Type::list(Type::string())),
                    values: Value::list_empty(Type::string()),
                    want: Some(Value::unknown(Type::map(Type::string())).refine_not_null()),
                    err: "",
                },
                Case {
                    keys: Value::list_empty(Type::string()),
                    values: Value::unknown(Type::list(Type::string())),
                    want: Some(Value::unknown(Type::map(Type::string())).refine_not_null()),
                    err: "",
                },
                Case {
                    keys: Value::unknown(Type::list(Type::string())),
                    values: Value::unknown(Type::empty_tuple()),
                    want: Some(Value::dynamic()),
                    err: "",
                },
                Case {
                    keys: Value::unknown(Type::list(Type::string())),
                    values: Value::empty_tuple(),
                    want: Some(Value::dynamic()),
                    err: "",
                },
                Case {
                    keys: Value::list_empty(Type::string()),
                    values: Value::unknown(Type::empty_tuple()),
                    want: Some(Value::unknown(Type::empty_object()).refine_not_null()),
                    err: "",
                },
            ];

            check(&tests);
        }

        // TestZipMap, marks:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L1671
        #[test]
        #[ignore = "not yet implemented"]
        fn marks() {
            let tests: Vec<Case> = vec![
                Case {
                    keys: Value::list([Value::string("bleep")]).mark("a"),
                    values: Value::list([Value::string("bloop")]),
                    want: Some(Value::map([("bleep", Value::string("bloop"))]).mark("a")),
                    err: "",
                },
                Case {
                    keys: Value::list([Value::string("bleep")]),
                    values: Value::list([Value::string("bloop")]).mark("b"),
                    want: Some(Value::map([("bleep", Value::string("bloop"))]).mark("b")),
                    err: "",
                },
                Case {
                    keys: Value::list([Value::string("bleep")]).mark("a"),
                    values: Value::list([Value::string("bloop")]).mark("b"),
                    want: Some(
                        Value::map([("bleep", Value::string("bloop"))])
                            .mark("a")
                            .mark("b"),
                    ),
                    err: "",
                },
                // cty map keys don't have individual marks, so marks on elements
                // in the keys list aggregate with the resulting map as a whole.
                Case {
                    keys: Value::list([Value::string("bleep").mark("a")]),
                    values: Value::list([Value::string("bloop")]),
                    want: Some(Value::map([("bleep", Value::string("bloop"))]).mark("a")),
                    err: "",
                },
                // cty map _values_ can have individual marks, so individual
                // elements in the values list should have their marks preserved.
                Case {
                    keys: Value::list([Value::string("bleep")]),
                    values: Value::list([Value::string("bloop").mark("a")]),
                    want: Some(Value::map([("bleep", Value::string("bloop").mark("a"))])),
                    err: "",
                },
                Case {
                    keys: Value::list([Value::string("bleep")]).mark("a"),
                    values: Value::tuple([Value::string("bloop")]),
                    want: Some(Value::object([("bleep", Value::string("bloop"))]).mark("a")),
                    err: "",
                },
                Case {
                    keys: Value::list([Value::string("bleep")]),
                    values: Value::tuple([Value::string("bloop")]).mark("b"),
                    want: Some(Value::object([("bleep", Value::string("bloop"))]).mark("b")),
                    err: "",
                },
                Case {
                    keys: Value::list([Value::string("bleep")]).mark("a"),
                    values: Value::tuple([Value::string("bloop")]).mark("b"),
                    want: Some(
                        Value::object([("bleep", Value::string("bloop"))])
                            .mark("a")
                            .mark("b"),
                    ),
                    err: "",
                },
                // cty object attributes don't have individual marks, so marks on
                // elements in the keys list aggregate with the resulting object as
                // a whole.
                Case {
                    keys: Value::list([Value::string("bleep").mark("a")]),
                    values: Value::tuple([Value::string("bloop")]),
                    want: Some(Value::object([("bleep", Value::string("bloop"))]).mark("a")),
                    err: "",
                },
                // cty attribute _values_ can have individual marks, so individual
                // elements in the values list should have their marks preserved.
                Case {
                    keys: Value::list([Value::string("bleep")]),
                    values: Value::tuple([Value::string("bloop").mark("a")]),
                    want: Some(Value::object([("bleep", Value::string("bloop").mark("a"))])),
                    err: "",
                },
            ];

            check(&tests);
        }
    }
}
