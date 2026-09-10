//! The cty type system: [`Type`] and its constructors and inspection methods.
//!
//! Mirrors go-cty's `cty.Type`. Naming follows the Go API converted to Rust
//! conventions; see `docs/api-mapping.md` for the full correspondence table.

use std::any::{Any, TypeId};
use std::collections::{BTreeMap, BTreeSet};
use unicode_normalization::UnicodeNormalization;

use crate::capsule::CapsuleOps;
use crate::error::CtyError;

/// A cty type: the type component of the cty dynamic type/value system.
///
/// Types are cheap to clone. `PartialEq`/`Eq` implement go-cty's `Type.Equals`
/// semantics (deep structural equality, with capsule types compared by identity).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// String types are Strings
    String,
    /// Number types are Numbers
    Number,
    /// Boolean types are Booleans
    Boolean,

    /// Dynamic type means that the specific type is currently unknown. This is used together with unknown values when we don't know the shape the data will have
    Dynamic,

    /// List describes a list of a certain type
    List(Box<Type>),
    /// Map describes a key-value map of a certain type, the keys must be strings
    Map(Box<Type>),
    /// List describes a set of a certain type
    Set(Box<Type>),

    #[non_exhaustive]
    /// Object describes a complex object as a string-keyed value map with a set of optional attributes
    Object(BTreeMap<String, Box<Type>>, BTreeSet<String>),

    /// Tuple describes a fixed-sized tuple of arbitrary types
    Tuple(Vec<Type>),
}

impl Type {
    // --- Primitive types (go-cty: cty.String, cty.Number, cty.Bool) ---

    /// The string primitive type (go-cty: `cty.String`).
    pub fn string() -> Type {
        Self::String
    }

    /// The number primitive type (go-cty: `cty.Number`). Numbers are arbitrary
    /// precision, matching go-cty's 512-bit `big.Float` behavior.
    pub fn number() -> Type {
        Self::Number
    }

    /// The boolean primitive type (go-cty: `cty.Bool`).
    pub fn bool() -> Type {
        Self::Boolean
    }

    /// The dynamic pseudo-type, used as a placeholder where any type is allowed
    /// (go-cty: `cty.DynamicPseudoType`).
    pub fn dynamic() -> Type {
        Self::Dynamic
    }

    // --- Compound type constructors ---

    /// A list type with the given element type (go-cty: `cty.List`).
    pub fn list(element_type: Type) -> Type {
        Self::List(Box::new(element_type))
    }

    /// A map type with the given element type (go-cty: `cty.Map`).
    pub fn map(element_type: Type) -> Type {
        Self::Map(Box::new(element_type))
    }

    /// A set type with the given element type (go-cty: `cty.Set`).
    pub fn set(element_type: Type) -> Type {
        Self::Set(Box::new(element_type))
    }

    /// An object type with the given attribute names and types (go-cty: `cty.Object`).
    ///
    /// `Self::object([] as [(&str, Type); 0])` is go-cty's `cty.EmptyObject`.
    pub fn object<N: Into<String>>(attr_types: impl IntoIterator<Item = (N, Type)>) -> Type {
        Self::object_with_optional_attrs(attr_types, &[])
    }

    /// An object type where the named attributes are optional for conversion
    /// purposes (go-cty: `cty.ObjectWithOptionalAttrs`).
    pub fn object_with_optional_attrs<N: Into<String>>(
        attr_types: impl IntoIterator<Item = (N, Type)>,
        optional: &[&str],
    ) -> Type {
        let inner = attr_types
            .into_iter()
            .map(|(n, t)| (n.into().nfc().collect(), Box::new(t)))
            .collect::<BTreeMap<String, Box<Type>>>();
        let op: BTreeSet<String> = optional
            .iter()
            .map(|i| String::from(*i).nfc().collect())
            .collect();

        Self::Object(inner, op)
    }

    /// The object type with no attributes (go-cty: `cty.EmptyObject`).
    pub fn empty_object() -> Type {
        let empty: Vec<(String, Type)> = Vec::new();
        Self::object(empty)
    }

    /// A tuple type with the given element types, in order (go-cty: `cty.Tuple`).
    pub fn tuple(element_types: impl IntoIterator<Item = Type>) -> Type {
        let inner = element_types.into_iter().collect::<Vec<Type>>();
        Self::Tuple(inner)
    }

    /// The tuple type with no elements (go-cty: `cty.EmptyTuple`).
    pub fn empty_tuple() -> Type {
        Self::tuple(Vec::new())
    }

