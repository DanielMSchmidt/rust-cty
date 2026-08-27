//! The cty value system: [`Value`] constructors and inspection.
//!
//! Operations (arithmetic, comparison, indexing, …) live in `value_ops.rs`;
//! mark-related methods in `marks.rs`; refinement-related methods in
//! `refinement.rs`. All are inherent methods on [`Value`].

use std::any::Any;
use std::collections::BTreeMap;

use crate::error::Error;
use crate::set::ValueSet;
use crate::types::Type;

/// A cty value: a dynamically-typed value carrying its [`Type`].
///
/// Values are immutable and cheap to clone. `PartialEq` implements go-cty's
/// `Value.RawEquals` semantics (exact structural equality including unknowns
/// and nulls), which is what tests should use for assertions; the
/// [`Value::equals`] method implements the user-facing `Equals` operation that
/// can return unknown.
#[derive(Debug, Clone)]
pub struct Value {
    _priv: (),
}

impl Value {
    // --- Primitive constructors ---

    /// A known string value (go-cty: `cty.StringVal`). The string is normalized
    /// to Unicode NFC form, as in go-cty.
    pub fn string(v: impl Into<String>) -> Value {
        let _ = v.into();
        todo!()
    }

    /// A known bool value (go-cty: `cty.BoolVal`; `cty.True` / `cty.False` are
    /// `Value::bool(true)` / `Value::bool(false)`).
    pub fn bool(v: bool) -> Value {
        let _ = v;
        todo!()
    }

    /// A number value from an `i64` (go-cty: `cty.NumberIntVal`).
    pub fn number_int(v: i64) -> Value {
        let _ = v;
        todo!()
    }

    /// A number value from a `u64` (go-cty: `cty.NumberUIntVal`).
    pub fn number_uint(v: u64) -> Value {
        let _ = v;
        todo!()
    }

    /// A number value from an `f64` (go-cty: `cty.NumberFloatVal`).
    pub fn number_float(v: f64) -> Value {
        let _ = v;
        todo!()
    }

    /// Parses a number value from a decimal string at full cty precision
    /// (go-cty: `cty.ParseNumberVal`; `cty.MustParseNumberVal` is
    /// `Value::parse_number(s).unwrap()`).
    pub fn parse_number(s: &str) -> Result<Value, Error> {
        let _ = s;
        todo!()
    }

    /// The number value representing positive infinity (go-cty: `cty.PositiveInfinity`).
    pub fn positive_infinity() -> Value {
        todo!()
    }

    /// The number value representing negative infinity (go-cty: `cty.NegativeInfinity`).
    pub fn negative_infinity() -> Value {
        todo!()
    }

    /// The number zero (go-cty: `cty.Zero`).
    pub fn zero() -> Value {
        todo!()
    }

    // --- Null, unknown, dynamic ---

    /// The null value of the given type (go-cty: `cty.NullVal`).
    pub fn null(ty: Type) -> Value {
        let _ = ty;
        todo!()
    }

    /// The unknown value of the given type (go-cty: `cty.UnknownVal`).
    pub fn unknown(ty: Type) -> Value {
        let _ = ty;
        todo!()
    }

    /// The wholly-unknown value of the dynamic pseudo-type
    /// (go-cty: `cty.DynamicVal`).
    pub fn dynamic() -> Value {
        todo!()
    }

    // --- Collection constructors ---
    //
    // As in go-cty, these panic when given inconsistent element types or
    // (for lists/maps/sets) an empty sequence; use the `*_empty` constructors
    // for empty collections.

    /// A list value from the given elements (go-cty: `cty.ListVal`).
    ///
    /// # Panics
    /// Panics if `values` is empty or the element types are inconsistent.
    pub fn list(values: impl IntoIterator<Item = Value>) -> Value {
        let _ = values.into_iter().collect::<Vec<_>>();
        todo!()
    }

    /// The empty list of the given element type (go-cty: `cty.ListValEmpty`).
    pub fn list_empty(element_type: Type) -> Value {
        let _ = element_type;
        todo!()
    }

    /// Whether [`Value::list`] would succeed for the given elements
    /// (go-cty: `cty.CanListVal`).
    pub fn can_list(values: &[Value]) -> bool {
        let _ = values;
        todo!()
    }

    /// A set value from the given elements (go-cty: `cty.SetVal`).
    ///
    /// # Panics
    /// Panics if `values` is empty, the element types are inconsistent, or an
    /// element is marked.
    pub fn set(values: impl IntoIterator<Item = Value>) -> Value {
        let _ = values.into_iter().collect::<Vec<_>>();
        todo!()
    }

    /// The empty set of the given element type (go-cty: `cty.SetValEmpty`).
    pub fn set_empty(element_type: Type) -> Value {
        let _ = element_type;
        todo!()
    }

    /// Whether [`Value::set`] would succeed for the given elements
    /// (go-cty: `cty.CanSetVal`).
    pub fn can_set(values: &[Value]) -> bool {
        let _ = values;
        todo!()
    }

    /// A set value from an already-constructed [`ValueSet`]
    /// (go-cty: `cty.SetValFromValueSet`).
    pub fn set_from_value_set(set: &ValueSet) -> Value {
        let _ = set;
        todo!()
    }

    /// A map value from the given keys and elements (go-cty: `cty.MapVal`).
    ///
    /// # Panics
    /// Panics if `values` is empty or the element types are inconsistent.
    pub fn map<K: Into<String>>(values: impl IntoIterator<Item = (K, Value)>) -> Value {
        let _ = values
            .into_iter()
            .map(|(k, v)| (k.into(), v))
            .collect::<Vec<_>>();
        todo!()
    }

