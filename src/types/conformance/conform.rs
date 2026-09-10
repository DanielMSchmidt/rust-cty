//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/type_conform_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use crate::Type;

// Ported from TestTypeTestConformance:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_conform_test.go#L9
//
// Upstream TestTypeTestConformance is a single table covering several concepts at once.
// It is split below into one test per concept so they can be activated one at
// a time. Rows are transcribed verbatim and keep upstream order within a test;
// nothing is added, dropped, or rewritten.
mod type_test_conformance {
    use super::*;

    /// Runs transcribed upstream rows; fixture plumbing only, every
    /// expected value is a literal from the upstream table.
    fn check(tests: &[(Type, Type, bool)]) {
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

    // TestTypeTestConformance, primitives:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_conform_test.go#L9
    #[test]
    #[ignore = "not yet implemented"]
    fn primitives() {
        let tests: Vec<(Type, Type, bool)> = vec![
            (Type::number(), Type::number(), true),
            (Type::number(), Type::string(), false),
            (Type::empty_object(), Type::empty_object(), true),
            (Type::empty_tuple(), Type::empty_tuple(), true),
        ];

        check(&tests);
    }

    // TestTypeTestConformance, collections:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_conform_test.go#L9
    #[test]
    #[ignore = "not yet implemented"]
    fn collections() {
        let tests: Vec<(Type, Type, bool)> = vec![
            (Type::list(Type::number()), Type::list(Type::number()), true),
            (Type::list(Type::number()), Type::map(Type::number()), false),
            (
                Type::list(Type::number()),
                Type::list(Type::string()),
                false,
            ),
            (Type::map(Type::number()), Type::map(Type::number()), true),
            (Type::map(Type::number()), Type::set(Type::number()), false),
            (Type::map(Type::number()), Type::map(Type::string()), false),
            (Type::set(Type::number()), Type::set(Type::number()), true),
            (Type::set(Type::number()), Type::list(Type::number()), false),
            (Type::set(Type::number()), Type::set(Type::string()), false),
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

        check(&tests);
    }

    // TestTypeTestConformance, unknowns:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_conform_test.go#L9
    #[test]
    #[ignore = "not yet implemented"]
    fn unknowns() {
        let tests: Vec<(Type, Type, bool)> = vec![
            (Type::number(), Type::dynamic(), true),
            (Type::dynamic(), Type::dynamic(), true),
            (Type::dynamic(), Type::number(), false),
            (
                Type::list(Type::number()),
                Type::list(Type::dynamic()),
                true,
            ),
            (
                Type::list(Type::number()),
                Type::map(Type::dynamic()),
                false,
            ),
            (Type::map(Type::number()), Type::map(Type::dynamic()), true),
            (
                Type::set(Type::number()),
                Type::list(Type::dynamic()),
                false,
            ),
            (Type::set(Type::number()), Type::set(Type::dynamic()), true),
        ];

        check(&tests);
    }

    // TestTypeTestConformance, optional attrs:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_conform_test.go#L9
    #[test]
    #[ignore = "not yet implemented"]
    fn optional_attrs() {
        let tests: Vec<(Type, Type, bool)> = vec![
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
        ];

        check(&tests);
    }
}
