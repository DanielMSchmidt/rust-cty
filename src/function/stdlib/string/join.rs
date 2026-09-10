//! `join` (go-cty: `cty/function/stdlib/string.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`join`] (go-cty: `stdlib.JoinFunc`).
pub fn join_func() -> Function {
    todo!()
}

/// The list elements joined with a separator (go-cty: `stdlib.Join`).
pub fn join(separator: &Value, lists: &[Value]) -> Result<Value, CtyError> {
    let _ = (separator, lists);
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
    use crate::{Value, ValueMarks};

    // Ported from TestJoin:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/string_test.go#L402
    #[test]
    #[ignore = "not yet implemented"]
    fn join() {
        let tests: Vec<(&str, Value, Vec<Value>, Value)> = vec![
            (
                "single two-element list",
                Value::string("-"),
                vec![Value::list([
                    Value::string("hello"),
                    Value::string("world"),
                ])],
                Value::string("hello-world"),
            ),
            (
                "multiple single-element lists",
                Value::string("-"),
                vec![
                    Value::list([Value::string("chicken")]),
                    Value::list([Value::string("egg")]),
                ],
                Value::string("chicken-egg"),
            ),
            (
                "single single-element list",
                Value::string("-"),
                vec![Value::list([Value::string("chicken")])],
                Value::string("chicken"),
            ),
            (
                "blank separator",
                Value::string(""),
                vec![Value::list([Value::string("horse"), Value::string("face")])],
                Value::string("horseface"),
            ),
            (
                "marked list",
                Value::string("-"),
                vec![
                    Value::list([Value::string("hello"), Value::string("world")]).mark("sensitive"),
                ],
                Value::string("hello-world").mark("sensitive"),
            ),
            (
                "marked separator",
                Value::string("-").mark("sensitive"),
                vec![Value::list([
                    Value::string("hello"),
                    Value::string("world"),
                ])],
                Value::string("hello-world").mark("sensitive"),
            ),
            (
                "list with some marked elements",
                Value::string("-"),
                vec![Value::list([
                    Value::string("hello").mark("sensitive"),
                    Value::string("world"),
                ])],
                Value::string("hello-world").mark("sensitive"),
            ),
            (
                "multiple marks",
                Value::string("-").mark("a"),
                vec![Value::list([
                    Value::string("hello").mark("b"),
                    Value::string("world").mark("c"),
                ])],
                Value::string("hello-world").with_marks([ValueMarks::from_marks(["a", "b", "c"])]),
            ),
        ];

        for (name, separator, lists, want) in tests.iter() {
            let got = stdlib::join(separator, lists)
                .unwrap_or_else(|err| panic!("{name}: unexpected error: {err}"));
            assert_eq!(got, *want, "{name}: wrong result");
        }
    }
}