    // --- Capsule types ---

    /// A new capsule type wrapping the native Rust type `T`
    /// (go-cty: `cty.Capsule`, with `T` in place of the Go `reflect.Type`).
    ///
    /// Each call creates a distinct type: capsule types compare by identity.
    pub fn capsule<T: Any>(name: &str) -> Type {
        let _ = name;
        todo!()
    }

    /// A new capsule type with associated custom operations
    /// (go-cty: `cty.CapsuleWithOps`).
    pub fn capsule_with_ops<T: Any>(name: &str, ops: CapsuleOps) -> Type {
        let _ = (name, ops);
        todo!()
    }

    /// Whether this is a capsule type (go-cty: `Type.IsCapsuleType`).
    pub fn is_capsule_type(&self) -> bool {
        todo!()
    }

    /// The `TypeId` of the native Rust type encapsulated by this capsule type
    /// (go-cty: `Type.EncapsulatedType`, which returns a `reflect.Type`).
    ///
    /// # Panics
    /// Panics if this is not a capsule type.
    pub fn encapsulated_type_id(&self) -> TypeId {
        todo!()
    }

    /// The custom operations of this capsule type, if any (go-cty: `Type.CapsuleOps`).
    ///
    /// # Panics
    /// Panics if this is not a capsule type.
    pub fn capsule_ops(&self) -> Option<&CapsuleOps> {
        todo!()
    }

    /// Arbitrary extension data associated with this capsule type via its
    /// `CapsuleOps::extension_data` hook (go-cty: `Type.CapsuleExtensionData`).
    pub fn capsule_extension_data(&self, key: &str) -> Option<Box<dyn Any>> {
        let _ = key;
        todo!()
    }

    // --- Predicates ---

    /// Whether this type equals another (go-cty: `Type.Equals`).
    /// Also available via `==` through `PartialEq`.
    pub fn equals(&self, other: &Type) -> bool {
        self.eq(other)
    }

    /// Whether this is one of the primitive types (go-cty: `Type.IsPrimitiveType`).
    pub fn is_primitive_type(&self) -> bool {
        match self {
            Self::Number | Self::Boolean | Self::String => true,
            _ => false,
        }
    }

    /// Whether this is a list type (go-cty: `Type.IsListType`).
    pub fn is_list_type(&self) -> bool {
        match self {
            Self::List(_) => true,
            _ => false,
        }
    }

    /// Whether this is a map type (go-cty: `Type.IsMapType`).
    pub fn is_map_type(&self) -> bool {
        match self {
            Self::Map(_) => true,
            _ => false,
        }
    }

    /// Whether this is a set type (go-cty: `Type.IsSetType`).
    pub fn is_set_type(&self) -> bool {
        match self {
            Self::Set(_) => true,
            _ => false,
        }
    }

    /// Whether this is a list, map, or set type (go-cty: `Type.IsCollectionType`).
    pub fn is_collection_type(&self) -> bool {
        match self {
            Self::List(_) | Self::Map(_) | Self::Set(_) => true,
            _ => false,
        }
    }

    /// Whether this is an object type (go-cty: `Type.IsObjectType`).
    pub fn is_object_type(&self) -> bool {
        match self {
            Self::Object(_, _) => true,
            _ => false,
        }
    }

    /// Whether this is a tuple type (go-cty: `Type.IsTupleType`).
    pub fn is_tuple_type(&self) -> bool {
        match self {
            Self::Tuple(_) => true,
            _ => false,
        }
    }

    /// Whether this is the dynamic pseudo-type.
    pub fn is_dynamic_type(&self) -> bool {
        match self {
            Self::Dynamic => true,
            _ => false,
        }
    }

    /// Whether this type or any nested type is the dynamic pseudo-type
    /// (go-cty: `Type.HasDynamicTypes`).
    pub fn has_dynamic_types(&self) -> bool {
        match self {
            Self::Dynamic => true,

            Self::List(inner) => inner.has_dynamic_types(),
            Self::Set(inner) => inner.has_dynamic_types(),
            Self::Map(inner) => inner.has_dynamic_types(),

            Self::Tuple(inner) => inner.iter().any(|item| item.has_dynamic_types()),
            Self::Object(inner, _) => inner.iter().any(|(_, item)| item.has_dynamic_types()),

            _ => false,
        }
    }

    // --- Collection type inspection ---

