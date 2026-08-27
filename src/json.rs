//! Type-directed JSON serialization: the analogue of go-cty's `cty/json`
//! package.
//!
//! JSON documents are handled as `String`s rather than byte slices; the
//! upstream tests' expected outputs port over verbatim.

use crate::error::Error;
use crate::types::Type;
use crate::value::Value;

/// Serializes a value as JSON, guided by the given type (which may include
/// dynamic pseudo-types, encoded as type/value pairs)
/// (go-cty: `json.Marshal`).
pub fn marshal(value: &Value, ty: &Type) -> Result<String, Error> {
    let _ = (value, ty);
    todo!()
}

/// Decodes a JSON document into a value of the given type
/// (go-cty: `json.Unmarshal`).
pub fn unmarshal(json: &str, ty: &Type) -> Result<Value, Error> {
    let _ = (json, ty);
    todo!()
}

/// Serializes a type itself as JSON (go-cty: `json.MarshalType` and
/// `cty.Type`'s `MarshalJSON`).
pub fn marshal_type(ty: &Type) -> Result<String, Error> {
    let _ = ty;
    todo!()
}

/// Decodes a JSON type description back into a type
/// (go-cty: `json.UnmarshalType` and `cty.Type`'s `UnmarshalJSON`).
pub fn unmarshal_type(json: &str) -> Result<Type, Error> {
    let _ = json;
    todo!()
}

/// The most suitable cty type to represent the given JSON document, such that
/// decoding into it loses no information (go-cty: `json.ImpliedType`).
pub fn implied_type(json: &str) -> Result<Type, Error> {
    let _ = json;
    todo!()
}

/// A value wrapper that serializes with an implied type rather than an
/// explicit one, for embedding cty values in ordinary JSON structures
/// (go-cty: `json.SimpleJSONValue`).
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleValue(pub Value);

impl SimpleValue {
    /// Serializes the wrapped value as JSON using its implied representation
    /// (go-cty: `SimpleJSONValue.MarshalJSON`).
    pub fn to_json(&self) -> Result<String, Error> {
        todo!()
    }

    /// Decodes JSON into a value of the implied type
    /// (go-cty: `SimpleJSONValue.UnmarshalJSON`).
    pub fn from_json(json: &str) -> Result<SimpleValue, Error> {
        let _ = json;
        todo!()
    }
}
