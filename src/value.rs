//! The cty value system: [`Value`] constructors and inspection.
//!
//! Operations (arithmetic, comparison, indexing, …) live in `value_ops.rs`;
//! mark-related methods in `marks.rs`; refinement-related methods in
//! `refinement.rs`. All are inherent methods on [`Value`].

use std::any::Any;
use std::collections::BTreeMap;

use unicode_normalization::UnicodeNormalization;

use crate::error::CtyError;
use crate::set::{ValueRules, ValueSet};
use crate::types::Type;

/// A cty value: a dynamically-typed value carrying its [`Type`].
///
/// Values are immutable and cheap to clone. `PartialEq` implements go-cty's
/// `Value.RawEquals` semantics (exact structural equality including unknowns
/// and nulls), which is what tests should use for assertions; the
/// [`Value::equals`] method implements the user-facing `Equals` operation that
/// can return unknown.
#[derive(Debug, Clone)]
pub enum Value {
    /// Strings will be stored NFC-normalized
    String(String),

    /// Boolean value
    Boolean(bool),

    /// Numbers will be stored as f64
    Number(f64),

    /// List of items
    List(Vec<Value>, Type),

    /// Set of items
    Set(ValueSet, ValueRules),

    /// Tuple of items
    Tuple(Vec<Value>),

    /// Map of items
    Map(BTreeMap<String, Value>, Type),

    /// Objects are well-defined key value structures
    Object(BTreeMap<String, Value>, Type),

    /// Unknown value of a certain type
    Unknown(Type),

    /// Null value of a certain type
    Null(Type),
}

impl Value {
    // --- Primitive constructors ---

    /// A known string value (go-cty: `cty.StringVal`). The string is normalized
    /// to Unicode NFC form, as in go-cty.
    pub fn string(v: impl Into<String>) -> Value {
        Self::String(v.into().nfc().to_string())
    }

    /// A known bool value (go-cty: `cty.BoolVal`; `cty.True` / `cty.False` are
    /// `Value::bool(true)` / `Value::bool(false)`).
    pub fn bool(v: bool) -> Value {
        Self::Boolean(v)
    }

    /// A number value from an `i64` (go-cty: `cty.NumberIntVal`).
    pub fn number(v: impl Into<f64>) -> Value {
        Self::Number(v.into())
    }

    /// Parses a number value from a decimal string at full cty precision
    /// (go-cty: `cty.MustParseNumberVal`; `cty.ParseNumberVal` is
    /// `Value::try_parse_number(s)`).
    pub fn parse_number(s: &str) -> Value {
        Self::try_parse_number(s).unwrap()
    }

    /// Same as parse_number but returns a result instead of panicing
    pub fn try_parse_number(s: &str) -> Result<Value, CtyError> {
        // TODO: Deal with invalid values
        let res = s.parse()?;
        Ok(Self::Number(res))
    }

    /// The number value representing positive infinity (go-cty: `cty.PositiveInfinity`).
    pub fn positive_infinity() -> Value {
        Self::Number(f64::INFINITY)
    }

    /// The number value representing negative infinity (go-cty: `cty.NegativeInfinity`).
    pub fn negative_infinity() -> Value {
        Self::Number(f64::NEG_INFINITY)
    }

    /// The number zero (go-cty: `cty.Zero`).
    pub fn zero() -> Value {
        Self::Number(0.0)
    }

    // --- Null, unknown, dynamic ---

    /// The null value of the given type (go-cty: `cty.NullVal`).
    pub fn null(ty: Type) -> Value {
        Self::Null(ty)
    }

    /// The unknown value of the given type (go-cty: `cty.UnknownVal`).
    pub fn unknown(ty: Type) -> Value {
        Self::Unknown(ty)
    }

    /// The wholly-unknown value of the dynamic pseudo-type
    /// (go-cty: `cty.DynamicVal`).
    pub fn dynamic() -> Value {
        Self::unknown(Type::Dynamic)
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
        Self::try_list(values).unwrap()
    }

    /// Same as list but does not panic
    pub fn try_list(values: impl IntoIterator<Item = Value>) -> Result<Value, CtyError> {
        let items = values.into_iter().collect::<Vec<_>>();

        if let Some(first) = items.first() {
            let ty = first.ty();
            if let Some(other_ty) = items
                .iter()
                .find_map(|i| if !i.ty().eq(&ty) { Some(i.ty()) } else { None })
            {
                Err(CtyError::InconsistentList {
                    expected: ty,
                    found: other_ty,
                })
            } else {
                Ok(Value::List(items, ty))
            }
        } else {
            return Err(CtyError::EmptyList);
        }
    }

