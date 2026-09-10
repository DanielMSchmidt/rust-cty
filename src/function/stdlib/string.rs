//! go-cty: `cty/function/stdlib/string.go`.

mod join;
mod lower;
mod reverse;
mod sort;
mod strlen;
mod substr;
mod upper;

pub use join::*;
pub use lower::*;
pub use reverse::*;
pub use sort::*;
pub use strlen::*;
pub use substr::*;
pub use upper::*;

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`split`] (go-cty: `stdlib.SplitFunc`).
pub fn split_func() -> Function {
    todo!()
}

/// The string split on a separator (go-cty: `stdlib.Split`).
pub fn split(separator: &Value, str_val: &Value) -> Result<Value, CtyError> {
    let _ = (separator, str_val);
    todo!()
}

/// The function behind [`chomp`] (go-cty: `stdlib.ChompFunc`).
pub fn chomp_func() -> Function {
    todo!()
}

/// The string with trailing newlines removed (go-cty: `stdlib.Chomp`).
pub fn chomp(str_val: &Value) -> Result<Value, CtyError> {
    let _ = str_val;
    todo!()
}

/// The function behind [`indent`] (go-cty: `stdlib.IndentFunc`).
pub fn indent_func() -> Function {
    todo!()
}

/// The string with each line after the first indented by the given number of
/// spaces (go-cty: `stdlib.Indent`).
pub fn indent(spaces: &Value, str_val: &Value) -> Result<Value, CtyError> {
    let _ = (spaces, str_val);
    todo!()
}

/// The function behind [`title`] (go-cty: `stdlib.TitleFunc`).
pub fn title_func() -> Function {
    todo!()
}

/// The string with the first letter of each word capitalized
/// (go-cty: `stdlib.Title`).
pub fn title(str_val: &Value) -> Result<Value, CtyError> {
    let _ = str_val;
    todo!()
}

/// The function behind [`trim_space`] (go-cty: `stdlib.TrimSpaceFunc`).
pub fn trim_space_func() -> Function {
    todo!()
}

/// The string with leading and trailing whitespace removed
/// (go-cty: `stdlib.TrimSpace`).
pub fn trim_space(str_val: &Value) -> Result<Value, CtyError> {
    let _ = str_val;
    todo!()
}

/// The function behind [`trim`] (go-cty: `stdlib.TrimFunc`).
pub fn trim_func() -> Function {
    todo!()
}

/// The string with leading and trailing characters from the cutset removed
/// (go-cty: `stdlib.Trim`).
pub fn trim(str_val: &Value, cutset: &Value) -> Result<Value, CtyError> {
    let _ = (str_val, cutset);
    todo!()
}

/// The function behind [`trim_prefix`] (go-cty: `stdlib.TrimPrefixFunc`).
pub fn trim_prefix_func() -> Function {
    todo!()
}

/// The string with the given prefix removed, if present
/// (go-cty: `stdlib.TrimPrefix`).
pub fn trim_prefix(str_val: &Value, prefix: &Value) -> Result<Value, CtyError> {
    let _ = (str_val, prefix);
    todo!()
}

/// The function behind [`trim_suffix`] (go-cty: `stdlib.TrimSuffixFunc`).
pub fn trim_suffix_func() -> Function {
    todo!()
}

/// The string with the given suffix removed, if present
/// (go-cty: `stdlib.TrimSuffix`).
pub fn trim_suffix(str_val: &Value, suffix: &Value) -> Result<Value, CtyError> {
    let _ = (str_val, suffix);
    todo!()
}
