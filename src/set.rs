//! Set internals: the generic [`Set`] with its [`Rules`] as a type parameter,
//! and the [`ValueSet`] convenience wrapper for sets of [`Value`].
//!
//! Mirrors go-cty's `cty/set` package and `cty.ValueSet`. Upstream stores the
//! rules as an interface value; here they are a type parameter — see the
//! "Set rules" section of `docs/api-mapping.md` for why.

use std::collections::BTreeMap;

use crate::types::Type;
use crate::value::Value;

/// The operations a set needs to perform on its element type
/// (go-cty: `set.Rules`).
///
/// Not object-safe, by design: `same_rules` takes `&Self`, so the type check
/// that go-cty does with a runtime type assertion is the compiler's job here.
pub trait Rules<T>: PartialEq {
    /// A hash bucket for the given value; equivalent values must hash equal
    /// (go-cty: `Rules.Hash`).
    fn hash(&self, value: &T) -> u64;

    /// Whether two values are equivalent for set membership purposes
    /// (go-cty: `Rules.Equivalent`).
    fn equivalent(&self, a: &T, b: &T) -> bool;

    /// Whether another rules value of the same type agrees with this one,
    /// making two sets compatible for binary operations
    /// (go-cty: `Rules.SameRules`).
    ///
    /// Two values of the same `Self` can still disagree — the cty element
    /// rules compare their element types.
    fn same_rules(&self, other: &Self) -> bool;
}

/// [`Rules`] that additionally define an ordering over their element type,
/// making set iteration order well-defined (go-cty: `set.OrderedRules`).
///
/// Implementing this trait is how a rules type opts in to ordering. Rules that
/// do not implement it (upstream: `pathSetRules`) have no ordered iteration,
/// and [`Set::ordered_values`] is not callable for a set that uses them.
pub trait OrderedRules<T>: Rules<T> {
    /// Whether `a` should sort before `b`. Returns `false` both when `b` sorts
    /// first and when the two values have no defined order — the order this
    /// describes is only partial (go-cty: `OrderedRules.Less`).
    fn less(&self, a: &T, b: &T) -> bool;
}

/// A set of values of type `T`, deduplicated per its [`Rules`]
/// (go-cty: `set.Set`).
pub struct Set<T, R> {
    val: BTreeMap<u64, Vec<T>>,
    rules: R,
}

impl<T, R> std::fmt::Debug for Set<T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Set").finish_non_exhaustive()
    }
}

impl<T: Clone, R: Clone> Clone for Set<T, R> {
    fn clone(&self) -> Self {
        Self {
            val: self.val.clone(),
            rules: self.rules.clone(),
        }
    }
}

impl<T: Clone, R: Rules<T> + Clone> Set<T, R> {
    /// An empty set with the given rules (go-cty: `set.NewSet`).
    pub fn new(rules: R) -> Set<T, R> {
        Self {
            val: BTreeMap::new(),
            rules,
        }
    }

    /// A set with the given rules and initial members
    /// (go-cty: `set.NewSetFromSlice`).
    pub fn from_slice(rules: R, values: impl IntoIterator<Item = T>) -> Set<T, R> {
        // TODO: There is probably a nicer way
        let mut set = Self::new(rules);
        values.into_iter().for_each(|i| set.add(i));
        set
    }

    /// The rules this set was created with (go-cty: `Set.Rules`).
    pub fn rules(&self) -> &R {
        &self.rules
    }

    /// Whether this set uses rules the same as the given ones
    /// (go-cty: `Set.HasRules`).
    pub fn has_rules(&self, rules: &R) -> bool {
        self.rules() == rules
    }

    /// Adds a value to the set, in place (go-cty: `Set.Add`).
    pub fn add(&mut self, value: T) {
        let key = self.rules().hash(&value);
        let bucket = self.val.get_mut(&key);
        if let Some(items) = bucket {
            if !items.iter().any(|item| self.rules.equivalent(&value, item)) {
                items.push(value);
            }
        } else {
            self.val.insert(key, vec![value]);
        }
    }

    /// Removes a value from the set, in place (go-cty: `Set.Remove`).
    pub fn remove(&mut self, value: &T) {
        let key = self.rules().hash(&value);
        let bucket = self.val.get_mut(&key);
        if let Some(items) = bucket {
            if let Some(pos) = items
                .iter()
                .position(|item| self.rules.equivalent(value, item))
            {
                items.remove(pos);
            }
        }
    }

