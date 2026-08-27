//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/convert/compare_types_test.go
//!   cty/convert/sort_types_test.go
//!   cty/convert/mismatch_msg_test.go
//!   cty/convert/conversion_capsule_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::any::Any;
use std::cmp::Ordering;

use cty::capsule::{CapsuleConversionFromFn, CapsuleConversionToFn};
use cty::convert::{self, internals};
use cty::{CapsuleOps, Path, Type, Value};

// Ported from TestCompareTypes:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/convert/compare_types_test.go#L10
#[test]
fn compare_types() {
    let tests: Vec<(Type, Type, Ordering)> = vec![
        // Primitives
        (Type::string(), Type::string(), Ordering::Equal),
        (Type::string(), Type::number(), Ordering::Less),
        (Type::number(), Type::string(), Ordering::Greater),
        (Type::string(), Type::bool(), Ordering::Less),
        (Type::bool(), Type::string(), Ordering::Greater),
        (Type::bool(), Type::number(), Ordering::Equal),
        (Type::number(), Type::bool(), Ordering::Equal),
        // Lists
        (
            Type::list(Type::string()),
            Type::list(Type::string()),
            Ordering::Equal,
        ),
        (
            Type::list(Type::string()),
            Type::list(Type::number()),
            Ordering::Less,
        ),
        (
            Type::list(Type::number()),
            Type::list(Type::string()),
            Ordering::Greater,
        ),
        (Type::list(Type::string()), Type::string(), Ordering::Equal),
        // Sets
        (
            Type::set(Type::string()),
            Type::set(Type::string()),
            Ordering::Equal,
        ),
        (
            Type::set(Type::string()),
            Type::set(Type::number()),
            Ordering::Less,
        ),
        (
            Type::set(Type::number()),
            Type::set(Type::string()),
            Ordering::Greater,
        ),
        (Type::set(Type::string()), Type::string(), Ordering::Equal),
        // Maps
        (
            Type::map(Type::string()),
            Type::map(Type::string()),
            Ordering::Equal,
        ),
        (
            Type::map(Type::string()),
            Type::map(Type::number()),
            Ordering::Less,
        ),
        (
            Type::map(Type::number()),
            Type::map(Type::string()),
            Ordering::Greater,
        ),
        (Type::map(Type::string()), Type::string(), Ordering::Equal),
        // Objects
        (Type::empty_object(), Type::empty_object(), Ordering::Equal),
        (
            Type::empty_object(),
            Type::object([("name", Type::string())]),
            Ordering::Equal,
        ),
        (
            Type::object([("name", Type::string())]),
            Type::object([("name", Type::string())]),
            Ordering::Equal,
        ),
        (
            Type::object([("name", Type::string()), ("number", Type::number())]),
            Type::object([("name", Type::string())]),
            Ordering::Equal,
        ),
        (
            Type::object([("number", Type::number())]),
            Type::object([("name", Type::string())]),
            Ordering::Equal,
        ),
        (
            Type::object([("name", Type::string()), ("number", Type::number())]),
            Type::object([("name", Type::string()), ("number", Type::number())]),
            Ordering::Equal,
        ),
        (
            Type::object([("name", Type::string()), ("number", Type::string())]),
            Type::object([("name", Type::string()), ("number", Type::number())]),
            Ordering::Less,
        ),
        (
            Type::object([("name", Type::string()), ("number", Type::number())]),
            Type::object([("name", Type::string()), ("number", Type::string())]),
            Ordering::Greater,
        ),
        (
            // This is the tricky case where comparing types doesn't tell
            // the whole story, because there is a third type C where both
            // attributes are strings which would be a common base type
            // of these.
            Type::object([("a", Type::string()), ("b", Type::number())]),
            Type::object([("a", Type::number()), ("b", Type::string())]),
            Ordering::Equal,
        ),
        // Tuples
        (Type::empty_tuple(), Type::empty_tuple(), Ordering::Equal),
        (
            Type::empty_tuple(),
            Type::tuple([Type::string()]),
            Ordering::Equal,
        ),
        (
            Type::tuple([Type::string()]),
            Type::tuple([Type::string()]),
            Ordering::Equal,
        ),
        (
            Type::tuple([Type::string(), Type::number()]),
            Type::tuple([Type::string()]),
            Ordering::Equal,
        ),
        (
            Type::tuple([Type::string(), Type::number()]),
            Type::tuple([Type::string(), Type::number()]),
            Ordering::Equal,
        ),
        (
            Type::tuple([Type::string(), Type::string()]),
            Type::tuple([Type::string(), Type::number()]),
            Ordering::Less,
        ),
        (
            Type::tuple([Type::string(), Type::number()]),
            Type::tuple([Type::string(), Type::string()]),
            Ordering::Greater,
        ),
        (
            // This is the tricky case where comparing types doesn't tell
            // the whole story, because there is a third type C where both
            // elements are strings which would be a common base type
            // of these.
            Type::tuple([Type::string(), Type::number()]),
            Type::tuple([Type::number(), Type::string()]),
            Ordering::Equal,
        ),
        // Lists and Sets
        (
            Type::set(Type::string()),
            Type::list(Type::string()),
            Ordering::Greater,
        ),
        (
            Type::list(Type::string()),
            Type::set(Type::string()),
            Ordering::Less,
        ),
        (
            Type::list(Type::string()),
            Type::set(Type::number()),
            Ordering::Less,
        ),
        (
            Type::set(Type::number()),
            Type::list(Type::string()),
            Ordering::Greater,
        ),
        (
            Type::list(Type::number()),
            Type::set(Type::string()),
            Ordering::Less,
        ),
        (
            Type::set(Type::string()),
            Type::list(Type::number()),
            Ordering::Greater,
        ),
        // Dynamics
        (Type::dynamic(), Type::dynamic(), Ordering::Equal),
        (Type::dynamic(), Type::string(), Ordering::Greater),
        (Type::string(), Type::dynamic(), Ordering::Less),
        (Type::number(), Type::dynamic(), Ordering::Less),
        (Type::dynamic(), Type::number(), Ordering::Greater),
        (Type::bool(), Type::dynamic(), Ordering::Less),
        (Type::dynamic(), Type::bool(), Ordering::Greater),
    ];

    for (i, (a, b, want)) in tests.iter().enumerate() {
        let got = internals::compare_types(a, b);
        assert_eq!(
            got, *want,
            "case {i}: wrong result\nA: {a:?}\nB: {b:?}\ngot:  {got:?}\nwant: {want:?}"
        );
    }
}

