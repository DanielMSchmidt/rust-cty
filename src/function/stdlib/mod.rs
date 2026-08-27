//! The standard library of functions: the analogue of go-cty's
//! `cty/function/stdlib` package.
//!
//! For every upstream `XxxFunc` variable there is an `xxx_func()` accessor
//! returning the [`Function`], and for every upstream convenience function
//! `Xxx(...)` a corresponding `xxx(...)` that calls it. Sections mirror the
//! upstream source files.

use crate::error::Error;
use crate::function::Function;
use crate::types::Type;
use crate::value::Value;

// --- bool.go ---

/// The function behind [`not`] (go-cty: `stdlib.NotFunc`).
pub fn not_func() -> Function {
    todo!()
}

/// The function behind [`and`] (go-cty: `stdlib.AndFunc`).
pub fn and_func() -> Function {
    todo!()
}

/// The function behind [`or`] (go-cty: `stdlib.OrFunc`).
pub fn or_func() -> Function {
    todo!()
}

/// Logical NOT of a bool value (go-cty: `stdlib.Not`).
pub fn not(val: &Value) -> Result<Value, Error> {
    let _ = val;
    todo!()
}

/// Logical AND of two bool values (go-cty: `stdlib.And`).
pub fn and(a: &Value, b: &Value) -> Result<Value, Error> {
    let _ = (a, b);
    todo!()
}

/// Logical OR of two bool values (go-cty: `stdlib.Or`).
pub fn or(a: &Value, b: &Value) -> Result<Value, Error> {
    let _ = (a, b);
    todo!()
}

// --- bytes.go ---

/// The capsule type encapsulating a byte buffer (go-cty: `stdlib.Bytes`).
pub fn bytes_type() -> Type {
    todo!()
}

/// Wraps a byte buffer as a value of the [`bytes_type`] capsule type
/// (go-cty: `stdlib.BytesVal`).
pub fn bytes_val(buf: Vec<u8>) -> Value {
    let _ = buf;
    todo!()
}

/// The function behind [`bytes_len`] (go-cty: `stdlib.BytesLenFunc`).
pub fn bytes_len_func() -> Function {
    todo!()
}

/// The function behind [`bytes_slice`] (go-cty: `stdlib.BytesSliceFunc`).
pub fn bytes_slice_func() -> Function {
    todo!()
}

/// The length of a bytes value (go-cty: `stdlib.BytesLen`).
pub fn bytes_len(buf: &Value) -> Result<Value, Error> {
    let _ = buf;
    todo!()
}

/// A subrange of a bytes value, sharing the underlying buffer
/// (go-cty: `stdlib.BytesSlice`).
pub fn bytes_slice(buf: &Value, offset: &Value, length: &Value) -> Result<Value, Error> {
    let _ = (buf, offset, length);
    todo!()
}

// --- collection.go ---

/// The function behind [`has_index`] (go-cty: `stdlib.HasIndexFunc`).
pub fn has_index_func() -> Function {
    todo!()
}

/// The function behind [`index`] (go-cty: `stdlib.IndexFunc`).
pub fn index_func() -> Function {
    todo!()
}

/// The function behind [`length`] (go-cty: `stdlib.LengthFunc`).
pub fn length_func() -> Function {
    todo!()
}

/// The function behind [`element`] (go-cty: `stdlib.ElementFunc`).
pub fn element_func() -> Function {
    todo!()
}

/// The function behind [`coalesce_list`] (go-cty: `stdlib.CoalesceListFunc`).
pub fn coalesce_list_func() -> Function {
    todo!()
}

/// The function behind [`compact`] (go-cty: `stdlib.CompactFunc`).
pub fn compact_func() -> Function {
    todo!()
}

/// The function behind [`contains`] (go-cty: `stdlib.ContainsFunc`).
pub fn contains_func() -> Function {
    todo!()
}

