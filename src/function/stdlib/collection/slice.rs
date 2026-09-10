//! `slice` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`slice`] (go-cty: `stdlib.SliceFunc`).
pub fn slice_func() -> Function {
    todo!()
}

/// A subrange of the list (go-cty: `stdlib.Slice`).
pub fn slice(list: &Value, start: &Value, end: &Value) -> Result<Value, CtyError> {
    let _ = (list, start, end);
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

    use crate::Value;
    use crate::function::stdlib;

    // Ported from TestSlice:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L2911
    #[test]
    #[ignore = "not yet implemented"]
    fn slice() {
        struct Case {
            input: Value,
            start: Value,
            end: Value,
            want: Value,
            err: &'static str,
        }

        let tests: Vec<Case> = vec![
            Case {
                input: Value::list([Value::string("a"), Value::string("b"), Value::string("c")]),
                start: Value::number(0),
                end: Value::number(2),
                want: Value::list([Value::string("a"), Value::string("b")]),
                err: "",
            },
            // The entire input list is marked, so the return should be marked
            Case {
                input: Value::list([Value::string("a"), Value::string("b"), Value::string("c")])
                    .mark("bloop"),
                start: Value::number(0),
                end: Value::number(2),
                want: Value::list([Value::string("a"), Value::string("b")]).mark("bloop"),
                err: "",
            },
            // individual element marks should be preserved
            Case {
                input: Value::list([
                    Value::string("a"),
                    Value::string("b").mark("bloop"),
                    Value::string("c"),
                ]),
                start: Value::number(0),
                end: Value::number(2),
                want: Value::list([Value::string("a"), Value::string("b").mark("bloop")]),
                err: "",
            },
        ];

        for (i, case) in tests.iter().enumerate() {
            let got = stdlib::slice(&case.input, &case.start, &case.end);
            if !case.err.is_empty() {
                let err = got.err().unwrap_or_else(|| {
                    panic!("case {i}: succeeded; want error");
                });
                assert_eq!(err.to_string(), case.err, "case {i}: wrong error");
                continue;
            }
            let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, case.want, "case {i}: wrong result");
        }
    }
}
