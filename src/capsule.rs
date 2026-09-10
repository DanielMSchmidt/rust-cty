//! Custom operations for capsule types (go-cty: `cty.CapsuleOps`).
//!
//! Constructed with [`CapsuleOps::default`] plus the hooks an application
//! needs, then passed to `Type::capsule_with_ops`.

use std::any::{Any, TypeId};

use crate::error::CtyError;
use crate::path::Path;
use crate::types::Type;
use crate::value::Value;

/// A conversion produced by [`CapsuleOps::conversion_from`], converting an
/// encapsulated native value to a value of some other cty type.
pub type CapsuleConversionFromFn = Box<dyn Fn(&dyn Any, &Path) -> Result<Value, CtyError>>;

/// A hook rendering an encapsulated value as a string, for `go_string`,
/// `display`, and `hash_key`.
pub type CapsuleStringFn = Box<dyn Fn(&dyn Any) -> String>;

/// A hook rendering the capsule type itself, for `type_go_string`.
pub type CapsuleTypeStringFn = Box<dyn Fn(TypeId) -> String>;

/// A hook implementing the `Equals` operation over two encapsulated values.
pub type CapsuleEqualsFn = Box<dyn Fn(&dyn Any, &dyn Any) -> Value>;

/// A hook implementing the `RawEquals` operation over two encapsulated values.
pub type CapsuleRawEqualsFn = Box<dyn Fn(&dyn Any, &dyn Any) -> bool>;

/// A hook selecting a conversion from the capsule type to a given type.
pub type CapsuleConversionFromProviderFn = Box<dyn Fn(&Type) -> Option<CapsuleConversionFromFn>>;

/// A hook selecting a conversion to the capsule type from a given type.
pub type CapsuleConversionToProviderFn = Box<dyn Fn(&Type) -> Option<CapsuleConversionToFn>>;

/// A hook serving application-defined extension data by key.
pub type CapsuleExtensionDataFn = Box<dyn Fn(&str) -> Option<Box<dyn Any>>>;

/// A conversion produced by [`CapsuleOps::conversion_to`], converting a cty
/// value of some other type into a native value to encapsulate.
pub type CapsuleConversionToFn = Box<dyn Fn(&Value, &Path) -> Result<Box<dyn Any>, CtyError>>;

/// Optional custom operations backing a capsule type
/// (go-cty: `cty.CapsuleOps`). All hooks default to `None`, selecting the
/// same fallback behaviors as go-cty (e.g. identity comparison for equality).
#[derive(Default)]
pub struct CapsuleOps {
    /// GoString implementation for values of the type
    /// (go-cty: `CapsuleOps.GoString`).
    pub go_string: Option<CapsuleStringFn>,

    /// Rust-syntax `Display` implementation for values of the type — the Rust
    /// analogue of `go_string`, used by `Value`'s `Display`.
    pub display: Option<CapsuleStringFn>,

    /// GoString implementation for the capsule type itself
    /// (go-cty: `CapsuleOps.TypeGoString`).
    pub type_go_string: Option<CapsuleTypeStringFn>,

    /// Rust-syntax `Display` implementation for the capsule type itself — the
    /// Rust analogue of `type_go_string`, used by `Type`'s `Display`.
    pub type_display: Option<CapsuleTypeStringFn>,

    /// Implementation of the `Equals` operation for known, non-null values;
    /// must return a bool value (go-cty: `CapsuleOps.Equals`).
    pub equals: Option<CapsuleEqualsFn>,

    /// Implementation of the `RawEquals` operation for known, non-null values
    /// (go-cty: `CapsuleOps.RawEquals`). When `None`, values compare by
    /// identity of the encapsulated allocation.
    pub raw_equals: Option<CapsuleRawEqualsFn>,

    /// Hashing hook used when values of this type participate in sets
    /// (go-cty: `CapsuleOps.HashKey`).
    pub hash_key: Option<CapsuleStringFn>,

    /// Provides conversions from this capsule type to another type, for the
    /// `convert` module; returns `None` when no conversion is available
    /// (go-cty: `CapsuleOps.ConversionFrom`).
    pub conversion_from: Option<CapsuleConversionFromProviderFn>,

    /// Provides conversions to this capsule type from another type, for the
    /// `convert` module; returns `None` when no conversion is available
    /// (go-cty: `CapsuleOps.ConversionTo`).
    pub conversion_to: Option<CapsuleConversionToProviderFn>,

    /// Application-defined extension point keyed by string
    /// (go-cty: `CapsuleOps.ExtensionData`, whose key in Go is `any`).
    pub extension_data: Option<CapsuleExtensionDataFn>,
}

impl std::fmt::Debug for CapsuleOps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CapsuleOps").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/capsule_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use std::any::Any;

    use crate::{CapsuleOps, Type, Value};

    // NOTE(port): upstream capsule_test.go declares the package-level fixtures
    // `capsuleTestType1Native`/`capsuleTestType2Native` and the capsule types
    // `capsuleTestType1`/`capsuleTestType2`, which are not used by any test in
    // that file — they only serve cty/value_ops_test.go. The corresponding Rust
    // fixtures live with the tests that use them (the value_ops conformance
    // files), since capsule types compare by identity and each conformance test
    // binary constructs its own.

    // Ported from TestCapsuleWithOps:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/capsule_test.go#L29
    #[test]
    #[ignore = "not yet implemented"]
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
            let want_prefix =
                r#"cty.CapsuleVal(cty.Capsule("without ops", reflect.TypeOf(0)), (*int)(0x"#;
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
    // Display twin of TestCapsuleWithOps:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/capsule_test.go#L29
    #[test]
    #[ignore = "not yet implemented"]
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

    // Ported from TestCapsuleExtensionData:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/capsule_test.go#L101
    #[test]
    #[ignore = "not yet implemented"]
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
}