/// The function behind [`distinct`] (go-cty: `stdlib.DistinctFunc`).
pub fn distinct_func() -> Function {
    todo!()
}

/// The function behind [`chunklist`] (go-cty: `stdlib.ChunklistFunc`).
pub fn chunklist_func() -> Function {
    todo!()
}

/// The function behind [`flatten`] (go-cty: `stdlib.FlattenFunc`).
pub fn flatten_func() -> Function {
    todo!()
}

/// The function behind [`keys`] (go-cty: `stdlib.KeysFunc`).
pub fn keys_func() -> Function {
    todo!()
}

/// The function behind [`lookup`] (go-cty: `stdlib.LookupFunc`).
pub fn lookup_func() -> Function {
    todo!()
}

/// The function behind [`merge`] (go-cty: `stdlib.MergeFunc`).
pub fn merge_func() -> Function {
    todo!()
}

/// The function behind [`reverse_list`] (go-cty: `stdlib.ReverseListFunc`).
pub fn reverse_list_func() -> Function {
    todo!()
}

/// The function behind [`set_product`] (go-cty: `stdlib.SetProductFunc`).
pub fn set_product_func() -> Function {
    todo!()
}

/// The function behind [`slice`] (go-cty: `stdlib.SliceFunc`).
pub fn slice_func() -> Function {
    todo!()
}

/// The function behind [`values`] (go-cty: `stdlib.ValuesFunc`).
pub fn values_func() -> Function {
    todo!()
}

/// The function behind [`zipmap`] (go-cty: `stdlib.ZipmapFunc`).
pub fn zipmap_func() -> Function {
    todo!()
}

/// Whether the collection has an element at the given key
/// (go-cty: `stdlib.HasIndex`).
pub fn has_index(collection: &Value, key: &Value) -> Result<Value, Error> {
    let _ = (collection, key);
    todo!()
}

/// The element of the collection at the given key (go-cty: `stdlib.Index`).
pub fn index(collection: &Value, key: &Value) -> Result<Value, Error> {
    let _ = (collection, key);
    todo!()
}

/// The number of elements of the collection (go-cty: `stdlib.Length`).
pub fn length(collection: &Value) -> Result<Value, Error> {
    let _ = collection;
    todo!()
}

/// The element of the list at the given index, wrapping around past the end
/// (go-cty: `stdlib.Element`).
pub fn element(list: &Value, index: &Value) -> Result<Value, Error> {
    let _ = (list, index);
    todo!()
}

/// The first non-empty list argument (go-cty: `stdlib.CoalesceList`).
pub fn coalesce_list(args: &[Value]) -> Result<Value, Error> {
    let _ = args;
    todo!()
}

/// The list of strings with null and empty entries removed
/// (go-cty: `stdlib.Compact`).
pub fn compact(list: &Value) -> Result<Value, Error> {
    let _ = list;
    todo!()
}

/// Whether the list contains the given value (go-cty: `stdlib.Contains`).
pub fn contains(list: &Value, value: &Value) -> Result<Value, Error> {
    let _ = (list, value);
    todo!()
}

/// The list with duplicate elements removed, keeping first occurrences
/// (go-cty: `stdlib.Distinct`).
pub fn distinct(list: &Value) -> Result<Value, Error> {
    let _ = list;
    todo!()
}

/// The list split into fixed-size chunks (go-cty: `stdlib.Chunklist`).
pub fn chunklist(list: &Value, size: &Value) -> Result<Value, Error> {
    let _ = (list, size);
    todo!()
}

/// The sequence with nested sequences flattened, recursively
/// (go-cty: `stdlib.Flatten`).
pub fn flatten(list: &Value) -> Result<Value, Error> {
    let _ = list;
    todo!()
}

/// The keys of a map or object, sorted (go-cty: `stdlib.Keys`).
pub fn keys(input_map: &Value) -> Result<Value, Error> {
    let _ = input_map;
    todo!()
}