// Ported from TestSortTypes:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/convert/sort_types_test.go#L10
#[test]
fn sort_types() {
    let tests: Vec<(Vec<Type>, Vec<Type>)> = vec![
        (vec![], vec![]),
        (
            vec![Type::string(), Type::number()],
            vec![Type::string(), Type::number()],
        ),
        (
            vec![Type::number(), Type::string()],
            vec![Type::string(), Type::number()],
        ),
        (
            vec![Type::string(), Type::bool()],
            vec![Type::string(), Type::bool()],
        ),
        (
            vec![Type::bool(), Type::string()],
            vec![Type::string(), Type::bool()],
        ),
        (
            vec![Type::bool(), Type::string(), Type::number()],
            vec![Type::string(), Type::bool(), Type::number()],
        ),
        (
            vec![Type::number(), Type::string(), Type::bool()],
            vec![Type::string(), Type::number(), Type::bool()],
        ),
        (
            vec![Type::string(), Type::string()],
            vec![Type::string(), Type::string()],
        ),
        (
            vec![Type::number(), Type::string(), Type::number()],
            vec![Type::string(), Type::number(), Type::number()],
        ),
        (
            vec![Type::string(), Type::list(Type::string())],
            vec![Type::string(), Type::list(Type::string())],
        ),
        (
            vec![Type::list(Type::string()), Type::string()],
            vec![Type::list(Type::string()), Type::string()],
        ),
        (
            // This result is somewhat arbitrary, but the important thing
            // is that it is consistent.
            vec![Type::bool(), Type::list(Type::string()), Type::string()],
            vec![Type::list(Type::string()), Type::string(), Type::bool()],
        ),
        (
            vec![Type::string(), Type::dynamic()],
            vec![Type::string(), Type::dynamic()],
        ),
        (
            vec![Type::dynamic(), Type::string()],
            vec![Type::string(), Type::dynamic()],
        ),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let idxs = internals::sort_types(input);

        assert_eq!(
            idxs.len(),
            input.len(),
            "case {i}: wrong number of indexes {}; want {}",
            idxs.len(),
            input.len()
        );

        let got: Vec<Type> = idxs.iter().map(|&idx| input[idx].clone()).collect();

        for (j, want_ty) in want.iter().enumerate() {
            assert_eq!(
                got[j], *want_ty,
                "case {i}: wrong order\ninput: {input:?}\ngot:   {got:?}\nwant:  {want:?}"
            );
        }
    }
}

