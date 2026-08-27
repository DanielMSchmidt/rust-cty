//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/function/function_test.go
//!   cty/function/unpredictable_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::function::{Function, ImplFunc, Parameter, Spec, TypeFunc, static_return_type};
use cty::{Error, Type, Value, ValueMarks};

// NOTE(port): upstream's stubType returns `cty.NilType, fmt.Errorf("should not
// be called")`; NilType has no Rust analogue — the error alone carries the
// "not called" signal through `Result`.
fn stub_type() -> TypeFunc {
    Box::new(|_args| Err(Error::new("should not be called")))
}

// NOTE(port): upstream's stubImpl returns `cty.NilVal, fmt.Errorf("should not
// be called")`; NilVal has no Rust analogue — the error alone carries the
// "not called" signal through `Result`.
fn stub_impl() -> ImplFunc {
    Box::new(|_args, _ret_type| Err(Error::new("should not be called")))
}

// Ported from TestReturnTypeForValues:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/function_test.go#L10
#[test]
#[ignore = "not yet implemented"]
fn return_type_for_values() {
    struct TestCase {
        spec: Spec,
        args: Vec<Value>,
        want_type: Option<Type>,
        want_err: bool,
    }

    // A `Spec.Type` function that rejects marked arguments, used by the last
    // two cases (upstream declares it inline per case).
    fn marks_rejecting_type_fn() -> TypeFunc {
        Box::new(|args| {
            let ty = Type::number();
            for (i, arg) in args.iter().enumerate() {
                if arg.contains_marked() {
                    return Err(Error::new(format!(
                        "arg {i} {} contains marks",
                        arg.go_string()
                    )));
                }
            }
            Ok(ty)
        })
    }

    let tests: Vec<TestCase> = vec![
        TestCase {
            spec: Spec {
                description: String::new(),
                params: vec![],
                var_param: None,
                type_fn: static_return_type(Type::number()),
                refine_result: None,
                impl_fn: stub_impl(),
            },
            args: vec![],
            want_type: Some(Type::number()),
            want_err: false,
        },
        TestCase {
            spec: Spec {
                description: String::new(),
                params: vec![],
                var_param: None,
                type_fn: static_return_type(Type::number()),
                refine_result: None,
                impl_fn: stub_impl(),
            },
            args: vec![Value::number_int(2)],
            want_type: None,
            want_err: true,
        },
        TestCase {
            spec: Spec {
                description: String::new(),
                params: vec![],
                var_param: None,
                type_fn: static_return_type(Type::number()),
                refine_result: None,
                impl_fn: stub_impl(),
            },
            args: vec![Value::unknown(Type::number())],
            want_type: None,
            want_err: true,
        },
        TestCase {
            spec: Spec {
                description: String::new(),
                params: vec![Parameter {
                    ty: Some(Type::number()),
                    ..Default::default()
                }],
                var_param: None,
                type_fn: static_return_type(Type::number()),
                refine_result: None,
                impl_fn: stub_impl(),
            },
            args: vec![Value::number_int(2)],
            want_type: Some(Type::number()),
            want_err: false,
        },
        TestCase {
            spec: Spec {
                description: String::new(),
                params: vec![Parameter {
                    ty: Some(Type::number()),
                    ..Default::default()
                }],
                var_param: None,
                type_fn: static_return_type(Type::number()),
                refine_result: None,
                impl_fn: stub_impl(),
            },
            args: vec![Value::unknown(Type::number())],
            want_type: Some(Type::number()),
            want_err: false,
        },
        TestCase {
            spec: Spec {
                description: String::new(),
                params: vec![Parameter {
                    ty: Some(Type::number()),
                    ..Default::default()
                }],
                var_param: None,
                type_fn: static_return_type(Type::number()),
                refine_result: None,
                impl_fn: stub_impl(),
            },
            args: vec![Value::dynamic()],
            want_type: Some(Type::dynamic()),
            want_err: false,
        },
        TestCase {
            spec: Spec {
                description: String::new(),
                params: vec![Parameter {
                    ty: Some(Type::number()),
                    allow_dynamic_type: true,
                    ..Default::default()
                }],
                var_param: None,
                type_fn: static_return_type(Type::number()),
                refine_result: None,
                impl_fn: stub_impl(),
            },
            args: vec![Value::dynamic()],
            want_type: Some(Type::number()),
            want_err: false,
        },
        TestCase {
            spec: Spec {
                description: String::new(),
                params: vec![Parameter {
                    ty: Some(Type::number()),
                    allow_dynamic_type: true,
                    ..Default::default()
                }],
                var_param: None,
                type_fn: static_return_type(Type::number()),
                refine_result: None,
                impl_fn: stub_impl(),
            },
            args: vec![Value::unknown(Type::string())],
            want_type: None,
            want_err: true,
        },
        TestCase {
            spec: Spec {
                description: String::new(),
                params: vec![Parameter {
                    ty: Some(Type::number()),
                    allow_dynamic_type: true,
                    ..Default::default()
                }],
                var_param: None,
                type_fn: static_return_type(Type::number()),
                refine_result: None,
                impl_fn: stub_impl(),
            },
            args: vec![Value::string("hello")],
            want_type: None,
            want_err: true,
        },
        TestCase {
            spec: Spec {
                description: String::new(),
                params: vec![Parameter {
                    ty: Some(Type::list(Type::dynamic())),
                    ..Default::default()
                }],
                var_param: None,
                type_fn: marks_rejecting_type_fn(),
                refine_result: None,
                impl_fn: stub_impl(),
            },
            args: vec![Value::list([Value::string("ok").mark("marked")])],
            want_type: Some(Type::number()),
            want_err: false,
        },
        TestCase {
            spec: Spec {
                description: String::new(),
                params: vec![Parameter {
                    ty: Some(Type::list(Type::string())),
                    ..Default::default()
                }],
                var_param: Some(Parameter {
                    ty: Some(Type::list(Type::string())),
                    ..Default::default()
                }),
                type_fn: marks_rejecting_type_fn(),
                refine_result: None,
                impl_fn: stub_impl(),
            },
            args: vec![
                Value::list([Value::string("one")]),
                Value::list([Value::string("two").mark("marked")]),
            ],
            want_type: Some(Type::number()),
            want_err: false,
        },
    ];

    for (i, test) in tests.into_iter().enumerate() {
        let f = Function::new(test.spec);
        let got = f.return_type_for_values(&test.args);

        if test.want_err {
            assert!(
                got.is_err(),
                "case {i}: succeeded with {:?}; want error",
                got.as_ref().ok()
            );
        } else {
            let got_type =
                got.unwrap_or_else(|err| panic!("case {i}: unexpected error\nerr: {err}"));
            // NOTE(port): upstream's `gotType == cty.NilType` validity check
            // has no Rust analogue — a returned `Type` is always valid.
            let want_type = test.want_type.as_ref().unwrap();
            assert!(
                got_type.equals(want_type),
                "case {i}: wrong return type\ngot:  {got_type:?}\nwant: {want_type:?}"
            );
        }
    }
}

