//! String normalization helpers (go-cty: the `ctystrings` package).

/// Normalizes a string to Unicode NFC form, as applied by `Value::string`
/// (go-cty: `ctystrings.Normalize`).
pub fn normalize(s: &str) -> String {
    let _ = s;
    todo!()
}

/// Trims a known string prefix so it is safe to use as a refinement even if
/// the final string continues with combining characters
/// (go-cty: `ctystrings.SafeKnownPrefix`).
pub fn safe_known_prefix(prefix: &str) -> String {
    let _ = prefix;
    todo!()
}
