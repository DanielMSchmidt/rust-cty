//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/primitive_type_test.go
//!   cty/type_test.go
//!   cty/object_type_test.go
//!   cty/tuple_type_test.go
//!   cty/set_type_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{CapsuleOps, Type, Value};

// upstream: cty/primitive_type_test.go TestTypeIsPrimitiveType
#[test]
fn type_is_primitive_type() {
    let tests: Vec<(Type, bool)> = vec![
        (Type::string(), true),
        (Type::number(), true),
        (Type::bool(), true),
        (Type::dynamic(), false),
        (Type::list(Type::string()), false),
        // Make sure our primitive constants are correctly constructed
        (Value::bool(true).ty(), true),
        (Value::bool(false).ty(), true),
        (Value::zero().ty(), true),
        (Value::positive_infinity().ty(), true),
        (Value::negative_infinity().ty(), true),
    ];

    for (i, (ty, want)) in tests.iter().enumerate() {
        let got = ty.is_primitive_type();
        assert_eq!(got, *want, "case {i}: wrong result for {ty:?}");
    }
}

// upstream: cty/type_test.go TestHasDynamicTypes
#[test]
fn has_dynamic_types() {
    let tests: Vec<(Type, bool)> = vec![
        (Type::dynamic(), true),
        (Type::list(Type::dynamic()), true),
        (Type::tuple([Type::string(), Type::dynamic()]), true),
        (
            Type::object([("a", Type::string()), ("unknown", Type::dynamic())]),
            true,
        ),
        (
            Type::list(Type::object([
                ("a", Type::string()),
                ("unknown", Type::dynamic()),
            ])),
            true,
        ),
        (
            Type::tuple([Type::object([
                ("a", Type::string()),
                ("unknown", Type::dynamic()),
            ])]),
            true,
        ),
    ];

    for (i, (ty, want)) in tests.iter().enumerate() {
        let got = ty.has_dynamic_types();
        assert_eq!(got, *want, "case {i}: wrong result for {ty:?}");
    }
}

// upstream: cty/type_test.go TestWithoutOptionalAttributesDeep
#[test]
fn without_optional_attributes_deep() {
    let tests: Vec<(Type, Type)> = vec![
        (Type::dynamic(), Type::dynamic()),
        (Type::list(Type::dynamic()), Type::list(Type::dynamic())),
        (
            Type::tuple([Type::string(), Type::dynamic()]),
            Type::tuple([Type::string(), Type::dynamic()]),
        ),
        (
            Type::object([("a", Type::string()), ("unknown", Type::dynamic())]),
            Type::object([("a", Type::string()), ("unknown", Type::dynamic())]),
        ),
        (
            Type::object_with_optional_attrs(
                [("a", Type::string()), ("unknown", Type::dynamic())],
                &["a"],
            ),
            Type::object([("a", Type::string()), ("unknown", Type::dynamic())]),
        ),
        (
            Type::map(Type::object_with_optional_attrs(
                [("a", Type::string()), ("unknown", Type::dynamic())],
                &["a"],
            )),
            Type::map(Type::object([
                ("a", Type::string()),
                ("unknown", Type::dynamic()),
            ])),
        ),
        (
            Type::set(Type::object_with_optional_attrs(
                [("a", Type::string()), ("unknown", Type::dynamic())],
                &["a"],
            )),
            Type::set(Type::object([
                ("a", Type::string()),
                ("unknown", Type::dynamic()),
            ])),
        ),
        (
            Type::list(Type::object_with_optional_attrs(
                [("a", Type::string()), ("unknown", Type::dynamic())],
                &["a"],
            )),
            Type::list(Type::object([
                ("a", Type::string()),
                ("unknown", Type::dynamic()),
            ])),
        ),
        (
            Type::tuple([
                Type::object_with_optional_attrs(
                    [("a", Type::string()), ("unknown", Type::dynamic())],
                    &["a"],
                ),
                Type::object_with_optional_attrs([("b", Type::number())], &["b"]),
            ]),
            Type::tuple([
                Type::object([("a", Type::string()), ("unknown", Type::dynamic())]),
                Type::object([("b", Type::number())]),
            ]),
        ),
    ];

    for (i, (ty, expected)) in tests.iter().enumerate() {
        let got = ty.without_optional_attributes_deep();
        assert!(
            expected.equals(&got),
            "case {i}: got {got:?}, want {expected:?}"
        );
    }
}

