//! `bytes_slice` (go-cty: `cty/function/stdlib/bytes.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`bytes_slice`] (go-cty: `stdlib.BytesSliceFunc`).
pub fn bytes_slice_func() -> Function {
    todo!()
}

/// A subrange of a bytes value, sharing the underlying buffer
/// (go-cty: `stdlib.BytesSlice`).
pub fn bytes_slice(buf: &Value, offset: &Value, length: &Value) -> Result<Value, CtyError> {
    let _ = (buf, offset, length);
    todo!()
}

#[cfg(test)]
mod conformance {
    //! Conformance tests transcribed from go-cty
    //! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
    //!   cty/function/stdlib/bool_test.go
    //!   cty/function/stdlib/bytes_test.go
    //!   cty/function/stdlib/csv_test.go
    //!   cty/function/stdlib/conversion_test.go
    //!   cty/function/stdlib/datetime_test.go
    //!   cty/function/stdlib/general_test.go
    //!   cty/function/stdlib/json_test.go
    //!
    //! Expected values are literals from the upstream tables; see
    //! docs/api-mapping.md for the Go→Rust API correspondence.

    use crate::Value;
    use crate::function::stdlib;

    // Ported from TestBytesSlice:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/bytes_test.go#L47
    #[test]
    #[ignore = "not yet implemented"]
    fn bytes_slice() {
        let tests: Vec<(Value, Value, Value, Value)> = vec![
            (
                stdlib::bytes_val(b"".to_vec()),
                Value::number(0),
                Value::number(0),
                stdlib::bytes_val(b"".to_vec()),
            ),
            (
                stdlib::bytes_val(b"a".to_vec()),
                Value::number(0),
                Value::number(1),
                stdlib::bytes_val(b"a".to_vec()),
            ),
            (
                stdlib::bytes_val(b"abc".to_vec()),
                Value::number(0),
                Value::number(2),
                stdlib::bytes_val(b"ab".to_vec()),
            ),
            (
                stdlib::bytes_val(b"abc".to_vec()),
                Value::number(1),
                Value::number(2),
                stdlib::bytes_val(b"bc".to_vec()),
            ),
            (
                stdlib::bytes_val(b"abc".to_vec()),
                Value::number(0),
                Value::number(3),
                stdlib::bytes_val(b"abc".to_vec()),
            ),
        ];

        for (i, (input, offset, length, want)) in tests.iter().enumerate() {
            let got = stdlib::bytes_slice(input, offset, length)
                .unwrap_or_else(|err| panic!("case {i}: {err}"));

            let got_bytes = got
                .encapsulated_value()
                .downcast_ref::<Vec<u8>>()
                .unwrap()
                .clone();
            let want_bytes = want
                .encapsulated_value()
                .downcast_ref::<Vec<u8>>()
                .unwrap()
                .clone();

            assert_eq!(got_bytes, want_bytes, "case {i}: wrong result");
        }
    }
}
