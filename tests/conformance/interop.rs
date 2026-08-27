//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/gocty/in_test.go
//!   cty/gocty/out_test.go
//!   cty/gocty/type_implied_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.
//!
//! gocty is reflection-based; the Rust analogue is the trait-based
//! `cty::interop` module, so upstream cases are translated per native type:
//! Go `int` → `i64`, `uint` → `u64`, `[]T` → `Vec<T>`, `map[string]T` →
//! `BTreeMap<String, T>`, `[N]T` → `[T; N]`, and pointers → `Option<T>`
//! (nil pointer ↔ `None` ↔ null). Cases relying on Go struct reflection with
//! `cty:"…"` tags, `interface{}` (`any`) values, or other reflection-only
//! features are kept as NOTE(port) comment blocks awaiting a derive macro.

use std::collections::BTreeMap;
use std::fmt::Debug;

use cty::interop::{self, CtyTyped, FromCty, IntoCty};
use cty::{Type, Value};

/// Upstream TestIn assertion body: `ToCtyValue` must succeed and the result
/// must `RawEquals` the wanted value (`==` on `Value` is RawEquals).
fn assert_in<T: IntoCty>(case: &str, go_value: T, ty: Type, want: Value) {
    let got = interop::to_cty_value(go_value, &ty)
        .unwrap_or_else(|err| panic!("{case}: ToCtyValue returned error: {err}"));
    assert_eq!(got, want, "{case}: wrong result");
}

/// Upstream TestOut assertion body: `FromCtyValue` must succeed and the
/// produced native value must equal the wanted one.
fn assert_out<T>(case: &str, value: Value, want: T)
where
    T: FromCty + PartialEq + Debug,
{
    let got: T = interop::from_cty_value(&value)
        .unwrap_or_else(|err| panic!("{case}: FromCtyValue returned error: {err}"));
    assert_eq!(got, want, "{case}: wrong result");
}

/// Upstream TestImpliedType assertion body: `ImpliedType` must succeed and
/// the result must equal (`Type.Equals`) the wanted type.
fn assert_implied<T: CtyTyped>(case: &str, want: Type) {
    let got = interop::implied_type::<T>()
        .unwrap_or_else(|err| panic!("{case}: unexpected error: {err}"));
    assert!(got.equals(&want), "{case}: got {got:?}, want {want:?}");
}