// NOTE(port): upstream TestNilTypeEquals (cty/type_test.go) pins the behavior
// of Go's zero-value `cty.NilType`, which has no Rust analogue — absence of a
// type is `Option<Type>` here. Deliberately omitted; see docs/api-mapping.md.

// upstream: cty/type_test.go TestTypeGoString
#[test]
fn type_go_string() {
    let tests: Vec<(Type, &str)> = vec![
        (Type::dynamic(), r#"cty.DynamicPseudoType"#),
        (Type::string(), r#"cty.String"#),
        (
            Type::tuple([Type::string(), Type::bool()]),
            r#"cty.Tuple([]cty.Type{cty.String, cty.Bool})"#,
        ),
        (Type::number(), r#"cty.Number"#),
        (Type::bool(), r#"cty.Bool"#),
        (Type::list(Type::string()), r#"cty.List(cty.String)"#),
        (
            Type::list(Type::list(Type::string())),
            r#"cty.List(cty.List(cty.String))"#,
        ),
        (Type::list(Type::bool()), r#"cty.List(cty.Bool)"#),
        (Type::set(Type::string()), r#"cty.Set(cty.String)"#),
        (
            Type::set(Type::map(Type::string())),
            r#"cty.Set(cty.Map(cty.String))"#,
        ),
        (Type::set(Type::bool()), r#"cty.Set(cty.Bool)"#),
        (
            Type::tuple([Type::bool()]),
            r#"cty.Tuple([]cty.Type{cty.Bool})"#,
        ),
        (Type::map(Type::string()), r#"cty.Map(cty.String)"#),
        (
            Type::map(Type::set(Type::string())),
            r#"cty.Map(cty.Set(cty.String))"#,
        ),
        (Type::map(Type::bool()), r#"cty.Map(cty.Bool)"#),
        (
            Type::object([("foo", Type::bool())]),
            r#"cty.Object(map[string]cty.Type{"foo":cty.Bool})"#,
        ),
        (
            Type::object_with_optional_attrs(
                [("foo", Type::bool()), ("bar", Type::string())],
                &["bar"],
            ),
            r#"cty.ObjectWithOptionalAttrs(map[string]cty.Type{"bar":cty.String, "foo":cty.Bool}, []string{"bar"})"#,
        ),
    ];

    for (i, (ty, want)) in tests.iter().enumerate() {
        let got = ty.go_string();
        assert_eq!(got, *want, "case {i}: wrong go_string result");
    }
}

// Rust-syntax twin of type_go_string: the same table with the expectations
// translated into this crate's constructor syntax, pinning `Display`.
#[test]
fn type_display() {
    let tests: Vec<(Type, &str)> = vec![
        (Type::dynamic(), "Type::dynamic()"),
        (Type::string(), "Type::string()"),
        (
            Type::tuple([Type::string(), Type::bool()]),
            "Type::tuple([Type::string(), Type::bool()])",
        ),
        (Type::number(), "Type::number()"),
        (Type::bool(), "Type::bool()"),
        (Type::list(Type::string()), "Type::list(Type::string())"),
        (
            Type::list(Type::list(Type::string())),
            "Type::list(Type::list(Type::string()))",
        ),
        (Type::list(Type::bool()), "Type::list(Type::bool())"),
        (Type::set(Type::string()), "Type::set(Type::string())"),
        (
            Type::set(Type::map(Type::string())),
            "Type::set(Type::map(Type::string()))",
        ),
        (Type::set(Type::bool()), "Type::set(Type::bool())"),
        (Type::tuple([Type::bool()]), "Type::tuple([Type::bool()])"),
        (Type::map(Type::string()), "Type::map(Type::string())"),
        (
            Type::map(Type::set(Type::string())),
            "Type::map(Type::set(Type::string()))",
        ),
        (Type::map(Type::bool()), "Type::map(Type::bool())"),
        (
            Type::object([("foo", Type::bool())]),
            r#"Type::object([("foo", Type::bool())])"#,
        ),
        (
            Type::object_with_optional_attrs(
                [("foo", Type::bool()), ("bar", Type::string())],
                &["bar"],
            ),
            r#"Type::object_with_optional_attrs([("bar", Type::string()), ("foo", Type::bool())], &["bar"])"#,
        ),
    ];

    for (i, (ty, want)) in tests.iter().enumerate() {
        let got = ty.to_string();
        assert_eq!(got, *want, "case {i}: wrong Display result");
    }
}

// upstream: cty/object_type_test.go TestObjectTypeEquals
#[test]
fn object_type_equals() {
    let tests: Vec<(Type, Type, bool)> = vec![
        (
            Type::object([] as [(&str, Type); 0]),
            Type::object([] as [(&str, Type); 0]),
            true,
        ),
        (
            Type::object([("name", Type::string())]),
            Type::object([("name", Type::string())]),
            true,
        ),
        (
            // Attribute names should be normalized
            Type::object([("h\u{e9}llo", Type::string())]), // precombined é
            Type::object([("he\u{301}llo", Type::string())]), // e with combining acute accent
            true,
        ),
        (
            Type::object([("person", Type::object([("name", Type::string())]))]),
            Type::object([("person", Type::object([("name", Type::string())]))]),
            true,
        ),
        (
            Type::object([("name", Type::string())]),
            Type::object([] as [(&str, Type); 0]),
            false,
        ),
        (
            Type::object([("name", Type::string())]),
            Type::object([("name", Type::number())]),
            false,
        ),
        (
            Type::object([("name", Type::string())]),
            Type::object([("nombre", Type::string())]),
            false,
        ),
        (
            Type::object([("name", Type::string())]),
            Type::object([("name", Type::string()), ("age", Type::number())]),
            false,
        ),
        (
            Type::object([("person", Type::object([("name", Type::string())]))]),
            Type::object([(
                "person",
                Type::object([("name", Type::string()), ("age", Type::number())]),
            )]),
            false,
        ),
        (
            Type::object_with_optional_attrs([("person", Type::bool())], &["person"]),
            Type::object_with_optional_attrs([("person", Type::bool())], &["person"]),
            true,
        ),
        (
            Type::object([("person", Type::object([("name", Type::string())]))]),
            Type::object_with_optional_attrs([("person", Type::bool())], &["person"]),
            false,
        ),
        (
            Type::object_with_optional_attrs([("person", Type::bool())], &["person"]),
            Type::object([("person", Type::object([("name", Type::string())]))]),
            false,
        ),
    ];

    for (i, (lhs, rhs, expected)) in tests.iter().enumerate() {
        let got = lhs.equals(rhs);
        assert_eq!(
            got, *expected,
            "case {i}: {lhs:?}.equals({rhs:?}) returned {got}, want {expected}"
        );
    }
}

// upstream: cty/tuple_type_test.go TestTupleTypeEquals
#[test]
fn tuple_type_equals() {
    let tests: Vec<(Type, Type, bool)> = vec![
        (Type::tuple([]), Type::tuple([]), true),
        (Type::empty_tuple(), Type::tuple([]), true),
        (
            Type::tuple([Type::string()]),
            Type::tuple([Type::string()]),
            true,
        ),
        (
            Type::tuple([Type::tuple([Type::string()])]),
            Type::tuple([Type::tuple([Type::string()])]),
            true,
        ),
        (Type::tuple([Type::string()]), Type::empty_tuple(), false),
        (
            Type::tuple([Type::string()]),
            Type::tuple([Type::number()]),
            false,
        ),
        (
            Type::tuple([Type::string()]),
            Type::tuple([Type::string(), Type::number()]),
            false,
        ),
        (
            Type::tuple([Type::string()]),
            Type::tuple([Type::tuple([Type::string()])]),
            false,
        ),
    ];

    for (i, (lhs, rhs, expected)) in tests.iter().enumerate() {
        let got = lhs.equals(rhs);
        assert_eq!(
            got, *expected,
            "case {i}: {lhs:?}.equals({rhs:?}) returned {got}, want {expected}"
        );
    }
}

// upstream: cty/set_type_test.go TestSetOperations
#[test]
fn set_operations() {
    // This test is for the mechanisms that allow a calling application to
    // implement set operations using the underlying set type. This is
    // not expected to be a common case but is useful, for example, for
    // implementing the set-related functions in function/stdlib.

    let s1 = Value::set([Value::string("a"), Value::string("b"), Value::string("c")]);
    let s2 = Value::set([Value::string("c"), Value::string("d"), Value::string("e")]);

    let s1r = s1.as_value_set();
    let s2r = s2.as_value_set();
    let s3r = s1r.union(&s2r);

    let s3 = Value::set_from_value_set(&s3r);

    assert_eq!(s3.length_int(), 5, "wrong length");

    for want_str in ["a", "b", "c", "d", "e"] {
        assert_eq!(
            s3.has_element(&Value::string(want_str)),
            Value::bool(true),
            "missing element {want_str:?}"
        );
    }
}

// upstream: cty/set_type_test.go TestSetOfCapsuleType
#[test]
fn set_of_capsule_type() {
    #[derive(Debug)]
    struct CapsuleTypeForSetTests {
        name: String,
    }

    fn encapsulated_names(vals: Vec<Value>) -> Vec<String> {
        let mut ret: Vec<String> = vals
            .iter()
            .map(|v| {
                v.encapsulated_value()
                    .downcast_ref::<CapsuleTypeForSetTests>()
                    .unwrap()
                    .name
                    .clone()
            })
            .collect();
        ret.sort();
        ret
    }

    fn capsule_named(name: &str) -> CapsuleTypeForSetTests {
        CapsuleTypeForSetTests {
            name: name.to_string(),
        }
    }

    let type_with_hash = Type::capsule_with_ops::<CapsuleTypeForSetTests>(
        "with hash function",
        CapsuleOps {
            raw_equals: Some(Box::new(|a, b| {
                a.downcast_ref::<CapsuleTypeForSetTests>().unwrap().name
                    == b.downcast_ref::<CapsuleTypeForSetTests>().unwrap().name
            })),
            hash_key: Some(Box::new(|v| {
                v.downcast_ref::<CapsuleTypeForSetTests>()
                    .unwrap()
                    .name
                    .clone()
            })),
            ..Default::default()
        },
    );
    let type_without_hash = Type::capsule_with_ops::<CapsuleTypeForSetTests>(
        "without hash function",
        CapsuleOps {
            raw_equals: Some(Box::new(|a, b| {
                a.downcast_ref::<CapsuleTypeForSetTests>().unwrap().name
                    == b.downcast_ref::<CapsuleTypeForSetTests>().unwrap().name
            })),
            ..Default::default()
        },
    );
    let type_without_equals = Type::capsule::<CapsuleTypeForSetTests>("without hash function");

    // with hash: a hashing function lets the set implementation spread values
    // over multiple smaller buckets.
    {
        let v = Value::set([
            Value::capsule(type_with_hash.clone(), capsule_named("a")),
            Value::capsule(type_with_hash.clone(), capsule_named("b")),
            Value::capsule(type_with_hash.clone(), capsule_named("a")),
            Value::capsule(type_with_hash.clone(), capsule_named("c")),
        ]);
        let got = encapsulated_names(v.as_value_slice());
        let want = vec!["a", "b", "c"];
        assert_eq!(got, want, "with hash: wrong element names");
    }

    // without hash: outward behavior is identical, with everything living in
    // one big bucket internally.
    {
        let v = Value::set([
            Value::capsule(type_without_hash.clone(), capsule_named("a")),
            Value::capsule(type_without_hash.clone(), capsule_named("b")),
            Value::capsule(type_without_hash.clone(), capsule_named("a")),
            Value::capsule(type_without_hash.clone(), capsule_named("c")),
        ]);
        let got = encapsulated_names(v.as_value_slice());
        let want = vec!["a", "b", "c"];
        assert_eq!(got, want, "without hash: wrong element names");
    }

    // without equals: values compare by identity of the encapsulated
    // allocation, so equal names don't coalesce but the same allocation does.
    // NOTE(port): upstream inserts the same Go pointer `d` twice; the Rust
    // analogue of shared identity is cloning the capsule Value.
    {
        let d = Value::capsule(type_without_equals.clone(), capsule_named("d"));
        let v = Value::set([
            Value::capsule(type_without_equals.clone(), capsule_named("a")),
            Value::capsule(type_without_equals.clone(), capsule_named("b")),
            d.clone(),
            Value::capsule(type_without_equals.clone(), capsule_named("a")),
            Value::capsule(type_without_equals.clone(), capsule_named("c")),
            d.clone(),
        ]);
        let got = encapsulated_names(v.as_value_slice());
        let want = vec!["a", "a", "b", "c", "d"];
        assert_eq!(got, want, "without equals: wrong element names");
    }
}