// Ported from TestMismatchMessage:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/convert/mismatch_msg_test.go#L10
#[test]
fn mismatch_message() {
    let tests: Vec<(Type, Type, &str)> = vec![
        (
            Type::bool(),
            Type::number(),
            "number required, but have bool",
        ),
        (
            Type::empty_object(),
            Type::object([("foo", Type::string())]),
            r#"attribute "foo" is required"#,
        ),
        (
            Type::empty_object(),
            Type::object([("foo", Type::string()), ("bar", Type::string())]),
            r#"attributes "bar" and "foo" are required"#,
        ),
        (
            Type::empty_object(),
            Type::object([
                ("foo", Type::string()),
                ("bar", Type::string()),
                ("baz", Type::string()),
            ]),
            r#"attributes "bar", "baz", and "foo" are required"#,
        ),
        (
            Type::empty_object(),
            Type::list(Type::object([
                ("foo", Type::string()),
                ("bar", Type::string()),
                ("baz", Type::string()),
            ])),
            "list of object required",
        ),
        (
            Type::list(Type::string()),
            Type::list(Type::object([("foo", Type::string())])),
            "incorrect list element type: object required, but have string",
        ),
        (
            Type::list(Type::empty_object()),
            Type::list(Type::object([("foo", Type::string())])),
            r#"incorrect list element type: attribute "foo" is required"#,
        ),
        (
            Type::tuple([Type::empty_object()]),
            Type::list(Type::object([("foo", Type::string())])),
            r#"element 0: attribute "foo" is required"#,
        ),
        (
            Type::list(Type::empty_object()),
            Type::set(Type::object([("foo", Type::string())])),
            r#"incorrect set element type: attribute "foo" is required"#,
        ),
        (
            Type::tuple([Type::empty_object()]),
            Type::set(Type::object([("foo", Type::string())])),
            r#"element 0: attribute "foo" is required"#,
        ),
        (
            Type::map(Type::empty_object()),
            Type::map(Type::object([("foo", Type::string())])),
            r#"incorrect map element type: attribute "foo" is required"#,
        ),
        (
            Type::object([("boop", Type::empty_object())]),
            Type::map(Type::object([("foo", Type::string())])),
            r#"element "boop": attribute "foo" is required"#,
        ),
        (
            Type::tuple([Type::empty_object(), Type::empty_tuple()]),
            Type::list(Type::dynamic()),
            "all list elements must have the same type",
        ),
        (
            Type::object([
                ("foo", Type::bool()),
                ("bar", Type::string()),
                ("baz", Type::object([("boop", Type::number())])),
            ]),
            Type::object([
                ("foo", Type::bool()),
                ("bar", Type::string()),
                (
                    "baz",
                    Type::object([("boop", Type::number()), ("beep", Type::bool())]),
                ),
            ]),
            r#"attribute "baz": attribute "beep" is required"#,
        ),
    ];

    for (i, (got_type, want_type, want_msg)) in tests.iter().enumerate() {
        let got = convert::mismatch_message(got_type, want_type);
        assert_eq!(
            got, *want_msg,
            "case {i}: wrong message\ngot type:  {got_type:?}\nwant type: {want_type:?}\ngot message:  {got}\nwant message: {want_msg}"
        );
    }
}

