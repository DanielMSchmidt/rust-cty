# go-cty → rust-cty API mapping

The contract between the upstream Go API and the Rust surface that the
conformance tests compile against. Pinned upstream:
`github.com/zclconf/go-cty` @ `a918e1174fcf2a25b7a222e7e78b00ea40ace26c`.

## General rules

- **Naming.** `CamelCase` → `snake_case`. Go package-level constructors become
  associated functions: `cty.StringVal("x")` → `Value::string("x")`,
  `cty.List(cty.String)` → `Type::list(Type::string())`.
- **Panics vs. errors.** Mirrors upstream: operations that panic on misuse in
  Go (wrong operand type, missing attribute, `ListVal` with mixed element
  types) panic in Rust; APIs that return `error` in Go return
  `Result<_, cty::Error>`. Tests assert panics with
  `std::panic::catch_unwind`.
- **Error messages** are observable behavior: `Error`'s `Display` must match
  the upstream `err.Error()` string wherever an upstream test asserts on it.
- **Equality.** `Value: PartialEq` is `RawEquals`; `Type: PartialEq` is
  `Type.Equals`; the cty-semantics `Value.Equals` (which can return unknown)
  is the `equals` method. Tests assert with `==`/`assert_eq!` where upstream
  used `RawEquals`, and with `.equals(..)` where upstream used `Equals`.
- **`GoString` vs. `Display`.** `go_string()` on `Value`, `Type`, `PathStep`,
  and `ValueMarks` reproduces the Go `GoString()` output **byte for byte**
  (`cty.StringVal("hello")`). `Display`/`to_string()` renders the analogous
  *Rust* constructor expression (`Value::string("hello")`); its expected
  strings in tests are the mechanical translation of the upstream `GoString`
  expectations into this crate's constructor syntax.
- **`NilVal` / `NilType` do not exist.** They are Go zero values; Rust uses
  `Option<Value>` / `Option<Type>` at the API edges that need absence
  (e.g. `convert::unify` returns `Option`). Upstream test cases that exist
  only to pin zero-value behavior are omitted with a
  `// NOTE(port): NilVal/NilType …` comment.
- **No `Must*` variants.** `cty.MustParseNumberVal(s)` →
  `Value::parse_number(s).unwrap()`.
- **Maps.** Go `map[string]X` in argument position becomes
  `impl IntoIterator<Item = (K, V)>`; in return position `BTreeMap<String, X>`
  (deterministic order). Upstream tests relying on Go map iteration order do
  not exist (go-cty sorts where order matters).

## Type correspondence