    /// The element type of a collection type (go-cty: `Type.ElementType`).
    ///
    /// # Panics
    /// Panics if this is not a collection type.
    pub fn element_type(&self) -> &Type {
        match self {
            Self::List(inner) | Self::Map(inner) | Self::Set(inner) => inner,
            _ => panic!("element_type called on non-collection type"),
        }
    }

    /// The element type if this is a list type, `None` otherwise
    /// (go-cty: `Type.ListElementType`).
    pub fn list_element_type(&self) -> Option<&Type> {
        if let Self::List(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    /// The element type if this is a map type, `None` otherwise
    /// (go-cty: `Type.MapElementType`).
    pub fn map_element_type(&self) -> Option<&Type> {
        if let Self::Map(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    /// The element type if this is a set type, `None` otherwise
    /// (go-cty: `Type.SetElementType`).
    pub fn set_element_type(&self) -> Option<&Type> {
        if let Self::Set(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    // --- Object type inspection ---

    /// Whether an object type has the named attribute (go-cty: `Type.HasAttribute`).
    ///
    /// # Panics
    /// Panics if this is not an object type.
    pub fn has_attribute(&self, name: &str) -> bool {
        match self {
            Self::Object(inner, _) => inner.contains_key(name),
            _ => panic!("called has_attribute on a non-object type"),
        }
    }

    /// The type of the named attribute of an object type (go-cty: `Type.AttributeType`).
    ///
    /// # Panics
    /// Panics if this is not an object type or has no such attribute.
    pub fn attribute_type(&self, name: &str) -> &Type {
        match self {
            Self::Object(inner, _) => inner.get(name).expect("no attribute with given name found"),
            _ => panic!("called attribute_type on a non-object type"),
        }
    }

    /// All attribute names and types of an object type (go-cty: `Type.AttributeTypes`).
    ///
    /// # Panics
    /// Panics if this is not an object type.
    pub fn attribute_types(&self) -> &BTreeMap<String, Box<Type>> {
        match self {
            Self::Object(inner, _) => inner,
            _ => panic!("called attribute_types on a non-object type"),
        }
    }

    /// The names of the optional attributes of an object type
    /// (go-cty: `Type.OptionalAttributes`).
    pub fn optional_attributes(&self) -> &BTreeSet<String> {
        match self {
            Self::Object(_, optional) => optional,
            _ => panic!("called optional_attributes on a non-object type"),
        }
    }

    /// Whether the named attribute of an object type is optional
    /// (go-cty: `Type.AttributeOptional`).
    ///
    /// # Panics
    /// Panics if this is not an object type or has no such attribute.
    pub fn attribute_optional(&self, name: &str) -> bool {
        match self {
            Self::Object(_, optional) => optional.contains(name),
            _ => panic!("called attribute_optional on a non-object type"),
        }
    }

    /// A copy of this type with all optional-attribute annotations removed,
    /// recursively (go-cty: `Type.WithoutOptionalAttributesDeep`).
    pub fn without_optional_attributes_deep(self) -> Type {
        match self {
            Self::Object(inner, optional) => Self::Object(
                inner
                    .into_iter()
                    .map(|(k, v)| (k, Box::new(v.without_optional_attributes_deep())))
                    .collect::<BTreeMap<String, Box<Type>>>(),
                BTreeSet::new(),
            ),

            Self::Tuple(types) => Self::tuple(
                types
                    .into_iter()
                    .map(|ty| ty.without_optional_attributes_deep())
                    .collect::<Vec<Type>>(),
            ),

            // PERF: We deep copy here, even though we don't know if the object might be equal to the one being created. If we switched from box to ARC and would (recursively) check that optional attributes is empty we might be able to reuse a pointer to the type rather than constructing a new inner type
            Self::List(inner) => Self::List(Box::new(inner.without_optional_attributes_deep())),
            Self::Set(inner) => Self::Set(Box::new(inner.without_optional_attributes_deep())),
            Self::Map(inner) => Self::Map(Box::new(inner.without_optional_attributes_deep())),

            // Primitive types don't need further introspection
            _ => self,
        }
    }

    // --- Tuple type inspection ---

    /// The number of elements of a tuple type (go-cty: `Type.Length`).
    ///
    /// # Panics
    /// Panics if this is not a tuple type.
    pub fn length(&self) -> usize {
        match self {
            Self::Tuple(inner) => inner.len(),
            _ => panic!("length called on a non-tuple type"),
        }
    }

    /// The type of the tuple element at the given index
    /// (go-cty: `Type.TupleElementType`).
    ///
    /// # Panics
    /// Panics if this is not a tuple type or the index is out of range.
    pub fn tuple_element_type(&self, index: usize) -> &Type {
        match self {
            Self::Tuple(inner) => inner.get(index).expect("index out of bounds"),
            _ => panic!("tuple_element_type called on a non-tuple type"),
        }
    }

    /// All element types of a tuple type, in order (go-cty: `Type.TupleElementTypes`).
    ///
    /// # Panics
    /// Panics if this is not a tuple type.
    pub fn tuple_element_types(&self) -> &Vec<Type> {
        match self {
            Self::Tuple(inner) => inner,
            _ => panic!("tuple_element_type called on a non-tuple type"),
        }
    }

    // --- Conformance and naming ---

    /// Checks whether this type conforms to `other` as a type specification,
    /// returning all conformance errors found (go-cty: `Type.TestConformance`).
    pub fn test_conformance(&self, other: &Type) -> Result<(), Vec<CtyError>> {
        todo!()
        // match other {
        //     Self::Dynamic => Ok(()),
        //     Self::String =>
        // }
    }

    /// A user-friendly name for this type, e.g. `"string"` or `"list of number"`
    /// (go-cty: `Type.FriendlyName`).
    pub fn friendly_name(&self) -> String {
        match self {
            Self::String => String::from("string"),
            Self::Number => String::from("number"),
            Self::Boolean => String::from("bool"),
            Self::Dynamic => String::from("dynamic"),
            Self::List(inner) => format!("list of {inner}"),
            Self::Set(inner) => format!("set of {inner}"),
            Self::Map(inner) => format!("map of {inner}"),
            Self::Tuple(_) => String::from("tuple"),
            Self::Object(_, _) => String::from("object"),
        }
    }

    /// Like [`Self::friendly_name`], but phrased for use as a type constraint,
    /// e.g. rendering the dynamic pseudo-type as `"any value"`
    /// (go-cty: `Type.FriendlyNameForConstraint`).
    pub fn friendly_name_for_constraint(&self) -> String {
        match self {
            Self::String => String::from("string"),
            Self::Number => String::from("number"),
            Self::Boolean => String::from("bool"),
            Self::Dynamic => String::from("any type"),
            Self::List(inner) => format!("list of {inner}"),
            Self::Set(inner) => format!("set of {inner}"),
            Self::Map(inner) => format!("map of {inner}"),
            Self::Tuple(_) => String::from("tuple"),
            Self::Object(_, _) => String::from("object"),
        }
    }

    /// The Go-syntax representation of this type, byte-for-byte identical to
    /// go-cty's `Type.GoString`, e.g. `cty.List(cty.String)`.
    pub fn go_string(&self) -> String {
        match self {
            Self::String => String::from("cty.String"),
            Self::Number => String::from("cty.Number"),
            Self::Boolean => String::from("cty.Bool"),
            Self::Dynamic => String::from("cty.DynamicPseudoType"),
            Self::List(inner) => format!("cty.List({})", inner.go_string()),
            Self::Set(inner) => format!("cty.Set({})", inner.go_string()),
            Self::Map(inner) => format!("cty.Map({})", inner.go_string()),
            Self::Tuple(inners) => {
                if inners.is_empty() {
                    String::from("cty.EmptyTuple")
                } else {
                    format!(
                        "cty.Tuple([]cty.Type{{{}}})",
                        inners
                            .iter()
                            .map(|inner| inner.go_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
            }
            Self::Object(map, optionals) => {
                if map.is_empty() {
                    String::from("cty.EmptyObject")
                } else if optionals.is_empty() {
                    format!(
                        "cty.Object(map[string]cty.Type{{{}}})",
                        map.iter()
                            .map(|(k, v)| format!("\"{}\":{}", k, v.go_string()))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                } else {
                    format!(
                        "cty.ObjectWithOptionalAttrs(map[string]cty.Type{{{}}}, []string{{{}}})",
                        map.iter()
                            .map(|(k, v)| format!("\"{}\":{}", k, v.go_string()))
                            .collect::<Vec<String>>()
                            .join(", "),
                        optionals
                            .iter()
                            .map(|f| format!("\"{}\"", f))
                            .collect::<Vec<String>>()
                            .join(",")
                    )
                }
            }
        }
    }
}

/// Renders the type as the Rust expression that constructs it, e.g.
/// `Self::list(Self::string())` — the Rust analogue of [`Self::go_string`].
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String => write!(f, "Type::string()"),
            Self::Number => write!(f, "Type::number()"),
            Self::Boolean => write!(f, "Type::bool()"),
            Self::Dynamic => write!(f, "Type::dynamic()"),
            Self::List(inner) => write!(f, "Type::list({})", inner),
            Self::Set(inner) => write!(f, "Type::set({})", inner),
            Self::Map(inner) => write!(f, "Type::map({})", inner),
            Self::Tuple(inners) => {
                if inners.is_empty() {
                    write!(f, "Type::empty_tuple")
                } else {
                    write!(
                        f,
                        "Type::tuple([{}])",
                        inners
                            .iter()
                            .map(|ty| ty.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
            }
            Self::Object(map, optionals) => {
                if map.is_empty() {
                    write!(f, "Type::empty_object")
                } else if optionals.is_empty() {
                    write!(
                        f,
                        "Type::object([{}])",
                        map.iter()
                            .map(|(k, v)| format!("(\"{}\", {})", k, v))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                } else {
                    write!(
                        f,
                        "Type::object_with_optional_attrs([{}], &[{}])",
                        map.iter()
                            .map(|(k, v)| format!("(\"{}\", {})", k, v))
                            .collect::<Vec<String>>()
                            .join(", "),
                        optionals
                            .iter()
                            .map(|f| format!("\"{}\"", f))
                            .collect::<Vec<String>>()
                            .join(",")
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod conformance {
    //! Conformance suites transcribed from the go-cty test tables for this
    //! module, one file per upstream test file.

    mod conform {
        //! Conformance tests transcribed from go-cty
        //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
        //!   cty/type_conform_test.go
        //!
        //! Expected values are literals from the upstream tables; see
        //! docs/api-mapping.md for the Go→Rust API correspondence.

        use crate::Type;

        // Ported from TestTypeTestConformance:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_conform_test.go#L9
        //
        // Upstream TestTypeTestConformance is a single table covering several concepts at once.
        // It is split below into one test per concept so they can be activated one at
        // a time. Rows are transcribed verbatim and keep upstream order within a test;
        // nothing is added, dropped, or rewritten.
        mod type_test_conformance {
            use super::*;

            /// Runs transcribed upstream rows; fixture plumbing only, every
            /// expected value is a literal from the upstream table.
            fn check(tests: &[(Type, Type, bool)]) {
                for (i, (receiver, given, conforms)) in tests.iter().enumerate() {
                    let result = receiver.test_conformance(given);
                    if *conforms {
                        assert!(
                            result.is_ok(),
                            "case {i}: ({receiver:?}).test_conformance({given:?}): unexpected errors\n{:?}",
                            result.unwrap_err()
                        );
                    } else {
                        assert!(
                            result.is_err(),
                            "case {i}: ({receiver:?}).test_conformance({given:?}): expected errors, but got none"
                        );
                    }
                }
            }

            // TestTypeTestConformance, primitives:
            // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_conform_test.go#L9
            #[test]
            #[ignore = "not yet implemented"]
            fn primitives() {
                let tests: Vec<(Type, Type, bool)> = vec![
                    (Type::number(), Type::number(), true),
                    (Type::number(), Type::string(), false),
                    (Type::empty_object(), Type::empty_object(), true),
                    (Type::empty_tuple(), Type::empty_tuple(), true),
                ];

                check(&tests);
            }

            // TestTypeTestConformance, collections:
            // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_conform_test.go#L9
            #[test]
            #[ignore = "not yet implemented"]
            fn collections() {
                let tests: Vec<(Type, Type, bool)> = vec![
                    (Type::list(Type::number()), Type::list(Type::number()), true),
                    (Type::list(Type::number()), Type::map(Type::number()), false),
                    (
                        Type::list(Type::number()),
                        Type::list(Type::string()),
                        false,
                    ),
                    (Type::map(Type::number()), Type::map(Type::number()), true),
                    (Type::map(Type::number()), Type::set(Type::number()), false),
                    (Type::map(Type::number()), Type::map(Type::string()), false),
                    (Type::set(Type::number()), Type::set(Type::number()), true),
                    (Type::set(Type::number()), Type::list(Type::number()), false),
                    (Type::set(Type::number()), Type::set(Type::string()), false),
                    (
                        Type::empty_object(),
                        Type::object([("name", Type::string())]),
                        false,
                    ),
                    (
                        Type::object([("name", Type::string())]),
                        Type::empty_object(),
                        false,
                    ),
                    (
                        Type::object([("name", Type::string())]),
                        Type::object([("name", Type::string())]),
                        true,
                    ),
                    (
                        Type::object([("name", Type::string())]),
                        Type::object([("gnome", Type::string())]),
                        false,
                    ),
                    (
                        Type::object([("name", Type::number())]),
                        Type::object([("name", Type::string())]),
                        false,
                    ),
                    (
                        Type::object([("name", Type::number())]),
                        Type::object([("name", Type::string()), ("number", Type::number())]),
                        false,
                    ),
                    (Type::empty_tuple(), Type::tuple([Type::string()]), false),
                    (Type::empty_tuple(), Type::tuple([Type::string()]), false),
                    (
                        Type::tuple([Type::string()]),
                        Type::tuple([Type::string()]),
                        true,
                    ),
                    (
                        Type::tuple([Type::string()]),
                        Type::tuple([Type::number()]),
                        false,
                    ),
                    (
                        Type::tuple([Type::string(), Type::number()]),
                        Type::tuple([Type::string(), Type::number()]),
                        true,
                    ),
                    (
                        Type::tuple([Type::string()]),
                        Type::tuple([Type::string(), Type::number()]),
                        false,
                    ),
                    (
                        Type::tuple([Type::string(), Type::number()]),
                        Type::tuple([Type::string()]),
                        false,
                    ),
                ];

                check(&tests);
            }

            // TestTypeTestConformance, unknowns:
            // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_conform_test.go#L9
            #[test]
            #[ignore = "not yet implemented"]
            fn unknowns() {
                let tests: Vec<(Type, Type, bool)> = vec![
                    (Type::number(), Type::dynamic(), true),
                    (Type::dynamic(), Type::dynamic(), true),
                    (Type::dynamic(), Type::number(), false),
                    (
                        Type::list(Type::number()),
                        Type::list(Type::dynamic()),
                        true,
                    ),
                    (
                        Type::list(Type::number()),
                        Type::map(Type::dynamic()),
                        false,
                    ),
                    (Type::map(Type::number()), Type::map(Type::dynamic()), true),
                    (
                        Type::set(Type::number()),
                        Type::list(Type::dynamic()),
                        false,
                    ),
                    (Type::set(Type::number()), Type::set(Type::dynamic()), true),
                ];

                check(&tests);
            }

            // TestTypeTestConformance, optional attrs:
            // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_conform_test.go#L9
            #[test]
            #[ignore = "not yet implemented"]
            fn optional_attrs() {
                let tests: Vec<(Type, Type, bool)> = vec![
                    (
                        Type::object_with_optional_attrs([("name", Type::number())], &["name"]),
                        Type::object([("name", Type::number())]),
                        true,
                    ),
                    (
                        Type::object_with_optional_attrs([("name", Type::number())], &["name"]),
                        Type::empty_object(),
                        false, // "optionalness" of attributes is only considered under conversion, not for conformance
                    ),
                ];

                check(&tests);
            }
        }
    }

    mod types {
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

        use crate::{CapsuleOps, Type, Value};

        // Ported from TestTypeIsPrimitiveType:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/primitive_type_test.go#L8
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

        // Ported from TestHasDynamicTypes:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_test.go#L8
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

        // Ported from TestWithoutOptionalAttributesDeep:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_test.go#L58
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

            for (i, (ty, expected)) in tests.into_iter().enumerate() {
                let got = ty.without_optional_attributes_deep();
                assert!(
                    expected.equals(&got),
                    "case {i}: got {got:?}, want {expected:?}"
                );
            }
        }

        // NOTE(port): upstream TestNilTypeEquals
        // (https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_test.go#L157)
        // pins the behavior
        // of Go's zero-value `cty.NilType`, which has no Rust analogue — absence of a
        // type is `Option<Type>` here. Deliberately omitted; see docs/api-mapping.md.

        // Ported from TestTypeGoString:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_test.go#L164
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
        // Display twin of TestTypeGoString:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/type_test.go#L164
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

        // Ported from TestObjectTypeEquals:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/object_type_test.go#L8
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

        // Ported from TestTupleTypeEquals:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/tuple_type_test.go#L8
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

        // Ported from TestSetOperations:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/set_type_test.go#L11
        #[test]
        #[ignore = "not yet implemented"]
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

        // Ported from TestSetOfCapsuleType:
        // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/set_type_test.go#L45
        #[test]
        #[ignore = "capsule types deferred"]
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
            let type_without_equals =
                Type::capsule::<CapsuleTypeForSetTests>("without hash function");

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
    }
}