// Ported from TestFunctionWithNewDescriptions ("no params"):
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/function_test.go#L209
#[test]
#[ignore = "not yet implemented"]
fn function_with_new_descriptions_no_params() {
    let f1 = Function::new(Spec {
        description: "old func".to_string(),
        params: vec![],
        var_param: None,
        type_fn: stub_type(),
        refine_result: None,
        impl_fn: stub_impl(),
    });
    let f2 = f1.with_new_descriptions("new func", &[]);

    assert_eq!(
        f1.description(),
        "old func",
        "wrong original func description"
    );
    assert_eq!(
        f2.description(),
        "new func",
        "wrong updated func description"
    );
}

// Ported from TestFunctionWithNewDescriptions ("one pos param"):
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/function_test.go#L209
#[test]
#[ignore = "not yet implemented"]
fn function_with_new_descriptions_one_pos_param() {
    let f1 = Function::new(Spec {
        description: "old func".to_string(),
        params: vec![Parameter {
            name: "a".to_string(),
            description: "old a".to_string(),
            ..Default::default()
        }],
        var_param: None,
        type_fn: stub_type(),
        refine_result: None,
        impl_fn: stub_impl(),
    });
    let f2 = f1.with_new_descriptions("new func", &["new a"]);

    assert_eq!(
        f1.description(),
        "old func",
        "wrong original func description"
    );
    assert_eq!(
        f2.description(),
        "new func",
        "wrong updated func description"
    );

    assert_eq!(f1.params().len(), 1, "wrong original param count");
    assert_eq!(f2.params().len(), 1, "wrong updated param count");
    assert_eq!(
        f1.params()[0].description,
        "old a",
        "wrong original param a description"
    );
    assert_eq!(
        f2.params()[0].description,
        "new a",
        "wrong updated param a description"
    );
}

