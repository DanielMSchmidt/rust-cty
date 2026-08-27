//! The cty type system: [`Type`] and its constructors and inspection methods.
//!
//! Mirrors go-cty's `cty.Type`. Naming follows the Go API converted to Rust
//! conventions; see `docs/api-mapping.md` for the full correspondence table.

use std::any::{Any, TypeId};
use std::collections::BTreeMap;

use crate::capsule::CapsuleOps;
use crate::error::Error;

/// A cty type: the type component of the cty dynamic type/value system.
///
/// Types are cheap to clone. `PartialEq`/`Eq` implement go-cty's `Type.Equals`
/// semantics (deep structural equality, with capsule types compared by identity).
#[derive(Debug, Clone)]
pub struct Type {
    _priv: (),
}

impl Type {
    // --- Primitive types (go-cty: cty.String, cty.Number, cty.Bool) ---

    /// The string primitive type (go-cty: `cty.String`).
    pub fn string() -> Type {
        todo!()
    }

    /// The number primitive type (go-cty: `cty.Number`). Numbers are arbitrary
    /// precision, matching go-cty's 512-bit `big.Float` behavior.
    pub fn number() -> Type {
        todo!()
    }

    /// The boolean primitive type (go-cty: `cty.Bool`).
    pub fn bool() -> Type {
        todo!()
    }

    /// The dynamic pseudo-type, used as a placeholder where any type is allowed
    /// (go-cty: `cty.DynamicPseudoType`).
    pub fn dynamic() -> Type {
        todo!()
    }

    // --- Compound type constructors ---

    /// A list type with the given element type (go-cty: `cty.List`).
    pub fn list(element_type: Type) -> Type {
        let _ = element_type;
        todo!()
    }

    /// A map type with the given element type (go-cty: `cty.Map`).
    pub fn map(element_type: Type) -> Type {
        let _ = element_type;
        todo!()
    }

    /// A set type with the given element type (go-cty: `cty.Set`).
    pub fn set(element_type: Type) -> Type {
        let _ = element_type;
        todo!()
    }

    /// An object type with the given attribute names and types (go-cty: `cty.Object`).
    ///
    /// `Type::object([] as [(&str, Type); 0])` is go-cty's `cty.EmptyObject`.
    pub fn object<N: Into<String>>(attr_types: impl IntoIterator<Item = (N, Type)>) -> Type {
        let _ = attr_types
            .into_iter()
            .map(|(n, t)| (n.into(), t))
            .collect::<Vec<_>>();
        todo!()
    }

    /// An object type where the named attributes are optional for conversion
    /// purposes (go-cty: `cty.ObjectWithOptionalAttrs`).
    pub fn object_with_optional_attrs<N: Into<String>>(
        attr_types: impl IntoIterator<Item = (N, Type)>,
        optional: &[&str],
    ) -> Type {
        let _ = (
            attr_types
                .into_iter()
                .map(|(n, t)| (n.into(), t))
                .collect::<Vec<_>>(),
            optional,
        );
        todo!()
    }

    /// The object type with no attributes (go-cty: `cty.EmptyObject`).
    pub fn empty_object() -> Type {
        todo!()
    }

    /// A tuple type with the given element types, in order (go-cty: `cty.Tuple`).
    pub fn tuple(element_types: impl IntoIterator<Item = Type>) -> Type {
        let _ = element_types.into_iter().collect::<Vec<_>>();
        todo!()
    }

    /// The tuple type with no elements (go-cty: `cty.EmptyTuple`).
    pub fn empty_tuple() -> Type {
        todo!()
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
        let _ = other;
        todo!()
    }

    /// Whether this is one of the primitive types (go-cty: `Type.IsPrimitiveType`).
    pub fn is_primitive_type(&self) -> bool {
        todo!()
    }

    /// Whether this is a list type (go-cty: `Type.IsListType`).
    pub fn is_list_type(&self) -> bool {
        todo!()
    }

    /// Whether this is a map type (go-cty: `Type.IsMapType`).
    pub fn is_map_type(&self) -> bool {
        todo!()
    }

    /// Whether this is a set type (go-cty: `Type.IsSetType`).
    pub fn is_set_type(&self) -> bool {
        todo!()
    }

    /// Whether this is a list, map, or set type (go-cty: `Type.IsCollectionType`).
    pub fn is_collection_type(&self) -> bool {
        todo!()
    }

    /// Whether this is an object type (go-cty: `Type.IsObjectType`).
    pub fn is_object_type(&self) -> bool {
        todo!()
    }

    /// Whether this is a tuple type (go-cty: `Type.IsTupleType`).
    pub fn is_tuple_type(&self) -> bool {
        todo!()
    }