/// The map element at the given key, or the default when absent
/// (go-cty: `stdlib.Lookup`).
pub fn lookup(input_map: &Value, key: &Value, default_value: &Value) -> Result<Value, Error> {
    let _ = (input_map, key, default_value);
    todo!()
}

/// The maps merged left-to-right, later values overriding earlier
/// (go-cty: `stdlib.Merge`).
pub fn merge(maps: &[Value]) -> Result<Value, Error> {
    let _ = maps;
    todo!()
}

/// The list with element order reversed (go-cty: `stdlib.ReverseList`).
pub fn reverse_list(list: &Value) -> Result<Value, Error> {
    let _ = list;
    todo!()
}

/// The cartesian product of the given sets or lists
/// (go-cty: `stdlib.SetProduct`).
pub fn set_product(sets: &[Value]) -> Result<Value, Error> {
    let _ = sets;
    todo!()
}

/// A subrange of the list (go-cty: `stdlib.Slice`).
pub fn slice(list: &Value, start: &Value, end: &Value) -> Result<Value, Error> {
    let _ = (list, start, end);
    todo!()
}

/// The values of a map or object, ordered by key (go-cty: `stdlib.Values`).
pub fn values(input_map: &Value) -> Result<Value, Error> {
    let _ = input_map;
    todo!()
}

/// A map built by zipping a list of keys with a list of values
/// (go-cty: `stdlib.Zipmap`).
pub fn zipmap(keys: &Value, values: &Value) -> Result<Value, Error> {
    let _ = (keys, values);
    todo!()
}

// --- conversion.go ---

/// A function converting its argument to the given type constraint
/// (go-cty: `stdlib.MakeToFunc`).
pub fn make_to_func(want_ty: Type) -> Function {
    let _ = want_ty;
    todo!()
}

/// The function behind [`assert_not_null`] (go-cty: `stdlib.AssertNotNullFunc`).
pub fn assert_not_null_func() -> Function {
    todo!()
}

/// Returns the value unchanged, or an error if it is null
/// (go-cty: `stdlib.AssertNotNull`).
pub fn assert_not_null(val: &Value) -> Result<Value, Error> {
    let _ = val;
    todo!()
}

// --- csv.go ---

/// The function behind [`csv_decode`] (go-cty: `stdlib.CSVDecodeFunc`).
pub fn csv_decode_func() -> Function {
    todo!()
}

/// Parses a CSV document into a list of objects, one per row
/// (go-cty: `stdlib.CSVDecode`).
pub fn csv_decode(str_val: &Value) -> Result<Value, Error> {
    let _ = str_val;
    todo!()
}

// --- datetime.go ---

/// The function behind [`format_date`] (go-cty: `stdlib.FormatDateFunc`).
pub fn format_date_func() -> Function {
    todo!()
}

/// The function behind [`time_add`] (go-cty: `stdlib.TimeAddFunc`).
pub fn time_add_func() -> Function {
    todo!()
}

/// Formats an RFC 3339 timestamp per the given format string
/// (go-cty: `stdlib.FormatDate`).
pub fn format_date(format: &Value, timestamp: &Value) -> Result<Value, Error> {
    let _ = (format, timestamp);
    todo!()
}

/// Adds a duration to an RFC 3339 timestamp (go-cty: `stdlib.TimeAdd`).
pub fn time_add(timestamp: &Value, duration: &Value) -> Result<Value, Error> {
    let _ = (timestamp, duration);
    todo!()
}

// --- format.go ---

/// The function behind [`format`] (go-cty: `stdlib.FormatFunc`).
pub fn format_func() -> Function {
    todo!()
}

/// The function behind [`format_list`] (go-cty: `stdlib.FormatListFunc`).
pub fn format_list_func() -> Function {
    todo!()
}

/// Produces a string by formatting values per a printf-like format string
/// (go-cty: `stdlib.Format`).
pub fn format(format: &Value, vals: &[Value]) -> Result<Value, Error> {
    let _ = (format, vals);
    todo!()
}