// Ported from TestIn:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/gocty/in_test.go#L13
#[test]
#[ignore = "not yet implemented"]
fn gocty_in() {
    // Bool
    assert_in("true into cty.Bool", true, Type::bool(), Value::bool(true));
    assert_in(
        "(*bool)(nil) into cty.Bool",
        None::<bool>,
        Type::bool(),
        Value::null(Type::bool()),
    );
    assert_in(
        "ptrToBool(true) into cty.Bool",
        Some(true),
        Type::bool(),
        Value::bool(true),
    );

    // String
    assert_in(
        r#""hello" into cty.String"#,
        "hello",
        Type::string(),
        Value::string("hello"),
    );
    assert_in(
        r#"ptrToString("hello") into cty.String"#,
        Some("hello"),
        Type::string(),
        Value::string("hello"),
    );
    assert_in(
        r#"ptrToPtrToString("hello") into cty.String"#,
        Some(Some("hello")),
        Type::string(),
        Value::string("hello"),
    );
    assert_in(
        "(*string)(nil) into cty.String",
        None::<String>,
        Type::string(),
        Value::null(Type::string()),
    );
    // NOTE(port): upstream converts an untyped Go `nil` (an empty `any`
    // interface with no dynamic type) into cty.String, producing
    // cty.NullVal(cty.String). Rust has no typeless nil value; the typed
    // nil-pointer cases around this one cover the None ↔ null behavior.
    // any nil is convertable to a null of any type:
    assert_in(
        "(*bool)(nil) into cty.String",
        None::<bool>,
        Type::string(),
        Value::null(Type::string()),
    );

    // Number
    assert_in(
        "int(1) into cty.Number",
        1i64,
        Type::number(),
        Value::number_int(1),
    );
    assert_in(
        "int8(1) into cty.Number",
        1i8,
        Type::number(),
        Value::number_int(1),
    );
    assert_in(
        "int16(1) into cty.Number",
        1i16,
        Type::number(),
        Value::number_int(1),
    );
    assert_in(
        "int32(1) into cty.Number",
        1i32,
        Type::number(),
        Value::number_int(1),
    );
    assert_in(
        "int64(1) into cty.Number",
        1i64,
        Type::number(),
        Value::number_int(1),
    );
    assert_in(
        "uint(1) into cty.Number",
        1u64,
        Type::number(),
        Value::number_int(1),
    );
    assert_in(
        "uint8(1) into cty.Number",
        1u8,
        Type::number(),
        Value::number_int(1),
    );
    assert_in(
        "uint16(1) into cty.Number",
        1u16,
        Type::number(),
        Value::number_int(1),
    );
    assert_in(
        "uint32(1) into cty.Number",
        1u32,
        Type::number(),
        Value::number_int(1),
    );
    assert_in(
        "uint64(1) into cty.Number",
        1u64,
        Type::number(),
        Value::number_int(1),
    );
    assert_in(
        "float32(1.5) into cty.Number",
        1.5f32,
        Type::number(),
        Value::number_float(1.5),
    );
    assert_in(
        "float64(1.5) into cty.Number",
        1.5f64,
        Type::number(),
        Value::number_float(1.5),
    );
    // NOTE(port): upstream converts big.NewFloat(1.5) and big.NewInt(5) into
    // cty.Number. There are no big-number types among the interop impls (a
    // dedicated big-number representation is an open implementation decision,
    // see docs/api-mapping.md), so those two cases have no Rust analogue.
    assert_in(
        "(*int)(nil) into cty.Number",
        None::<i64>,
        Type::number(),
        Value::null(Type::number()),
    );

    // Lists
    assert_in(
        "[]int{} into cty.List(cty.Number)",
        Vec::<i64>::new(),
        Type::list(Type::number()),
        Value::list_empty(Type::number()),
    );
    assert_in(
        "[]int{1, 2} into cty.List(cty.Number)",
        vec![1i64, 2],
        Type::list(Type::number()),
        Value::list([Value::number_int(1), Value::number_int(2)]),
    );
    assert_in(
        "&[]int{1, 2} into cty.List(cty.Number)",
        Some(vec![1i64, 2]),
        Type::list(Type::number()),
        Value::list([Value::number_int(1), Value::number_int(2)]),
    );
    // NOTE(port): upstream converts `[]int(nil)` (a nil Go slice, a zero
    // value distinct from a nil pointer) into cty.NullVal(cty.List(cty.Number)).
    // Rust's Vec has no nil state; the nil-pointer analogue is the
    // `None::<Vec<i64>>` case below.
    assert_in(
        "(*[]int)(nil) into cty.List(cty.Number)",
        None::<Vec<i64>>,
        Type::list(Type::number()),
        Value::null(Type::list(Type::number())),
    );
    assert_in(
        "[2]int{1, 2} into cty.List(cty.Number)",
        [1i64, 2],
        Type::list(Type::number()),
        Value::list([Value::number_int(1), Value::number_int(2)]),
    );
    assert_in(
        "[0]int{} into cty.List(cty.Number)",
        [] as [i64; 0],
        Type::list(Type::number()),
        Value::list_empty(Type::number()),
    );
    assert_in(
        "[]int{} into cty.Set(cty.Number)",
        Vec::<i64>::new(),
        Type::set(Type::number()),
        Value::set_empty(Type::number()),
    );

    // Sets
    assert_in(
        "[]int{1, 2} into cty.Set(cty.Number)",
        vec![1i64, 2],
        Type::set(Type::number()),
        Value::set([Value::number_int(1), Value::number_int(2)]),
    );
    assert_in(
        "[]int{2, 2} into cty.Set(cty.Number)",
        vec![2i64, 2],
        Type::set(Type::number()),
        Value::set([Value::number_int(2)]),
    );
    assert_in(
        "&[]int{1, 2} into cty.Set(cty.Number)",
        Some(vec![1i64, 2]),
        Type::set(Type::number()),
        Value::set([Value::number_int(1), Value::number_int(2)]),
    );
    // NOTE(port): `[]int(nil)` into cty.Set(cty.Number) — nil Go slice, no
    // Rust analogue (see the list section above); the nil-pointer analogue
    // is the case below.
    assert_in(
        "(*[]int)(nil) into cty.Set(cty.Number)",
        None::<Vec<i64>>,
        Type::set(Type::number()),
        Value::null(Type::set(Type::number())),
    );
    assert_in(
        "[2]int{1, 2} into cty.Set(cty.Number)",
        [1i64, 2],
        Type::set(Type::number()),
        Value::set([Value::number_int(1), Value::number_int(2)]),
    );
    assert_in(
        "[0]int{} into cty.Set(cty.Number)",
        [] as [i64; 0],
        Type::set(Type::number()),
        Value::set_empty(Type::number()),
    );
    // NOTE(port): upstream also converts go-cty's internal `set.Set`
    // container (set.NewSet / set.NewSetFromSlice with custom `testSetRules`
    // over `any` elements) into cty.Set values. The Rust interop impls have
    // no analogue for the raw set container, so those two cases are omitted.

    // Maps
    assert_in(
        "map[string]int{} into cty.Map(cty.Number)",
        BTreeMap::<String, i64>::new(),
        Type::map(Type::number()),
        Value::map_empty(Type::number()),
    );
    assert_in(
        r#"map[string]int{"one": 1, "two": 2} into cty.Map(cty.Number)"#,
        BTreeMap::from([("one".to_string(), 1i64), ("two".to_string(), 2i64)]),
        Type::map(Type::number()),
        Value::map([("one", Value::number_int(1)), ("two", Value::number_int(2))]),
    );

    // Objects
    // NOTE(port): the seven upstream object cases all depend on Go
    // reflection with no Rust analogue yet, and await a derive macro:
    //   - struct{}{} into cty.EmptyObject → cty.EmptyObjectVal
    //   - struct{ Ignored int }{1} into cty.EmptyObject → cty.EmptyObjectVal
    //   - struct{}{} into cty.Object{"name": cty.String} →
    //     cty.ObjectVal{"name": cty.NullVal(cty.String)}
    //   - struct{ Name string `cty:"name"`; Number int `cty:"number"` }
    //     {"Steven", 1} into cty.Object{"name","number"} →
    //     cty.ObjectVal{"name": "Steven", "number": 1}
    //   - the same struct with only the `cty:"name"` tag → the untagged
    //     field is left null
    //   - map[string]any{"name": "Steven", "number": 1} (heterogeneous
    //     `any`-valued map) into cty.Object{"name","number"}
    //   - map[string]any{"number": 1} into cty.Object{"name","number"} →
    //     missing key becomes cty.NullVal(cty.String)

    // Tuples
    // NOTE(port): the six upstream tuple cases likewise depend on
    // reflection over `any` slices or untagged structs and are omitted:
    //   - []any{} into cty.EmptyTuple → cty.EmptyTupleVal
    //   - struct{}{} into cty.EmptyTuple → cty.EmptyTupleVal
    //   - testTupleStruct{"Stephen", 23} into cty.Tuple([String, Number])
    //   - []any{1, 2, 3} into cty.Tuple([Number, Number, Number])
    //   - []any{1, "hello", 3} into cty.Tuple([Number, String, Number])
    //   - []any(nil) into cty.Tuple([Number]) → cty.NullVal of that tuple

    // Capsules
    // NOTE(port): upstream converts `capsuleANative` (a *capsuleType1Native
    // pointer) into a capsule type built with reflect.TypeOf. The interop
    // impls have no capsule conversion (capsule values are constructed
    // directly with Value::capsule), so this case is omitted.

    // Dynamic
    assert_in(
        "cty.NumberIntVal(2) into cty.DynamicPseudoType",
        Value::number_int(2),
        Type::dynamic(),
        Value::number_int(2),
    );
    assert_in(
        "[]cty.Value{cty.NumberIntVal(2)} into cty.List(cty.DynamicPseudoType)",
        vec![Value::number_int(2)],
        Type::list(Type::dynamic()),
        Value::list([Value::number_int(2)]),
    );
    assert_in(
        r#"map[string]cty.Value{"number": cty.NumberIntVal(2)} into cty.Map(cty.DynamicPseudoType)"#,
        BTreeMap::from([("number".to_string(), Value::number_int(2))]),
        Type::map(Type::dynamic()),
        Value::map([("number", Value::number_int(2))]),
    );

    // Passthrough
    assert_in(
        "cty.NumberIntVal(2) into cty.Number",
        Value::number_int(2),
        Type::number(),
        Value::number_int(2),
    );
    assert_in(
        r#"cty.StringVal("hi") into cty.String"#,
        Value::string("hi"),
        Type::string(),
        Value::string("hi"),
    );
}

