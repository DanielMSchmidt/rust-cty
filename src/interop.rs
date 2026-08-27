//! Conversion between cty values and native Rust values: the analogue of
//! go-cty's `cty/gocty` package.
//!
//! Where gocty uses runtime reflection over arbitrary Go values, the Rust
//! analogue is trait-based: implementations of [`IntoCty`] / [`FromCty`] are
//! provided for primitives, `String`, `Vec<T>`, maps, `Option<T>` (the
//! analogue of Go pointers, mapping `None` to null), and fixed-size arrays.
//! Go struct reflection with `cty:"name"` tags has no direct analogue; a
//! future derive macro would fill that role, and the conformance tests note
//! the upstream cases that depend on it.

use crate::error::Error;
use crate::types::Type;
use crate::value::Value;

/// Conversion from a native Rust value to a cty value of a given type
/// (go-cty: `gocty.ToCtyValue`).
pub trait IntoCty {
    /// Converts `self` to a cty value conforming to `ty`.
    fn into_cty(self, ty: &Type) -> Result<Value, Error>;
}

/// Conversion from a cty value to a native Rust value
/// (go-cty: `gocty.FromCtyValue`).
pub trait FromCty: Sized {
    /// Converts a cty value into `Self`.
    fn from_cty(value: &Value) -> Result<Self, Error>;
}

/// A native Rust type with a preferred cty type representation
/// (go-cty: `gocty.ImpliedType`).
pub trait CtyTyped {
    /// The cty type that best represents this Rust type.
    fn implied_type() -> Result<Type, Error>;
}

/// Converts a native value to a cty value of the given type; convenience
/// free-function form of [`IntoCty`] (go-cty: `gocty.ToCtyValue`).
pub fn to_cty_value<T: IntoCty>(value: T, ty: &Type) -> Result<Value, Error> {
    value.into_cty(ty)
}

/// Converts a cty value to a native value; convenience free-function form of
/// [`FromCty`] (go-cty: `gocty.FromCtyValue`).
pub fn from_cty_value<T: FromCty>(value: &Value) -> Result<T, Error> {
    T::from_cty(value)
}

/// The cty type that best represents the Rust type `T`; convenience
/// free-function form of [`CtyTyped`] (go-cty: `gocty.ImpliedType`).
pub fn implied_type<T: CtyTyped>() -> Result<Type, Error> {
    T::implied_type()
}

macro_rules! declare_interop_stub {
    ($($t:ty),* $(,)?) => {
        $(
            impl IntoCty for $t {
                fn into_cty(self, ty: &Type) -> Result<Value, Error> {
                    let _ = ty;
                    todo!()
                }
            }

            impl FromCty for $t {
                fn from_cty(value: &Value) -> Result<Self, Error> {
                    let _ = value;
                    todo!()
                }
            }

            impl CtyTyped for $t {
                fn implied_type() -> Result<Type, Error> {
                    todo!()
                }
            }
        )*
    };
}

declare_interop_stub!(bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, String);

impl IntoCty for &str {
    fn into_cty(self, ty: &Type) -> Result<Value, Error> {
        let _ = ty;
        todo!()
    }
}

impl<T: IntoCty> IntoCty for Vec<T> {
    fn into_cty(self, ty: &Type) -> Result<Value, Error> {
        let _ = ty;
        todo!()
    }
}

impl<T: FromCty> FromCty for Vec<T> {
    fn from_cty(value: &Value) -> Result<Self, Error> {
        let _ = value;
        todo!()
    }
}

impl<T: CtyTyped> CtyTyped for Vec<T> {
    fn implied_type() -> Result<Type, Error> {
        todo!()
    }
}

impl<T: IntoCty> IntoCty for std::collections::BTreeMap<String, T> {
    fn into_cty(self, ty: &Type) -> Result<Value, Error> {
        let _ = ty;
        todo!()
    }
}

impl<T: FromCty> FromCty for std::collections::BTreeMap<String, T> {
    fn from_cty(value: &Value) -> Result<Self, Error> {
        let _ = value;
        todo!()
    }
}

impl<T: CtyTyped> CtyTyped for std::collections::BTreeMap<String, T> {
    fn implied_type() -> Result<Type, Error> {
        todo!()
    }
}

/// `Option` is the analogue of a Go pointer: `None` converts to and from null.
impl<T: IntoCty> IntoCty for Option<T> {
    fn into_cty(self, ty: &Type) -> Result<Value, Error> {
        let _ = ty;
        todo!()
    }
}

impl<T: FromCty> FromCty for Option<T> {
    fn from_cty(value: &Value) -> Result<Self, Error> {
        let _ = value;
        todo!()
    }
}

impl<T: CtyTyped> CtyTyped for Option<T> {
    fn implied_type() -> Result<Type, Error> {
        todo!()
    }
}

impl<T: IntoCty, const N: usize> IntoCty for [T; N] {
    fn into_cty(self, ty: &Type) -> Result<Value, Error> {
        let _ = ty;
        todo!()
    }
}

impl<T: FromCty, const N: usize> FromCty for [T; N] {
    fn from_cty(value: &Value) -> Result<Self, Error> {
        let _ = value;
        todo!()
    }
}

/// Passing a `Value` through is the identity conversion, as in gocty.
impl IntoCty for Value {
    fn into_cty(self, ty: &Type) -> Result<Value, Error> {
        let _ = ty;
        todo!()
    }
}

impl FromCty for Value {
    fn from_cty(value: &Value) -> Result<Self, Error> {
        let _ = value;
        todo!()
    }
}

/// The implied cty type of a `Value` target is dynamic, as in gocty's
/// reflection over `cty.Value` (go-cty: `gocty.ImpliedType`).
impl CtyTyped for Value {
    fn implied_type() -> Result<Type, Error> {
        todo!()
    }
}
