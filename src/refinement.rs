//! Unknown value refinements: constraining what an unknown value could
//! eventually be. Mirrors go-cty's `unknown_refinement.go`.

use crate::types::Type;
use crate::value::Value;

/// Builder for refining an unknown (or known) value, obtained from
/// [`Value::refine`] (go-cty: `cty.RefinementBuilder`).
///
/// As in go-cty, refining a known value asserts facts about it: the builder
/// panics if a refinement contradicts the known value, and `new_value` returns
/// the known value unchanged otherwise.
#[derive(Debug)]
pub struct RefinementBuilder {
    _priv: (),
}

impl RefinementBuilder {
    /// Refines the value as definitely not null (go-cty: `RefinementBuilder.NotNull`).
    pub fn not_null(self) -> Self {
        todo!()
    }

    /// Refines the value as definitely null (go-cty: `RefinementBuilder.Null`).
    pub fn null(self) -> Self {
        todo!()
    }

    /// Constrains a number value's lower bound (go-cty:
    /// `RefinementBuilder.NumberRangeLowerBound`).
    pub fn number_range_lower_bound(self, min: Value, inclusive: bool) -> Self {
        let _ = (min, inclusive);
        todo!()
    }

    /// Constrains a number value's upper bound (go-cty:
    /// `RefinementBuilder.NumberRangeUpperBound`).
    pub fn number_range_upper_bound(self, max: Value, inclusive: bool) -> Self {
        let _ = (max, inclusive);
        todo!()
    }

    /// Constrains a number value to an inclusive range (go-cty:
    /// `RefinementBuilder.NumberRangeInclusive`).
    pub fn number_range_inclusive(self, min: Value, max: Value) -> Self {
        let _ = (min, max);
        todo!()
    }

    /// Constrains a string value's prefix, conservatively truncated to avoid
    /// combining-character hazards (go-cty: `RefinementBuilder.StringPrefix`).
    pub fn string_prefix(self, prefix: impl Into<String>) -> Self {
        let _ = prefix.into();
        todo!()
    }

    /// Constrains a string value's prefix without the safety truncation
    /// (go-cty: `RefinementBuilder.StringPrefixFull`).
    pub fn string_prefix_full(self, prefix: impl Into<String>) -> Self {
        let _ = prefix.into();
        todo!()
    }

    /// Constrains a collection value's element count exactly
    /// (go-cty: `RefinementBuilder.CollectionLength`).
    pub fn collection_length(self, length: usize) -> Self {
        let _ = length;
        todo!()
    }

    /// Constrains a collection value's minimum element count
    /// (go-cty: `RefinementBuilder.CollectionLengthLowerBound`).
    pub fn collection_length_lower_bound(self, min: usize) -> Self {
        let _ = min;
        todo!()
    }

    /// Constrains a collection value's maximum element count
    /// (go-cty: `RefinementBuilder.CollectionLengthUpperBound`).
    pub fn collection_length_upper_bound(self, max: usize) -> Self {
        let _ = max;
        todo!()
    }

    /// Finalizes the refinements into a value (go-cty:
    /// `RefinementBuilder.NewValue`).
    pub fn new_value(self) -> Value {
        todo!()
    }
}

/// A description of the range of values an unknown value could take, obtained
/// from [`Value::range`] (go-cty: `cty.ValueRange`).
#[derive(Debug)]
pub struct ValueRange {
    _priv: (),
}

impl ValueRange {
    /// The type all possible values conform to (go-cty: `ValueRange.TypeConstraint`).
    pub fn type_constraint(&self) -> Type {
        todo!()
    }

    /// Whether null is in the range (go-cty: `ValueRange.CouldBeNull`).
    pub fn could_be_null(&self) -> bool {
        todo!()
    }

    /// Whether the value is definitely not null (go-cty:
    /// `ValueRange.DefinitelyNotNull`).
    pub fn definitely_not_null(&self) -> bool {
        todo!()
    }

    /// A number value's lower bound and whether it is inclusive
    /// (go-cty: `ValueRange.NumberLowerBound`).
    pub fn number_lower_bound(&self) -> (Value, bool) {
        todo!()
    }

    /// A number value's upper bound and whether it is inclusive
    /// (go-cty: `ValueRange.NumberUpperBound`).
    pub fn number_upper_bound(&self) -> (Value, bool) {
        todo!()
    }

    /// A string value's known prefix, possibly empty
    /// (go-cty: `ValueRange.StringPrefix`).
    pub fn string_prefix(&self) -> String {
        todo!()
    }

    /// A collection value's minimum element count
    /// (go-cty: `ValueRange.LengthLowerBound`).
    pub fn length_lower_bound(&self) -> usize {
        todo!()
    }

    /// A collection value's maximum element count
    /// (go-cty: `ValueRange.LengthUpperBound`).
    pub fn length_upper_bound(&self) -> usize {
        todo!()
    }

    /// Whether the given value is possibly within this range, as a bool value
    /// that may be unknown (go-cty: `ValueRange.Includes`).
    pub fn includes(&self, value: &Value) -> Value {
        let _ = value;
        todo!()
    }
}

impl Value {
    /// Begins refining this value (go-cty: `Value.Refine`).
    pub fn refine(&self) -> RefinementBuilder {
        todo!()
    }

    /// Shorthand for `refine().not_null().new_value()`
    /// (go-cty: `Value.RefineNotNull`).
    pub fn refine_not_null(&self) -> Value {
        todo!()
    }

    /// Applies the given refinement closures in order
    /// (go-cty: `Value.RefineWith`).
    pub fn refine_with(
        &self,
        refiners: impl IntoIterator<Item = fn(RefinementBuilder) -> RefinementBuilder>,
    ) -> Value {
        let _ = refiners.into_iter().collect::<Vec<_>>();
        todo!()
    }

    /// The range of values this value could be (go-cty: `Value.Range`).
    pub fn range(&self) -> ValueRange {
        todo!()
    }
}