// Ported from TestFunctionWithNewDescriptions ("two pos params"):
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/function_test.go#L209
#[test]
#[ignore = "not yet implemented"]
fn function_with_new_descriptions_two_pos_params() {
    let f1 = Function::new(Spec {
        description: "old func".to_string(),
        params: vec![
            Parameter {
                name: "a".to_string(),
                description: "old a".to_string(),
                ..Default::default()
            },
            Parameter {
                name: "b".to_string(),
                description: "old b".to_string(),
                ..Default::default()
            },
        ],
        var_param: None,
        type_fn: stub_type(),
        refine_result: None,
        impl_fn: stub_impl(),
    });
    let f2 = f1.with_new_descriptions("new func", &["new a", "new b"]);

    assert_eq!(
        f1.description(),
        "old func",
        "wrong original func description"
    );
    assert_eq!(
        f2.description(),
        "new func",
        "wrong updated func description"
    );

    assert_eq!(f1.params().len(), 2, "wrong original param count");
    assert_eq!(f2.params().len(), 2, "wrong updated param count");
    assert_eq!(
        f1.params()[0].description,
        "old a",
        "wrong original param a description"
    );
    assert_eq!(
        f2.params()[0].description,
        "new a",
        "wrong updated param a description"
    );
    assert_eq!(
        f1.params()[1].description,
        "old b",
        "wrong original param b description"
    );
    assert_eq!(
        f2.params()[1].description,
        "new b",
        "wrong updated param b description"
    );
}

// Ported from TestFunctionWithNewDescriptions ("varparam overridden"):
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/function_test.go#L209
#[test]
#[ignore = "not yet implemented"]
fn function_with_new_descriptions_varparam_overridden() {
    let f1 = Function::new(Spec {
        description: "old func".to_string(),
        params: vec![Parameter {
            name: "a".to_string(),
            description: "old a".to_string(),
            ..Default::default()
        }],
        var_param: Some(Parameter {
            name: "b".to_string(),
            description: "old b".to_string(),
            ..Default::default()
        }),
        type_fn: stub_type(),
        refine_result: None,
        impl_fn: stub_impl(),
    });
    let f2 = f1.with_new_descriptions("new func", &["new a", "new b"]);

    assert_eq!(
        f1.description(),
        "old func",
        "wrong original func description"
    );
    assert_eq!(
        f2.description(),
        "new func",
        "wrong updated func description"
    );

    assert_eq!(f1.params().len(), 1, "wrong original param count");
    assert_eq!(f2.params().len(), 1, "wrong updated param count");
    assert_eq!(
        f1.params()[0].description,
        "old a",
        "wrong original param a description"
    );
    assert_eq!(
        f2.params()[0].description,
        "new a",
        "wrong updated param a description"
    );
    assert_eq!(
        f1.var_param().unwrap().description,
        "old b",
        "wrong original param b description"
    );
    assert_eq!(
        f2.var_param().unwrap().description,
        "new b",
        "wrong updated param b description"
    );
}

