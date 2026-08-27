//! Custom operations for capsule types (go-cty: `cty.CapsuleOps`).
//!
//! Constructed with [`CapsuleOps::default`] plus the hooks an application
//! needs, then passed to `Type::capsule_with_ops`.

use std::any::{Any, TypeId};

use crate::error::Error;
use crate::path::Path;
use crate::types::Type;
use crate::value::Value;

/// A conversion produced by [`CapsuleOps::conversion_from`], converting an
/// encapsulated native value to a value of some other cty type.
pub type CapsuleConversionFromFn = Box<dyn Fn(&dyn Any, &Path) -> Result<Value, Error>>;

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
pub type CapsuleConversionToFn = Box<dyn Fn(&Value, &Path) -> Result<Box<dyn Any>, Error>>;

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