    /// The empty map of the given element type (go-cty: `cty.MapValEmpty`).
    pub fn map_empty(element_type: Type) -> Value {
        let _ = element_type;
        todo!()
    }

    /// Whether [`Value::map`] would succeed for the given entries
    /// (go-cty: `cty.CanMapVal`).
    pub fn can_map(values: &[(String, Value)]) -> bool {
        let _ = values;
        todo!()
    }

    /// An object value with the given attribute names and values
    /// (go-cty: `cty.ObjectVal`; `cty.EmptyObjectVal` is `Value::empty_object()`).
    pub fn object<K: Into<String>>(attrs: impl IntoIterator<Item = (K, Value)>) -> Value {
        let _ = attrs
            .into_iter()
            .map(|(k, v)| (k.into(), v))
            .collect::<Vec<_>>();
        todo!()
    }

    /// The object value with no attributes (go-cty: `cty.EmptyObjectVal`).
    pub fn empty_object() -> Value {
        todo!()
    }

    /// A tuple value with the given elements, in order (go-cty: `cty.TupleVal`;
    /// `cty.EmptyTupleVal` is `Value::empty_tuple()`).
    pub fn tuple(values: impl IntoIterator<Item = Value>) -> Value {
        let _ = values.into_iter().collect::<Vec<_>>();
        todo!()
    }

    /// The tuple value with no elements (go-cty: `cty.EmptyTupleVal`).
    pub fn empty_tuple() -> Value {
        todo!()
    }

    // --- Capsule constructor ---

    /// A value of the given capsule type wrapping a native Rust value
    /// (go-cty: `cty.CapsuleVal`).
    ///
    /// # Panics
    /// Panics if `ty` is not a capsule type or `value`'s type does not match
    /// the capsule's encapsulated type.
    pub fn capsule(ty: Type, value: impl Any) -> Value {
        let _ = (ty, Box::new(value) as Box<dyn Any>);
        todo!()
    }

    /// A shared reference to the native value inside a capsule value
    /// (go-cty: `Value.EncapsulatedValue`). Downcast with `downcast_ref`.
    ///
    /// # Panics
    /// Panics if this is not a known, non-null capsule value.
    pub fn encapsulated_value(&self) -> &dyn Any {
        todo!()
    }

    // --- Basic inspection ---

    /// The type of this value (go-cty: `Value.Type`).
    pub fn ty(&self) -> Type {
        todo!()
    }

    /// Whether this value is known (go-cty: `Value.IsKnown`).
    pub fn is_known(&self) -> bool {
        todo!()
    }

    /// Whether this value and all nested values are known
    /// (go-cty: `Value.IsWhollyKnown`).
    pub fn is_wholly_known(&self) -> bool {
        todo!()
    }

    /// Whether this value's type contains no dynamic pseudo-types once all
    /// unknowns are accounted for (go-cty: `Value.HasWhollyKnownType`).
    pub fn has_wholly_known_type(&self) -> bool {
        todo!()
    }

    /// Whether this value is null (go-cty: `Value.IsNull`).
    pub fn is_null(&self) -> bool {
        todo!()
    }

    // --- Native extraction ---

    /// The native string inside a known string value (go-cty: `Value.AsString`).
    ///
    /// # Panics
    /// Panics if this is not a known, non-null, unmarked string.
    pub fn as_string(&self) -> &str {
        todo!()
    }

    /// The number inside a known number value, approximated as `f64`
    /// (go-cty: `Value.AsBigFloat`, lossily; cty numbers have greater
    /// precision than `f64`).
    ///
    /// # Panics
    /// Panics if this is not a known, non-null, unmarked number.
    pub fn as_f64(&self) -> f64 {
        todo!()
    }

    /// Whether a known bool value is true (go-cty: `Value.True`).
    ///
    /// # Panics
    /// Panics if this is not a known, non-null, unmarked bool.
    pub fn is_true(&self) -> bool {
        todo!()
    }

    /// Whether a known bool value is false (go-cty: `Value.False`).
    ///
    /// # Panics
    /// Panics if this is not a known, non-null, unmarked bool.
    pub fn is_false(&self) -> bool {
        todo!()
    }

    /// The elements of a known collection or tuple as a `Vec`
    /// (go-cty: `Value.AsValueSlice`).
    pub fn as_value_slice(&self) -> Vec<Value> {
        todo!()
    }

    /// The entries of a known map or object as an ordered map
    /// (go-cty: `Value.AsValueMap`).
    pub fn as_value_map(&self) -> BTreeMap<String, Value> {
        todo!()
    }

    /// The elements of a known set value as a [`ValueSet`]
    /// (go-cty: `Value.AsValueSet`).
    pub fn as_value_set(&self) -> ValueSet {
        todo!()
    }

    // --- String renderings ---

    /// The Go-syntax representation of this value, byte-for-byte identical to
    /// go-cty's `Value.GoString`, e.g. `cty.StringVal("hello")`.
    pub fn go_string(&self) -> String {
        todo!()
    }

    /// An implementation-defined hash usable for grouping values into buckets
    /// (go-cty: `Value.Hash`). Equal values (per [`Value::raw_equals`]) have
    /// equal hashes; the reverse does not hold.
    pub fn hash_code(&self) -> u64 {
        todo!()
    }
}

/// Renders the value as the Rust expression that constructs it, e.g.
/// `Value::string("hello")` — the Rust analogue of [`Value::go_string`].
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        todo!()
    }
}

/// `==` implements go-cty's `Value.RawEquals`: exact structural equality,
/// treating unknowns and nulls as equal to themselves. This is the equality
/// tests should assert with; the [`Value::equals`] method is the user-facing
/// operation that can return unknown.
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.raw_equals(other)
    }
}
