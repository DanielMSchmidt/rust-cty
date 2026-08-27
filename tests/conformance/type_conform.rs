//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/type_conform_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::Type;

// Ported from TestTypeTestConformance:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_conform_test.go#L9
#[test]
#[ignore = "not yet implemented"]
fn type_test_conformance() {
    let tests: Vec<(Type, Type, bool)> = vec![
        (Type::number(), Type::number(), true),
        (Type::number(), Type::string(), false),
        (Type::number(), Type::dynamic(), true),
        (Type::dynamic(), Type::dynamic(), true),
        (Type::dynamic(), Type::number(), false),
        (Type::list(Type::number()), Type::list(Type::number()), true),
        (Type::list(Type::number()), Type::map(Type::number()), false),
        (
            Type::list(Type::number()),
            Type::list(Type::dynamic()),
            true,
        ),
        (
            Type::list(Type::number()),
            Type::list(Type::string()),
            false,
        ),
        (Type::map(Type::number()), Type::map(Type::number()), true),
        (Type::map(Type::number()), Type::set(Type::number()), false),
        (
            Type::list(Type::number()),
            Type::map(Type::dynamic()),
            false,
        ),
        (Type::map(Type::number()), Type::map(Type::dynamic()), true),
        (Type::map(Type::number()), Type::map(Type::string()), false),
        (Type::set(Type::number()), Type::set(Type::number()), true),
        (Type::set(Type::number()), Type::list(Type::number()), false),
        (
            Type::set(Type::number()),
            Type::list(Type::dynamic()),
            false,
        ),
        (Type::set(Type::number()), Type::set(Type::dynamic()), true),
        (Type::set(Type::number()), Type::set(Type::string()), false),
        (Type::empty_object(), Type::empty_object(), true),
        (
            Type::empty_object(),
            Type::object([("name", Type::string())]),
            false,
        ),
        (
            Type::object([("name", Type::string())]),
            Type::empty_object(),
            false,
        ),
        (
            Type::object([("name", Type::string())]),
            Type::object([("name", Type::string())]),
            true,
        ),
        (
            Type::object([("name", Type::string())]),
            Type::object([("gnome", Type::string())]),
            false,
        ),
        (
            Type::object([("name", Type::number())]),
            Type::object([("name", Type::string())]),
            false,
        ),
        (
            Type::object([("name", Type::number())]),
            Type::object([("name", Type::string()), ("number", Type::number())]),
            false,
        ),
        (
            Type::object_with_optional_attrs([("name", Type::number())], &["name"]),
            Type::object([("name", Type::number())]),
            true,
        ),
        (
            Type::object_with_optional_attrs([("name", Type::number())], &["name"]),
            Type::empty_object(),
            false, // "optionalness" of attributes is only considered under conversion, not for conformance
        ),
        (Type::empty_tuple(), Type::empty_tuple(), true),
        (Type::empty_tuple(), Type::tuple([Type::string()]), false),
        (Type::empty_tuple(), Type::tuple([Type::string()]), false),
        (
            Type::tuple([Type::string()]),
            Type::tuple([Type::string()]),
            true,
        ),
        (
            Type::tuple([Type::string()]),
            Type::tuple([Type::number()]),
            false,
        ),
        (
            Type::tuple([Type::string(), Type::number()]),
            Type::tuple([Type::string(), Type::number()]),
            true,
        ),
        (
            Type::tuple([Type::string()]),
            Type::tuple([Type::string(), Type::number()]),
            false,
        ),
        (
            Type::tuple([Type::string(), Type::number()]),
            Type::tuple([Type::string()]),
            false,
        ),
    ];

    for (i, (receiver, given, conforms)) in tests.iter().enumerate() {
        let result = receiver.test_conformance(given);
        if *conforms {
            assert!(
                result.is_ok(),
                "case {i}: ({receiver:?}).test_conformance({given:?}): unexpected errors\n{:?}",
                result.unwrap_err()
            );
        } else {
            assert!(
                result.is_err(),
                "case {i}: ({receiver:?}).test_conformance({given:?}): expected errors, but got none"
            );
        }
    }
}