// Ported from TestFunctionWithNewDescriptions ("varparam not overridden"):
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/function_test.go#L209
#[test]
#[ignore = "not yet implemented"]
fn function_with_new_descriptions_varparam_not_overridden() {
    let f1 = Function::new(Spec {
        description: "old func".to_string(),
        params: vec![Parameter {
            name: "a".to_string(),
            description: "old a".to_string(),
            ..Default::default()
        }],
        var_param: Some(Parameter {
            name: "b".to_string(),
            description: "old b".to_string(),
            ..Default::default()
        }),
        type_fn: stub_type(),
        refine_result: None,
        impl_fn: stub_impl(),
    });
    let f2 = f1.with_new_descriptions("new func", &["new a"]);

    assert_eq!(
        f1.description(),
        "old func",
        "wrong original func description"
    );
    assert_eq!(
        f2.description(),
        "new func",
        "wrong updated func description"
    );

    assert_eq!(f1.params().len(), 1, "wrong original param count");
    assert_eq!(f2.params().len(), 1, "wrong updated param count");
    assert_eq!(
        f1.params()[0].description,
        "old a",
        "wrong original param a description"
    );
    assert_eq!(
        f2.params()[0].description,
        "new a",
        "wrong updated param a description"
    );
    assert_eq!(
        f1.var_param().unwrap().description,
        "old b",
        "wrong original param b description"
    );
    // This is the one case where we allow the caller to leave one of
    // the param descriptions unchanged, because we want to allow
    // a function to grow a variadic parameter later without it being
    // a breaking change for existing callers that might be overriding
    // descriptions.
    assert_eq!(
        f2.var_param().unwrap().description,
        "old b",
        "wrong updated param b description"
    );
}

// Ported from TestFunctionWithNewDescriptions ("solo varparam overridden"):
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/function_test.go#L209
#[test]
#[ignore = "not yet implemented"]
fn function_with_new_descriptions_solo_varparam_overridden() {
    let f1 = Function::new(Spec {
        description: "old func".to_string(),
        params: vec![],
        var_param: Some(Parameter {
            name: "a".to_string(),
            description: "old a".to_string(),
            ..Default::default()
        }),
        type_fn: stub_type(),
        refine_result: None,
        impl_fn: stub_impl(),
    });
    let f2 = f1.with_new_descriptions("new func", &["new a"]);

    assert_eq!(
        f1.description(),
        "old func",
        "wrong original func description"
    );
    assert_eq!(
        f2.description(),
        "new func",
        "wrong updated func description"
    );

    assert_eq!(f1.params().len(), 0, "wrong original param count");
    assert_eq!(f2.params().len(), 0, "wrong updated param count");
    assert_eq!(
        f1.var_param().unwrap().description,
        "old a",
        "wrong original param b description"
    );
    assert_eq!(
        f2.var_param().unwrap().description,
        "new a",
        "wrong updated param b description"
    );
}

// Ported from TestFunctionWithNewDescriptions ("solo varparam not overridden"):
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/function_test.go#L209
#[test]
#[ignore = "not yet implemented"]
fn function_with_new_descriptions_solo_varparam_not_overridden() {
    let f1 = Function::new(Spec {
        description: "old func".to_string(),
        params: vec![],
        var_param: Some(Parameter {
            name: "a".to_string(),
            description: "old a".to_string(),
            ..Default::default()
        }),
        type_fn: stub_type(),
        refine_result: None,
        impl_fn: stub_impl(),
    });
    let f2 = f1.with_new_descriptions("new func", &[]);

    assert_eq!(
        f1.description(),
        "old func",
        "wrong original func description"
    );
    assert_eq!(
        f2.description(),
        "new func",
        "wrong updated func description"
    );

    assert_eq!(f1.params().len(), 0, "wrong original param count");
    assert_eq!(f2.params().len(), 0, "wrong updated param count");
    assert_eq!(
        f1.var_param().unwrap().description,
        "old a",
        "wrong original param b description"
    );
    // This is the one case where we allow the caller to leave one of
    // the param descriptions unchanged, because we want to allow
    // a function to grow a variadic parameter later without it being
    // a breaking change for existing callers that might be overriding
    // descriptions.
    assert_eq!(
        f2.var_param().unwrap().description,
        "old a",
        "wrong updated param b description"
    );
}

