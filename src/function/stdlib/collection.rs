//! go-cty: `cty/function/stdlib/collection.go`.

mod chunklist;
mod coalesce_list;
mod contains;
mod distinct;
mod element;
mod flatten;
mod has_index;
mod index;
mod keys;
mod length;
mod lookup;
mod merge;
mod reverse_list;
mod set_product;
mod slice;
mod values;
mod zipmap;

pub use chunklist::*;
pub use coalesce_list::*;
pub use contains::*;
pub use distinct::*;
pub use element::*;
pub use flatten::*;
pub use has_index::*;
pub use index::*;
pub use keys::*;
pub use length::*;
pub use lookup::*;
pub use merge::*;
pub use reverse_list::*;
pub use set_product::*;
pub use slice::*;
pub use values::*;
pub use zipmap::*;

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`compact`] (go-cty: `stdlib.CompactFunc`).
pub fn compact_func() -> Function {
    todo!()
}

/// The list of strings with null and empty entries removed
/// (go-cty: `stdlib.Compact`).
pub fn compact(list: &Value) -> Result<Value, CtyError> {
    let _ = list;
    todo!()
}
