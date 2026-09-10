//! `lookup` (go-cty: `cty/function/stdlib/collection.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`lookup`] (go-cty: `stdlib.LookupFunc`).
pub fn lookup_func() -> Function {
    todo!()
}

/// The map element at the given key, or the default when absent
/// (go-cty: `stdlib.Lookup`).
pub fn lookup(input_map: &Value, key: &Value, default_value: &Value) -> Result<Value, CtyError> {
    let _ = (input_map, key, default_value);
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
    use crate::{Type, Value, ValueMarks};

    // Ported from TestLookup:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/collection_test.go#L1064
    #[test]
    #[ignore = "not yet implemented"]
    fn lookup() {
        let tests: Vec<(Value, Value, Value, Value)> = vec![
            (
                Value::map_empty(Type::string()),
                Value::string("baz"),
                Value::string("foo"),
                Value::string("foo"),
            ),
            (
                Value::map([("foo", Value::string("bar"))]),
                Value::string("foo"),
                Value::string("nope"),
                Value::string("bar"),
            ),
            // successful marked collection lookup returns marked value
            (
                Value::map([("boop", Value::string("beep"))]).mark("a"),
                Value::string("boop"),
                Value::string("nope"),
                Value::string("beep").mark("a"),
            ),
            // apply collection marks to unknown return vaue
            (
                Value::map([
                    ("boop", Value::string("beep")),
                    ("frob", Value::unknown(Type::string())),
                ])
                .mark("a"),
                Value::string("boop"),
                Value::string("nope"),
                Value::unknown(Type::string()).mark("a"),
            ),
            // propagate collection marks to default when returning
            (
                Value::map([("boop", Value::string("beep"))]).mark("a"),
                Value::string("frob"),
                Value::string("nope").mark("b"),
                Value::string("nope").with_marks([ValueMarks::from_marks(["a", "b"])]),
            ),
            // on unmarked collection, return only marks from found value
            (
                Value::map([
                    ("boop", Value::string("beep").mark("a")),
                    ("frob", Value::string("honk").mark("b")),
                ]),
                Value::string("frob"),
                Value::string("nope").mark("c"),
                Value::string("honk").mark("b"),
            ),
            // on unmarked collection, return default exactly on missing
            (
                Value::map([
                    ("boop", Value::string("beep").mark("a")),
                    ("frob", Value::string("honk").mark("b")),
                ]),
                Value::string("squish"),
                Value::string("nope").mark("c"),
                Value::string("nope").mark("c"),
            ),
            // retain marks on default if converted
            (
                Value::map([
                    ("boop", Value::string("beep").mark("a")),
                    ("frob", Value::string("honk").mark("b")),
                ]),
                Value::string("squish"),
                Value::number(5).mark("c"),
                Value::string("5").mark("c"),
            ),
            // propagate marks from key
            (
                Value::map([
                    ("boop", Value::string("beep")),
                    ("frob", Value::string("honk")),
                ]),
                Value::string("boop").mark("a"),
                Value::string("nope"),
                Value::string("beep").mark("a"),
            ),
        ];

        for (i, (collection, key, default, want)) in tests.iter().enumerate() {
            let got = stdlib::lookup(collection, key, default)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
