//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/unknown_refinement_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Type, Value};

/// The expected outcome of building a refined value: either a value (compared
/// with RawEquals semantics, i.e. `==`) or a panic with the given message.
enum Expect {
    Value(fn() -> Value),
    Panic(&'static str),
}

/// Extracts the string message from a caught panic payload, mirroring the
/// upstream test's comparison of the recovered panic value against a string.
fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        String::from("<non-string panic payload>")
    }
}

/// One upstream case: name, builder, and expected outcome.
type Case = (&'static str, fn() -> Value, Expect);

// Ported from TestValueRefine:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/unknown_refinement_test.go#L9
#[test]
fn value_refine() {
    let tests: Vec<Case> = vec![
        (
            "DynamicVal silently ignores all refinements",
            || {
                // This particular value, unlike any other value, will just
                // accept whatever refinements that are thrown at it and
                // completely ignore all of them and just continue being
                // itself.
                // This is a compromise for backward-compatiblity because
                // existing codebases expect cty.DynamicVal itself to be
                // the only value that is an unknown value of an unknown
                // type, aside from the possibility of marks.
                Value::dynamic()
                    .refine()
                    .not_null()
                    .string_prefix("beep")
                    .number_range_inclusive(Value::zero(), Value::number_int(10))
                    .collection_length(5)
                    .new_value()
            },
            Expect::Value(Value::dynamic),
        ),
        (
            "untyped null can be refined as being null",
            || Value::null(Type::dynamic()).refine().null().new_value(),
            Expect::Value(|| Value::null(Type::dynamic())),
        ),
        (
            "untyped null cannot be refined as being non-null",
            || Value::null(Type::dynamic()).refine_not_null(),
            Expect::Panic("refining null value as non-null"),
        ),
        (
            "unknown object can be refined non-null",
            || Value::unknown(Type::empty_object()).refine_not_null(),
            Expect::Value(|| Value::unknown(Type::empty_object()).refine_not_null()),
        ),
        (
            "unknown tuple can be refined non-null",
            || Value::unknown(Type::empty_tuple()).refine_not_null(),
            Expect::Value(|| Value::unknown(Type::empty_tuple()).refine_not_null()),
        ),
        (
            "unknown list can be refined non-null",
            || Value::unknown(Type::list(Type::string())).refine_not_null(),
            Expect::Value(|| Value::unknown(Type::list(Type::string())).refine_not_null()),
        ),
        (
            "unknown map can be refined non-null",
            || Value::unknown(Type::map(Type::string())).refine_not_null(),
            Expect::Value(|| Value::unknown(Type::map(Type::string())).refine_not_null()),
        ),
        (
            "unknown set can be refined non-null",
            || Value::unknown(Type::set(Type::string())).refine_not_null(),
            Expect::Value(|| Value::unknown(Type::set(Type::string())).refine_not_null()),
        ),
        (
            "unknown string can be refined non-null",
            || Value::unknown(Type::string()).refine_not_null(),
            Expect::Value(|| Value::unknown(Type::string()).refine_not_null()),
        ),
        (
            "unknown number can be refined non-null",
            || Value::unknown(Type::number()).refine_not_null(),
            Expect::Value(|| Value::unknown(Type::number()).refine_not_null()),
        ),
        (
            "unknown bool can be refined non-null",
            || Value::unknown(Type::bool()).refine_not_null(),
            Expect::Value(|| Value::unknown(Type::bool()).refine_not_null()),
        ),
        (
            "known null value can have its nullness confirmed",
            || Value::null(Type::bool()).refine().null().new_value(),
            Expect::Value(|| Value::null(Type::bool())),
        ),
        (
            "known null value cannot be refined as not null",
            || Value::null(Type::bool()).refine_not_null(),
            Expect::Panic("refining null value as non-null"),
        ),
        // String refinements
        (
            "unknown string can be refined with a prefix",
            || {
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix("foo-")
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("foo-")
                    .new_value()
            }),
        ),
        (
            "string prefix gets truncated if it might combine (latin diacritics)",
            || {
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix("foo")
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("fo")
                    .new_value()
            }),
        ),
        (
            "string prefix gets truncated if it might combine (emoji sequences)",
            || {
                Value::unknown(Type::string())
                    .refine()
                    // Can combine with "clouds" to produce "face in clouds"
                    .string_prefix("a\u{1f636}")
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("a")
                    .new_value()
            }),
        ),
        (
            "string prefix forced despite possibility of combining",
            || {
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("foo")
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("foo")
                    .new_value()
            }),
        ),
        (
            "a string prefix can be extended",
            || {
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("foo-")
                    .string_prefix_full("foo-bar-")
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("foo-bar-")
                    .new_value()
            }),
        ),
        (
            "cannot provide a string prefix that conflicts with existing refinement",
            || {
                Value::unknown(Type::string())
                    .refine()
                    .string_prefix_full("foo-")
                    .string_prefix_full("bar-")
                    .new_value()
            },
            Expect::Panic("refined prefix is inconsistent with previous refined prefix"),
        ),
        (
            "a known string can have its prefix confirmed",
            || {
                Value::string("foo-baz")
                    .refine()
                    .string_prefix_full("foo-")
                    .new_value()
            },
            Expect::Value(|| Value::string("foo-baz")),
        ),
        (
            "a known string does not accept a conflicting prefix",
            || {
                Value::string("foo-baz")
                    .refine()
                    .string_prefix_full("bar-")
                    .new_value()
            },
            Expect::Panic("refined prefix is inconsistent with known value"),
        ),
        (
            "non-string values cannot be refined with string prefix",
            || {
                Value::unknown(Type::number())
                    .refine()
                    .string_prefix_full("foo")
                    .new_value()
            },
            Expect::Panic("cannot refine string prefix for a cty.Number value"),
        ),
        // Number refinements
        (
            "unknown number can have refined lower bound",
            || {
                Value::unknown(Type::number())
                    .refine()
                    .number_range_lower_bound(Value::number_int(1), true)
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::number())
                    .refine()
                    .number_range_lower_bound(Value::number_int(1), true)
                    .new_value()
            }),
        ),
        (
            "unknown number can have refined upper bound",
            || {
                Value::unknown(Type::number())
                    .refine()
                    .number_range_upper_bound(Value::number_int(1), true)
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::number())
                    .refine()
                    .number_range_upper_bound(Value::number_int(1), true)
                    .new_value()
            }),
        ),
        (
            "unknown number can have refined both bounds",
            || {
                Value::unknown(Type::number())
                    .refine()
                    .number_range_lower_bound(Value::number_int(1), true)
                    .number_range_upper_bound(Value::number_int(2), false)
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::number())
                    .refine()
                    .number_range_lower_bound(Value::number_int(1), true)
                    .number_range_upper_bound(Value::number_int(2), false)
                    .new_value()
            }),
        ),
        (
            "refining unknown non-null number with equal upper and lower bound produces known number",
            || {
                Value::unknown(Type::number())
                    .refine()
                    .number_range_lower_bound(Value::number_int(1), true)
                    .number_range_upper_bound(Value::number_int(1), true)
                    .not_null()
                    .new_value()
            },
            Expect::Value(|| Value::number_int(1)),
        ),
        (
            "unknown number cannot have conflicting bounds",
            || {
                Value::unknown(Type::number())
                    .refine()
                    .number_range_lower_bound(Value::number_int(2), true)
                    .number_range_upper_bound(Value::number_int(1), false)
                    .new_value()
            },
            Expect::Panic(
                "number lower bound cty.NumberIntVal(2) is greater than upper bound cty.NumberIntVal(1)",
            ),
        ),
        (
            "known number can have its bounds confirmed",
            || {
                Value::number_int(1)
                    .refine()
                    .number_range_lower_bound(Value::number_int(0), true)
                    .number_range_upper_bound(Value::number_int(2), true)
                    .not_null()
                    .new_value()
            },
            Expect::Value(|| Value::number_int(1)),
        ),
        (
            "can't refine a known number with non-matching bounds",
            || {
                Value::number_int(10)
                    .refine()
                    .number_range_lower_bound(Value::number_int(0), true)
                    .number_range_upper_bound(Value::number_int(2), true)
                    .not_null()
                    .new_value()
            },
            Expect::Panic("refining cty.NumberIntVal(10) to be <= cty.NumberIntVal(2)"),
        ),
        // List length refinements
        (
            "unknown list can be refined with length lower bound",
            || {
                Value::unknown(Type::list(Type::string()))
                    .refine()
                    .collection_length_lower_bound(1)
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::list(Type::string()))
                    .refine()
                    .collection_length_lower_bound(1)
                    .new_value()
            }),
        ),
        (
            "unknown list can be refined with length upper bound",
            || {
                Value::unknown(Type::list(Type::string()))
                    .refine()
                    .collection_length_upper_bound(1)
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::list(Type::string()))
                    .refine()
                    .collection_length_upper_bound(1)
                    .new_value()
            }),
        ),
        (
            "unknown list can be refined with length bounds",
            || {
                Value::unknown(Type::list(Type::string()))
                    .refine()
                    .collection_length_lower_bound(1)
                    .collection_length_upper_bound(3)
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::list(Type::string()))
                    .refine()
                    .collection_length_lower_bound(1)
                    .collection_length_upper_bound(3)
                    .new_value()
            }),
        ),
        (
            "unknown non-null list with known length becomes known list of unknowns",
            || {
                Value::unknown(Type::list(Type::string()))
                    .refine()
                    .not_null()
                    .collection_length(2)
                    .new_value()
            },
            Expect::Value(|| {
                Value::list([
                    Value::unknown(Type::string()),
                    Value::unknown(Type::string()),
                ])
            }),
        ),
        (
            "unknown non-null list with known zero length becomes known list",
            || {
                Value::unknown(Type::list(Type::string()))
                    .refine()
                    .not_null()
                    .collection_length(0)
                    .new_value()
            },
            Expect::Value(|| Value::list_empty(Type::string())),
        ),
        (
            "known list can have its length confirmed with a refinement",
            || {
                Value::list_empty(Type::string())
                    .refine()
                    .collection_length(0)
                    .new_value()
            },
            Expect::Value(|| Value::list_empty(Type::string())),
        ),
        (
            "cannot refine known list with conflicting length bounds",
            || {
                Value::list_empty(Type::string())
                    .refine()
                    .collection_length(1)
                    .new_value()
            },
            Expect::Panic("refining collection of length cty.NumberIntVal(0) with lower bound 1"),
        ),
        // Map length refinements
        (
            "unknown map can be refined with length lower bound",
            || {
                Value::unknown(Type::map(Type::string()))
                    .refine()
                    .collection_length_lower_bound(1)
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::map(Type::string()))
                    .refine()
                    .collection_length_lower_bound(1)
                    .new_value()
            }),
        ),
        (
            "unknown map can be refined with length upper bound",
            || {
                Value::unknown(Type::map(Type::string()))
                    .refine()
                    .collection_length_upper_bound(1)
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::map(Type::string()))
                    .refine()
                    .collection_length_upper_bound(1)
                    .new_value()
            }),
        ),
        (
            "unknown map can be refined with length bounds",
            || {
                Value::unknown(Type::map(Type::string()))
                    .refine()
                    .collection_length_lower_bound(1)
                    .collection_length_upper_bound(3)
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::map(Type::string()))
                    .refine()
                    .collection_length_lower_bound(1)
                    .collection_length_upper_bound(3)
                    .new_value()
            }),
        ),
        (
            "unknown map can be refined with known length",
            || {
                Value::unknown(Type::map(Type::string()))
                    .refine()
                    .not_null()
                    .collection_length(2)
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::map(Type::string()))
                    .refine()
                    .not_null()
                    .collection_length(2)
                    .new_value()
            }),
        ),
        (
            "unknown non-null map with known zero length becomes known map",
            || {
                Value::unknown(Type::map(Type::string()))
                    .refine()
                    .not_null()
                    .collection_length(0)
                    .new_value()
            },
            Expect::Value(|| Value::map_empty(Type::string())),
        ),
        (
            "known map can have its length confirmed with a refinement",
            || {
                Value::map_empty(Type::string())
                    .refine()
                    .collection_length(0)
                    .new_value()
            },
            Expect::Value(|| Value::map_empty(Type::string())),
        ),
        (
            "cannot refine known map with conflicting length bounds",
            || {
                Value::map_empty(Type::string())
                    .refine()
                    .collection_length(1)
                    .new_value()
            },
            Expect::Panic("refining collection of length cty.NumberIntVal(0) with lower bound 1"),
        ),
        // Set length refinements
        (
            "unknown set can be refined with length lower bound",
            || {
                Value::unknown(Type::set(Type::string()))
                    .refine()
                    .collection_length_lower_bound(1)
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::set(Type::string()))
                    .refine()
                    .collection_length_lower_bound(1)
                    .new_value()
            }),
        ),
        (
            "unknown set can be refined with length upper bound",
            || {
                Value::unknown(Type::set(Type::string()))
                    .refine()
                    .collection_length_upper_bound(1)
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::set(Type::string()))
                    .refine()
                    .collection_length_upper_bound(1)
                    .new_value()
            }),
        ),
        (
            "unknown set can be refined with length bounds",
            || {
                Value::unknown(Type::set(Type::string()))
                    .refine()
                    .collection_length_lower_bound(1)
                    .collection_length_upper_bound(3)
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::set(Type::string()))
                    .refine()
                    .collection_length_lower_bound(1)
                    .collection_length_upper_bound(3)
                    .new_value()
            }),
        ),
        (
            "unknown set can be refined with known length",
            || {
                Value::unknown(Type::set(Type::string()))
                    .refine()
                    .not_null()
                    .collection_length(2)
                    .new_value()
            },
            Expect::Value(|| {
                Value::unknown(Type::set(Type::string()))
                    .refine()
                    .not_null()
                    .collection_length(2)
                    .new_value()
            }),
        ),
        (
            "unknown non-null set with known zero length becomes known empty set",
            || {
                Value::unknown(Type::set(Type::string()))
                    .refine()
                    .not_null()
                    .collection_length(0)
                    .new_value()
            },
            Expect::Value(|| Value::set_empty(Type::string())),
        ),
        (
            "known set can have its length confirmed with a refinement",
            || {
                Value::set_empty(Type::string())
                    .refine()
                    .collection_length(0)
                    .new_value()
            },
            Expect::Value(|| Value::set_empty(Type::string())),
        ),
        (
            "cannot refine known set with conflicting length bounds",
            || {
                Value::set_empty(Type::string())
                    .refine()
                    .collection_length(1)
                    .new_value()
            },
            Expect::Panic("refining collection of length cty.NumberIntVal(0) with lower bound 1"),
        ),
    ];

    for (i, (name, build, expect)) in tests.iter().enumerate() {
        let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(build));
        match expect {
            Expect::Value(want) => {
                let got = match got {
                    Ok(v) => v,
                    Err(payload) => panic!(
                        "case {i} ({name}): unexpected panic: {}",
                        panic_message(payload)
                    ),
                };
                let want = want();
                assert_eq!(
                    got, want,
                    "case {i} ({name}): wrong result\ngot:  {got:?}\nwant: {want:?}"
                );
            }
            Expect::Panic(want_panic) => {
                let payload = match got {
                    Ok(v) => panic!(
                        "case {i} ({name}): unexpected success\nresult: {v:?}\nwant panic: {want_panic:?}"
                    ),
                    Err(payload) => payload,
                };
                let msg = panic_message(payload);
                assert_eq!(
                    msg, *want_panic,
                    "case {i} ({name}): wrong panic value\ngot:  {msg:?}\nwant: {want_panic:?}"
                );
            }
        }
    }
}