/// Like [`format`], but iterating over sequence arguments to produce a list
/// of strings (go-cty: `stdlib.FormatList`).
pub fn format_list(format: &Value, vals: &[Value]) -> Result<Value, Error> {
    let _ = (format, vals);
    todo!()
}

// --- general.go ---

/// The function behind [`equal`] (go-cty: `stdlib.EqualFunc`).
pub fn equal_func() -> Function {
    todo!()
}

/// The function behind [`not_equal`] (go-cty: `stdlib.NotEqualFunc`).
pub fn not_equal_func() -> Function {
    todo!()
}

/// The function behind [`coalesce`] (go-cty: `stdlib.CoalesceFunc`).
pub fn coalesce_func() -> Function {
    todo!()
}

/// Whether the two values are equal (go-cty: `stdlib.Equal`).
pub fn equal(a: &Value, b: &Value) -> Result<Value, Error> {
    let _ = (a, b);
    todo!()
}

/// Whether the two values are not equal (go-cty: `stdlib.NotEqual`).
pub fn not_equal(a: &Value, b: &Value) -> Result<Value, Error> {
    let _ = (a, b);
    todo!()
}

/// The first non-null argument (go-cty: `stdlib.Coalesce`).
pub fn coalesce(vals: &[Value]) -> Result<Value, Error> {
    let _ = vals;
    todo!()
}

// --- json.go ---

/// The function behind [`json_encode`] (go-cty: `stdlib.JSONEncodeFunc`).
pub fn json_encode_func() -> Function {
    todo!()
}

/// The function behind [`json_decode`] (go-cty: `stdlib.JSONDecodeFunc`).
pub fn json_decode_func() -> Function {
    todo!()
}

/// Encodes a value as a JSON string (go-cty: `stdlib.JSONEncode`).
pub fn json_encode(val: &Value) -> Result<Value, Error> {
    let _ = val;
    todo!()
}

/// Decodes a JSON string into a value of an implied type
/// (go-cty: `stdlib.JSONDecode`).
pub fn json_decode(str_val: &Value) -> Result<Value, Error> {
    let _ = str_val;
    todo!()
}

// --- number.go ---

/// The function behind [`absolute`] (go-cty: `stdlib.AbsoluteFunc`).
pub fn absolute_func() -> Function {
    todo!()
}

/// The function behind [`add`] (go-cty: `stdlib.AddFunc`).
pub fn add_func() -> Function {
    todo!()
}

/// The function behind [`subtract`] (go-cty: `stdlib.SubtractFunc`).
pub fn subtract_func() -> Function {
    todo!()
}

/// The function behind [`multiply`] (go-cty: `stdlib.MultiplyFunc`).
pub fn multiply_func() -> Function {
    todo!()
}

/// The function behind [`divide`] (go-cty: `stdlib.DivideFunc`).
pub fn divide_func() -> Function {
    todo!()
}

/// The function behind [`modulo`] (go-cty: `stdlib.ModuloFunc`).
pub fn modulo_func() -> Function {
    todo!()
}

/// The function behind [`negate`] (go-cty: `stdlib.NegateFunc`).
pub fn negate_func() -> Function {
    todo!()
}

/// The function behind [`greater_than`] (go-cty: `stdlib.GreaterThanFunc`).
pub fn greater_than_func() -> Function {
    todo!()
}

/// The function behind [`greater_than_or_equal_to`]
/// (go-cty: `stdlib.GreaterThanOrEqualToFunc`).
pub fn greater_than_or_equal_to_func() -> Function {
    todo!()
}

/// The function behind [`less_than`] (go-cty: `stdlib.LessThanFunc`).
pub fn less_than_func() -> Function {
    todo!()
}

