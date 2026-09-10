//! Type-directed MessagePack serialization: the analogue of go-cty's
//! `cty/msgpack` package, including its unknown-value extension codes and
//! refinement encoding.

use crate::error::CtyError;
use crate::types::Type;
use crate::value::Value;

/// Serializes a value as MessagePack, guided by the given type
/// (go-cty: `msgpack.Marshal`).
pub fn marshal(value: &Value, ty: &Type) -> Result<Vec<u8>, CtyError> {
    let _ = (value, ty);
    todo!()
}

/// Decodes a MessagePack document into a value of the given type
/// (go-cty: `msgpack.Unmarshal`).
pub fn unmarshal(bytes: &[u8], ty: &Type) -> Result<Value, CtyError> {
    let _ = (bytes, ty);
    todo!()
}

/// The most suitable cty type to represent the given MessagePack document
/// (go-cty: `msgpack.ImpliedType`).
pub fn implied_type(bytes: &[u8]) -> Result<Type, CtyError> {
    let _ = bytes;
    todo!()
}