    /// The empty list of the given element type (go-cty: `cty.ListValEmpty`).
    pub fn list_empty(element_type: Type) -> Value {
        Self::List(vec![], Type::List(Box::new(element_type)))
    }

    /// Whether [`Value::list`] would succeed for the given elements
    /// (go-cty: `cty.CanListVal`).
    pub fn can_list(values: impl IntoIterator<Item = Value>) -> bool {
        Self::try_list(values).is_ok()
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
        match self {
            Self::String(_) => Type::String,
            Self::Number(_) => Type::Number,
            Self::Boolean(_) => Type::Boolean,

            Self::Null(ty) => ty.clone(),
            Self::Unknown(ty) => ty.clone(),
            Self::Object(_, ty) => ty.clone(),

            Self::Map(_, ty) => Type::Map(Box::new(ty.clone())),
            Self::List(_, ty) => Type::List(Box::new(ty.clone())),

            Self::Set(val, _) => Type::Set(Box::new(val.element_type())),
            Self::Tuple(val) => Type::Tuple(val.iter().map(|v| v.ty().clone()).collect()),
        }
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

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/value_init_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::{Value, ValueMarks};

    // Ported from TestSetVal:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_init_test.go#L8
    #[test]
    #[ignore = "not yet implemented"]
    fn set_val() {
        let plain = Value::set([Value::bool(true)]);
        let marked = Value::set([Value::bool(true)]).mark(1_i64);
        let deep_marked =
            Value::set([Value::bool(true).mark(2_i64), Value::bool(true).mark(3_i64)]);

        assert_ne!(
            plain, marked,
            "plain should be unequal to marked\nplain:  {plain:?}\nmarked: {marked:?}"
        );
        assert_ne!(
            marked, deep_marked,
            "marked should be unequal to deepMarked\nmarked:      {marked:?}\ndeepmarked: {deep_marked:?}"
        );
        {
            let got = marked.marks();
            let want = ValueMarks::from_marks([1_i64]);
            assert_eq!(got, want, "wrong marks for marked");
        }
        {
            let got = deep_marked.marks();
            let want = ValueMarks::from_marks([2_i64, 3_i64]);
            // Both 2 and 3 marks are preserved even though both of them are
            // marking the same value True, and thus the resulting set contains
            // only one element.
            assert_eq!(got, want, "wrong marks for deepMarked");
        }

        // NOTE(port): upstream calls the unexported `unmarkForce`, which is
        // `Unmark` with the returned marks discarded.
        {
            let (got, _) = deep_marked.unmark();
            let want = Value::set([Value::bool(true)]);
            assert_eq!(got, want, "wrong unmarked value for deepMarked");
        }
    }

    // Ported from TestSetVal_nestedStructures:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_init_test.go#L34
    #[test]
    #[ignore = "not yet implemented"]
    fn set_val_nested_structures() {
        let test_cases: Vec<(&str, Vec<Value>)> = vec![
            ("set", vec![Value::set([Value::number(5)])]),
            (
                "doubly nested set",
                vec![Value::set([Value::set([Value::number(5)])])],
            ),
            ("list", vec![Value::list([Value::number(5)])]),
            (
                "doubly nested list",
                vec![Value::list([Value::list([Value::number(5)])])],
            ),
            ("map", vec![Value::map([("key", Value::number(5))])]),
            (
                "doubly nested map",
                vec![Value::map([(
                    "key",
                    Value::map([("child", Value::string("hello world"))]),
                )])],
            ),
            ("tuple", vec![Value::tuple([Value::number(5)])]),
            (
                "doubly nested tuple",
                vec![Value::tuple([Value::tuple([Value::number(5)])])],
            ),
        ];

        for (i, (name, elems)) in test_cases.into_iter().enumerate() {
            // Each case just needs to construct without panicking.
            let result =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| Value::set(elems)));
            assert!(result.is_ok(), "case {i}-{name}: Value::set panicked");
        }
    }

    // Ported from TestCanListVal:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_init_test.go#L120
    #[test]
    #[ignore = "not yet implemented"]
    fn can_list_val() {
        let test_cases: Vec<(Vec<Value>, bool)> = vec![
            // Valid lists
            (vec![Value::string("Hello"), Value::string("World")], true),
            (vec![Value::number(13), Value::number(31)], true),
            (vec![Value::bool(true), Value::bool(false)], true),
            (
                vec![
                    Value::list([Value::string("Hello"), Value::string("World")]),
                    Value::list([
                        Value::string("beep"),
                        Value::string("boop"),
                        Value::string("bloop"),
                    ]),
                ],
                true,
            ),
            (
                vec![
                    Value::map([("a", Value::string("Hello"))]),
                    Value::map([("c", Value::string("World"))]),
                ],
                true,
            ),
            (
                vec![
                    Value::set([Value::string("Hello"), Value::string("World")]),
                    Value::set([
                        Value::string("beep"),
                        Value::string("boop"),
                        Value::string("bloop"),
                    ]),
                ],
                true,
            ),
            // invalid list elements
            (vec![Value::string("hello"), Value::number(13)], false),
            (
                vec![
                    Value::list([Value::string("Hello"), Value::string("World")]),
                    Value::map([("a", Value::string("bloop"))]),
                ],
                false,
            ),
            // List of string and List of lists
            (
                vec![
                    Value::list([Value::string("Hello"), Value::string("World")]),
                    Value::list([
                        Value::list([Value::string("a"), Value::string("b")]),
                        Value::list([Value::string("c"), Value::string("d")]),
                    ]),
                ],
                false,
            ),
            // Inconsistent map elements
            (
                vec![
                    Value::map([("a", Value::string("Hello"))]),
                    Value::map([("a", Value::bool(true))]),
                ],
                false,
            ),
        ];

        for (i, (elems, want)) in test_cases.iter().enumerate() {
            let got = Value::can_list(elems.clone());
            assert_eq!(
                got, *want,
                "case {i}: wrong result for elements {elems:?}:\ngot {got}, want {want}"
            );
        }
    }

    // Ported from TestCanSetVal:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_init_test.go#L224
    #[test]
    #[ignore = "not yet implemented"]
    fn can_set_val() {
        let test_cases: Vec<(Vec<Value>, bool)> = vec![
            // Valid set elements
            (vec![Value::string("Hello"), Value::string("World")], true),
            (
                vec![
                    Value::string("Hello").mark(1_i64),
                    Value::string("World").mark(2_i64),
                ],
                true,
            ),
            (vec![Value::number(13), Value::number(31)], true),
            (vec![Value::bool(true), Value::bool(false)], true),
            (
                vec![
                    Value::list([Value::string("Hello"), Value::string("World")]),
                    Value::list([
                        Value::string("beep"),
                        Value::string("boop"),
                        Value::string("bloop"),
                    ]),
                ],
                true,
            ),
            (
                vec![
                    Value::map([("a", Value::string("Hello"))]),
                    Value::map([("c", Value::string("World"))]),
                ],
                true,
            ),
            (
                vec![
                    Value::set([Value::string("Hello"), Value::string("World")]),
                    Value::set([
                        Value::string("beep"),
                        Value::string("boop"),
                        Value::string("bloop"),
                    ]),
                ],
                true,
            ),
            // invalid set elements
            (vec![Value::string("hello"), Value::number(13)], false),
            (
                vec![
                    Value::list([Value::string("Hello"), Value::string("World")]),
                    Value::map([("a", Value::string("bloop"))]),
                ],
                false,
            ),
            // List of string and List of lists
            (
                vec![
                    Value::list([Value::string("Hello"), Value::string("World")]),
                    Value::list([
                        Value::list([Value::string("a"), Value::string("b")]),
                        Value::list([Value::string("c"), Value::string("d")]),
                    ]),
                ],
                false,
            ),
            // Inconsistent map elements
            (
                vec![
                    Value::map([("a", Value::string("Hello"))]),
                    Value::map([("a", Value::bool(true))]),
                ],
                false,
            ),
        ];

        for (i, (elems, want)) in test_cases.iter().enumerate() {
            let got = Value::can_set(elems);
            assert_eq!(
                got, *want,
                "case {i}: wrong result for elements {elems:?}:\ngot {got}, want {want}"
            );
        }
    }

    // Ported from TestCanMapVal:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_init_test.go#L332
    #[test]
    #[ignore = "not yet implemented"]
    fn can_map_val() {
        fn entries<const N: usize>(pairs: [(&str, Value); N]) -> Vec<(String, Value)> {
            pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
        }

        let test_cases: Vec<(Vec<(String, Value)>, bool)> = vec![
            // Valid lists
            (
                entries([("a", Value::string("Hello")), ("b", Value::string("World"))]),
                true,
            ),
            (
                entries([("one", Value::number(13)), ("two", Value::number(31))]),
                true,
            ),
            (
                entries([("one", Value::bool(true)), ("two", Value::bool(false))]),
                true,
            ),
            (
                entries([
                    (
                        "lista",
                        Value::list([Value::string("Hello"), Value::string("World")]),
                    ),
                    (
                        "listb",
                        Value::list([
                            Value::string("beep"),
                            Value::string("boop"),
                            Value::string("bloop"),
                        ]),
                    ),
                ]),
                true,
            ),
            (
                entries([
                    ("map_a", Value::map([("a", Value::string("Hello"))])),
                    ("map_b", Value::map([("c", Value::string("World"))])),
                ]),
                true,
            ),
            (
                entries([
                    (
                        "set_a",
                        Value::set([Value::string("Hello"), Value::string("World")]),
                    ),
                    (
                        "set_b",
                        Value::set([
                            Value::string("beep"),
                            Value::string("boop"),
                            Value::string("bloop"),
                        ]),
                    ),
                ]),
                true,
            ),
            // invalid map elements
            (
                entries([("one", Value::string("hello")), ("two", Value::number(13))]),
                false,
            ),
            (
                entries([
                    (
                        "one",
                        Value::list([Value::string("Hello"), Value::string("World")]),
                    ),
                    ("two", Value::map([("a", Value::string("bloop"))])),
                ]),
                false,
            ),
            (
                entries([
                    (
                        "one",
                        Value::list([Value::string("Hello"), Value::string("World")]),
                    ),
                    (
                        "two",
                        Value::list([
                            Value::list([Value::string("a"), Value::string("b")]),
                            Value::list([Value::string("c"), Value::string("d")]),
                        ]),
                    ),
                ]),
                false,
            ),
            // Inconsistent map elements
            (
                entries([
                    ("one", Value::map([("a", Value::string("Hello"))])),
                    ("two", Value::map([("a", Value::bool(true))])),
                ]),
                false,
            ),
        ];

        for (i, (elems, want)) in test_cases.iter().enumerate() {
            let got = Value::can_map(elems);
            assert_eq!(
                got, *want,
                "case {i}: wrong result for elements {elems:?}:\ngot {got}, want {want}"
            );
        }
    }

    mod value_ops {
        //! Conformance tests transcribed from go-cty
        //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
        //!   cty/value_ops_test.go (TestValueGoString, TestHasWhollyKnownType, TestFloatCopy)
        //!
        //! Expected values are literals from the upstream tables; see
        //! docs/api-mapping.md for the Go→Rust API correspondence.

        use crate::{Type, Value};

        // Ported from TestValueGoString:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L3667
        //
        // Upstream TestValueGoString is a single table covering several concepts at once.
        // It is split below into one test per concept so they can be activated one at
        // a time. Rows are transcribed verbatim and keep upstream order within a test;
        // nothing is added, dropped, or rewritten.
        mod value_go_string {
            use super::*;

            /// Runs transcribed upstream rows; fixture plumbing only, every
            /// expected value is a literal from the upstream table.
            fn check(tests: &[(Value, &str)]) {
                for (i, (value, want)) in tests.iter().enumerate() {
                    let got = value.go_string();
                    assert_eq!(got, *want, "case {i}: wrong go_string result");
                }
            }

            // TestValueGoString, primitives:
            // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L3667
            #[test]
            #[ignore = "not yet implemented"]
            fn primitives() {
                let tests: Vec<(Value, &str)> = vec![
                    (Value::null(Type::string()), r#"cty.NullVal(cty.String)"#),
                    (Value::string(""), r#"cty.StringVal("")"#),
                    (Value::string("hello"), r#"cty.StringVal("hello")"#),
                    (Value::zero(), r#"cty.NumberIntVal(0)"#),
                    (Value::number(1.2), r#"cty.NumberFloatVal(1.2)"#),
                    (
                        // the "float-ness" of the input is lost because its value is a
                        // whole number
                        Value::number(1.0),
                        r#"cty.NumberIntVal(1)"#,
                    ),
                    (
                        Value::parse_number(
                            "3.14159265358979323846264338327950288419716939937510582097494459",
                        ),
                        r#"cty.MustParseNumberVal("3.14159265358979323846264338327950288419716939937510582097494459")"#,
                    ),
                    (Value::bool(true), r#"cty.True"#),
                    (Value::bool(false), r#"cty.False"#),
                    (
                        Value::list_empty(Type::string()),
                        r#"cty.ListValEmpty(cty.String)"#,
                    ),
                    (
                        Value::set_empty(Type::string()),
                        r#"cty.SetValEmpty(cty.String)"#,
                    ),
                    (Value::empty_tuple(), r#"cty.EmptyTupleVal"#),
                    (
                        Value::map_empty(Type::string()),
                        r#"cty.MapValEmpty(cty.String)"#,
                    ),
                    (Value::empty_object(), r#"cty.EmptyObjectVal"#),
                ];

                check(&tests);
            }

            // TestValueGoString, collections:
            // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L3667
            #[test]
            #[ignore = "not yet implemented"]
            fn collections() {
                let tests: Vec<(Value, &str)> = vec![
                    (
                        Value::null(Type::tuple([Type::string(), Type::bool()])),
                        r#"cty.NullVal(cty.Tuple([]cty.Type{cty.String, cty.Bool}))"#,
                    ),
                    (
                        Value::list_empty(Type::list(Type::string())),
                        r#"cty.ListValEmpty(cty.List(cty.String))"#,
                    ),
                    (
                        Value::list([Value::bool(true)]),
                        r#"cty.ListVal([]cty.Value{cty.True})"#,
                    ),
                    (
                        Value::set_empty(Type::map(Type::string())),
                        r#"cty.SetValEmpty(cty.Map(cty.String))"#,
                    ),
                    (
                        Value::set([Value::bool(true)]),
                        r#"cty.SetVal([]cty.Value{cty.True})"#,
                    ),
                    (Value::tuple([] as [Value; 0]), r#"cty.EmptyTupleVal"#),
                    (
                        Value::tuple([Value::bool(true)]),
                        r#"cty.TupleVal([]cty.Value{cty.True})"#,
                    ),
                    (
                        Value::map_empty(Type::set(Type::string())),
                        r#"cty.MapValEmpty(cty.Set(cty.String))"#,
                    ),
                    (
                        Value::map([("boop", Value::bool(true))]),
                        r#"cty.MapVal(map[string]cty.Value{"boop":cty.True})"#,
                    ),
                    (
                        Value::object([] as [(&str, Value); 0]),
                        r#"cty.EmptyObjectVal"#,
                    ),
                    (
                        Value::object([("foo", Value::bool(true))]),
                        r#"cty.ObjectVal(map[string]cty.Value{"foo":cty.True})"#,
                    ),
                ];

                check(&tests);
            }

            // TestValueGoString, unknowns:
            // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L3667
            #[test]
            #[ignore = "not yet implemented"]
            fn unknowns() {
                let tests: Vec<(Value, &str)> = vec![
                    (
                        Value::null(Type::dynamic()),
                        r#"cty.NullVal(cty.DynamicPseudoType)"#,
                    ),
                    (Value::unknown(Type::dynamic()), r#"cty.DynamicVal"#),
                    (
                        Value::unknown(Type::string()),
                        r#"cty.UnknownVal(cty.String)"#,
                    ),
                    (
                        Value::unknown(Type::tuple([Type::string(), Type::bool()])),
                        r#"cty.UnknownVal(cty.Tuple([]cty.Type{cty.String, cty.Bool}))"#,
                    ),
                    (
                        Value::unknown(Type::string())
                            .refine()
                            .not_null()
                            .new_value(),
                        r#"cty.UnknownVal(cty.String).RefineNotNull()"#,
                    ),
                    (
                        Value::unknown(Type::string())
                            .refine()
                            .not_null()
                            .string_prefix("a-")
                            .new_value(),
                        r#"cty.UnknownVal(cty.String).Refine().NotNull().StringPrefixFull("a-").NewValue()"#,
                    ),
                    (
                        // The last character of the prefix gets discarded in case the
                        // next character is a combining diacritic
                        Value::unknown(Type::string())
                            .refine()
                            .not_null()
                            .string_prefix("foo")
                            .new_value(),
                        r#"cty.UnknownVal(cty.String).Refine().NotNull().StringPrefixFull("fo").NewValue()"#,
                    ),
                    (
                        Value::unknown(Type::bool()).refine().not_null().new_value(),
                        r#"cty.UnknownVal(cty.Bool).RefineNotNull()"#,
                    ),
                    (
                        Value::unknown(Type::number())
                            .refine()
                            .number_range_inclusive(Value::zero(), Value::unknown(Type::number()))
                            .new_value(),
                        r#"cty.UnknownVal(cty.Number).Refine().NumberLowerBound(cty.NumberIntVal(0), true).NewValue()"#,
                    ),
                    (
                        Value::unknown(Type::number())
                            .refine()
                            .number_range_inclusive(Value::zero(), Value::number(1))
                            .new_value(),
                        r#"cty.UnknownVal(cty.Number).Refine().NumberLowerBound(cty.NumberIntVal(0), true).NumberUpperBound(cty.NumberIntVal(1), true).NewValue()"#,
                    ),
                ];

                check(&tests);
            }
        }

        // Rust-syntax twin of value_go_string: the same table with the expectations
        // translated into this crate's constructor syntax, pinning `Display`.
        // Display twin of TestValueGoString:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L3667
        //
        // Upstream value_display is a single table covering several concepts at once.
        // It is split below into one test per concept so they can be activated one at
        // a time. Rows are transcribed verbatim and keep upstream order within a test;
        // nothing is added, dropped, or rewritten.
        mod value_display {
            use super::*;

            /// Runs transcribed upstream rows; fixture plumbing only, every
            /// expected value is a literal from the upstream table.
            fn check(tests: &[(Value, &str)]) {
                for (i, (value, want)) in tests.iter().enumerate() {
                    let got = value.to_string();
                    assert_eq!(got, *want, "case {i}: wrong Display result");
                }
            }

            // value_display, primitives:
            // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L3667
            #[test]
            #[ignore = "not yet implemented"]
            fn primitives() {
                let tests: Vec<(Value, &str)> = vec![
                    (Value::null(Type::string()), "Value::null(Type::string())"),
                    (Value::string(""), r#"Value::string("")"#),
                    (Value::string("hello"), r#"Value::string("hello")"#),
                    (Value::zero(), "Value::number(0)"),
                    (Value::number(1.2), "Value::number(1.2)"),
                    (
                        // the "float-ness" of the input is lost because its value is a
                        // whole number
                        Value::number(1.0),
                        "Value::number(1)",
                    ),
                    (
                        Value::parse_number(
                            "3.14159265358979323846264338327950288419716939937510582097494459",
                        ),
                        r#"Value::parse_number("3.14159265358979323846264338327950288419716939937510582097494459").unwrap()"#,
                    ),
                    (Value::bool(true), "Value::bool(true)"),
                    (Value::bool(false), "Value::bool(false)"),
                    (
                        Value::list_empty(Type::string()),
                        "Value::list_empty(Type::string())",
                    ),
                    (
                        Value::set_empty(Type::string()),
                        "Value::set_empty(Type::string())",
                    ),
                    (Value::empty_tuple(), "Value::empty_tuple()"),
                    (
                        Value::map_empty(Type::string()),
                        "Value::map_empty(Type::string())",
                    ),
                    (Value::empty_object(), "Value::empty_object()"),
                ];

                check(&tests);
            }

            // value_display, collections:
            // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L3667
            #[test]
            #[ignore = "not yet implemented"]
            fn collections() {
                let tests: Vec<(Value, &str)> = vec![
                    (
                        Value::null(Type::tuple([Type::string(), Type::bool()])),
                        "Value::null(Type::tuple([Type::string(), Type::bool()]))",
                    ),
                    (
                        Value::list_empty(Type::list(Type::string())),
                        "Value::list_empty(Type::list(Type::string()))",
                    ),
                    (
                        Value::list([Value::bool(true)]),
                        "Value::list([Value::bool(true)])",
                    ),
                    (
                        Value::set_empty(Type::map(Type::string())),
                        "Value::set_empty(Type::map(Type::string()))",
                    ),
                    (
                        Value::set([Value::bool(true)]),
                        "Value::set([Value::bool(true)])",
                    ),
                    (Value::tuple([] as [Value; 0]), "Value::empty_tuple()"),
                    (
                        Value::tuple([Value::bool(true)]),
                        "Value::tuple([Value::bool(true)])",
                    ),
                    (
                        Value::map_empty(Type::set(Type::string())),
                        "Value::map_empty(Type::set(Type::string()))",
                    ),
                    (
                        Value::map([("boop", Value::bool(true))]),
                        r#"Value::map([("boop", Value::bool(true))])"#,
                    ),
                    (
                        Value::object([] as [(&str, Value); 0]),
                        "Value::empty_object()",
                    ),
                    (
                        Value::object([("foo", Value::bool(true))]),
                        r#"Value::object([("foo", Value::bool(true))])"#,
                    ),
                ];

                check(&tests);
            }

            // value_display, unknowns:
            // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L3667
            #[test]
            #[ignore = "not yet implemented"]
            fn unknowns() {
                let tests: Vec<(Value, &str)> = vec![
                    (Value::null(Type::dynamic()), "Value::null(Type::dynamic())"),
                    (Value::unknown(Type::dynamic()), "Value::dynamic()"),
                    (
                        Value::unknown(Type::string()),
                        "Value::unknown(Type::string())",
                    ),
                    (
                        Value::unknown(Type::tuple([Type::string(), Type::bool()])),
                        "Value::unknown(Type::tuple([Type::string(), Type::bool()]))",
                    ),
                    (
                        Value::unknown(Type::string())
                            .refine()
                            .not_null()
                            .new_value(),
                        "Value::unknown(Type::string()).refine_not_null()",
                    ),
                    (
                        Value::unknown(Type::string())
                            .refine()
                            .not_null()
                            .string_prefix("a-")
                            .new_value(),
                        r#"Value::unknown(Type::string()).refine().not_null().string_prefix_full("a-").new_value()"#,
                    ),
                    (
                        // The last character of the prefix gets discarded in case the
                        // next character is a combining diacritic
                        Value::unknown(Type::string())
                            .refine()
                            .not_null()
                            .string_prefix("foo")
                            .new_value(),
                        r#"Value::unknown(Type::string()).refine().not_null().string_prefix_full("fo").new_value()"#,
                    ),
                    (
                        Value::unknown(Type::bool()).refine().not_null().new_value(),
                        "Value::unknown(Type::bool()).refine_not_null()",
                    ),
                    (
                        Value::unknown(Type::number())
                            .refine()
                            .number_range_inclusive(Value::zero(), Value::unknown(Type::number()))
                            .new_value(),
                        "Value::unknown(Type::number()).refine().number_range_lower_bound(Value::number(0), true).new_value()",
                    ),
                    (
                        Value::unknown(Type::number())
                            .refine()
                            .number_range_inclusive(Value::zero(), Value::number(1))
                            .new_value(),
                        "Value::unknown(Type::number()).refine().number_range_lower_bound(Value::number(0), true).number_range_upper_bound(Value::number(1), true).new_value()",
                    ),
                ];

                check(&tests);
            }
        }

        // Ported from TestHasWhollyKnownType:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/value_ops_test.go#L3833
        #[test]
        #[ignore = "not yet implemented"]
        fn has_wholly_known_type() {
            let tests: Vec<(Value, bool)> = vec![
                (Value::dynamic(), false),
                (Value::object([("dyn", Value::dynamic())]), false),
                (Value::null(Type::object([("dyn", Type::dynamic())])), true),
                (
                    Value::tuple([Value::string("a"), Value::null(Type::dynamic())]),
                    true,
                ),
                (
                    Value::list([Value::object([("null", Value::null(Type::dynamic()))])]),
                    true,
                ),
                (
                    Value::list([Value::null(Type::object([("dyn", Type::dynamic())]))]),
                    true,
                ),
                (
                    Value::object([(
                        "tuple",
                        Value::tuple([Value::string("a"), Value::null(Type::dynamic())]),
                    )]),
                    true,
                ),
                (
                    Value::object([(
                        "tuple",
                        Value::tuple([Value::object([("dyn", Value::dynamic())])]),
                    )]),
                    false,
                ),
            ];

            for (i, (value, want)) in tests.iter().enumerate() {
                let got = value.has_wholly_known_type();
                assert_eq!(got, *want, "case {i}: wrong result for {value:?}");
            }
        }
    }
}