    /// Whether the set contains an equivalent value (go-cty: `Set.Has`).
    pub fn has(&self, value: &T) -> bool {
        let key = self.rules().hash(&value);
        let bucket = self.val.get(&key);
        if let Some(items) = bucket {
            items.iter().any(|item| self.rules.equivalent(&value, item))
        } else {
            false
        }
    }

    /// A shallow copy of the set (go-cty: `Set.Copy`).
    pub fn copy(&self) -> Set<T, R> {
        self.clone()
    }

    /// The number of values in the set (go-cty: `Set.Length`).
    pub fn length(&self) -> usize {
        self.val.len()
    }

    /// The values of the set, in an unspecified but consistent order
    /// (go-cty: `Set.Values`, for rules that are not `OrderedRules`).
    ///
    /// Upstream traverses buckets in ascending hash order, so two sets built
    /// the same way enumerate the same way.
    pub fn values(&self) -> Vec<&T> {
        self.val.iter().flat_map(|(k, v)| v).collect()
    }

    fn must_have_same_rules(&self, other: &Set<T, R>) {
        if !self.rules.same_rules(&other.rules) {
            panic!("The rules are not the same for the two sets")
        }
    }

    /// The union of this set and another (go-cty: `Set.Union`).
    ///
    /// # Panics
    /// As in go-cty, binary operations panic when the two sets' rules
    /// disagree. Rules of *different* types cannot meet here at all — that is
    /// a compile error — so this is the [`Rules::same_rules`] case.
    pub fn union(&self, other: &Set<T, R>) -> Set<T, R> {
        self.must_have_same_rules(other);
        let mut new_set = self.clone();

        for (_, values) in other.val.iter() {
            for value in values {
                new_set.add(value.clone())
            }
        }

        new_set
    }

    /// The intersection of this set and another (go-cty: `Set.Intersection`).
    /// # Panics
    /// As in go-cty, binary operations panic when the two sets' rules
    /// disagree. Rules of *different* types cannot meet here at all — that is
    /// a compile error — so this is the [`Rules::same_rules`] case.
    pub fn intersection(&self, other: &Set<T, R>) -> Set<T, R> {
        self.must_have_same_rules(other);
        let mut new_set = Self::new(self.rules.clone());

        for value in self.values() {
            if other.has(value) {
                new_set.add(value.clone())
            }
        }

        new_set
    }

    /// The values in this set that are not in the other (go-cty: `Set.Subtract`).
    pub fn subtract(&self, other: &Set<T, R>) -> Set<T, R> {
        self.must_have_same_rules(other);
        let mut new_set = Self::new(self.rules.clone());

        let other_values = other.values();
        for value in self.values() {
            if !other_values
                .iter()
                .any(|other_value| new_set.rules.equivalent(value, other_value))
            {
                new_set.add(value.clone())
            }
        }

        new_set
    }

    /// The values in exactly one of the two sets
    /// (go-cty: `Set.SymmetricDifference`).
    pub fn symmetric_difference(&self, other: &Set<T, R>) -> Set<T, R> {
        self.must_have_same_rules(other);
        let mut new_set = Self::new(self.rules.clone());

        let intersection = self.intersection(other);
        let not_to_add = intersection.values();
        let mut all = self.values();
        all.append(other.values().as_mut());

        for val in all {
            if !not_to_add.iter().any(|v| self.rules.equivalent(val, v)) {
                new_set.add(val.clone())
            }
        }

        new_set
    }
}

impl<T, R: OrderedRules<T>> Set<T, R> {
    /// The values of the set ordered by [`OrderedRules::less`]
    /// (go-cty: `Set.Values`, for rules that are `OrderedRules`).
    ///
    /// Where the rules' order is only partial, the relative order of
    /// incomparable values is unspecified but consistent, as upstream.
    pub fn ordered_values(&self) -> Vec<&T> {
        todo!()
    }
}

/// The [`Rules`] used by set values of a given element type
/// (go-cty: the unexported `setRules` in `cty/set_internals.go`).
///
/// Public only because [`crate::internals::set_rules`] hands it to the
/// conformance suite; not part of the supported API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueRules {
    element_type: Type,
}

impl ValueRules {
    /// The rules for sets whose elements have the given type
    /// (go-cty: `setRules{ElementType: ty}`).
    pub fn new(element_type: Type) -> ValueRules {
        ValueRules { element_type }
    }
}

impl Rules<Value> for ValueRules {
    fn hash(&self, value: &Value) -> u64 {
        let _ = value;
        todo!()
    }

