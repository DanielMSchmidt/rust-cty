//! go-cty: `cty/function/stdlib/bytes.go`.

mod bytes_len;
mod bytes_slice;

pub use bytes_len::*;
pub use bytes_slice::*;

use crate::types::Type;
use crate::value::Value;

/// The capsule type encapsulating a byte buffer (go-cty: `stdlib.Bytes`).
pub fn bytes_type() -> Type {
    todo!()
}

/// Wraps a byte buffer as a value of the [`bytes_type`] capsule type
/// (go-cty: `stdlib.BytesVal`).
pub fn bytes_val(buf: Vec<u8>) -> Value {
    let _ = buf;
    todo!()
}
