//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/value_ops_test.go (TestValueRawEquals)
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Type, Value};

// Ported from TestValueRawEquals:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L904
#[test]
fn value_raw_equals() {
    // Mirrors the capsule test types declared in upstream cty/capsule_test.go.
    #[derive(Debug)]
    #[allow(dead_code)] // the field mirrors the upstream struct; only identity matters here
    struct CapsuleTestType1Native {
        name: String,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct CapsuleTestType2Native {
        name: String,
    }

    let capsule_test_type1 = Type::capsule::<CapsuleTestType1Native>("capsule test type 1");
    let capsule_test_type2 = Type::capsule::<CapsuleTestType2Native>("capsule test type 2");

    // NOTE(port): upstream reuses the same Go pointer for both operands of the
    // `capsuleA.RawEquals(capsuleA)` case; the Rust analogue of that shared
    // identity is `.clone()` of the capsule Value.
    let capsule_a = Value::capsule(
        capsule_test_type1.clone(),
        CapsuleTestType1Native {
            name: "capsuleA".to_string(),
        },
    );
    let capsule_b = Value::capsule(
        capsule_test_type1.clone(),
        CapsuleTestType1Native {
            name: "capsuleB".to_string(),
        },
    );
    let capsule_c = Value::capsule(
        capsule_test_type2.clone(),
        CapsuleTestType2Native {
            name: "capsuleC".to_string(),
        },
    );

    let tests: Vec<(Value, Value, bool)> = vec![
        // Booleans
        (Value::bool(true), Value::bool(true), true),
        (Value::bool(false), Value::bool(false), true),
        (Value::bool(true), Value::bool(false), false),
        // Numbers
        (Value::number_int(1), Value::number_int(2), false),
        (Value::number_int(2), Value::number_int(2), true),
        // Strings
        (Value::string(""), Value::string(""), true),
        (Value::string("hello"), Value::string("hello"), true),
        (Value::string("hello"), Value::string("world"), false),
        (Value::string("0"), Value::string(""), false),
        (Value::string("a\u{f1}os"), Value::string("a\u{f1}os"), true),
        (
            // Combining marks are normalized by Value::string
            Value::string("a\u{f1}os"),   // (precomposed tilde-n)
            Value::string("an\u{303}os"), // (combining tilde followed by bare n)
            true,
        ),
        (
            // tilde-n does not normalize with bare n
            Value::string("a\u{f1}os"),
            Value::string("anos"),
            false,
        ),
        // Objects
        (
            Value::object([] as [(&str, Value); 0]),
            Value::object([] as [(&str, Value); 0]),
            true,
        ),
        (
            Value::object([("num", Value::number_int(1))]),
            Value::object([("num", Value::number_int(1))]),
            true,
        ),
        (
            Value::object([("h\u{e9}llo", Value::number_int(1))]), // precombined é
            Value::object([("he\u{301}llo", Value::number_int(1))]), // e with combining acute accent
            true,
        ),
        (
            Value::object([("num", Value::number_int(1))]),
            Value::object([] as [(&str, Value); 0]),
            false,
        ),
        (
            Value::object([("num", Value::number_int(1)), ("flag", Value::bool(true))]),
            Value::object([("num", Value::number_int(1)), ("flag", Value::bool(true))]),
            true,
        ),
        (
            Value::object([("num", Value::number_int(1))]),
            Value::object([("num", Value::number_int(2))]),
            false,
        ),
        (
            Value::object([("num", Value::number_int(1))]),
            Value::object([("othernum", Value::number_int(1))]),
            false,
        ),
        (
            Value::object([("num", Value::number_int(1)), ("flag", Value::bool(true))]),
            Value::object([("num", Value::number_int(1))]),
            false,
        ),
        (
            Value::object([("num", Value::number_int(1)), ("flag", Value::bool(true))]),
            Value::object([("num", Value::number_int(1)), ("flag", Value::bool(false))]),
            false,
        ),
        // Tuples
        (Value::empty_tuple(), Value::empty_tuple(), true),
        (
            Value::tuple([Value::number_int(1)]),
            Value::tuple([Value::number_int(1)]),
            true,
        ),
        (
            Value::tuple([Value::number_int(1)]),
            Value::tuple([Value::number_int(2)]),
            false,
        ),
        (
            Value::tuple([Value::string("hi")]),
            Value::tuple([Value::number_int(1)]),
            false,
        ),
        (
            Value::tuple([Value::number_int(1)]),
            Value::tuple([Value::number_int(1), Value::number_int(2)]),
            false,
        ),
        (
            Value::tuple([Value::number_int(1), Value::number_int(2)]),
            Value::tuple([Value::number_int(1)]),
            false,
        ),
        (
            Value::tuple([Value::number_int(1), Value::number_int(2)]),
            Value::tuple([Value::number_int(1), Value::number_int(2)]),
            true,
        ),
        (
            Value::tuple([Value::unknown(Type::number())]),
            Value::tuple([Value::number_int(1)]),
            false,
        ),
        (
            Value::tuple([Value::unknown(Type::number())]),
            Value::tuple([Value::unknown(Type::number())]),
            true,
        ),
        (
            Value::tuple([Value::number_int(1)]),
            Value::tuple([Value::unknown(Type::number())]),
            false,
        ),
        (
            Value::tuple([Value::number_int(1)]),
            Value::tuple([Value::dynamic()]),
            false,
        ),
        (
            Value::tuple([Value::dynamic()]),
            Value::tuple([Value::number_int(1)]),
            false,
        ),
        (
            Value::tuple([Value::number_int(1)]),
            Value::unknown(Type::tuple([Type::number()])),
            false,
        ),
        (
            Value::unknown(Type::tuple([Type::number()])),
            Value::tuple([Value::number_int(1)]),
            false,
        ),
        (
            Value::dynamic(),
            Value::tuple([Value::number_int(1)]),
            false,
        ),
        (
            Value::tuple([Value::number_int(1)]),
            Value::dynamic(),
            false,
        ),
        // Lists
        (
            Value::list_empty(Type::number()),
            Value::list_empty(Type::number()),
            true,
        ),
        (
            Value::list_empty(Type::number()),
            Value::list_empty(Type::bool()),
            false,
        ),
        (
            Value::list([Value::number_int(1)]),
            Value::list([Value::number_int(1)]),
            true,
        ),
        (
            Value::list([Value::number_int(1)]),
            Value::list_empty(Type::string()),
            false,
        ),
        (
            Value::list([Value::number_int(1), Value::number_int(2)]),
            Value::list([Value::number_int(1), Value::number_int(2)]),
            true,
        ),
        (
            Value::list([Value::number_int(1)]),
            Value::list([Value::number_int(2)]),
            false,
        ),
        (
            Value::list([Value::number_int(1), Value::number_int(2)]),
            Value::list([Value::number_int(1)]),
            false,
        ),
        (
            Value::list([Value::number_int(1)]),
            Value::list([Value::number_int(1), Value::number_int(2)]),
            false,
        ),
        // Maps
        (
            Value::map_empty(Type::number()),
            Value::map_empty(Type::number()),
            true,
        ),
        (
            Value::map_empty(Type::number()).mark("a"),
            Value::map_empty(Type::number()).mark("a"),
            true,
        ),
        (
            Value::map_empty(Type::number()).mark("a"),
            Value::map_empty(Type::number()),
            false,
        ),
        (
            Value::map_empty(Type::number()),
            Value::map_empty(Type::number()).mark("a"),
            false,
        ),
        (
            Value::map_empty(Type::number()).mark("a"),
            Value::map_empty(Type::number()).mark("a").mark("b"),
            false,
        ),
        (
            Value::map_empty(Type::number()),
            Value::map_empty(Type::bool()),
            false,
        ),
        (
            Value::map([("num", Value::number_int(1))]),
            Value::map([("num", Value::number_int(1))]),
            true,
        ),
        (
            Value::map([("h\u{e9}llo", Value::number_int(1))]), // precombined é
            Value::map([("he\u{301}llo", Value::number_int(1))]), // e with combining acute accent
            true,
        ),
        (
            Value::map([("num", Value::number_int(1))]),
            Value::map_empty(Type::string()),
            false,
        ),
        (
            Value::map([
                ("num1", Value::number_int(1)),
                ("num2", Value::number_int(2)),
            ]),
            Value::map([
                ("num1", Value::number_int(1)),
                ("num2", Value::number_int(2)),
            ]),
            true,
        ),
        (
            Value::map([("num", Value::number_int(1))]),
            Value::map([("num", Value::number_int(2))]),
            false,
        ),
        (
            Value::map([("num", Value::number_int(1))]),
            Value::map([("othernum", Value::number_int(1))]),
            false,
        ),
        (
            Value::map([
                ("num1", Value::number_int(1)),
                ("num2", Value::number_int(2)),
            ]),
            Value::map([("num1", Value::number_int(1))]),
            false,
        ),
        (
            Value::map([("num1", Value::number_int(1))]),
            Value::map([
                ("num1", Value::number_int(1)),
                ("num2", Value::number_int(2)),
            ]),
            false,
        ),
        (
            Value::map([
                ("num1", Value::number_int(1)),
                ("num2", Value::number_int(2)),
            ]),
            Value::map([
                ("num1", Value::number_int(1)),
                ("num2", Value::number_int(3)),
            ]),
            false,
        ),
        // Sets
        (
            Value::set_empty(Type::number()),
            Value::set_empty(Type::number()),
            true,
        ),
        (
            Value::set_empty(Type::number()),
            Value::set_empty(Type::bool()),
            false,
        ),
        (
            Value::set([Value::number_int(1)]),
            Value::set([Value::number_int(1)]),
            true,
        ),
        (
            Value::set([Value::number_int(1)]),
            Value::set_empty(Type::string()),
            false,
        ),
        (
            Value::set([Value::number_int(1), Value::number_int(2)]),
            Value::set([Value::number_int(2), Value::number_int(1)]),
            true,
        ),
        (
            Value::set([Value::number_int(1)]),
            Value::set([Value::number_int(2)]),
            false,
        ),
        (
            Value::set([Value::number_int(1), Value::number_int(2)]),
            Value::set([Value::number_int(1)]),
            false,
        ),
        (
            Value::set([Value::number_int(1)]),
            Value::set([Value::number_int(1), Value::number_int(2)]),
            false,
        ),
        // Capsules
        (capsule_a.clone(), capsule_a.clone(), true),
        (capsule_a.clone(), capsule_b, false),
        (capsule_a.clone(), capsule_c, false),
        (
            capsule_a.clone(),
            Value::unknown(capsule_test_type1), // same type
            false,
        ),
        (
            capsule_a,
            Value::unknown(capsule_test_type2), // different type
            false,
        ),
        // Unknowns and Dynamics
        (Value::number_int(2), Value::unknown(Type::number()), false),
        (Value::number_int(1), Value::dynamic(), false),
        (Value::dynamic(), Value::bool(true), false),
        (
            Value::dynamic(),
            Value::dynamic(),
            true, //?
        ),
        (
            Value::list([Value::string("hi"), Value::dynamic()]),
            Value::list([Value::string("hi"), Value::dynamic()]),
            true,
        ),
        (
            Value::list([Value::string("hi"), Value::unknown(Type::string())]),
            Value::list([Value::string("hi"), Value::unknown(Type::string())]),
            true,
        ),
        (
            Value::map([
                ("static", Value::string("hi")),
                ("dynamic", Value::dynamic()),
            ]),
            Value::map([
                ("static", Value::string("hi")),
                ("dynamic", Value::dynamic()),
            ]),
            true,
        ),
        (
            Value::map([
                ("static", Value::string("hi")),
                ("dynamic", Value::unknown(Type::string())),
            ]),
            Value::map([
                ("static", Value::string("hi")),
                ("dynamic", Value::unknown(Type::string())),
            ]),
            true,
        ),
        (
            Value::null(Type::string()),
            Value::null(Type::dynamic()),
            false,
        ),
        (
            Value::null(Type::string()),
            Value::null(Type::string()),
            true,
        ),
        (
            Value::unknown(Type::string()),
            Value::unknown(Type::number()),
            false,
        ),
        (Value::string(""), Value::null(Type::dynamic()), false),
        (Value::string(""), Value::null(Type::string()), false),
        (Value::string(""), Value::unknown(Type::string()), false),
        (
            Value::null(Type::dynamic()),
            Value::null(Type::dynamic()),
            true,
        ),
        (
            Value::null(Type::string()),
            Value::unknown(Type::number()),
            false, // because second operand might eventually be null
        ),
        (
            Value::unknown(Type::string()),
            Value::null(Type::number()),
            false, // because first operand might eventually be null
        ),
        (
            Value::unknown(Type::string()),
            Value::unknown(Type::number()),
            false, // because both operands might eventually be null
        ),
        (
            Value::string("hello"),
            Value::unknown(Type::number()),
            false, // because no number value -- even null -- can be equal to a non-null string
        ),
        (
            Value::unknown(Type::string()),
            Value::number_int(1),
            false, // because no string value -- even null -- can be equal to a non-null number
        ),
        (
            Value::object([("a", Value::string("a"))]),
            // A null value is always known
            Value::object([("a", Value::null(Type::dynamic()))]),
            false,
        ),
        (
            Value::object([("a", Value::null(Type::dynamic()))]),
            Value::object([("a", Value::null(Type::dynamic()))]),
            true,
        ),
        (
            Value::object([
                ("a", Value::string("a")),
                ("b", Value::unknown(Type::number())),
            ]),
            // While we have a dynamic type, the different object types should
            // still compare false
            Value::object([
                ("a", Value::null(Type::dynamic())),
                ("c", Value::unknown(Type::number())),
            ]),
            false,
        ),
        (
            Value::object([
                ("a", Value::string("a")),
                ("b", Value::unknown(Type::number())),
            ]),
            // While we have a dynamic type, the different object types should
            // still compare false
            Value::object([
                ("a", Value::dynamic()),
                ("c", Value::unknown(Type::number())),
            ]),
            false,
        ),
        (
            Value::object([("a", Value::null(Type::dynamic()))]),
            Value::object([("a", Value::dynamic())]),
            false,
        ),
        (
            Value::object([("a", Value::null(Type::list(Type::string())))]),
            // While the unknown val does contain dynamic types, the overall
            // container types can't conform.
            Value::object([("a", Value::unknown(Type::list(Type::list(Type::dynamic()))))]),
            false,
        ),
        (
            Value::object([("a", Value::null(Type::list(Type::list(Type::string()))))]),
            Value::object([("a", Value::unknown(Type::list(Type::list(Type::dynamic()))))]),
            false,
        ),
        (
            Value::object([(
                "a",
                Value::set([Value::object([("b", Value::unknown(Type::string()))])]),
            )]),
            Value::object([(
                "a",
                Value::set([Value::object([("b", Value::unknown(Type::string()))])]),
            )]),
            true,
        ),
        (
            Value::object([(
                "a",
                Value::set([Value::object([
                    ("b", Value::unknown(Type::string())),
                    ("c", Value::string("cee")),
                ])]),
            )]),
            Value::object([(
                "a",
                Value::set([Value::object([
                    ("b", Value::unknown(Type::string())),
                    ("c", Value::string("cee")),
                ])]),
            )]),
            true,
        ),
        (
            Value::object([(
                "a",
                Value::set([Value::object([("b", Value::unknown(Type::string()))])]),
            )]),
            Value::object([(
                "a",
                Value::set([Value::object([("c", Value::unknown(Type::string()))])]),
            )]),
            false,
        ),
        (
            Value::unknown(Type::bool()).refine().not_null().new_value(),
            Value::unknown(Type::bool()),
            false,
        ),
        (
            Value::unknown(Type::bool()),
            Value::unknown(Type::bool()).refine().not_null().new_value(),
            false,
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_inclusive(Value::zero(), Value::number_int(1))
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .number_range_inclusive(Value::zero(), Value::number_int(2))
                .new_value(),
            false,
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_inclusive(Value::zero(), Value::number_int(1))
                .new_value(),
            Value::unknown(Type::number())
                .refine()
                .number_range_inclusive(Value::zero(), Value::number_int(1))
                .new_value(),
            true,
        ),
        (
            Value::unknown(Type::string()),
            Value::unknown(Type::string())
                .refine()
                .string_prefix("foo")
                .new_value(),
            false,
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix("foo")
                .new_value(),
            Value::unknown(Type::string())
                .refine()
                .string_prefix("foo")
                .new_value(),
            true,
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .not_null()
                .string_prefix("foo")
                .new_value(),
            Value::unknown(Type::string())
                .refine()
                .string_prefix("foo")
                .new_value(),
            false,
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix("foo")
                .new_value(),
            Value::unknown(Type::string())
                .refine()
                .string_prefix("bar")
                .new_value(),
            false,
        ),
        (
            Value::unknown(Type::string()).refine().null().new_value(),
            Value::null(Type::string()),
            true, // The refinement expression collapses into a simple null
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .number_range_inclusive(Value::zero(), Value::zero())
                .new_value(),
            Value::zero(),
            false, // Refinement can't collapse to zero because it might be null
        ),
        (
            Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_inclusive(Value::zero(), Value::zero())
                .new_value(),
            Value::zero(),
            true, // Refinement collapses to zero because it's not null and the two bounds are equal
        ),
        (
            Value::unknown(Type::list(Type::string()))
                .refine()
                .not_null()
                .collection_length_upper_bound(0)
                .new_value(),
            Value::list_empty(Type::string()),
            true, // Colection length lower bound is always at least zero so this refinement collapses to an empty list
        ),
        // Marks
        (Value::string("a").mark(1), Value::string("b"), false),
        (Value::string("a"), Value::string("b").mark(2), false),
        (
            Value::string("a").mark(1),
            Value::string("b").mark(2),
            false,
        ),
    ];

    for (i, (lhs, rhs, expected)) in tests.iter().enumerate() {
        let got = lhs.raw_equals(rhs);
        assert_eq!(
            got, *expected,
            "case {i}: {lhs:?}.raw_equals({rhs:?}) returned {got}, want {expected}"
        );
    }
}