    fn equivalent(&self, a: &Value, b: &Value) -> bool {
        let _ = (a, b);
        todo!()
    }

    fn same_rules(&self, other: &Self) -> bool {
        let _ = other;
        todo!()
    }
}

impl OrderedRules<Value> for ValueRules {
    fn less(&self, a: &Value, b: &Value) -> bool {
        let _ = (a, b);
        todo!()
    }
}

/// A mutable set of [`Value`]s of a particular element type, as extracted from
/// or used to build a set value (go-cty: `cty.ValueSet`).
#[derive(Debug, Clone)]
pub struct ValueSet {
    val: BTreeMap<i64, Vec<Value>>,
    rules: ValueRules,
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

pub struct IntoIter {
    outer: <BTreeMap<i64, Vec<Value>> as IntoIterator>::IntoIter,
    inner: std::vec::IntoIter<Value>,
}

impl Iterator for IntoIter {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(value) = self.inner.next() {
                return Some(value);
            }
            // Advance to the next non-exhausted bucket, skipping empty Vecs.
            let (_, values) = self.outer.next()?;
            self.inner = values.into_iter();
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.inner.len(), None)
    }
}

impl IntoIterator for ValueSet {
    type Item = Value;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            outer: self.val.into_iter(),
            inner: Vec::new().into_iter(),
        }
    }
}

pub struct Iter<'a> {
    outer: std::collections::btree_map::Iter<'a, i64, Vec<Value>>,
    inner: std::slice::Iter<'a, Value>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(value) = self.inner.next() {
                return Some(value);
            }
            let (_, values) = self.outer.next()?;
            self.inner = values.iter();
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.inner.len(), None)
    }
}

