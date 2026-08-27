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
//! (nil pointer ↔ `None` ↔ null). Go struct reflection with `cty:"…"` tags
//! translates to the stub derives from the `cty-derive` crate
//! (`#[derive(IntoCty, FromCty, CtyTyped)]` with `#[cty(attr = "…")]` field
//! attributes; Go defined types become derived newtypes); cases relying on
//! `interface{}` (`any`) values or other Go-only reflection are kept as
//! NOTE(port) comment blocks.

use std::collections::BTreeMap;
use std::fmt::Debug;

use cty::interop::{self, CtyTyped, FromCty, IntoCty};
use cty::{Type, Value};
use cty_derive::{CtyTyped as CtyTypedDerive, FromCty as FromCtyDerive, IntoCty as IntoCtyDerive};

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

// ---------------------------------------------------------------------------
// The Rust analogues of upstream's reflected Go test structs, using the stub
// derives from the cty-derive crate: each struct mirrors the corresponding Go
// struct, with `#[cty(attr = "…")]` for each `cty:"…"` tag and untagged Go
// fields left unannotated; single-field newtypes mirror Go defined types
// (the `*Alias` types in out_test.go). See docs/api-mapping.md for the
// attribute grammar. The `allow(dead_code)` is because the derive stubs emit
// todo!() bodies that read no fields yet.
// ---------------------------------------------------------------------------

/// Go: `struct{}{}`.
#[derive(Debug, PartialEq, IntoCtyDerive, FromCtyDerive)]
struct EmptyStruct;

/// Go: `struct{ Ignored int }` — no cty tag, so the field is ignored.
#[allow(dead_code)]
#[derive(IntoCtyDerive)]
struct IgnoredFieldStruct {
    ignored: i64,
}

/// Go: `struct{ Name string `cty:"name"`; Number int `cty:"number"` }`.
#[allow(dead_code)]
#[derive(IntoCtyDerive)]
struct NameNumberStruct {
    #[cty(attr = "name")]
    name: String,
    #[cty(attr = "number")]
    number: i64,
}

/// Go: `struct{ Name string `cty:"name"`; Number int }` — Number untagged.
#[allow(dead_code)]
#[derive(IntoCtyDerive)]
struct NameUntaggedNumberStruct {
    #[cty(attr = "name")]
    name: String,
    number: i64,
}

/// Go: `type testStruct struct{ Name string `cty:"name"`; Number *int `cty:"number"` }`
/// (out_test.go / type_implied_test.go).
#[allow(dead_code)]
#[derive(Debug, PartialEq, FromCtyDerive, CtyTypedDerive)]
struct TestStruct {
    #[cty(attr = "name")]
    name: String,
    #[cty(attr = "number")]
    number: Option<i64>,
}

/// Go: `type testTupleStruct struct{ Name string; Number int }` — untagged,
/// so fields map to tuple elements positionally.
#[allow(dead_code)]
#[derive(Debug, PartialEq, IntoCtyDerive, FromCtyDerive)]
struct TestTupleStruct {
    name: String,
    number: i64,
}

/// Go: `type boolAlias bool`.
#[allow(dead_code)]
#[derive(Debug, PartialEq, FromCtyDerive)]
struct BoolAlias(bool);

/// Go: `type stringAlias string`.
#[allow(dead_code)]
#[derive(Debug, PartialEq, FromCtyDerive)]
struct StringAlias(String);

/// Go: `type intAlias int`.
#[allow(dead_code)]
#[derive(Debug, PartialEq, FromCtyDerive)]
struct IntAlias(i64);

/// Go: `type float32Alias float32`.
#[allow(dead_code)]
#[derive(Debug, PartialEq, FromCtyDerive)]
struct Float32Alias(f32);

/// Go: `type float64Alias float64`.
#[allow(dead_code)]
#[derive(Debug, PartialEq, FromCtyDerive)]
struct Float64Alias(f64);

/// Go: `type listIntAlias []int`.
#[allow(dead_code)]
#[derive(Debug, PartialEq, FromCtyDerive)]
struct ListIntAlias(Vec<i64>);