// Ported from TestFunctionCallWithUnknownVals ("params"):
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/function_test.go#L497
#[test]
#[ignore = "not yet implemented"]
fn function_call_with_unknown_vals_params() {
    let f = Function::new(Spec {
        description: String::new(),
        params: vec![
            Parameter {
                name: "foo".to_string(),
                ty: Some(Type::string()),
                ..Default::default()
            },
            Parameter {
                name: "bar".to_string(),
                ty: Some(Type::string()),
                ..Default::default()
            },
        ],
        var_param: None,
        type_fn: static_return_type(Type::string()),
        refine_result: None,
        impl_fn: stub_impl(),
    });
    let marks = ValueMarks::from_marks(["special", "extra"]);
    let unknown_with_marks = Value::unknown(Type::string()).with_marks([marks.clone()]);
    let known_with_marks = Value::string("ok").with_marks([marks.clone()]);
    let got = f
        .call(&[unknown_with_marks, known_with_marks])
        .unwrap_or_else(|err| panic!("{err}"));
    assert_eq!(
        got.marks(),
        marks,
        "unexpected marks\ngot:  {:?}\nwant: {:?}",
        got.marks(),
        marks
    );
}

// Ported from TestFunctionCallWithUnknownVals ("params-partial-marks"):
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/function_test.go#L497
#[test]
#[ignore = "not yet implemented"]
fn function_call_with_unknown_vals_params_partial_marks() {
    let f = Function::new(Spec {
        description: String::new(),
        params: vec![
            Parameter {
                name: "foo".to_string(),
                ty: Some(Type::string()),
                ..Default::default()
            },
            Parameter {
                name: "bar".to_string(),
                ty: Some(Type::string()),
                // AllowMarked means we can't include this value's marks in
                // the early return unknown value.
                allow_marked: true,
                ..Default::default()
            },
        ],
        var_param: None,
        type_fn: static_return_type(Type::string()),
        refine_result: None,
        impl_fn: stub_impl(),
    });
    let marks = ValueMarks::from_marks(["special"]);
    let unknown_with_marks = Value::unknown(Type::string()).with_marks([marks.clone()]);
    let known_with_marks = Value::string("ok").mark("allow_marked");
    let got = f
        .call(&[unknown_with_marks, known_with_marks])
        .unwrap_or_else(|err| panic!("{err}"));
    assert_eq!(
        got.marks(),
        marks,
        "unexpected marks\ngot:  {:?}\nwant: {:?}",
        got.marks(),
        marks
    );
}

// Ported from TestFunctionCallWithUnknownVals ("varparam"):
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/function_test.go#L497
#[test]
#[ignore = "not yet implemented"]
fn function_call_with_unknown_vals_varparam() {
    let f = Function::new(Spec {
        description: String::new(),
        params: vec![],
        var_param: Some(Parameter {
            name: "foo".to_string(),
            ty: Some(Type::string()),
            ..Default::default()
        }),
        type_fn: static_return_type(Type::string()),
        refine_result: None,
        impl_fn: stub_impl(),
    });
    let marks = ValueMarks::from_marks(["special", "extra"]);
    let unknown_with_marks = Value::unknown(Type::string()).with_marks([marks.clone()]);
    let got = f
        .call(&[unknown_with_marks])
        .unwrap_or_else(|err| panic!("{err}"));
    assert_eq!(
        got.marks(),
        marks,
        "unexpected marks\ngot:  {:?}\nwant: {:?}",
        got.marks(),
        marks
    );
}

// Ported from TestFunctionCallWithUnknownVals ("refined-marked"):
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/function_test.go#L497
#[test]
#[ignore = "not yet implemented"]
fn function_call_with_unknown_vals_refined_marked() {
    let f = Function::new(Spec {
        description: String::new(),
        params: vec![
            Parameter {
                name: "first".to_string(),
                ty: Some(Type::string()),
                ..Default::default()
            },
            Parameter {
                name: "second".to_string(),
                ty: Some(Type::string()),
                allow_marked: true,
                allow_unknown: true,
                ..Default::default()
            },
        ],
        var_param: None,
        type_fn: static_return_type(Type::string()),
        refine_result: Some(Box::new(|b| b.not_null())),
        impl_fn: stub_impl(),
    });
    let got = f
        .call(&[
            Value::unknown(Type::string()).mark("first"),
            Value::unknown(Type::string()).mark("second"),
        ])
        .unwrap_or_else(|err| panic!("{err}"));
    // since the second parameter allows marked values, we should only
    // expect the fist mark when given unknown arguments.
    let expected = Value::unknown(Type::string())
        .refine_not_null()
        .mark("first");
    assert_eq!(got, expected, "expected {expected:?}\ngot: {got:?}");
}