/// The function behind [`less_than_or_equal_to`]
/// (go-cty: `stdlib.LessThanOrEqualToFunc`).
pub fn less_than_or_equal_to_func() -> Function {
    todo!()
}

/// The function behind [`min`] (go-cty: `stdlib.MinFunc`).
pub fn min_func() -> Function {
    todo!()
}

/// The function behind [`max`] (go-cty: `stdlib.MaxFunc`).
pub fn max_func() -> Function {
    todo!()
}

/// The function behind [`int`] (go-cty: `stdlib.IntFunc`).
pub fn int_func() -> Function {
    todo!()
}

/// The function behind [`ceil`] (go-cty: `stdlib.CeilFunc`).
pub fn ceil_func() -> Function {
    todo!()
}

/// The function behind [`floor`] (go-cty: `stdlib.FloorFunc`).
pub fn floor_func() -> Function {
    todo!()
}

/// The function behind [`log`] (go-cty: `stdlib.LogFunc`).
pub fn log_func() -> Function {
    todo!()
}

/// The function behind [`pow`] (go-cty: `stdlib.PowFunc`).
pub fn pow_func() -> Function {
    todo!()
}

/// The function behind [`signum`] (go-cty: `stdlib.SignumFunc`).
pub fn signum_func() -> Function {
    todo!()
}

/// The function behind [`parse_int`] (go-cty: `stdlib.ParseIntFunc`).
pub fn parse_int_func() -> Function {
    todo!()
}

/// The absolute value of a number (go-cty: `stdlib.Absolute`).
pub fn absolute(num: &Value) -> Result<Value, Error> {
    let _ = num;
    todo!()
}

/// The sum of two numbers (go-cty: `stdlib.Add`).
pub fn add(a: &Value, b: &Value) -> Result<Value, Error> {
    let _ = (a, b);
    todo!()
}

/// The difference of two numbers (go-cty: `stdlib.Subtract`).
pub fn subtract(a: &Value, b: &Value) -> Result<Value, Error> {
    let _ = (a, b);
    todo!()
}

/// The product of two numbers (go-cty: `stdlib.Multiply`).
pub fn multiply(a: &Value, b: &Value) -> Result<Value, Error> {
    let _ = (a, b);
    todo!()
}

/// The quotient of two numbers (go-cty: `stdlib.Divide`).
pub fn divide(a: &Value, b: &Value) -> Result<Value, Error> {
    let _ = (a, b);
    todo!()
}

/// The remainder of dividing two numbers (go-cty: `stdlib.Modulo`).
pub fn modulo(a: &Value, b: &Value) -> Result<Value, Error> {
    let _ = (a, b);
    todo!()
}

/// The additive inverse of a number (go-cty: `stdlib.Negate`).
pub fn negate(num: &Value) -> Result<Value, Error> {
    let _ = num;
    todo!()
}

/// Whether `a` is greater than `b` (go-cty: `stdlib.GreaterThan`).
pub fn greater_than(a: &Value, b: &Value) -> Result<Value, Error> {
    let _ = (a, b);
    todo!()
}

/// Whether `a` is greater than or equal to `b`
/// (go-cty: `stdlib.GreaterThanOrEqualTo`).
pub fn greater_than_or_equal_to(a: &Value, b: &Value) -> Result<Value, Error> {
    let _ = (a, b);
    todo!()
}

/// Whether `a` is less than `b` (go-cty: `stdlib.LessThan`).
pub fn less_than(a: &Value, b: &Value) -> Result<Value, Error> {
    let _ = (a, b);
    todo!()
}

/// Whether `a` is less than or equal to `b`
/// (go-cty: `stdlib.LessThanOrEqualTo`).
pub fn less_than_or_equal_to(a: &Value, b: &Value) -> Result<Value, Error> {
    let _ = (a, b);
    todo!()
}

/// The smallest of the given numbers (go-cty: `stdlib.Min`).
pub fn min(numbers: &[Value]) -> Result<Value, Error> {
    let _ = numbers;
    todo!()
}

