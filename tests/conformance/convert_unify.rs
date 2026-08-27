//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/convert/unify_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::Type;
use cty::convert;

// upstream: cty/convert/unify_test.go TestUnify
//
// Upstream returns `(cty.NilType, nil)` when no unification is possible;
// here that maps to `unify` returning `None`. A `Some` expectation carries
// the unified type plus, for each input, whether a conversion is needed
// (`true` = the Go slice held a non-nil Conversion).
#[test]
fn unify() {
    #[allow(clippy::type_complexity)]
    let tests: Vec<(Vec<Type>, Option<(Type, Vec<bool>)>)> = vec![
        (vec![], None),
        (vec![Type::string()], Some((Type::string(), vec![false]))),
        (vec![Type::number()], Some((Type::number(), vec![false]))),
        (
            vec![Type::number(), Type::number()],
            Some((Type::number(), vec![false, false])),
        ),
        (
            vec![Type::number(), Type::string()],
            Some((Type::string(), vec![true, false])),
        ),
        (
            vec![Type::string(), Type::number()],
            Some((Type::string(), vec![false, true])),
        ),
        (
            vec![Type::bool(), Type::string(), Type::number()],
            Some((Type::string(), vec![true, false, true])),
        ),
        (vec![Type::bool(), Type::number()], None),
        (
            vec![
                Type::object([("foo", Type::string())]),
                Type::object([("foo", Type::string())]),
            ],
            Some((Type::object([("foo", Type::string())]), vec![false, false])),
        ),
        (
            vec![
                Type::object([("foo", Type::string())]),
                Type::object([("foo", Type::number())]),
            ],
            Some((Type::object([("foo", Type::string())]), vec![false, true])),
        ),
        (
            vec![
                Type::object([("foo", Type::string())]),
                Type::object([("bar", Type::number())]),
            ],
            Some((Type::map(Type::string()), vec![true, true])),
        ),
        (
            vec![
                Type::object([("foo", Type::string())]),
                Type::empty_object(),
            ],
            Some((Type::map(Type::string()), vec![true, true])),
        ),
        (
            vec![
                Type::object([("foo", Type::bool())]),
                Type::object([("bar", Type::number())]),
            ],
            None,
        ),
        (
            vec![
                Type::object([("foo", Type::bool())]),
                Type::object([("foo", Type::number())]),
            ],
            None,
        ),
        (
            vec![Type::tuple([Type::string()]), Type::tuple([Type::string()])],
            Some((Type::tuple([Type::string()]), vec![false, false])),
        ),
        (
            vec![Type::tuple([Type::string()]), Type::tuple([Type::number()])],
            Some((Type::tuple([Type::string()]), vec![false, true])),
        ),
        (
            vec![
                Type::tuple([Type::string()]),
                Type::tuple([Type::string(), Type::number()]),
            ],
            Some((Type::list(Type::string()), vec![true, true])),
        ),
        (
            vec![Type::tuple([Type::string()]), Type::empty_tuple()],
            Some((Type::list(Type::string()), vec![true, true])),
        ),
        (
            vec![Type::tuple([Type::bool()]), Type::tuple([Type::number()])],
            None,
        ),
        (
            // objects can unify as map(string) within the tuples
            vec![
                Type::tuple([
                    Type::object([("a", Type::string())]),
                    Type::object([("a", Type::string())]),
                ]),
                Type::tuple([Type::object([("a", Type::string()), ("b", Type::string())])]),
            ],
            Some((Type::list(Type::map(Type::string())), vec![true, true])),
        ),
        (
            // The second tuple value could be anything, so we can't unify
            // these as a list.
            // FIXME: While a unification is possible, we get a NilType for
            // now until we can handle more complex recursive unification.
            vec![
                Type::tuple([Type::object([("a", Type::string())]), Type::dynamic()]),
                Type::list(Type::dynamic()),
            ],
            None,
        ),
        (
            // unifies to the same result as above, since the only difference
            // is the addition of a list
            vec![
                Type::list(Type::object([("a", Type::string())])),
                Type::tuple([Type::object([("a", Type::string()), ("b", Type::string())])]),
                Type::tuple([
                    Type::object([("a", Type::string()), ("b", Type::string())]),
                    Type::object([("c", Type::string()), ("d", Type::string())]),
                ]),
            ],
            Some((
                Type::list(Type::map(Type::string())),
                vec![true, true, true],
            )),
        ),
        (
            // Ensure the map does not change the unification process
            vec![
                Type::list(Type::object([("a", Type::string())])),
                Type::list(Type::map(Type::string())),
                Type::tuple([
                    Type::map(Type::string()),
                    Type::object([("a", Type::string()), ("b", Type::string())]),
                ]),
            ],
            Some((
                Type::list(Type::map(Type::string())),
                vec![true, false, true],
            )),
        ),
        (
            // different tuple lengths unify as a list, and the objects can
            // unify as maps
            vec![
                Type::tuple([
                    Type::object([("a", Type::string()), ("b", Type::number())]),
                    Type::object([("a", Type::string()), ("b", Type::number())]),
                ]),
                Type::tuple([Type::object([("a", Type::string())])]),
            ],
            Some((Type::list(Type::map(Type::string())), vec![true, true])),
        ),
        (
            // the equivalent tuple lengths still unify as a tuple, though the
            // objects are unified as a map
            vec![
                Type::tuple([Type::object([("a", Type::string()), ("b", Type::number())])]),
                Type::tuple([Type::object([("a", Type::string())])]),
            ],
            Some((Type::tuple([Type::map(Type::string())]), vec![true, true])),
        ),
        (
            // This should unify to like the tuple above
            vec![
                Type::list(Type::object([("a", Type::number()), ("b", Type::string())])),
                Type::tuple([Type::object([("a", Type::string())])]),
            ],
            Some((Type::list(Type::map(Type::string())), vec![true, true])),
        ),
        (
            // This should also unify like the previous 2 examples
            vec![
                Type::list(Type::object([("a", Type::number()), ("b", Type::string())])),
                Type::list(Type::object([("a", Type::string())])),
            ],
            Some((Type::list(Type::map(Type::string())), vec![true, true])),
        ),
        (
            // Objects and maps should unify along with the surrounding lists
            // and tuples.
            vec![
                Type::list(Type::object([
                    ("a", Type::object([("a", Type::string())])),
                    (
                        "b",
                        Type::object([("a", Type::string()), ("b", Type::string())]),
                    ),
                ])),
                Type::list(Type::map(Type::object([
                    ("a", Type::string()),
                    ("b", Type::string()),
                ]))),
            ],
            Some((
                Type::list(Type::map(Type::map(Type::string()))),
                vec![true, true],
            )),
        ),
        (
            // objects can unify as maps within objects
            vec![
                Type::object([("a", Type::object([("a", Type::string())]))]),
                Type::object([(
                    "a",
                    Type::object([("a", Type::string()), ("b", Type::string())]),
                )]),
            ],
            Some((
                Type::object([("a", Type::map(Type::string()))]),
                vec![true, true],
            )),
        ),
        (
            // nested objects can unify as maps
            vec![
                Type::object([
                    ("a", Type::object([("a", Type::string())])),
                    (
                        "b",
                        Type::object([("a", Type::string()), ("b", Type::string())]),
                    ),
                ]),
                Type::map(Type::object([("a", Type::string()), ("b", Type::string())])),
            ],
            Some((Type::map(Type::map(Type::string())), vec![true, true])),
        ),
        (
            // nested tuples and lists can unify along with the surrounding
            // objects and maps
            vec![
                Type::object([
                    ("a", Type::object([("a", Type::list(Type::string()))])),
                    (
                        "b",
                        Type::object([
                            ("a", Type::tuple([Type::string()])),
                            ("b", Type::list(Type::string())),
                        ]),
                    ),
                ]),
                Type::map(Type::object([
                    ("a", Type::list(Type::string())),
                    ("b", Type::list(Type::string())),
                ])),
            ],
            Some((
                Type::map(Type::map(Type::list(Type::string()))),
                vec![true, true],
            )),
        ),
        (
            // objects can unify as maps containing objects when all attributes
            // match
            vec![
                Type::object([
                    ("a", Type::object([("a", Type::string())])),
                    ("b", Type::object([("a", Type::string())])),
                ]),
                Type::map(Type::object([("a", Type::string())])),
            ],
            Some((
                Type::map(Type::object([("a", Type::string())])),
                vec![true, false],
            )),
        ),
        (
            // objects can unify as maps with dynamic types
            vec![
                Type::object([
                    ("a", Type::object([("a", Type::string())])),
                    ("b", Type::object([("a", Type::string())])),
                ]),
                Type::map(Type::dynamic()),
                Type::map(Type::object([("a", Type::string())])),
            ],
            Some((Type::map(Type::dynamic()), vec![true, false, true])),
        ),
        (
            // deeply nested objects and maps can unify
            vec![
                Type::object([
                    (
                        "a",
                        Type::object([("a", Type::object([("a", Type::string())]))]),
                    ),
                    (
                        "b",
                        Type::object([("c", Type::object([("d", Type::string())]))]),
                    ),
                ]),
                Type::map(Type::map(Type::map(Type::string()))),
            ],
            Some((
                Type::map(Type::map(Type::map(Type::string()))),
                vec![true, false],
            )),
        ),
        (
            // deeply nested objects with maps can unify as maps
            vec![
                Type::map(Type::map(Type::map(Type::string()))),
                Type::object([
                    (
                        "a",
                        Type::object([
                            ("a", Type::object([("a", Type::string())])),
                            ("b", Type::map(Type::string())),
                        ]),
                    ),
                    ("b", Type::map(Type::map(Type::string()))),
                ]),
            ],
            Some((
                Type::map(Type::map(Type::map(Type::string()))),
                vec![false, true],
            )),
        ),
        (
            vec![Type::dynamic(), Type::tuple([Type::number()])],
            Some((Type::dynamic(), vec![true, true])),
        ),
        (
            vec![Type::dynamic(), Type::object([("num", Type::number())])],
            Some((Type::dynamic(), vec![true, true])),
        ),
        (
            vec![
                Type::tuple([Type::number()]),
                Type::dynamic(),
                Type::object([("num", Type::number())]),
            ],
            None,
        ),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got = convert::unify(input);
        match (got, want) {
            (None, None) => {
                // Success!
            }
            (Some((got_type, _)), None) => {
                panic!("case {i} ({input:?}): got unified type {got_type:?}; want None");
            }
            (None, Some((want_type, want_convs))) => {
                panic!(
                    "case {i} ({input:?}): got None; want type {want_type:?} with conversions {want_convs:?}"
                );
            }
            (Some((got_type, got_convs)), Some((want_type, want_convs))) => {
                assert!(
                    want_type.equals(&got_type),
                    "case {i} ({input:?}): wrong result type\ngot:  {got_type:?}\nwant: {want_type:?}"
                );

                let got_convs_bool: Vec<bool> = got_convs.iter().map(|c| c.is_some()).collect();
                assert_eq!(
                    got_convs_bool, *want_convs,
                    "case {i} ({input:?}): wrong conversions"
                );
            }
        }
    }
}
