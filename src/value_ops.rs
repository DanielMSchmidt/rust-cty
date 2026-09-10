//! Operations on [`Value`]: equality, arithmetic, comparison, logic, indexing,
//! and iteration. Mirrors go-cty's `value_ops.go`.
//!
//! As in go-cty, these operations follow "unknown-tolerant" semantics: an
//! operation on an unknown operand generally yields an unknown result rather
//! than an error, and operations panic (rather than return `Err`) when used
//! with operands of the wrong type — that is a programming error, not a
//! dynamic condition.

mod absolute;
mod add;
mod and;
mod divide;
mod element_iterator;
mod equals;
mod for_each_element;
mod get_attr;
mod greater_than;
mod greater_than_or_equal_to;
mod has_element;
mod has_index;
mod index;
mod less_than;
mod less_than_or_equal_to;
mod modulo;
mod multiply;
mod negate;
mod not;
mod or;
mod raw_equals;
mod subtract;

use crate::value::Value;

/// An iterator over the `(key, value)` element pairs of a collection value,
/// as produced by [`Value::element_iterator`] (go-cty: `ElementIterator`).
///
/// For lists and tuples the key is the number index; for maps and objects it
/// is the string key or attribute name; for sets the key is the element itself.
#[derive(Debug)]
pub struct ElementIterator {
    _priv: (),
}
impl Iterator for ElementIterator {
    type Item = (Value, Value);
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl Value {
    /// The negation of [`Value::equals`] (go-cty: `Value.NotEqual`).
    pub fn not_equal(&self, other: &Value) -> Value {
        let _ = other;
        todo!()
    }

    /// The number of elements of a collection, tuple, object, or string-ish
    /// value, as a number value (go-cty: `Value.Length`).
    pub fn length(&self) -> Value {
        todo!()
    }

    /// The number of elements as a native `usize` (go-cty: `Value.LengthInt`).
    ///
    /// # Panics
    /// Panics if the length is not known.
    pub fn length_int(&self) -> usize {
        todo!()
    }

    /// Whether [`Value::element_iterator`] can be used on this value
    /// (go-cty: `Value.CanIterateElements`).
    pub fn can_iterate_elements(&self) -> bool {
        todo!()
    }
}