/// The largest of the given numbers (go-cty: `stdlib.Max`).
pub fn max(numbers: &[Value]) -> Result<Value, Error> {
    let _ = numbers;
    todo!()
}

/// The integer part of a number, truncating toward zero (go-cty: `stdlib.Int`).
pub fn int(num: &Value) -> Result<Value, Error> {
    let _ = num;
    todo!()
}

/// The smallest integer greater than or equal to the number
/// (go-cty: `stdlib.Ceil`).
pub fn ceil(num: &Value) -> Result<Value, Error> {
    let _ = num;
    todo!()
}

/// The largest integer less than or equal to the number
/// (go-cty: `stdlib.Floor`).
pub fn floor(num: &Value) -> Result<Value, Error> {
    let _ = num;
    todo!()
}

/// The logarithm of `num` in base `base` (go-cty: `stdlib.Log`).
pub fn log(num: &Value, base: &Value) -> Result<Value, Error> {
    let _ = (num, base);
    todo!()
}

/// `num` raised to the power `power` (go-cty: `stdlib.Pow`).
pub fn pow(num: &Value, power: &Value) -> Result<Value, Error> {
    let _ = (num, power);
    todo!()
}

/// The sign of the number as -1, 0, or 1 (go-cty: `stdlib.Signum`).
pub fn signum(num: &Value) -> Result<Value, Error> {
    let _ = num;
    todo!()
}

/// Parses an integer from a string in the given base
/// (go-cty: `stdlib.ParseInt`).
pub fn parse_int(num: &Value, base: &Value) -> Result<Value, Error> {
    let _ = (num, base);
    todo!()
}

// --- regexp.go ---

/// The function behind [`regex`] (go-cty: `stdlib.RegexFunc`).
pub fn regex_func() -> Function {
    todo!()
}

/// The function behind [`regex_all`] (go-cty: `stdlib.RegexAllFunc`).
pub fn regex_all_func() -> Function {
    todo!()
}

/// The captures of the first match of the pattern in the string
/// (go-cty: `stdlib.Regex`).
pub fn regex(pattern: &Value, str_val: &Value) -> Result<Value, Error> {
    let _ = (pattern, str_val);
    todo!()
}

/// The captures of all matches of the pattern in the string
/// (go-cty: `stdlib.RegexAll`).
pub fn regex_all(pattern: &Value, str_val: &Value) -> Result<Value, Error> {
    let _ = (pattern, str_val);
    todo!()
}

// --- sequence.go ---

/// The function behind [`concat`] (go-cty: `stdlib.ConcatFunc`).
pub fn concat_func() -> Function {
    todo!()
}

/// The function behind [`range`] (go-cty: `stdlib.RangeFunc`).
pub fn range_func() -> Function {
    todo!()
}

/// The given sequences concatenated into a single tuple or list
/// (go-cty: `stdlib.Concat`).
pub fn concat(seqs: &[Value]) -> Result<Value, Error> {
    let _ = seqs;
    todo!()
}

/// A list of numbers counted from a start to a limit by a step; accepts one,
/// two, or three arguments (go-cty: `stdlib.Range`).
pub fn range(params: &[Value]) -> Result<Value, Error> {
    let _ = params;
    todo!()
}

// --- set.go ---

/// The function behind [`set_has_element`] (go-cty: `stdlib.SetHasElementFunc`).
pub fn set_has_element_func() -> Function {
    todo!()
}

/// The function behind [`set_union`] (go-cty: `stdlib.SetUnionFunc`).
pub fn set_union_func() -> Function {
    todo!()
}

/// The function behind [`set_intersection`]
/// (go-cty: `stdlib.SetIntersectionFunc`).
pub fn set_intersection_func() -> Function {
    todo!()
}

/// The function behind [`set_subtract`] (go-cty: `stdlib.SetSubtractFunc`).
pub fn set_subtract_func() -> Function {
    todo!()
}