/// Go: `type mapIntAlias map[string]int`.
#[allow(dead_code)]
#[derive(Debug, PartialEq, FromCtyDerive)]
struct MapIntAlias(BTreeMap<String, i64>);

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
    assert_in(
        "struct{}{} into cty.EmptyObject",
        EmptyStruct,
        Type::empty_object(),
        Value::empty_object(),
    );
    assert_in(
        "struct{ Ignored int }{1} into cty.EmptyObject",
        IgnoredFieldStruct { ignored: 1 },
        Type::empty_object(),
        Value::empty_object(),
    );
    assert_in(
        "struct{}{} into cty.Object({name: String})",
        EmptyStruct,
        Type::object([("name", Type::string())]),
        Value::object([("name", Value::null(Type::string()))]),
    );
    assert_in(
        r#"tagged struct{"Steven", 1} into cty.Object({name, number})"#,
        NameNumberStruct {
            name: "Steven".to_string(),
            number: 1,
        },
        Type::object([("name", Type::string()), ("number", Type::number())]),
        Value::object([
            ("name", Value::string("Steven")),
            ("number", Value::number_int(1)),
        ]),
    );
    assert_in(
        r#"struct with untagged Number{"Steven", 1} into cty.Object({name, number})"#,
        NameUntaggedNumberStruct {
            name: "Steven".to_string(),
            number: 1,
        },
        Type::object([("name", Type::string()), ("number", Type::number())]),
        Value::object([
            ("name", Value::string("Steven")),
            ("number", Value::null(Type::number())),
        ]),
    );
    // NOTE(port): the two remaining upstream object cases convert
    // heterogeneous `map[string]any` values (one with both keys, one where
    // the missing "name" becomes cty.NullVal(cty.String)); Go interface
    // reflection has no Rust analogue, so they stay omitted.

    // Tuples
    // NOTE(port): upstream first converts `[]any{}` into cty.EmptyTuple;
    // heterogeneous `any` slices have no Rust analogue (see the note after
    // the struct cases below).
    assert_in(
        "struct{}{} into cty.EmptyTuple",
        EmptyStruct,
        Type::empty_tuple(),
        Value::empty_tuple(),
    );
    assert_in(
        r#"testTupleStruct{"Stephen", 23} into cty.Tuple([String, Number])"#,
        TestTupleStruct {
            name: "Stephen".to_string(),
            number: 23,
        },
        Type::tuple([Type::string(), Type::number()]),
        Value::tuple([Value::string("Stephen"), Value::number_int(23)]),
    );
    // NOTE(port): the three remaining upstream tuple cases convert `any`
    // slices — []any{1, 2, 3}, []any{1, "hello", 3}, and []any(nil) (which
    // becomes cty.NullVal of the tuple type); Go interface reflection has no
    // Rust analogue, so they stay omitted.

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
    // Bool
    assert_out("cty.True into bool", Value::bool(true), true);
    assert_out("cty.False into bool", Value::bool(false), false);
    assert_out("cty.True into *bool", Value::bool(true), Some(true));
    assert_out(
        "cty.NullVal(cty.Bool) into *bool",
        Value::null(Type::bool()),
        None::<bool>,
    );
    assert_out(
        "cty.True into boolAlias",
        Value::bool(true),
        BoolAlias(true),
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
    assert_out(
        r#"cty.StringVal("hello") into stringAlias"#,
        Value::string("hello"),
        StringAlias("hello".to_string()),
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
    // *big.Float, cty.NumberIntVal(5) into *big.Int, and cty.NumberIntVal(5)
    // into *bigIntAlias. There are no big-number types among the interop
    // impls (see docs/api-mapping.md), so those three cases are omitted.
    assert_out(
        "cty.NumberIntVal(5) into intAlias",
        Value::number_int(5),
        IntAlias(5),
    );
    assert_out(
        "cty.NumberFloatVal(1.5) into float32Alias",
        Value::number_float(1.5),
        Float32Alias(1.5),
    );
    assert_out(
        "cty.NumberFloatVal(1.5) into float64Alias",
        Value::number_float(1.5),
        Float64Alias(1.5),
    );

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
    assert_out(
        "cty.ListVal([1, 5]) into listIntAlias",
        Value::list([Value::number_int(1), Value::number_int(5)]),
        ListIntAlias(vec![1, 5]),
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
    assert_out(
        r#"cty.MapVal({"one": 1, "five": 5}) into mapIntAlias"#,
        Value::map([
            ("one", Value::number_int(1)),
            ("five", Value::number_int(5)),
        ]),
        MapIntAlias(BTreeMap::from([
            ("one".to_string(), 1i64),
            ("five".to_string(), 5i64),
        ])),
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
    assert_out(
        "cty.EmptyObjectVal into struct{}{}",
        Value::empty_object(),
        EmptyStruct,
    );
    assert_out(
        r#"cty.ObjectVal({name: "Stephen"}) into testStruct"#,
        Value::object([("name", Value::string("Stephen"))]),
        TestStruct {
            name: "Stephen".to_string(),
            number: None,
        },
    );
    assert_out(
        r#"cty.ObjectVal({name: "Stephen", number: 12}) into testStruct"#,
        Value::object([
            ("name", Value::string("Stephen")),
            ("number", Value::number_int(12)),
        ]),
        TestStruct {
            name: "Stephen".to_string(),
            number: Some(12),
        },
    );

    // Tuples
    assert_out(
        "cty.EmptyTupleVal into struct{}{}",
        Value::empty_tuple(),
        EmptyStruct,
    );
    assert_out(
        r#"cty.TupleVal(["Stephen", 5]) into testTupleStruct"#,
        Value::tuple([Value::string("Stephen"), Value::number_int(5)]),
        TestTupleStruct {
            name: "Stephen".to_string(),
            number: 5,
        },
    );

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
    assert_implied::<TestStruct>(
        "testStruct{}",
        Type::object([("name", Type::string()), ("number", Type::number())]),
    );

    // Pointers (unwrapped and ignored)
    assert_implied::<Option<i64>>("*int", Type::number());
    assert_implied::<Option<bool>>("*bool", Type::bool());
    assert_implied::<Option<String>>("*string", Type::string());
    assert_implied::<Option<TestStruct>>(
        "&testStruct{}",
        Type::object([("name", Type::string()), ("number", Type::number())]),
    );

    // Dynamic
    // NOTE(port): upstream passes cty.NilVal, whose static Go type is
    // cty.Value; ImpliedType reflects on that static type and returns
    // cty.DynamicPseudoType. The Rust analogue is the implied type of
    // `Value` itself.
    assert_implied::<Value>("cty.Value", Type::dynamic());
}
