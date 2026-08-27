//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/value_ops_test.go (TestValueEquals)
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Type, Value, ValueMarks};

// Capsule test types shared by the upstream cty package tests
// (cty/capsule_test.go: capsuleTestType1Native / capsuleTestType2Native,
// capsuleTestType1 / capsuleTestType2).
#[derive(Debug)]
struct CapsuleTestType1Native {
    #[allow(dead_code)]
    name: String,
}

#[derive(Debug)]
struct CapsuleTestType2Native {
    #[allow(dead_code)]
    name: String,
}

fn capsule_test_type_1() -> Type {
    Type::capsule::<CapsuleTestType1Native>("capsule test type 1")
}

fn capsule_test_type_2() -> Type {
    Type::capsule::<CapsuleTestType2Native>("capsule test type 2")
}

// Ported from TestValueEquals:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L11
#[test]
#[ignore = "not yet implemented"]
fn value_equals() {
    // NOTE(port): upstream constructs each capsule value once and reuses the
    // same Go pointer across cases; the Rust analogue of that shared identity
    // is cloning the capsule Value.
    let capsule_a = Value::capsule(
        capsule_test_type_1(),
        CapsuleTestType1Native {
            name: "capsuleA".to_string(),
        },
    );
    let capsule_b = Value::capsule(
        capsule_test_type_1(),
        CapsuleTestType1Native {
            name: "capsuleB".to_string(),
        },
    );
    let capsule_c = Value::capsule(
        capsule_test_type_2(),
        CapsuleTestType2Native {
            name: "capsuleC".to_string(),
        },
    );

    let unknown_result = Value::unknown(Type::bool()).refine_not_null();

    let tests: Vec<(Value, Value, Value)> = vec![
        // Booleans
        (Value::bool(true), Value::bool(true), Value::bool(true)),
        (Value::bool(false), Value::bool(false), Value::bool(true)),
        (Value::bool(true), Value::bool(false), Value::bool(false)),
        // Numbers
        (
            Value::number_int(1),
            Value::number_int(2),
            Value::bool(false),
        ),
        (
            Value::number_int(2),
            Value::number_int(2),
            Value::bool(true),
        ),
        (
            Value::number_int(2),
            Value::number_float(2.2),
            Value::bool(false),
        ),
        (
            Value::number_float(2.0),
            Value::number_float(2.2),
            Value::bool(false),
        ),
        (
            Value::parse_number("0.0").unwrap(),
            Value::parse_number("-0.0").unwrap(), // a statically-generated negative zero
            Value::bool(true),
        ),
        (
            Value::number_float(0.0),
            Value::number_float(0.0).multiply(&Value::number_int(-1)), // a dynamically-generated negative zero
            Value::bool(true),
        ),
        (
            Value::parse_number("3.14159265358979323846264338327950288419716939937510582097494459")
                .unwrap(),
            Value::parse_number("3.14159265358979323846264338327950288419716939937510582097494459")
                .unwrap(),
            Value::bool(true),
        ),
        (
            Value::parse_number(
                "-3.14159265358979323846264338327950288419716939937510582097494459",
            )
            .unwrap(),
            Value::parse_number(
                "-3.14159265358979323846264338327950288419716939937510582097494459",
            )
            .unwrap(),
            Value::bool(true),
        ),
        (
            Value::parse_number("3.14159265358979323846264338327950288419716939937510582097494459")
                .unwrap(),
            Value::parse_number(
                "-3.14159265358979323846264338327950288419716939937510582097494459",
            )
            .unwrap(),
            Value::bool(false),
        ),
        (
            Value::parse_number("1.2").unwrap(),
            Value::number_float(1.2),
            Value::bool(true),
        ),
        (
            Value::parse_number("1.22222").unwrap(),
            Value::number_float(1.22222),
            Value::bool(true),
        ),
        (
            Value::parse_number("9223372036854775808").unwrap(),
            Value::number_float(9223372036854775808.0),
            Value::bool(true),
        ),
        // Strings
        (Value::string(""), Value::string(""), Value::bool(true)),
        (
            Value::string("hello"),
            Value::string("hello"),
            Value::bool(true),
        ),
        (
            Value::string("hello"),
            Value::string("world"),
            Value::bool(false),
        ),
        (Value::string("0"), Value::string(""), Value::bool(false)),
        (
            Value::string("a\u{f1}os"),
            Value::string("a\u{f1}os"),
            Value::bool(true),
        ),
        (
            // Combining marks are normalized by Value::string
            Value::string("a\u{f1}os"),   // (precomposed tilde-n)
            Value::string("an\u{303}os"), // (combining tilde following bare n)
            Value::bool(true),
        ),
        (
            // tilde-n does not normalize with bare n
            Value::string("a\u{f1}os"),
            Value::string("anos"),
            Value::bool(false),
        ),
        // Objects
        (
            Value::object([] as [(&str, Value); 0]),
            Value::object([] as [(&str, Value); 0]),
            Value::bool(true),
        ),
        (
            Value::object([("num", Value::number_int(1))]),
            Value::object([("num", Value::number_int(1))]),
            Value::bool(true),
        ),
        (
            Value::object([("h\u{e9}llo", Value::number_int(1))]), // precombined é
            Value::object([("he\u{301}llo", Value::number_int(1))]), // e with combining acute accent
            Value::bool(true),
        ),
        (
            Value::object([("num", Value::number_int(1))]),
            Value::object([] as [(&str, Value); 0]),
            Value::bool(false),
        ),
        (
            Value::object([("num", Value::number_int(1)), ("flag", Value::bool(true))]),
            Value::object([("num", Value::number_int(1)), ("flag", Value::bool(true))]),
            Value::bool(true),
        ),
        (
            Value::object([("num", Value::number_int(1))]),
            Value::object([("num", Value::number_int(2))]),
            Value::bool(false),
        ),
        (
            Value::object([("num", Value::number_int(1))]),
            Value::object([("othernum", Value::number_int(1))]),
            Value::bool(false),
        ),
        (
            Value::object([("num", Value::number_int(1)), ("flag", Value::bool(true))]),
            Value::object([("num", Value::number_int(1))]),
            Value::bool(false),
        ),
        (
            Value::object([("num", Value::number_int(1)), ("flag", Value::bool(true))]),
            Value::object([("num", Value::number_int(1)), ("flag", Value::bool(false))]),
            Value::bool(false),
        ),
        // Tuples
        (
            Value::empty_tuple(),
            Value::empty_tuple(),
            Value::bool(true),
        ),
        (
            Value::tuple([Value::number_int(1)]),
            Value::tuple([Value::number_int(1)]),
            Value::bool(true),
        ),
        (
            Value::tuple([Value::number_int(1)]),
            Value::tuple([Value::number_int(2)]),
            Value::bool(false),
        ),
        (
            Value::tuple([Value::string("hi")]),
            Value::tuple([Value::number_int(1)]),
            Value::bool(false),
        ),
        (
            Value::tuple([Value::number_int(1)]),
            Value::tuple([Value::number_int(1), Value::number_int(2)]),
            Value::bool(false),
        ),
        (
            Value::tuple([Value::number_int(1), Value::number_int(2)]),
            Value::tuple([Value::number_int(1)]),
            Value::bool(false),
        ),
        (
            Value::tuple([Value::number_int(1), Value::number_int(2)]),
            Value::tuple([Value::number_int(1), Value::number_int(2)]),
            Value::bool(true),
        ),
        (
            Value::tuple([Value::unknown(Type::number())]),
            Value::tuple([Value::number_int(1)]),
            unknown_result.clone(),
        ),
        (
            Value::tuple([Value::unknown(Type::number())]),
            Value::tuple([Value::unknown(Type::number())]),
            unknown_result.clone(),
        ),
        (
            Value::tuple([Value::number_int(1)]),
            Value::tuple([Value::unknown(Type::number())]),
            unknown_result.clone(),
        ),
        (
            Value::tuple([Value::number_int(1)]),
            Value::tuple([Value::dynamic()]),
            unknown_result.clone(),
        ),
        (
            Value::tuple([Value::dynamic()]),
            Value::tuple([Value::number_int(1)]),
            unknown_result.clone(),
        ),
        (
            Value::tuple([Value::number_int(1)]),
            Value::unknown(Type::tuple([Type::number()])),
            unknown_result.clone(),
        ),
        (
            Value::unknown(Type::tuple([Type::number()])),
            Value::tuple([Value::number_int(1)]),
            unknown_result.clone(),
        ),
        (
            Value::dynamic(),
            Value::tuple([Value::number_int(1)]),
            unknown_result.clone(),
        ),
        (
            Value::tuple([Value::number_int(1)]),
            Value::dynamic(),
            unknown_result.clone(),
        ),
        // Lists
        (
            Value::list_empty(Type::number()),
            Value::list_empty(Type::number()),
            Value::bool(true),
        ),
        (
            Value::list_empty(Type::number()),
            Value::list_empty(Type::bool()),
            Value::bool(false),
        ),
        (
            Value::list([Value::number_int(1)]),
            Value::list([Value::number_int(1)]),
            Value::bool(true),
        ),
        (
            Value::list([Value::number_int(1)]),
            Value::list_empty(Type::string()),
            Value::bool(false),
        ),
        (
            Value::list([Value::number_int(1), Value::number_int(2)]),
            Value::list([Value::number_int(1), Value::number_int(2)]),
            Value::bool(true),
        ),
        (
            Value::list([Value::number_int(1)]),
            Value::list([Value::number_int(2)]),
            Value::bool(false),
        ),
        (
            Value::list([Value::number_int(1), Value::number_int(2)]),
            Value::list([Value::number_int(1)]),
            Value::bool(false),
        ),
        (
            Value::list([Value::number_int(1)]),
            Value::list([Value::number_int(1), Value::number_int(2)]),
            Value::bool(false),
        ),
        // Maps
        (
            Value::map_empty(Type::number()),
            Value::map_empty(Type::number()),
            Value::bool(true),
        ),
        (
            Value::map_empty(Type::number()),
            Value::map_empty(Type::bool()),
            Value::bool(false),
        ),
        (
            Value::map([("num", Value::number_int(1))]),
            Value::map([("num", Value::number_int(1))]),
            Value::bool(true),
        ),
        (
            Value::map([("h\u{e9}llo", Value::number_int(1))]), // precombined é
            Value::map([("he\u{301}llo", Value::number_int(1))]), // e with combining acute accent
            Value::bool(true),
        ),
        (
            Value::map([("num", Value::number_int(1))]),
            Value::map_empty(Type::string()),
            Value::bool(false),
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
            Value::bool(true),
        ),
        (
            Value::map([("num", Value::number_int(1))]),
            Value::map([("num", Value::number_int(2))]),
            Value::bool(false),
        ),
        (
            Value::map([("num", Value::number_int(1))]),
            Value::map([("othernum", Value::number_int(1))]),
            Value::bool(false),
        ),
        (
            Value::map([
                ("num1", Value::number_int(1)),
                ("num2", Value::number_int(2)),
            ]),
            Value::map([("num1", Value::number_int(1))]),
            Value::bool(false),
        ),
        (
            Value::map([("num1", Value::number_int(1))]),
            Value::map([
                ("num1", Value::number_int(1)),
                ("num2", Value::number_int(2)),
            ]),
            Value::bool(false),
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
            Value::bool(false),
        ),
        // Sets
        (
            Value::set_empty(Type::number()),
            Value::set_empty(Type::number()),
            Value::bool(true),
        ),
        (
            Value::set_empty(Type::number()),
            Value::set_empty(Type::bool()),
            Value::bool(false),
        ),
        (
            Value::set([Value::number_int(1)]),
            Value::set([Value::number_int(1)]),
            Value::bool(true),
        ),
        (
            Value::set([Value::number_int(1)]),
            Value::set_empty(Type::string()),
            Value::bool(false),
        ),
        (
            Value::set([Value::number_int(1), Value::number_int(2)]),
            Value::set([Value::number_int(2), Value::number_int(1)]),
            Value::bool(true),
        ),
        (
            Value::set([Value::number_int(1)]),
            Value::set([Value::number_int(2)]),
            Value::bool(false),
        ),
        (
            Value::set([Value::number_int(1), Value::number_int(2)]),
            Value::set([Value::number_int(1)]),
            Value::bool(false),
        ),
        (
            Value::set([Value::number_int(1)]),
            Value::set([Value::number_int(1), Value::number_int(2)]),
            Value::bool(false),
        ),
        (
            Value::set([Value::number_int(1)]),
            Value::set([Value::unknown(Type::number())]),
            unknown_result.clone(),
        ),
        (
            Value::set([Value::number_int(1)]),
            Value::set([Value::number_int(1), Value::unknown(Type::number())]),
            unknown_result.clone(),
        ),
        (
            Value::set([Value::number_int(1), Value::unknown(Type::number())]),
            Value::set([Value::number_int(1)]),
            unknown_result.clone(),
        ),
        // Capsules
        (capsule_a.clone(), capsule_a.clone(), Value::bool(true)),
        (capsule_a.clone(), capsule_b.clone(), Value::bool(false)),
        (capsule_a.clone(), capsule_c.clone(), Value::bool(false)),
        (
            capsule_a.clone(),
            Value::unknown(capsule_test_type_1()), // same type
            unknown_result.clone(),
        ),
        (
            capsule_a.clone(),
            Value::unknown(capsule_test_type_2()), // different type
            Value::bool(false),
        ),
        // Unknowns and Dynamics
        (
            Value::number_int(2),
            Value::unknown(Type::number()),
            unknown_result.clone(),
        ),
        (
            Value::number_int(1),
            Value::dynamic(),
            unknown_result.clone(),
        ),
        (
            Value::number_int(2),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::zero(), true)
                .new_value(),
            unknown_result.clone(),
        ),
        (
            Value::number_int(2),
            Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(4), true)
                .new_value(),
            Value::bool(false), // deduction from refinement
        ),
        (Value::dynamic(), Value::bool(true), unknown_result.clone()),
        (Value::dynamic(), Value::dynamic(), unknown_result.clone()),
        (
            Value::list([Value::string("hi"), Value::dynamic()]),
            Value::list([Value::string("hi"), Value::dynamic()]),
            unknown_result.clone(),
        ),
        (
            Value::list([Value::string("hi"), Value::unknown(Type::string())]),
            Value::list([Value::string("hi"), Value::unknown(Type::string())]),
            unknown_result.clone(),
        ),
        (
            Value::unknown(Type::list(Type::string()))
                .refine()
                .collection_length_lower_bound(1)
                .new_value(),
            Value::list_empty(Type::string()),
            Value::bool(false), // deduction from refinement
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
            unknown_result.clone(),
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
            unknown_result.clone(),
        ),
        (
            Value::null(Type::string()),
            Value::null(Type::dynamic()),
            Value::bool(true),
        ),
        (
            Value::null(Type::string()),
            Value::null(Type::string()),
            Value::bool(true),
        ),
        (
            Value::unknown(Type::string()),
            Value::unknown(Type::number()),
            unknown_result.clone(),
        ),
        (
            Value::string(""),
            Value::null(Type::dynamic()),
            Value::bool(false),
        ),
        (
            Value::string(""),
            Value::null(Type::string()),
            Value::bool(false),
        ),
        (
            Value::string(""),
            Value::unknown(Type::string()),
            unknown_result.clone(),
        ),
        (
            Value::null(Type::dynamic()),
            Value::null(Type::dynamic()),
            Value::bool(true),
        ),
        (
            Value::null(Type::string()),
            Value::unknown(Type::number()),
            unknown_result.clone(), // because second operand might eventually be null
        ),
        (
            Value::unknown(Type::string()),
            Value::null(Type::number()),
            unknown_result.clone(), // because first operand might eventually be null
        ),
        (
            Value::unknown(Type::string()),
            Value::unknown(Type::number()),
            unknown_result.clone(), // because both operands might eventually be null
        ),
        (
            Value::string("hello"),
            Value::unknown(Type::number()),
            Value::bool(false), // because no number value -- even null -- can be equal to a non-null string
        ),
        (
            Value::unknown(Type::string()),
            Value::number_int(1),
            Value::bool(false), // because no string value -- even null -- can be equal to a non-null number
        ),
        (
            Value::object([("a", Value::string("a"))]),
            // A null value is always known
            Value::object([("a", Value::null(Type::dynamic()))]),
            Value::bool(false),
        ),
        (
            Value::object([("a", Value::null(Type::dynamic()))]),
            Value::object([("a", Value::null(Type::dynamic()))]),
            Value::bool(true),
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
            Value::bool(false),
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
            Value::bool(false),
        ),
        (
            Value::object([("a", Value::null(Type::dynamic()))]),
            Value::object([("a", Value::dynamic())]),
            unknown_result.clone(),
        ),
        (
            Value::object([("a", Value::null(Type::list(Type::string())))]),
            // While the unknown val does contain dynamic types, the overall
            // container types can't conform.
            Value::object([("a", Value::unknown(Type::list(Type::list(Type::dynamic()))))]),
            Value::bool(false),
        ),
        (
            Value::object([("a", Value::null(Type::list(Type::list(Type::string()))))]),
            Value::object([("a", Value::unknown(Type::list(Type::list(Type::dynamic()))))]),
            unknown_result.clone(),
        ),
        (
            Value::null(Type::string()),
            Value::unknown(Type::string())
                .refine()
                .not_null()
                .new_value(),
            Value::bool(false),
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .not_null()
                .new_value(),
            Value::null(Type::string()),
            Value::bool(false),
        ),
        (
            Value::unknown(Type::string()).refine().null().new_value(),
            Value::null(Type::string()),
            Value::bool(true), // NOTE: The refinement should collapse to Value::null(Type::string())
        ),
        (
            Value::null(Type::string()),
            Value::unknown(Type::string()).refine().null().new_value(),
            Value::bool(true), // NOTE: The refinement should collapse to Value::null(Type::string())
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix("foo-")
                .new_value(),
            Value::string("notfoo-bar"),
            Value::bool(false), // Deduction from refinement
        ),
        (
            Value::string(""),
            Value::unknown(Type::string())
                .refine()
                .string_prefix("foo-")
                .new_value(),
            Value::bool(false), // Deduction from refinement
        ),
        (
            Value::string("").mark("a"),
            Value::unknown(Type::string())
                .mark("b")
                .refine()
                .string_prefix("foo-")
                .new_value(),
            Value::bool(false).mark("a").mark("b"), // Deduction from refinement
        ),
        (
            Value::unknown(Type::string())
                .refine()
                .string_prefix("foo-")
                .new_value(),
            Value::string("foo-bar"),
            unknown_result.clone(),
        ),
        // Marks
        (
            Value::string("a").mark(1),
            Value::string("b"),
            Value::bool(false).mark(1),
        ),
        (
            Value::string("a"),
            Value::string("b").mark(2),
            Value::bool(false).mark(2),
        ),
        (
            Value::string("a").mark(1),
            Value::string("b").mark(2),
            Value::bool(false).with_marks([ValueMarks::from_marks([1, 2])]),
        ),
        (
            Value::map([("a", Value::string("a").mark("boop"))]),
            Value::map([("a", Value::string("a").mark("blop"))]),
            Value::bool(true).with_marks([ValueMarks::from_marks(["boop", "blop"])]),
        ),
        (
            Value::object([("a", Value::string("a").mark("nested"))]).mark("toplevel a"),
            Value::null(Type::object([("a", Type::string())])).mark("toplevel b"),
            Value::bool(false).with_marks([ValueMarks::from_marks(["toplevel a", "toplevel b"])]),
        ),
        (
            Value::null(Type::object([("a", Type::string())])).mark("toplevel a"),
            Value::null(Type::object([("a", Type::string())])).mark("toplevel b"),
            Value::bool(true).with_marks([ValueMarks::from_marks(["toplevel a", "toplevel b"])]),
        ),
    ];

    for (i, (lhs, rhs, expected)) in tests.iter().enumerate() {
        let got = lhs.equals(rhs);
        assert_eq!(
            got, *expected,
            "case {i}: {lhs:?}.equals({rhs:?}) returned {got:?}, want {expected:?}"
        );
    }
}