// Ported from TestFunctionCallWithUnknownVals ("marked-dynamic-not-refined"):
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/function_test.go#L497
#[test]
#[ignore = "not yet implemented"]
fn function_call_with_unknown_vals_marked_dynamic_not_refined() {
    let f = Function::new(Spec {
        description: String::new(),
        params: vec![
            Parameter {
                name: "first".to_string(),
                ty: Some(Type::string()),
                ..Default::default()
            },
            Parameter {
                name: "second".to_string(),
                ty: Some(Type::string()),
                allow_marked: true,
                allow_unknown: true,
                ..Default::default()
            },
        ],
        var_param: None,
        type_fn: Box::new(|_args| {
            // this isn't called with known args, so a static dynamic type is OK
            Ok(Type::dynamic())
        }),
        refine_result: Some(Box::new(|b| b.not_null())),
        impl_fn: stub_impl(),
    });
    let got = f
        .call(&[
            Value::dynamic().mark("first"),
            Value::dynamic().mark("second"),
        ])
        .unwrap_or_else(|err| panic!("{err}"));
    // Since the second parameter allows marked values, we should only
    // expect the fist mark when given unknown arguments.
    // Because the type is unknown, the result should not be refined.
    let expected = Value::dynamic().mark("first");
    assert_eq!(got, expected, "expected {expected:?}\ngot: {got:?}");
}

// Ported from TestUnpredictable:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/unpredictable_test.go#L9
#[test]
#[ignore = "not yet implemented"]
fn unpredictable() {
    let f = Function::new(Spec {
        description: String::new(),
        params: vec![Parameter {
            name: "fixed".to_string(),
            ty: Some(Type::bool()),
            ..Default::default()
        }],
        var_param: Some(Parameter {
            name: "variadic".to_string(),
            ty: Some(Type::string()),
            ..Default::default()
        }),
        type_fn: Box::new(|args| {
            if args.len() == 1 {
                Ok(Type::bool())
            } else {
                Ok(Type::string())
            }
        }),
        refine_result: None,
        impl_fn: Box::new(|_args, ret_type| Ok(Value::null(ret_type.clone()))),
    });

    let uf = cty::function::unpredictable(f.clone());

    {
        let pred_val = f
            .call(&[Value::bool(true)])
            .unwrap_or_else(|err| panic!("{err}"));
        assert_eq!(
            pred_val,
            Value::null(Type::bool()),
            "wrong predictable result"
        );
    }

    // subtest "argument type error"
    {
        let result = uf.call(&[Value::string("hello")]);
        assert!(
            result.is_err(),
            "argument type error: call successful; want error"
        );
    }

    // subtest "type check 1"
    {
        let ty = uf
            .return_type_for_values(&[Value::bool(true)])
            .unwrap_or_else(|err| panic!("{err}"));
        assert!(
            ty.equals(&Type::bool()),
            "type check 1: wrong type {ty:?}; want {:?}",
            Type::bool()
        );
    }

    // subtest "type check 2"
    {
        let ty = uf
            .return_type_for_values(&[Value::bool(true), Value::string("hello")])
            .unwrap_or_else(|err| panic!("{err}"));
        assert!(
            ty.equals(&Type::string()),
            "type check 2: wrong type {ty:?}; want {:?}",
            Type::string()
        );
    }

    // subtest "call"
    {
        let v = uf
            .call(&[Value::bool(true)])
            .unwrap_or_else(|err| panic!("{err}"));
        assert_eq!(
            v,
            Value::unknown(Type::bool()),
            "wrong result {v:?}; want {:?}",
            Value::unknown(Type::bool())
        );
    }
}