/// The function behind [`set_symmetric_difference`]
/// (go-cty: `stdlib.SetSymmetricDifferenceFunc`).
pub fn set_symmetric_difference_func() -> Function {
    todo!()
}

/// Whether the set contains the given element
/// (go-cty: `stdlib.SetHasElement`).
pub fn set_has_element(set: &Value, elem: &Value) -> Result<Value, Error> {
    let _ = (set, elem);
    todo!()
}

/// The union of the given sets (go-cty: `stdlib.SetUnion`).
pub fn set_union(sets: &[Value]) -> Result<Value, Error> {
    let _ = sets;
    todo!()
}

/// The intersection of the given sets (go-cty: `stdlib.SetIntersection`).
pub fn set_intersection(sets: &[Value]) -> Result<Value, Error> {
    let _ = sets;
    todo!()
}

/// The elements of `a` not present in `b` (go-cty: `stdlib.SetSubtract`).
pub fn set_subtract(a: &Value, b: &Value) -> Result<Value, Error> {
    let _ = (a, b);
    todo!()
}

/// The elements present in exactly one of the given sets
/// (go-cty: `stdlib.SetSymmetricDifference`).
pub fn set_symmetric_difference(sets: &[Value]) -> Result<Value, Error> {
    let _ = sets;
    todo!()
}

// --- string.go ---

/// The function behind [`upper`] (go-cty: `stdlib.UpperFunc`).
pub fn upper_func() -> Function {
    todo!()
}

/// The function behind [`lower`] (go-cty: `stdlib.LowerFunc`).
pub fn lower_func() -> Function {
    todo!()
}

/// The function behind [`reverse`] (go-cty: `stdlib.ReverseFunc`).
pub fn reverse_func() -> Function {
    todo!()
}

/// The function behind [`strlen`] (go-cty: `stdlib.StrlenFunc`).
pub fn strlen_func() -> Function {
    todo!()
}

/// The function behind [`substr`] (go-cty: `stdlib.SubstrFunc`).
pub fn substr_func() -> Function {
    todo!()
}

/// The function behind [`join`] (go-cty: `stdlib.JoinFunc`).
pub fn join_func() -> Function {
    todo!()
}

/// The function behind [`sort`] (go-cty: `stdlib.SortFunc`).
pub fn sort_func() -> Function {
    todo!()
}

/// The function behind [`split`] (go-cty: `stdlib.SplitFunc`).
pub fn split_func() -> Function {
    todo!()
}

/// The function behind [`chomp`] (go-cty: `stdlib.ChompFunc`).
pub fn chomp_func() -> Function {
    todo!()
}

/// The function behind [`indent`] (go-cty: `stdlib.IndentFunc`).
pub fn indent_func() -> Function {
    todo!()
}

/// The function behind [`title`] (go-cty: `stdlib.TitleFunc`).
pub fn title_func() -> Function {
    todo!()
}

/// The function behind [`trim_space`] (go-cty: `stdlib.TrimSpaceFunc`).
pub fn trim_space_func() -> Function {
    todo!()
}

/// The function behind [`trim`] (go-cty: `stdlib.TrimFunc`).
pub fn trim_func() -> Function {
    todo!()
}

/// The function behind [`trim_prefix`] (go-cty: `stdlib.TrimPrefixFunc`).
pub fn trim_prefix_func() -> Function {
    todo!()
}

/// The function behind [`trim_suffix`] (go-cty: `stdlib.TrimSuffixFunc`).
pub fn trim_suffix_func() -> Function {
    todo!()
}

/// The string converted to uppercase (go-cty: `stdlib.Upper`).
pub fn upper(str_val: &Value) -> Result<Value, Error> {
    let _ = str_val;
    todo!()
}

/// The string converted to lowercase (go-cty: `stdlib.Lower`).
pub fn lower(str_val: &Value) -> Result<Value, Error> {
    let _ = str_val;
    todo!()
}

