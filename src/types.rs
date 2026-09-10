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
mod conformance;