// Ported from TestConvertCapsuleType:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/convert/conversion_capsule_test.go#L11
#[test]
fn convert_capsule_type() {
    // Upstream encapsulates a Go *string (reflect.TypeOf("")); the Rust
    // analogue encapsulates a String.
    let cap_ty = Type::capsule_with_ops::<String>(
        "test thingy",
        CapsuleOps {
            go_string: Some(Box::new(|raw_v| {
                let v = raw_v.downcast_ref::<String>().unwrap();
                format!("capTy({v:?})")
            })),
            type_go_string: Some(Box::new(|_ty| "capTy".to_string())),
            raw_equals: Some(Box::new(|a, b| {
                a.downcast_ref::<String>().unwrap() == b.downcast_ref::<String>().unwrap()
            })),
            conversion_from: Some(Box::new(|src_ty| {
                if *src_ty != Type::string() {
                    return None;
                }
                Some(Box::new(|raw_v: &dyn Any, _path: &Path| {
                    let v = raw_v.downcast_ref::<String>().unwrap();
                    Ok(Value::string(v.clone()))
                }) as CapsuleConversionFromFn)
            })),
            conversion_to: Some(Box::new(|dst_ty| {
                if *dst_ty != Type::string() {
                    return None;
                }
                Some(Box::new(|from: &Value, _path: &Path| {
                    let s = from.as_string().to_string();
                    Ok(Box::new(s) as Box<dyn Any>)
                }) as CapsuleConversionToFn)
            })),
            ..Default::default()
        },
    );

    let cap_val = {
        let cap_ty = cap_ty.clone();
        move |s: &str| Value::capsule(cap_ty.clone(), s.to_string())
    };

    // Upstream encapsulates a Go *int (reflect.TypeOf(0)); the Rust analogue
    // encapsulates an i64.
    let cap_int_ty = Type::capsule_with_ops::<i64>("int test thingy", {
        let cap_ty = cap_ty.clone();
        CapsuleOps {
            conversion_from: Some(Box::new(move |src| {
                if *src == cap_ty {
                    let cap_ty = cap_ty.clone();
                    return Some(Box::new(move |v: &dyn Any, _p: &Path| {
                        let i = v.downcast_ref::<i64>().unwrap();
                        Ok(Value::capsule(cap_ty.clone(), format!("{i}")))
                    }) as CapsuleConversionFromFn);
                }
                None
            })),
            ..Default::default()
        }
    });
    let cap_int_val = {
        let cap_int_ty = cap_int_ty.clone();
        move |i: i64| Value::capsule(cap_int_ty.clone(), i)
    };

    // (From, To, Want, WantErr)
    let tests: Vec<(Value, Type, Option<Value>, Option<&str>)> = vec![
        (
            cap_val("hello"),
            Type::string(),
            Some(Value::string("hello")),
            None,
        ),
        (
            Value::string("hello"),
            cap_ty.clone(),
            Some(cap_val("hello")),
            None,
        ),
        (
            Value::bool(true),
            cap_ty.clone(),
            None,
            Some("test thingy required, but have bool"),
        ),
        (
            cap_val("hello"),
            Type::bool(),
            None,
            Some("bool required, but have test thingy"),
        ),
        (
            Value::unknown(cap_ty.clone()),
            Type::string(),
            Some(Value::unknown(Type::string())),
            None,
        ),
        (
            Value::null(cap_ty.clone()),
            Type::string(),
            Some(Value::null(Type::string())),
            None,
        ),
        (
            Value::unknown(Type::bool()),
            cap_ty.clone(),
            None,
            Some("test thingy required, but have bool"),
        ),
        (
            Value::null(Type::bool()),
            cap_ty.clone(),
            None,
            Some("test thingy required, but have bool"),
        ),
        (
            Value::unknown(cap_ty.clone()),
            Type::bool(),
            None,
            Some("bool required, but have test thingy"),
        ),
        (
            Value::null(cap_ty.clone()),
            Type::bool(),
            None,
            Some("bool required, but have test thingy"),
        ),
        (cap_int_val(42), cap_ty.clone(), Some(cap_val("42")), None),
    ];

    for (i, (from, to, want, want_err)) in tests.iter().enumerate() {
        let result = convert::convert(from, to);

        match want_err {
            None => {
                let got = result.unwrap_or_else(|err| {
                    panic!("case {i}: wrong error\nwant: <no error>\ngot:  {err}")
                });
                let want = want.as_ref().unwrap();
                assert!(
                    want.raw_equals(&got),
                    "case {i}: wrong result\nwant: {want:?}\ngot:  {got:?}"
                );
            }
            Some(want_err) => {
                let err = match result {
                    Ok(_) => panic!("case {i}: wrong error\nwant: {want_err}\ngot:  <no error>"),
                    Err(err) => err,
                };
                assert_eq!(err.to_string(), *want_err, "case {i}: wrong error message");
            }
        }
    }
}