// Ported from TestOut:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/gocty/out_test.go#L12
#[test]
#[ignore = "not yet implemented"]
fn gocty_out() {
    // NOTE(port): upstream also decodes into Go defined types (aliases):
    // boolAlias, stringAlias, intAlias, float32Alias, float64Alias,
    // bigIntAlias, listIntAlias, and mapIntAlias. Reflection handles those
    // by Kind; Rust newtypes get no blanket FromCty impl (a future derive
    // macro would provide one), so the eight alias cases are omitted
    // throughout this test.

    // Bool
    assert_out("cty.True into bool", Value::bool(true), true);
    assert_out("cty.False into bool", Value::bool(false), false);
    assert_out("cty.True into *bool", Value::bool(true), Some(true));
    assert_out(
        "cty.NullVal(cty.Bool) into *bool",
        Value::null(Type::bool()),
        None::<bool>,
    );

    // String
    assert_out(
        r#"cty.StringVal("hello") into string"#,
        Value::string("hello"),
        "hello".to_string(),
    );
    assert_out(
        r#"cty.StringVal("") into string"#,
        Value::string(""),
        String::new(),
    );
    assert_out(
        r#"cty.StringVal("hello") into *string"#,
        Value::string("hello"),
        Some("hello".to_string()),
    );
    assert_out(
        "cty.NullVal(cty.String) into *string",
        Value::null(Type::string()),
        None::<String>,
    );

    // Number
    assert_out("cty.NumberIntVal(5) into int", Value::number_int(5), 5i64);
    assert_out("cty.NumberIntVal(5) into int8", Value::number_int(5), 5i8);
    assert_out("cty.NumberIntVal(5) into int16", Value::number_int(5), 5i16);
    assert_out("cty.NumberIntVal(5) into int32", Value::number_int(5), 5i32);
    assert_out("cty.NumberIntVal(5) into int64", Value::number_int(5), 5i64);
    assert_out("cty.NumberIntVal(5) into uint", Value::number_int(5), 5u64);
    assert_out("cty.NumberIntVal(5) into uint8", Value::number_int(5), 5u8);
    assert_out(
        "cty.NumberIntVal(5) into uint16",
        Value::number_int(5),
        5u16,
    );
    assert_out(
        "cty.NumberIntVal(5) into uint32",
        Value::number_int(5),
        5u32,
    );
    assert_out(
        "cty.NumberIntVal(5) into uint64",
        Value::number_int(5),
        5u64,
    );
    assert_out(
        "cty.NumberFloatVal(1.5) into float32",
        Value::number_float(1.5),
        1.5f32,
    );
    assert_out(
        "cty.NumberFloatVal(1.5) into float64",
        Value::number_float(1.5),
        1.5f64,
    );
    // NOTE(port): upstream also decodes cty.NumberFloatVal(1.5) into
    // *big.Float and cty.NumberIntVal(5) into *big.Int. There are no
    // big-number types among the interop impls (see docs/api-mapping.md),
    // so those two cases are omitted (the four alias cases are covered by
    // the grouped note at the top of this test).

    // Lists
    assert_out(
        "cty.ListValEmpty(cty.Number) into []int",
        Value::list_empty(Type::number()),
        Vec::<i64>::new(),
    );
    assert_out(
        "cty.ListVal([1, 5]) into []int",
        Value::list([Value::number_int(1), Value::number_int(5)]),
        vec![1i64, 5],
    );
    // NOTE(port): upstream decodes cty.NullVal(cty.List(cty.Number)) into a
    // nil `[]int` slice; the Rust analogue of that absent slice is
    // `None::<Vec<i64>>`.
    assert_out(
        "cty.NullVal(cty.List(cty.Number)) into []int",
        Value::null(Type::list(Type::number())),
        None::<Vec<i64>>,
    );
    assert_out(
        "cty.ListVal([1, 5]) into [2]int",
        Value::list([Value::number_int(1), Value::number_int(5)]),
        [1i64, 5],
    );
    assert_out(
        "cty.ListValEmpty(cty.Number) into [0]int",
        Value::list_empty(Type::number()),
        [] as [i64; 0],
    );
    assert_out(
        "cty.ListValEmpty(cty.Number) into *[0]int",
        Value::list_empty(Type::number()),
        Some([] as [i64; 0]),
    );

    // Maps
    assert_out(
        "cty.MapValEmpty(cty.Number) into map[string]int",
        Value::map_empty(Type::number()),
        BTreeMap::<String, i64>::new(),
    );
    assert_out(
        r#"cty.MapVal({"one": 1, "five": 5}) into map[string]int"#,
        Value::map([
            ("one", Value::number_int(1)),
            ("five", Value::number_int(5)),
        ]),
        BTreeMap::from([("one".to_string(), 1i64), ("five".to_string(), 5i64)]),
    );
    // NOTE(port): upstream decodes cty.NullVal(cty.Map(cty.Number)) into a
    // nil Go map; the Rust analogue of that absent map is
    // `None::<BTreeMap<String, i64>>`.
    assert_out(
        "cty.NullVal(cty.Map(cty.Number)) into map[string]int",
        Value::null(Type::map(Type::number())),
        None::<BTreeMap<String, i64>>,
    );

    // Sets
    assert_out(
        "cty.SetValEmpty(cty.Number) into []int",
        Value::set_empty(Type::number()),
        Vec::<i64>::new(),
    );
    assert_out(
        "cty.SetVal([1, 5]) into []int",
        Value::set([Value::number_int(1), Value::number_int(5)]),
        vec![1i64, 5],
    );
    assert_out(
        "cty.SetVal([1, 5]) into [2]int",
        Value::set([Value::number_int(1), Value::number_int(5)]),
        [1i64, 5],
    );

    // Objects
    // NOTE(port): the three upstream object cases decode into Go structs
    // (struct{}{} from cty.EmptyObjectVal, and `testStruct` — fields tagged
    // `cty:"name"` / `cty:"number"` with a *int pointer field — from object
    // values, one leaving the absent "number" as a nil pointer and one
    // recovering ptrToInt(12)). Struct decoding relies on reflection over
    // cty tags and awaits a derive macro.

    // Tuples
    // NOTE(port): the two upstream tuple cases decode cty.EmptyTupleVal into
    // struct{}{} and cty.TupleVal(["Stephen", 5]) into the untagged
    // testTupleStruct{Name, Number} by field order — struct reflection with
    // no Rust analogue yet.

    // Capsules
    // NOTE(port): the two upstream capsule cases decode a capsule value into
    // capsuleType1Native (a copy) and into *capsuleType1Native (recovering
    // the original pointer — Go pointer identity). The interop impls have no
    // capsule conversion; encapsulated values are reached via
    // Value::encapsulated_value() instead.

    // Passthrough
    assert_out(
        "cty.NumberIntVal(2) into cty.Value",
        Value::number_int(2),
        Value::number_int(2),
    );
    assert_out(
        "cty.UnknownVal(cty.Bool) into cty.Value",
        Value::unknown(Type::bool()),
        Value::unknown(Type::bool()),
    );
    assert_out(
        "cty.NullVal(cty.Bool) into cty.Value",
        Value::null(Type::bool()),
        Value::null(Type::bool()),
    );
    assert_out(
        "cty.DynamicVal into cty.Value",
        Value::dynamic(),
        Value::dynamic(),
    );
    assert_out(
        "cty.NullVal(cty.DynamicPseudoType) into cty.Value",
        Value::null(Type::dynamic()),
        Value::null(Type::dynamic()),
    );
}

