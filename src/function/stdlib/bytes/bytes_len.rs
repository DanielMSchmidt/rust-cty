//! `bytes_len` (go-cty: `cty/function/stdlib/bytes.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`bytes_len`] (go-cty: `stdlib.BytesLenFunc`).
pub fn bytes_len_func() -> Function {
    todo!()
}

/// The length of a bytes value (go-cty: `stdlib.BytesLen`).
pub fn bytes_len(buf: &Value) -> Result<Value, CtyError> {
    let _ = buf;
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

    // Ported from TestBytesLen:
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/bytes_test.go#L10
    #[test]
    #[ignore = "not yet implemented"]
    fn bytes_len() {
        let tests: Vec<(Value, Value)> = vec![
            (stdlib::bytes_val(b"".to_vec()), Value::number(0)),
            (stdlib::bytes_val(b"a".to_vec()), Value::number(1)),
            (stdlib::bytes_val(b"abc".to_vec()), Value::number(3)),
        ];

        for (i, (input, want)) in tests.iter().enumerate() {
            let got = stdlib::bytes_len(input).unwrap_or_else(|err| panic!("case {i}: {err}"));
            assert_eq!(got, *want, "case {i}: wrong result");
        }
    }
}
