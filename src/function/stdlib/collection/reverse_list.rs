//! `reverse_list` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`reverse_list`] (go-cty: `stdlib.ReverseListFunc`).
pub fn reverse_list_func() -> Function {
    todo!()
}

/// The list with element order reversed (go-cty: `stdlib.ReverseList`).
pub fn reverse_list(list: &Value) -> Result<Value, CtyError> {
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

    // Ported from TestReverseList:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L2808
    #[test]
    #[ignore = "not yet implemented"]
    fn reverse_list() {
        // NOTE(port): upstream's first case passes `cty.NilVal` as the input. Its
        // `IsNull()` is true (cty/value.go:59-64), so it trips the AllowNull check
        // in the function machinery (cty/function/function.go:169) before any type
        // handling happens — the behavior under test is null rejection, not the Go
        // zero value. It is therefore ported as a typed null, the same shape the
        // null cases in TestValues/TestKeys/TestDistinct already use in this file.
        // Upstream's `Want` here is NilVal and is never read; the `want` column
        // below is likewise unread whenever an error is expected.
        let tests: Vec<(Value, Value, &'static str)> = vec![
            (
                Value::null(Type::list(Type::string())),
                Value::null(Type::list(Type::string())),
                "argument must not be null",
            ),
            (
                Value::list_empty(Type::string()),
                Value::list_empty(Type::string()),
                "",
            ),
            (
                Value::list_empty(Type::string()).mark("foo"),
                Value::list_empty(Type::string()).mark("foo"),
                "",
            ),
            (
                Value::unknown(Type::list(Type::string())),
                Value::unknown(Type::list(Type::string())).refine_not_null(),
                "",
            ),
            // marks on list elements
            (
                Value::list([
                    Value::string("beep").mark("boop"),
                    Value::string("bop"),
                    Value::string("bloop"),
                ]),
                Value::list([
                    Value::string("bloop"),
                    Value::string("bop"),
                    Value::string("beep").mark("boop"),
                ]),
                "",
            ),
            // marks on the entire input are preserved
            (
                Value::list([
                    Value::string("beep").mark("boop"),
                    Value::string("bop"),
                    Value::string("bloop"),
                ])
                .mark("outer"),
                Value::list([
                    Value::string("bloop"),
                    Value::string("bop"),
                    Value::string("beep").mark("boop"),
                ])
                .mark("outer"),
                "",
            ),
            // marks on tuple elements
            (
                Value::tuple([
                    Value::string("beep").mark("boop"),
                    Value::string("bop"),
                    Value::string("bloop"),
                ]),
                Value::tuple([
                    Value::string("bloop"),
                    Value::string("bop"),
                    Value::string("beep").mark("boop"),
                ]),
                "",
            ),
            // Set elements don't support individual marks; any marks on elements get propegated to the entire set.
            (
                Value::set([
                    Value::string("beep").mark("boop"),
                    Value::string("bop"),
                    Value::string("bloop"),
                ]),
                // sets end up sorted alphabetically when converted to lists
                Value::list([
                    Value::string("bop"),
                    Value::string("bloop"),
                    Value::string("beep"),
                ])
                .mark("boop"),
                "",
            ),
        ];

        for (i, (input, want, want_err)) in tests.iter().enumerate() {
            let got = stdlib::reverse_list(input);
            if !want_err.is_empty() {
                let err = got.err().unwrap_or_else(|| {
                    panic!("case {i}: succeeded; want error");
                });
                assert_eq!(err.to_string(), *want_err, "case {i}: wrong error");
                continue;
            }
            let got = got.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
