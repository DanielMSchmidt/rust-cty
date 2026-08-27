//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/set/ops_test.go
//!   cty/set/rules_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::rc::Rc;

use cty::set::{Rules, Set};

// upstream: cty/set/rules_test.go testRules
//
// TestRules is a rules implementation that is used for testing. It only
// accepts ints as values, and it has a hash function that just returns the
// given value modulo 16 so that we can easily and dependably test the
// situation where two non-equivalent values have the same hash value.
struct TestRules;

// upstream: cty/set/rules_test.go newTestRules
fn new_test_rules() -> Rc<dyn Rules<i64>> {
    Rc::new(TestRules)
}

impl Rules<i64> for TestRules {
    fn hash(&self, val: &i64) -> u64 {
        (val % 16) as u64
    }

    fn equivalent(&self, val1: &i64, val2: &i64) -> bool {
        val1 == val2
    }

    fn same_rules(&self, _other: &dyn Rules<i64>) -> bool {
        // NOTE(port): upstream type-asserts `other.(testRules)` ("All
        // testRules values are equal, so type-checking is enough"). A Rust
        // trait object cannot be downcast without an `Any` bound, and every
        // rules instance in these tests is a TestRules, so unconditionally
        // returning true is behaviorally equivalent here.
        true
    }
}

/// The set's values sorted ascending, as upstream does with
/// `sort.Ints` after collecting via `EachValue`.
fn sorted_values(s: &Set<i64>) -> Vec<i64> {
    let mut vals = s.values();
    vals.sort_unstable();
    vals
}

// upstream: cty/set/ops_test.go TestBasicSetOps
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

// upstream: cty/set/ops_test.go TestUnion
#[test]
fn union() {
    let tests: Vec<(Set<i64>, Set<i64>, Vec<i64>)> = vec![
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
        let mut got_values = got.values();
        got_values.sort_unstable();
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

// upstream: cty/set/ops_test.go TestIntersection
#[test]
fn intersection() {
    let tests: Vec<(Set<i64>, Set<i64>, Vec<i64>)> = vec![
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
        let mut got_values = got.values();
        got_values.sort_unstable();
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

// upstream: cty/set/ops_test.go TestSubtract
#[test]
fn subtract() {
    let tests: Vec<(Set<i64>, Set<i64>, Vec<i64>)> = vec![
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
        let mut got_values = got.values();
        got_values.sort_unstable();
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

// upstream: cty/set/ops_test.go TestSymmetricDifference
#[test]
fn symmetric_difference() {
    let tests: Vec<(Set<i64>, Set<i64>, Vec<i64>)> = vec![
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
        let mut got_values = got.values();
        got_values.sort_unstable();
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