/// The string with its grapheme clusters in reverse order
/// (go-cty: `stdlib.Reverse`).
pub fn reverse(str_val: &Value) -> Result<Value, Error> {
    let _ = str_val;
    todo!()
}

/// The number of grapheme clusters in the string (go-cty: `stdlib.Strlen`).
pub fn strlen(str_val: &Value) -> Result<Value, Error> {
    let _ = str_val;
    todo!()
}

/// A substring by grapheme-cluster offset and length
/// (go-cty: `stdlib.Substr`).
pub fn substr(str_val: &Value, offset: &Value, length: &Value) -> Result<Value, Error> {
    let _ = (str_val, offset, length);
    todo!()
}

/// The list elements joined with a separator (go-cty: `stdlib.Join`).
pub fn join(separator: &Value, lists: &[Value]) -> Result<Value, Error> {
    let _ = (separator, lists);
    todo!()
}

/// The list of strings sorted lexically (go-cty: `stdlib.Sort`).
pub fn sort(list: &Value) -> Result<Value, Error> {
    let _ = list;
    todo!()
}

/// The string split on a separator (go-cty: `stdlib.Split`).
pub fn split(separator: &Value, str_val: &Value) -> Result<Value, Error> {
    let _ = (separator, str_val);
    todo!()
}

/// The string with trailing newlines removed (go-cty: `stdlib.Chomp`).
pub fn chomp(str_val: &Value) -> Result<Value, Error> {
    let _ = str_val;
    todo!()
}

/// The string with each line after the first indented by the given number of
/// spaces (go-cty: `stdlib.Indent`).
pub fn indent(spaces: &Value, str_val: &Value) -> Result<Value, Error> {
    let _ = (spaces, str_val);
    todo!()
}

/// The string with the first letter of each word capitalized
/// (go-cty: `stdlib.Title`).
pub fn title(str_val: &Value) -> Result<Value, Error> {
    let _ = str_val;
    todo!()
}

/// The string with leading and trailing whitespace removed
/// (go-cty: `stdlib.TrimSpace`).
pub fn trim_space(str_val: &Value) -> Result<Value, Error> {
    let _ = str_val;
    todo!()
}

/// The string with leading and trailing characters from the cutset removed
/// (go-cty: `stdlib.Trim`).
pub fn trim(str_val: &Value, cutset: &Value) -> Result<Value, Error> {
    let _ = (str_val, cutset);
    todo!()
}

/// The string with the given prefix removed, if present
/// (go-cty: `stdlib.TrimPrefix`).
pub fn trim_prefix(str_val: &Value, prefix: &Value) -> Result<Value, Error> {
    let _ = (str_val, prefix);
    todo!()
}

/// The string with the given suffix removed, if present
/// (go-cty: `stdlib.TrimSuffix`).
pub fn trim_suffix(str_val: &Value, suffix: &Value) -> Result<Value, Error> {
    let _ = (str_val, suffix);
    todo!()
}

// --- string_replace.go ---

/// The function behind [`replace`] (go-cty: `stdlib.ReplaceFunc`).
pub fn replace_func() -> Function {
    todo!()
}

/// The function behind [`regex_replace`] (go-cty: `stdlib.RegexReplaceFunc`).
pub fn regex_replace_func() -> Function {
    todo!()
}

/// The string with all occurrences of a substring replaced
/// (go-cty: `stdlib.Replace`).
pub fn replace(str_val: &Value, substr: &Value, replace: &Value) -> Result<Value, Error> {
    let _ = (str_val, substr, replace);
    todo!()
}

/// The string with all matches of a regular expression pattern replaced
/// (go-cty: `stdlib.RegexReplace`).
pub fn regex_replace(str_val: &Value, pattern: &Value, replace: &Value) -> Result<Value, Error> {
    let _ = (str_val, pattern, replace);
    todo!()
}