    /// Whether this is the dynamic pseudo-type.
    pub fn is_dynamic_type(&self) -> bool {
        todo!()
    }

    /// Whether this type or any nested type is the dynamic pseudo-type
    /// (go-cty: `Type.HasDynamicTypes`).
    pub fn has_dynamic_types(&self) -> bool {
        todo!()
    }

    // --- Collection type inspection ---

    /// The element type of a collection type (go-cty: `Type.ElementType`).
    ///
    /// # Panics
    /// Panics if this is not a collection type.
    pub fn element_type(&self) -> Type {
        todo!()
    }

    /// The element type if this is a list type, `None` otherwise
    /// (go-cty: `Type.ListElementType`).
    pub fn list_element_type(&self) -> Option<Type> {
        todo!()
    }

    /// The element type if this is a map type, `None` otherwise
    /// (go-cty: `Type.MapElementType`).
    pub fn map_element_type(&self) -> Option<Type> {
        todo!()
    }

    /// The element type if this is a set type, `None` otherwise
    /// (go-cty: `Type.SetElementType`).
    pub fn set_element_type(&self) -> Option<Type> {
        todo!()
    }

    // --- Object type inspection ---

    /// Whether an object type has the named attribute (go-cty: `Type.HasAttribute`).
    ///
    /// # Panics
    /// Panics if this is not an object type.
    pub fn has_attribute(&self, name: &str) -> bool {
        let _ = name;
        todo!()
    }

    /// The type of the named attribute of an object type (go-cty: `Type.AttributeType`).
    ///
    /// # Panics
    /// Panics if this is not an object type or has no such attribute.
    pub fn attribute_type(&self, name: &str) -> Type {
        let _ = name;
        todo!()
    }

    /// All attribute names and types of an object type (go-cty: `Type.AttributeTypes`).
    ///
    /// # Panics
    /// Panics if this is not an object type.
    pub fn attribute_types(&self) -> BTreeMap<String, Type> {
        todo!()
    }

    /// The names of the optional attributes of an object type
    /// (go-cty: `Type.OptionalAttributes`).
    pub fn optional_attributes(&self) -> Vec<String> {
        todo!()
    }

    /// Whether the named attribute of an object type is optional
    /// (go-cty: `Type.AttributeOptional`).
    ///
    /// # Panics
    /// Panics if this is not an object type or has no such attribute.
    pub fn attribute_optional(&self, name: &str) -> bool {
        let _ = name;
        todo!()
    }

    /// A copy of this type with all optional-attribute annotations removed,
    /// recursively (go-cty: `Type.WithoutOptionalAttributesDeep`).
    pub fn without_optional_attributes_deep(&self) -> Type {
        todo!()
    }

    // --- Tuple type inspection ---

    /// The number of elements of a tuple type (go-cty: `Type.Length`).
    ///
    /// # Panics
    /// Panics if this is not a tuple type.
    pub fn length(&self) -> usize {
        todo!()
    }

    /// The type of the tuple element at the given index
    /// (go-cty: `Type.TupleElementType`).
    ///
    /// # Panics
    /// Panics if this is not a tuple type or the index is out of range.
    pub fn tuple_element_type(&self, index: usize) -> Type {
        let _ = index;
        todo!()
    }

    /// All element types of a tuple type, in order (go-cty: `Type.TupleElementTypes`).
    ///
    /// # Panics
    /// Panics if this is not a tuple type.
    pub fn tuple_element_types(&self) -> Vec<Type> {
        todo!()
    }

    // --- Conformance and naming ---

    /// Checks whether this type conforms to `other` as a type specification,
    /// returning all conformance errors found (go-cty: `Type.TestConformance`).
    pub fn test_conformance(&self, other: &Type) -> Result<(), Vec<Error>> {
        let _ = other;
        todo!()
    }

    /// A user-friendly name for this type, e.g. `"string"` or `"list of number"`
    /// (go-cty: `Type.FriendlyName`).
    pub fn friendly_name(&self) -> String {
        todo!()
    }

    /// Like [`Type::friendly_name`], but phrased for use as a type constraint,
    /// e.g. rendering the dynamic pseudo-type as `"any value"`
    /// (go-cty: `Type.FriendlyNameForConstraint`).
    pub fn friendly_name_for_constraint(&self) -> String {
        todo!()
    }

    /// The Go-syntax representation of this type, byte-for-byte identical to
    /// go-cty's `Type.GoString`, e.g. `cty.List(cty.String)`.
    pub fn go_string(&self) -> String {
        todo!()
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Eq for Type {}

/// Renders the type as the Rust expression that constructs it, e.g.
/// `Type::list(Type::string())` — the Rust analogue of [`Type::go_string`].
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        todo!()
    }
}
