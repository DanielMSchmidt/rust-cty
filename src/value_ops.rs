//! Operations on [`Value`]: equality, arithmetic, comparison, logic, indexing,
//! and iteration. Mirrors go-cty's `value_ops.go`.
//!
//! As in go-cty, these operations follow "unknown-tolerant" semantics: an
//! operation on an unknown operand generally yields an unknown result rather
//! than an error, and operations panic (rather than return `Err`) when used
//! with operands of the wrong type — that is a programming error, not a
//! dynamic condition.

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
    // --- Equality ---

    /// cty's user-facing equality operation (go-cty: `Value.Equals`).
    ///
    /// Returns a bool value, which is unknown if either operand is unknown
    /// enough that the result cannot be decided.
    pub fn equals(&self, other: &Value) -> Value {
        let _ = other;
        todo!()
    }

    /// The negation of [`Value::equals`] (go-cty: `Value.NotEqual`).
    pub fn not_equal(&self, other: &Value) -> Value {
        let _ = other;
        todo!()
    }

    /// Exact structural equality, treating unknowns and nulls as equal to
    /// themselves (go-cty: `Value.RawEquals`). Intended for tests; also
    /// exposed as `==` via `PartialEq`.
    pub fn raw_equals(&self, other: &Value) -> bool {
        let _ = other;
        todo!()
    }

    // --- Arithmetic (number operands) ---

    /// Sum of two numbers (go-cty: `Value.Add`).
    pub fn add(&self, other: &Value) -> Value {
        let _ = other;
        todo!()
    }

    /// Difference of two numbers (go-cty: `Value.Subtract`).
    pub fn subtract(&self, other: &Value) -> Value {
        let _ = other;
        todo!()
    }

    /// Additive inverse of a number (go-cty: `Value.Negate`).
    pub fn negate(&self) -> Value {
        todo!()
    }

    /// Product of two numbers (go-cty: `Value.Multiply`).
    pub fn multiply(&self, other: &Value) -> Value {
        let _ = other;
        todo!()
    }

    /// Quotient of two numbers (go-cty: `Value.Divide`). Division by zero
    /// yields infinity, as in go-cty.
    pub fn divide(&self, other: &Value) -> Value {
        let _ = other;
        todo!()
    }

    /// Remainder of division, with the sign behavior of Go's
    /// `big.Float`-based implementation (go-cty: `Value.Modulo`).
    pub fn modulo(&self, other: &Value) -> Value {
        let _ = other;
        todo!()
    }

    /// Absolute value of a number (go-cty: `Value.Absolute`).
    pub fn absolute(&self) -> Value {
        todo!()
    }

    // --- Comparison (number operands) ---

    /// Whether this number is less than another (go-cty: `Value.LessThan`).
    pub fn less_than(&self, other: &Value) -> Value {
        let _ = other;
        todo!()
    }

    /// Whether this number is less than or equal to another
    /// (go-cty: `Value.LessThanOrEqualTo`).
    pub fn less_than_or_equal_to(&self, other: &Value) -> Value {
        let _ = other;
        todo!()
    }

    /// Whether this number is greater than another (go-cty: `Value.GreaterThan`).
    pub fn greater_than(&self, other: &Value) -> Value {
        let _ = other;
        todo!()
    }

    /// Whether this number is greater than or equal to another
    /// (go-cty: `Value.GreaterThanOrEqualTo`).
    pub fn greater_than_or_equal_to(&self, other: &Value) -> Value {
        let _ = other;
        todo!()
    }

    // --- Logic (bool operands) ---

    /// Logical AND (go-cty: `Value.And`).
    pub fn and(&self, other: &Value) -> Value {
        let _ = other;
        todo!()
    }

    /// Logical OR (go-cty: `Value.Or`).
    pub fn or(&self, other: &Value) -> Value {
        let _ = other;
        todo!()
    }

    /// Logical NOT (go-cty: `Value.Not`).
    pub fn not(&self) -> Value {
        todo!()
    }

    // --- Collections ---

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

    /// The element at the given key of a list, map, or tuple
    /// (go-cty: `Value.Index`).
    ///
    /// # Panics
    /// Panics if the key does not exist; check with [`Value::has_index`] first.
    pub fn index(&self, key: &Value) -> Value {
        let _ = key;
        todo!()
    }

    /// Whether the collection has an element at the given key, as a bool value
    /// (go-cty: `Value.HasIndex`).
    pub fn has_index(&self, key: &Value) -> Value {
        let _ = key;
        todo!()
    }

    /// Whether a set contains the given element, as a bool value
    /// (go-cty: `Value.HasElement`).
    pub fn has_element(&self, element: &Value) -> Value {
        let _ = element;
        todo!()
    }

    /// The value of the named attribute of an object value
    /// (go-cty: `Value.GetAttr`).
    ///
    /// # Panics
    /// Panics if this is not an object value or has no such attribute.
    pub fn get_attr(&self, name: &str) -> Value {
        let _ = name;
        todo!()
    }

    // --- Iteration ---

    /// Whether [`Value::element_iterator`] can be used on this value
    /// (go-cty: `Value.CanIterateElements`).
    pub fn can_iterate_elements(&self) -> bool {
        todo!()
    }

    /// An iterator over the `(key, value)` element pairs of a known collection,
    /// tuple, or object value (go-cty: `Value.ElementIterator` / `Value.Elements`).
    ///
    /// # Panics
    /// Panics if [`Value::can_iterate_elements`] would return `false`.
    pub fn element_iterator(&self) -> ElementIterator {
        todo!()
    }

    /// Calls `callback(key, value)` for each element; the callback returns
    /// `true` to stop early. Returns whether iteration was stopped early
    /// (go-cty: `Value.ForEachElement`).
    pub fn for_each_element(&self, callback: impl FnMut(Value, Value) -> bool) -> bool {
        let _ = &callback;
        todo!()
    }
}