impl ValueSet {
    /// Iterate over the values stored in the set
    pub fn iter(&self) -> Iter<'_> {
        const EMPTY: &[Value] = &[];
        Iter {
            outer: self.val.iter(),
            inner: EMPTY.iter(),
        }
    }
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/set/ops_test.go
    //!   cty/set/rules_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::set::{Rules, Set};

    // Ported from testRules:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/set/rules_test.go#L7
    //
    // TestRules is a rules implementation that is used for testing. It only
    // accepts ints as values, and it has a hash function that just returns the
    // given value modulo 16 so that we can easily and dependably test the
    // situation where two non-equivalent values have the same hash value.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct TestRules;

    /// One upstream row for the binary set operations: the two operands and the
    /// expected result, in the set's own ordering.
    type BinOpCase = (Set<i64, TestRules>, Set<i64, TestRules>, Vec<i64>);

    // Ported from newTestRules:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/set/rules_test.go#L9
    fn new_test_rules() -> TestRules {
        TestRules
    }

    impl Rules<i64> for TestRules {
        fn hash(&self, val: &i64) -> u64 {
            (val % 16) as u64
        }

        fn equivalent(&self, val1: &i64, val2: &i64) -> bool {
            val1 == val2
        }

        fn same_rules(&self, _other: &Self) -> bool {
            // NOTE(port): upstream type-asserts `other.(testRules)` ("All
            // testRules values are equal, so type-checking is enough"). With rules
            // as a type parameter the compiler has already done that check, so
            // every reachable call here compares two TestRules and is true.
            true
        }
    }

    /// The set's values sorted ascending, as upstream does with
    /// `sort.Ints` after collecting via `EachValue`.
    ///
    /// NOTE(port): `Set::values` hands back borrowed elements, so this copies them
    /// out to keep the expected values below as plain literals.
    fn sorted_values(s: &Set<i64, TestRules>) -> Vec<i64> {
        let mut vals: Vec<i64> = s.values().into_iter().copied().collect();
        vals.sort_unstable();
        vals
    }

    // Ported from TestBasicSetOps:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/set/ops_test.go#L13
    //
    // TestBasicSetOps tests the fundamental operations, whose implementations
    // operate directly on the underlying data structure. The remaining operations
    // are implemented in terms of these.
    //
    // NOTE(port): upstream asserts on the unexported `s.vals` bucket map with
    // reflect.DeepEqual after each mutation (e.g. `want[1] = []int{1, 17, 33}`
    // once the colliding values 17 and 33 share bucket 1). The Rust Set does not
    // expose its internal buckets, so each of those checks is expressed as the
    // sorted list of values that the expected bucket contents would produce.
    #[test]
    fn basic_set_ops() {
        let mut s = Set::new(new_test_rules());
        assert_eq!(
            sorted_values(&s),
            Vec::<i64>::new(),
            "new set has unexpected contents"
        );
        s.add(1);
        assert_eq!(
            sorted_values(&s),
            vec![1],
            "after s.add(1) set has unexpected contents"
        );
        assert!(s.has(&1), "s.has(&1) returned false; want true");
        s.add(2);
        assert_eq!(
            sorted_values(&s),
            vec![1, 2],
            "after s.add(2) set has unexpected contents"
        );
        assert!(s.has(&2), "s.has(&2) returned false; want true");

        // Our TestRules cause 17 and 33 to return the same hash value as 1, so we
        // can use this to test the situation where multiple values are in a
        // bucket.
        assert!(!s.has(&17), "s.has(&17) returned true; want false");
        s.add(17);
        s.add(33);
        assert_eq!(
            sorted_values(&s),
            vec![1, 2, 17, 33],
            "after s.add(17) and s.add(33) set has unexpected contents"
        );
        assert!(s.has(&17), "s.has(&17) returned false; want true");
        assert!(s.has(&33), "s.has(&33) returned false; want true");

        let vals = sorted_values(&s);
        assert_eq!(vals, vec![1, 2, 17, 33], "wrong values from values()");

        s.remove(&2);
        assert_eq!(
            sorted_values(&s),
            vec![1, 17, 33],
            "after s.remove(&2) set has unexpected contents"
        );

        s.remove(&17);
        assert_eq!(
            sorted_values(&s),
            vec![1, 33],
            "after s.remove(&17) set has unexpected contents"
        );

        s.remove(&1);
        assert_eq!(
            sorted_values(&s),
            vec![33],
            "after s.remove(&1) set has unexpected contents"
        );

        s.remove(&33);
        assert_eq!(
            sorted_values(&s),
            Vec::<i64>::new(),
            "after s.remove(&33) set has unexpected contents"
        );

        let vals = s.values();
        assert!(
            vals.is_empty(),
            "s.values() produced values {vals:?}; want none"
        );
    }

    // Ported from TestUnion:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/set/ops_test.go#L96
    #[test]
    fn union() {
        let tests: Vec<BinOpCase> = vec![
            (
                Set::new(new_test_rules()),
                Set::new(new_test_rules()),
                vec![],
            ),
            (
                Set::from_slice(new_test_rules(), [1]),
                Set::new(new_test_rules()),
                vec![1],
            ),
            (
                Set::from_slice(new_test_rules(), [1]),
                Set::from_slice(new_test_rules(), [2]),
                vec![1, 2],
            ),
            (
                Set::from_slice(new_test_rules(), [1]),
                Set::from_slice(new_test_rules(), [1]),
                vec![1],
            ),
            (
                Set::from_slice(new_test_rules(), [17, 33]),
                Set::from_slice(new_test_rules(), [1]),
                vec![1, 17, 33],
            ),
            (
                Set::from_slice(new_test_rules(), [17, 33]),
                Set::from_slice(new_test_rules(), [2, 1]),
                vec![1, 2, 17, 33],
            ),
        ];

        for (i, (s1, s2, want_values)) in tests.iter().enumerate() {
            let got = s1.union(s2);
            let got_values = sorted_values(&got);
            let mut want_values = want_values.clone();
            want_values.sort_unstable();
            assert_eq!(
                got_values,
                want_values,
                "case {i}: wrong result for {:?} union {:?}",
                s1.values(),
                s2.values(),
            );
        }
    }

    // Ported from TestIntersection:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/set/ops_test.go#L158
    #[test]
    fn intersection() {
        let tests: Vec<BinOpCase> = vec![
            (
                Set::new(new_test_rules()),
                Set::new(new_test_rules()),
                vec![],
            ),
            (
                Set::from_slice(new_test_rules(), [1]),
                Set::new(new_test_rules()),
                vec![],
            ),
            (
                Set::from_slice(new_test_rules(), [1]),
                Set::from_slice(new_test_rules(), [2]),
                vec![],
            ),
            (
                Set::from_slice(new_test_rules(), [1]),
                Set::from_slice(new_test_rules(), [1]),
                vec![1],
            ),
            (
                Set::from_slice(new_test_rules(), [1, 17]),
                Set::from_slice(new_test_rules(), [1, 2, 3]),
                vec![1],
            ),
            (
                Set::from_slice(new_test_rules(), [3, 2, 1]),
                Set::from_slice(new_test_rules(), [1, 2, 3]),
                vec![1, 2, 3],
            ),
            (
                Set::from_slice(new_test_rules(), [17, 33]),
                Set::from_slice(new_test_rules(), [1]),
                vec![],
            ),
            (
                Set::from_slice(new_test_rules(), [17, 33]),
                Set::from_slice(new_test_rules(), [2, 1]),
                vec![],
            ),
        ];

        for (i, (s1, s2, want_values)) in tests.iter().enumerate() {
            let got = s1.intersection(s2);
            let got_values = sorted_values(&got);
            let mut want_values = want_values.clone();
            want_values.sort_unstable();
            assert_eq!(
                got_values,
                want_values,
                "case {i}: wrong result for {:?} intersection {:?}",
                s1.values(),
                s2.values(),
            );
        }
    }

    // Ported from TestSubtract:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/set/ops_test.go#L230
    #[test]
    fn subtract() {
        let tests: Vec<BinOpCase> = vec![
            (
                Set::new(new_test_rules()),
                Set::new(new_test_rules()),
                vec![],
            ),
            (
                Set::from_slice(new_test_rules(), [1]),
                Set::new(new_test_rules()),
                vec![1],
            ),
            (
                Set::from_slice(new_test_rules(), [1]),
                Set::from_slice(new_test_rules(), [2]),
                vec![1],
            ),
            (
                Set::from_slice(new_test_rules(), [1]),
                Set::from_slice(new_test_rules(), [1]),
                vec![],
            ),
            (
                Set::from_slice(new_test_rules(), [1, 17]),
                Set::from_slice(new_test_rules(), [1, 2, 3]),
                vec![17],
            ),
            (
                Set::from_slice(new_test_rules(), [3, 2, 1]),
                Set::from_slice(new_test_rules(), [1, 2, 3]),
                vec![],
            ),
            (
                Set::from_slice(new_test_rules(), [17, 33]),
                Set::from_slice(new_test_rules(), [1]),
                vec![17, 33],
            ),
            (
                Set::from_slice(new_test_rules(), [17, 33]),
                Set::from_slice(new_test_rules(), [2, 1]),
                vec![17, 33],
            ),
        ];

        for (i, (s1, s2, want_values)) in tests.iter().enumerate() {
            let got = s1.subtract(s2);
            let got_values = sorted_values(&got);
            let mut want_values = want_values.clone();
            want_values.sort_unstable();
            assert_eq!(
                got_values,
                want_values,
                "case {i}: wrong result for {:?} subtract {:?}",
                s1.values(),
                s2.values(),
            );
        }
    }

    // Ported from TestSymmetricDifference:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/set/ops_test.go#L302
    #[test]
    fn symmetric_difference() {
        let tests: Vec<BinOpCase> = vec![
            (
                Set::new(new_test_rules()),
                Set::new(new_test_rules()),
                vec![],
            ),
            (
                Set::from_slice(new_test_rules(), [1]),
                Set::new(new_test_rules()),
                vec![1],
            ),
            (
                Set::from_slice(new_test_rules(), [1]),
                Set::from_slice(new_test_rules(), [2]),
                vec![1, 2],
            ),
            (
                Set::from_slice(new_test_rules(), [1]),
                Set::from_slice(new_test_rules(), [1]),
                vec![],
            ),
            (
                Set::from_slice(new_test_rules(), [1, 17]),
                Set::from_slice(new_test_rules(), [1, 2, 3]),
                vec![2, 3, 17],
            ),
            (
                Set::from_slice(new_test_rules(), [3, 2, 1]),
                Set::from_slice(new_test_rules(), [1, 2, 3]),
                vec![],
            ),
            (
                Set::from_slice(new_test_rules(), [17, 33]),
                Set::from_slice(new_test_rules(), [1]),
                vec![1, 17, 33],
            ),
            (
                Set::from_slice(new_test_rules(), [17, 33]),
                Set::from_slice(new_test_rules(), [2, 1]),
                vec![1, 2, 17, 33],
            ),
        ];

        for (i, (s1, s2, want_values)) in tests.iter().enumerate() {
            let got = s1.symmetric_difference(s2);
            let got_values = sorted_values(&got);
            let mut want_values = want_values.clone();
            want_values.sort_unstable();
            assert_eq!(
                got_values,
                want_values,
                "case {i}: wrong result for {:?} symmetric difference {:?}",
                s1.values(),
                s2.values(),
            );
        }
    }
}