| go-cty | rust-cty |
|---|---|
| `cty.Type`, `cty.Value` | `Type`, `Value` |
| `cty.String/Number/Bool` | `Type::string()/number()/bool()` |
| `cty.DynamicPseudoType` | `Type::dynamic()` |
| `cty.EmptyObject`, `cty.EmptyTuple` | `Type::empty_object()`, `Type::empty_tuple()` |
| `cty.True/False` | `Value::bool(true/false)` |
| `cty.Zero` | `Value::zero()` |
| `cty.PositiveInfinity/NegativeInfinity` | `Value::positive_infinity()/negative_infinity()` |
| `cty.DynamicVal` | `Value::dynamic()` |
| `cty.EmptyObjectVal/EmptyTupleVal` | `Value::empty_object()/empty_tuple()` |
| `cty.NullVal(ty)/UnknownVal(ty)` | `Value::null(ty)/unknown(ty)` |
| `val.Type()` | `val.ty()` |
| `val.True()/False()` | `val.is_true()/is_false()` |
| `val.AsBigFloat()` | `val.as_f64()` (lossy; a dedicated big-number type is an open implementation decision) |
| `cty.Path{}` / `cty.GetAttrPath("a").Index(v)` | `Path::new()` / `Path::new().attr("a").index(v)` |
| `cty.IndexIntPath(0)` / `cty.IndexStringPath("k")` | `Path::new().index_int(0)` / `Path::new().index_string("k")` |
| marks (`val.Mark("x")`, mark values are `any`) | `val.mark("x")`; a mark is `Mark` (`From<&str>/String/i64/bool`, `Mark::of` for other types) |
| `cty.NewValueMarks("a", "b")` | `ValueMarks::from_marks(["a", "b"])` |
| `cty.ValueMarksOfType[T](v)` / deep | `v.marks_of_type::<T>()` / `v.marks_of_type_deep::<T>()` |
| `ctymarks.WrangleKeep/Drop/Expand/Replace(m)` | `WrangleAction::{Keep, Drop, Expand, Replace(m)}` |
| element iteration (`ElementIterator`, `Elements()`) | `val.element_iterator()` (a std `Iterator<Item = (Value, Value)>`) |
| `val.ForEachElement(cb)` (cb returns stop) | `val.for_each_element(|k, v| stop)` |
| capsules: `cty.Capsule("name", reflect.TypeOf(T{}))` | `Type::capsule::<T>("name")` |
| `cty.CapsuleVal(ty, &v)` / `EncapsulatedValue()` | `Value::capsule(ty, v)` / `encapsulated_value()` (`&dyn Any`, downcast in tests) |
| `Type.EncapsulatedType()` | `Type::encapsulated_type_id()` (`TypeId`) |
| refinements (`v.Refine().NotNull().NewValue()`) | `v.refine().not_null().new_value()` |
| `cty.Walk/Transform/TransformWithTransformer` | `walk/transform/transform_with_transformer` |
| `cty.UnknownAsNull` | `unknown_as_null` |
| `ctystrings.Normalize/SafeKnownPrefix` | `strings::normalize/safe_known_prefix` |
| `convert.Convert/GetConversion/Unify/MismatchMessage` | `convert::{convert, get_conversion, unify, mismatch_message}` |
| `convert.Conversion` (a func; nil = none/identity) | `convert::Conversion` struct with `.apply()`; absence is `Option` |
| unexported `compareTypes`/`sortTypes` | `convert::internals::{compare_types, sort_types}` (conformance-only) |
| unexported set hash bytes | `internals::set_hash_bytes` (conformance-only) |
| `function.New(&Spec{...})` | `Function::new(Spec { ... })` |
| `Spec.Type` / `Spec.Impl` | `Spec::type_fn` / `Spec::impl_fn` (boxed closures) |
| `Parameter.Type` | `Parameter::ty` (`Option<Type>`; `None` only for `Parameter::default()` in construction) |
| `function.StaticReturnType(ty)` | `function::static_return_type(ty)` |
| `function.Unpredictable(f)` | `function::unpredictable(f)` |
| `stdlib.UpperFunc` / `stdlib.Upper(v)` | `stdlib::upper_func()` / `stdlib::upper(&v)` |
| `stdlib.Bytes` / `stdlib.BytesVal` | `stdlib::bytes_type()` / `stdlib::bytes_val(vec)` |
| `json.Marshal` (returns `[]byte`) | `json::marshal` (returns `String`) |
| `json.SimpleJSONValue` | `json::SimpleValue` with `to_json`/`from_json` |
| `msgpack.Marshal/Unmarshal/ImpliedType` | `msgpack::{marshal, unmarshal, implied_type}` (`Vec<u8>`) |
| `gocty.ToCtyValue/FromCtyValue/ImpliedType` | `interop::{to_cty_value, from_cty_value, implied_type}` via `IntoCty`/`FromCty`/`CtyTyped` traits |
| Go pointers in gocty | `Option<T>` (`None` ↔ null) |
| Go structs with `cty:"…"` tags in gocty | `#[derive(IntoCty, FromCty, CtyTyped)]` from the `cty-derive` crate (see below) |

## Conformance test conventions

Each test file under `tests/conformance/` starts with a header comment naming
the upstream file it transcribes and the pinned commit SHA. Table-driven Go
tests become loops over arrays of case structs (or tuples), preserving the
upstream case order and literal expected values. Cases that cannot be
expressed are kept as comments with a `NOTE(port):` explanation rather than
silently dropped.

Upstream `GoString` expectations are ported twice: once against `go_string()`
with the literal Go string, and once against `to_string()` with the
translated Rust-syntax string.

## The `cty-derive` attribute grammar

The `cty-derive` proc-macro crate provides `#[derive(IntoCty)]`,
`#[derive(FromCty)]`, and `#[derive(CtyTyped)]`, the analogue of gocty's
struct reflection. Like the rest of the workspace, the derives currently emit
`todo!()` bodies — they define the contract the conformance tests compile
against; the generated logic is to be written by hand later.

- `#[cty(attr = "name")]` on a field ≙ a Go `cty:"name"` tag: the field maps
  to the object attribute `name`. Fields without the attribute are ignored
  for object conversion, exactly as untagged Go fields are.
- Tuple conversion uses no field attributes: struct fields map to tuple
  elements positionally, as in gocty.
- `Option<T>` fields ≙ Go pointer fields (`None` ↔ null).
- Deriving on a single-field newtype (`struct StringAlias(String);`) ≙ a Go
  defined type (`type stringAlias string`), delegating to the inner type.
