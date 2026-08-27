//! Set internals: the generic [`Set`] with pluggable [`Rules`], and the
//! [`ValueSet`] convenience wrapper for sets of [`Value`].
//!
//! Mirrors go-cty's `cty/set` package and `cty.ValueSet`.

use std::rc::Rc;

use crate::types::Type;
use crate::value::Value;

/// The operations a set needs to perform on its element type
/// (go-cty: `set.Rules`).
pub trait Rules<T> {
    /// A hash bucket for the given value; equivalent values must hash equal
    /// (go-cty: `Rules.Hash`).
    fn hash(&self, value: &T) -> u64;

    /// Whether two values are equivalent for set membership purposes
    /// (go-cty: `Rules.Equivalent`).
    fn equivalent(&self, a: &T, b: &T) -> bool;

    /// Whether another rules instance is the same, making two sets compatible
    /// for binary operations (go-cty: `Rules.SameRules`).
    fn same_rules(&self, other: &dyn Rules<T>) -> bool;

    /// Optional ordering between two values, making iteration order
    /// well-defined (go-cty: `OrderedRules.Less`). Rules without a meaningful
    /// order return `None` (the default).
    fn less(&self, a: &T, b: &T) -> Option<bool> {
        let _ = (a, b);
        None
    }
}

/// A set of values of type `T`, deduplicated per its [`Rules`]
/// (go-cty: `set.Set`).
pub struct Set<T> {
    _priv: std::marker::PhantomData<T>,
}

impl<T> std::fmt::Debug for Set<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Set").finish_non_exhaustive()
    }
}

impl<T> Clone for Set<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T> Set<T> {
    /// An empty set with the given rules (go-cty: `set.NewSet`).
    pub fn new(rules: Rc<dyn Rules<T>>) -> Set<T> {
        let _ = rules;
        todo!()
    }

    /// A set with the given rules and initial members
    /// (go-cty: `set.NewSetFromSlice`).
    pub fn from_slice(rules: Rc<dyn Rules<T>>, values: impl IntoIterator<Item = T>) -> Set<T> {
        let _ = (rules, values.into_iter().collect::<Vec<_>>());
        todo!()
    }

    /// The rules this set was created with (go-cty: `Set.Rules`).
    pub fn rules(&self) -> Rc<dyn Rules<T>> {
        todo!()
    }

    /// Whether this set uses rules the same as the given ones
    /// (go-cty: `Set.HasRules`).
    pub fn has_rules(&self, rules: &dyn Rules<T>) -> bool {
        let _ = rules;
        todo!()
    }

    /// Adds a value to the set, in place (go-cty: `Set.Add`).
    pub fn add(&mut self, value: T) {
        let _ = value;
        todo!()
    }

    /// Removes a value from the set, in place (go-cty: `Set.Remove`).
    pub fn remove(&mut self, value: &T) {
        let _ = value;
        todo!()
    }

    /// Whether the set contains an equivalent value (go-cty: `Set.Has`).
    pub fn has(&self, value: &T) -> bool {
        let _ = value;
        todo!()
    }

    /// A shallow copy of the set (go-cty: `Set.Copy`).
    pub fn copy(&self) -> Set<T> {
        todo!()
    }

    /// The number of values in the set (go-cty: `Set.Length`).
    pub fn length(&self) -> usize {
        todo!()
    }

    /// The values of the set; in order if the rules are ordered, otherwise in
    /// an unspecified order (go-cty: `Set.Values` / `Set.Iterator`).
    pub fn values(&self) -> Vec<T> {
        todo!()
    }

    /// The union of this set and another (go-cty: `Set.Union`).
    ///
    /// # Panics
    /// As in go-cty, binary operations panic when the two sets' rules differ.
    pub fn union(&self, other: &Set<T>) -> Set<T> {
        let _ = other;
        todo!()
    }

    /// The intersection of this set and another (go-cty: `Set.Intersection`).
    pub fn intersection(&self, other: &Set<T>) -> Set<T> {
        let _ = other;
        todo!()
    }

    /// The values in this set that are not in the other (go-cty: `Set.Subtract`).
    pub fn subtract(&self, other: &Set<T>) -> Set<T> {
        let _ = other;
        todo!()
    }

    /// The values in exactly one of the two sets
    /// (go-cty: `Set.SymmetricDifference`).
    pub fn symmetric_difference(&self, other: &Set<T>) -> Set<T> {
        let _ = other;
        todo!()
    }
}

/// A mutable set of [`Value`]s of a particular element type, as extracted from
/// or used to build a set value (go-cty: `cty.ValueSet`).
#[derive(Debug, Clone)]
pub struct ValueSet {
    _priv: (),
}

impl ValueSet {
    /// An empty value set with the given element type
    /// (go-cty: `cty.NewValueSet`).
    pub fn new(element_type: Type) -> ValueSet {
        let _ = element_type;
        todo!()
    }

    /// The element type of the set (go-cty: `ValueSet.ElementType`).
    pub fn element_type(&self) -> Type {
        todo!()
    }

    /// Adds a value, which must conform to the element type, in place
    /// (go-cty: `ValueSet.Add`).
    pub fn add(&mut self, value: Value) {
        let _ = value;
        todo!()
    }

    /// Removes a value, in place (go-cty: `ValueSet.Remove`).
    pub fn remove(&mut self, value: &Value) {
        let _ = value;
        todo!()
    }

    /// Whether the set contains the given value (go-cty: `ValueSet.Has`).
    pub fn has(&self, value: &Value) -> bool {
        let _ = value;
        todo!()
    }

    /// A copy of this set (go-cty: `ValueSet.Copy`).
    pub fn copy(&self) -> ValueSet {
        todo!()
    }

    /// The number of values in the set (go-cty: `ValueSet.Length`).
    pub fn length(&self) -> usize {
        todo!()
    }

    /// The values of the set in an unspecified but consistent order
    /// (go-cty: `ValueSet.Values`).
    pub fn values(&self) -> Vec<Value> {
        todo!()
    }

    /// The union of this set and another (go-cty: `ValueSet.Union`).
    pub fn union(&self, other: &ValueSet) -> ValueSet {
        let _ = other;
        todo!()
    }

    /// The intersection of this set and another
    /// (go-cty: `ValueSet.Intersection`).
    pub fn intersection(&self, other: &ValueSet) -> ValueSet {
        let _ = other;
        todo!()
    }

    /// The values in this set that are not in the other
    /// (go-cty: `ValueSet.Subtract`).
    pub fn subtract(&self, other: &ValueSet) -> ValueSet {
        let _ = other;
        todo!()
    }

    /// The values in exactly one of the two sets
    /// (go-cty: `ValueSet.SymmetricDifference`).
    pub fn symmetric_difference(&self, other: &ValueSet) -> ValueSet {
        let _ = other;
        todo!()
    }
}