// Ported from TestImpliedType:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/gocty/type_implied_test.go#L10
#[test]
#[ignore = "not yet implemented"]
fn gocty_implied_type() {
    // Primitive types
    assert_implied::<i64>("int", Type::number());
    assert_implied::<i8>("int8", Type::number());
    assert_implied::<i16>("int16", Type::number());
    assert_implied::<i32>("int32", Type::number());
    assert_implied::<i64>("int64", Type::number());
    assert_implied::<u64>("uint", Type::number());
    assert_implied::<u8>("uint8", Type::number());
    assert_implied::<u16>("uint16", Type::number());
    assert_implied::<u32>("uint32", Type::number());
    assert_implied::<u64>("uint64", Type::number());
    assert_implied::<f32>("float32", Type::number());
    assert_implied::<f64>("float64", Type::number());
    assert_implied::<bool>("bool", Type::bool());
    assert_implied::<String>("string", Type::string());

    // Collection types
    assert_implied::<Vec<i64>>("[]int", Type::list(Type::number()));
    assert_implied::<Vec<Vec<i64>>>("[][]int", Type::list(Type::list(Type::number())));
    assert_implied::<BTreeMap<String, i64>>("map[string]int", Type::map(Type::number()));
    assert_implied::<BTreeMap<String, BTreeMap<String, i64>>>(
        "map[string]map[string]int",
        Type::map(Type::map(Type::number())),
    );
    assert_implied::<BTreeMap<String, Vec<i64>>>(
        "map[string][]int",
        Type::map(Type::list(Type::number())),
    );

    // Structs
    // NOTE(port): upstream implies
    // cty.Object({"name": cty.String, "number": cty.Number}) from
    // `testStruct{}` via its `cty:"…"` field tags — struct reflection with
    // no Rust analogue yet; awaits a derive macro.

    // Pointers (unwrapped and ignored)
    assert_implied::<Option<i64>>("*int", Type::number());
    assert_implied::<Option<bool>>("*bool", Type::bool());
    assert_implied::<Option<String>>("*string", Type::string());
    // NOTE(port): `&testStruct{}` implies the same object type as the
    // struct case above — omitted for the same reason.

    // Dynamic
    // NOTE(port): upstream passes cty.NilVal, whose static Go type is
    // cty.Value; ImpliedType reflects on that static type and returns
    // cty.DynamicPseudoType. The Rust analogue is the implied type of
    // `Value` itself.
    assert_implied::<Value>("cty.Value", Type::dynamic());
}
