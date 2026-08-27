//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/set_internals_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{CapsuleOps, Type, Value, ValueMarks, internals};

// upstream: cty/set_internals_test.go TestSetHashBytes
#[test]
fn set_hash_bytes() {
    #[derive(Debug)]
    struct Encapsulated {
        name: String,
    }

    let type_with_hash = Type::capsule_with_ops::<Encapsulated>(
        "with hash function",
        CapsuleOps {
            raw_equals: Some(Box::new(|a, b| {
                a.downcast_ref::<Encapsulated>().unwrap().name
                    == b.downcast_ref::<Encapsulated>().unwrap().name
            })),
            hash_key: Some(Box::new(|v| {
                v.downcast_ref::<Encapsulated>().unwrap().name.clone()
            })),
            ..Default::default()
        },
    );
    let type_without_hash = Type::capsule_with_ops::<Encapsulated>(
        "without hash function",
        CapsuleOps {
            raw_equals: Some(Box::new(|a, b| {
                a.downcast_ref::<Encapsulated>().unwrap().name
                    == b.downcast_ref::<Encapsulated>().unwrap().name
            })),
            ..Default::default()
        },
    );

    let tests: Vec<(Value, &str, ValueMarks)> = vec![
        (Value::unknown(Type::number()), "?", ValueMarks::new()),
        (Value::unknown(Type::string()), "?", ValueMarks::new()),
        (Value::null(Type::number()), "~", ValueMarks::new()),
        (Value::null(Type::string()), "~", ValueMarks::new()),
        (Value::dynamic(), "?", ValueMarks::new()),
        (Value::number_float(12.0), "12", ValueMarks::new()),
        (Value::string(""), r#""""#, ValueMarks::new()),
        (Value::string("pizza"), r#""pizza""#, ValueMarks::new()),
        (Value::bool(true), "T", ValueMarks::new()),
        (Value::bool(false), "F", ValueMarks::new()),
        (Value::list_empty(Type::bool()), "[]", ValueMarks::new()),
        (Value::list_empty(Type::dynamic()), "[]", ValueMarks::new()),
        (
            Value::list([Value::bool(true), Value::bool(false)]),
            "[T;F;]",
            ValueMarks::new(),
        ),
        (
            Value::list([Value::unknown(Type::bool())]),
            "[?;]",
            ValueMarks::new(),
        ),
        (
            Value::list([Value::list_empty(Type::bool())]),
            "[[];]",
            ValueMarks::new(),
        ),
        (Value::map_empty(Type::bool()), "{}", ValueMarks::new()),
        (
            Value::map([("true", Value::bool(true)), ("false", Value::bool(false))]),
            r#"{"false":F;"true":T;}"#,
            ValueMarks::new(),
        ),
        (
            Value::map([
                ("true", Value::bool(true)),
                ("unknown", Value::unknown(Type::bool())),
                ("dynamic", Value::dynamic()),
            ]),
            r#"{"dynamic":?;"true":T;"unknown":?;}"#,
            ValueMarks::new(),
        ),
        (Value::set_empty(Type::bool()), "[]", ValueMarks::new()),
        (
            Value::set([Value::bool(true), Value::bool(true), Value::bool(false)]),
            "[F;T;]",
            ValueMarks::new(),
        ),
        (
            Value::set([Value::unknown(Type::bool()), Value::unknown(Type::bool())]),
            "[?;?;]", // unknowns are never equal, so we can have multiple of them
            ValueMarks::new(),
        ),
        (Value::empty_object(), "<>", ValueMarks::new()),
        (
            Value::object([
                ("name", Value::string("ermintrude")),
                ("age", Value::number_float(54.0)),
            ]),
            r#"<54;"ermintrude";>"#,
            ValueMarks::new(),
        ),
        (Value::empty_tuple(), "<>", ValueMarks::new()),
        (
            Value::tuple([Value::string("ermintrude"), Value::number_float(54.0)]),
            r#"<"ermintrude";54;>"#,
            ValueMarks::new(),
        ),
        // Marked values
        (
            Value::string("pizza").mark(1i64),
            r#""pizza""#,
            ValueMarks::from_marks([1i64]),
        ),
        (
            Value::object([
                ("name", Value::string("ermintrude").mark(1i64)),
                ("age", Value::number_float(54.0).mark(2i64)),
            ]),
            r#"<54;"ermintrude";>"#,
            ValueMarks::from_marks([1i64, 2i64]),
        ),
        // Encapsulated values
        (
            Value::capsule(
                type_with_hash.clone(),
                Encapsulated {
                    name: "boop".to_string(),
                },
            ),
            r#"«"boop"»"#, // we use the guillemets to differentiate this from a cty.String hash
            ValueMarks::new(),
        ),
        (
            Value::capsule(
                type_without_hash.clone(),
                Encapsulated {
                    name: "boop".to_string(),
                },
            ),
            "«?»", // we use the guillemets to differentiate a known value without a hash func from an unknown value
            ValueMarks::new(),
        ),
    ];

    for (i, (value, want, want_marks)) in tests.iter().enumerate() {
        let (got, got_marks) = internals::set_hash_bytes(value);
        assert_eq!(
            got, *want,
            "case {i}: wrong result for {value:?}\ngot:  {got}\nwant: {want}"
        );
        assert_eq!(
            &got_marks, want_marks,
            "case {i}: wrong result marks for {value:?}\ngot:  {got_marks:?}\nwant: {want_marks:?}"
        );
    }
}

// upstream: cty/set_internals_test.go TestSetOrder
#[test]
fn set_order() {
    let tests: Vec<(Value, Value, bool)> = vec![
        // Strings sort lexicographically (this is a compatibility constraint)
        (Value::string("a"), Value::string("b"), true),
        (Value::string("b"), Value::string("a"), false),
        (Value::unknown(Type::string()), Value::string("a"), false),
        (Value::string("a"), Value::unknown(Type::string()), true),
        // Numbers sort numerically (this is a compatibility constraint)
        (Value::zero(), Value::number_int(1), true),
        (Value::number_int(1), Value::zero(), false),
        // Booleans sort false before true (this is a compatibility constraint)
        (Value::bool(false), Value::bool(true), true),
        (Value::bool(true), Value::bool(false), false),
        // Unknown and Null values push to the end of a sort (this is a
        // compatibility constraint)
        (
            Value::unknown(Type::string()),
            Value::unknown(Type::string()),
            false, // no defined ordering
        ),
        (Value::null(Type::string()), Value::string("a"), false),
        (Value::string("a"), Value::null(Type::string()), true),
        (
            Value::unknown(Type::string()),
            Value::null(Type::string()),
            true,
        ),
        (
            Value::null(Type::string()),
            Value::unknown(Type::string()),
            false,
        ),
        // All other types just use an arbitrary fallback sort. These results
        // are _not_ compatibility constraints but we are testing them here
        // to verify that the result is consistent between runs for a
        // specific version of cty.
        (
            Value::list_empty(Type::string()),
            Value::list([Value::string("boop")]),
            false,
        ),
        (
            Value::list([Value::string("boop")]),
            Value::list_empty(Type::string()),
            true,
        ),
        (
            Value::set_empty(Type::string()),
            Value::set([Value::string("boop")]),
            false,
        ),
        (
            Value::set([Value::string("boop")]),
            Value::set_empty(Type::string()),
            true,
        ),
        (
            Value::map_empty(Type::string()),
            Value::map([("blah", Value::string("boop"))]),
            false,
        ),
        (
            Value::map([("blah", Value::string("boop"))]),
            Value::map_empty(Type::string()),
            true,
        ),
    ];

    for (i, (a, b, want)) in tests.iter().enumerate() {
        let rules = internals::set_rules(a.ty()); // both values are assumed to have the same type
        let got = rules.less(a, b);
        // NOTE(port): Go's `setRules.Less` returns a bare bool because
        // `setRules` always implements `set.OrderedRules`; the Rust `Rules`
        // trait folds `OrderedRules` into an `Option`-returning `less`, so the
        // set rules must always return `Some` here.
        assert_eq!(
            got,
            Some(*want),
            "case {i}: wrong result\na: {a:?}\nb: {b:?}"
        );
    }
}

// upstream: cty/set_internals_test.go TestSetRulesSameRules
#[test]
fn set_rules_same_rules() {
    let tests: Vec<(Type, Type, bool)> = vec![
        (Type::empty_object(), Type::dynamic(), false),
        (Type::empty_object(), Type::empty_object(), true),
        (Type::string(), Type::string(), true),
        (
            Type::object([("a", Type::string())]),
            Type::object([("a", Type::string())]),
            true,
        ),
        (
            Type::object([("a", Type::string())]),
            Type::object([("a", Type::bool())]),
            false,
        ),
    ];

    for (i, (a, b, want)) in tests.iter().enumerate() {
        let a_rules = internals::set_rules(a.clone());
        let b_rules = internals::set_rules(b.clone());
        let got = a_rules.same_rules(b_rules.as_ref());
        assert_eq!(
            got, *want,
            "case {i}: wrong result\na: {a:?}\nb: {b:?}\ngot {got:?}, want {want:?}"
        );
    }
}
