//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/capsule_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::any::Any;

use cty::{CapsuleOps, Type, Value};

// NOTE(port): upstream capsule_test.go declares the package-level fixtures
// `capsuleTestType1Native`/`capsuleTestType2Native` and the capsule types
// `capsuleTestType1`/`capsuleTestType2`, which are not used by any test in
// that file — they only serve cty/value_ops_test.go. The corresponding Rust
// fixtures live with the tests that use them (the value_ops conformance
// files), since capsule types compare by identity and each conformance test
// binary constructs its own.

// upstream: cty/capsule_test.go TestCapsuleWithOps
#[test]
fn capsule_with_ops() {
    let i = 0_i64;
    let i2 = 0_i64;
    let i3 = 1_i64;

    // t.Run("with ops", ...)
    {
        let ty = Type::capsule_with_ops::<i64>(
            "with ops",
            CapsuleOps {
                go_string: Some(Box::new(|v: &dyn Any| {
                    let i = v.downcast_ref::<i64>().unwrap();
                    format!("test.WithOpsVal({i})")
                })),
                type_go_string: Some(Box::new(|_ty| {
                    // Upstream formats the encapsulated reflect.Type, which
                    // renders as `int`; the Rust hook receives a `TypeId`,
                    // which has no name, so the rendering is spelled out.
                    "test.WithOps(int)".to_string()
                })),
                equals: Some(Box::new(|a: &dyn Any, b: &dyn Any| {
                    let a = a.downcast_ref::<i64>().unwrap();
                    let b = b.downcast_ref::<i64>().unwrap();
                    Value::bool(a == b)
                })),
                raw_equals: Some(Box::new(|a: &dyn Any, b: &dyn Any| {
                    let a = a.downcast_ref::<i64>().unwrap();
                    let b = b.downcast_ref::<i64>().unwrap();
                    a == b
                })),
                ..Default::default()
            },
        );
        let v = Value::capsule(ty.clone(), i);
        let v2 = Value::capsule(ty.clone(), i2);
        let v3 = Value::capsule(ty.clone(), i3);

        assert_eq!(
            v.go_string(),
            "test.WithOpsVal(0)",
            "with ops: wrong GoString result"
        );
        assert_eq!(
            ty.go_string(),
            "test.WithOps(int)",
            "with ops: wrong TypeGoString result"
        );
        assert_eq!(
            v.equals(&v2),
            Value::bool(true),
            "with ops: wrong Equals.Yes result"
        );
        assert_eq!(
            v.equals(&v3),
            Value::bool(false),
            "with ops: wrong Equals.No result"
        );
    }

    // t.Run("without ops", ...)
    {
        let ty = Type::capsule::<i64>("without ops");
        let v = Value::capsule(ty.clone(), i);
        let v2 = Value::capsule(ty.clone(), i2);

        // NOTE(port): upstream expects the exact string
        // `cty.CapsuleVal(cty.Capsule("without ops", reflect.TypeOf(0)), (*int)(0x%x))`
        // where %x is the runtime address of the encapsulated Go pointer. The
        // address is not a literal even upstream (it is computed from &i at
        // run time), so here we pin the deterministic prefix and the closing
        // parentheses around the address rendering.
        let got = v.go_string();
        let want_prefix = r#"cty.CapsuleVal(cty.Capsule("without ops", reflect.TypeOf(0)), (*int)(0x"#;
        assert!(
            got.starts_with(want_prefix) && got.ends_with("))"),
            "without ops: wrong GoString result: {got}"
        );
        assert_eq!(
            ty.go_string(),
            r#"cty.Capsule("without ops", reflect.TypeOf(0))"#,
            "without ops: wrong TypeGoString result"
        );
        assert_eq!(
            v.equals(&v2),
            Value::bool(false),
            "without ops: wrong Equals result"
        );
        assert!(!v.raw_equals(&v2), "without ops: wrong RawEquals result");
    }
}

// Rust-syntax twin of capsule_with_ops: the same fixtures with the string
// expectations translated into this crate's constructor syntax, pinning
// `Display`. The hooks provided are the `display`/`type_display` analogues of
// upstream's `GoString`/`TypeGoString` hooks; Go's `int` translates to `i64`.
#[test]
fn capsule_with_ops_display() {
    let i = 0_i64;

    // t.Run("with ops", ...)
    {
        let ty = Type::capsule_with_ops::<i64>(
            "with ops",
            CapsuleOps {
                display: Some(Box::new(|v: &dyn Any| {
                    let i = v.downcast_ref::<i64>().unwrap();
                    format!("test.WithOpsVal({i})")
                })),
                type_display: Some(Box::new(|_ty| "test.WithOps(i64)".to_string())),
                ..Default::default()
            },
        );
        let v = Value::capsule(ty.clone(), i);

        assert_eq!(
            v.to_string(),
            "test.WithOpsVal(0)",
            "with ops: wrong Display result"
        );
        assert_eq!(
            ty.to_string(),
            "test.WithOps(i64)",
            "with ops: wrong type Display result"
        );
    }

    // t.Run("without ops", ...)
    {
        let ty = Type::capsule::<i64>("without ops");
        let v = Value::capsule(ty.clone(), i);

        // NOTE(port): like the Go GoString expectation, the rendering of the
        // encapsulated allocation itself is runtime-dependent (upstream embeds
        // the Go pointer's address), so only the deterministic prefix and the
        // closing parenthesis are pinned.
        let got = v.to_string();
        let want_prefix = r#"Value::capsule(Type::capsule::<i64>("without ops"), "#;
        assert!(
            got.starts_with(want_prefix) && got.ends_with(')'),
            "without ops: wrong Display result: {got}"
        );
        assert_eq!(
            ty.to_string(),
            r#"Type::capsule::<i64>("without ops")"#,
            "without ops: wrong type Display result"
        );
    }
}

// upstream: cty/capsule_test.go TestCapsuleExtensionData
#[test]
fn capsule_extension_data() {
    let ty = Type::capsule_with_ops::<i64>(
        "with extension data",
        CapsuleOps {
            extension_data: Some(Box::new(|key: &str| {
                match key {
                    // Note that this is a bad example of a key, just using a
                    // plain string for easier testing. Real-world extension
                    // keys should be named types belonging to a package in
                    // the application that is defining them.
                    "hello" => Some(Box::new("world".to_string()) as Box<dyn Any>),
                    _ => None,
                }
            })),
            ..Default::default()
        },
    );

    let got = ty.capsule_extension_data("hello");
    let got = got.expect("wrong result for 'hello': got None, want Some(\"world\")");
    assert_eq!(
        got.downcast_ref::<String>(),
        Some(&"world".to_string()),
        "wrong result for 'hello'"
    );

    let got = ty.capsule_extension_data("nonexistent");
    assert!(
        got.is_none(),
        "wrong result for 'nonexistent': got Some, want None"
    );

    let ty2 = Type::capsule::<i64>("without extension data");
    let got = ty2.capsule_extension_data("hello");
    assert!(
        got.is_none(),
        "wrong result for 'hello' without extension data: got Some, want None"
    );
}
