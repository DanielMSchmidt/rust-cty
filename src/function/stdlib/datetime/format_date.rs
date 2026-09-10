//! `format_date` (go-cty: `cty/function/stdlib/datetime.go`).

use crate::error::CtyError;
use crate::function::Function;
use crate::value::Value;

/// The function behind [`format_date`] (go-cty: `stdlib.FormatDateFunc`).
pub fn format_date_func() -> Function {
    todo!()
}

/// Formats an RFC 3339 timestamp per the given format string
/// (go-cty: `stdlib.FormatDate`).
pub fn format_date(format: &Value, timestamp: &Value) -> Result<Value, CtyError> {
    let _ = (format, timestamp);
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

    // Ported from TestFormatDate (format table):
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/datetime_test.go#L11
    #[test]
    #[ignore = "not yet implemented"]
    fn format_date() {
        // NOTE(port): upstream uses cty.NilVal as the (unused) Want in the error
        // cases; represented here as None since NilVal has no Rust analogue.
        let tests: Vec<(Value, Option<Value>, &str)> = vec![
            (
                Value::string(""), // pointless, but valid
                Some(Value::string("")),
                "",
            ),
            (
                Value::string("YYYY-MM-DD"),
                Some(Value::string("2006-01-02")),
                "",
            ),
            (
                Value::string("EEE, MMM D ''YY"),
                Some(Value::string("Mon, Jan 2 '06")),
                "",
            ),
            (
                Value::string("hh:mm:ss"),
                Some(Value::string("15:04:05")),
                "",
            ),
            (
                Value::string("H 'o''clock' AA"),
                Some(Value::string("3 o'clock PM")),
                "",
            ),
            (
                Value::string("H 'o''clock'"),
                Some(Value::string("3 o'clock")),
                "",
            ),
            (
                Value::string("hh:mm:ssZZZZ"),
                Some(Value::string("15:04:05+0000")),
                "",
            ),
            (
                Value::string("hh:mm:ssZZZZZ"),
                Some(Value::string("15:04:05+00:00")),
                "",
            ),
            (Value::string("MMMM"), Some(Value::string("January")), ""),
            (Value::string("EEEE"), Some(Value::string("Monday")), ""),
            (Value::string("aa"), Some(Value::string("pm")), ""),
            // Some common standard machine-oriented formats
            (
                Value::string("YYYY-MM-DD'T'hh:mm:ssZ"),     // RFC3339
                Some(Value::string("2006-01-02T15:04:05Z")), // (since RFC3339 is the input format too, this is a bit pointless)
                "",
            ),
            (
                Value::string("DD MMM YYYY hh:mm ZZZ"), // RFC822
                Some(Value::string("02 Jan 2006 15:04 UTC")),
                "",
            ),
            (
                Value::string("EEEE, DD-MMM-YY hh:mm:ss ZZZ"), // RFC850
                Some(Value::string("Monday, 02-Jan-06 15:04:05 UTC")),
                "",
            ),
            (
                Value::string("EEE, DD MMM YYYY hh:mm:ss ZZZ"), // RFC1123
                Some(Value::string("Mon, 02 Jan 2006 15:04:05 UTC")),
                "",
            ),
            // Invalids
            (
                Value::string("Y"),
                None,
                r#"invalid date format verb "Y": year must either be "YY" or "YYYY""#,
            ),
            (
                Value::string("YYYYY"),
                None,
                r#"invalid date format verb "YYYYY": year must either be "YY" or "YYYY""#,
            ),
            (
                Value::string("A"),
                None,
                r#"invalid date format verb "A": must be "AA""#,
            ),
            (
                Value::string("a"),
                None,
                r#"invalid date format verb "a": must be "aa""#,
            ),
            (Value::string("'blah blah"), None, "unterminated literal '"),
            (Value::string("'"), None, "unterminated literal '"),
        ];

        // Upstream: time.Date(2006, time.January, 2, 15, 04, 05, 0, time.UTC)
        // formatted as RFC3339.
        let time_val = Value::string("2006-01-02T15:04:05Z");

        for (i, (format, want, want_err)) in tests.iter().enumerate() {
            let result = stdlib::format_date(format, &time_val);

            if !want_err.is_empty() {
                let err = match result {
                    Ok(_) => panic!("case {i}: no error; want error {want_err:?}"),
                    Err(err) => err,
                };
                assert_eq!(
                    err.to_string(),
                    *want_err,
                    "case {i}: wrong error\ngot:  {err}\nwant: {want_err}"
                );
            } else {
                let got = result.unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
                assert_eq!(got, *want.as_ref().unwrap(), "case {i}: wrong result");
            }
        }
    }

    // Ported from TestFormatDate (parse error tests):
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/datetime_test.go#L11
    #[test]
    #[ignore = "not yet implemented"]
    fn format_date_parse_errors() {
        let parse_err_tests: Vec<(Value, &str)> = vec![
            (
                Value::string(""),
                "not a valid RFC3339 timestamp: end of string before year",
            ),
            (
                Value::string("2017-01-02"),
                "not a valid RFC3339 timestamp: missing required time introducer 'T'",
            ),
            (
                Value::string("2017-12-02t00:00:00Z"),
                "not a valid RFC3339 timestamp: missing required time introducer 'T'",
            ),
            (
                Value::string("2017:01:02"),
                r#"not a valid RFC3339 timestamp: found ":01:02" where "-" is expected"#,
            ),
            (
                Value::string("2017"),
                r#"not a valid RFC3339 timestamp: end of string where "-" is expected"#,
            ),
            (
                Value::string("2017-01-02T"),
                "not a valid RFC3339 timestamp: end of string before hour",
            ),
            (
                Value::string("2017-01-02T00"),
                r#"not a valid RFC3339 timestamp: end of string where ":" is expected"#,
            ),
            (
                Value::string("2017-01-02T00:00:00"),
                "not a valid RFC3339 timestamp: end of string before UTC offset",
            ),
            (
                Value::string("2017-01-02T26:00:00Z"),
                "not a valid RFC3339 timestamp: hour must be between 0 and 23 inclusive",
            ),
            (
                Value::string("2017-13-02T00:00:00Z"),
                // This one generates an odd message due to an apparent quirk in
                // the Go time parser. Ideally it would use "13" as the errant string.
                r#"not a valid RFC3339 timestamp: cannot use "-02T00:00:00Z" as month"#,
            ),
            (
                Value::string("2017-02-31T00:00:00Z"),
                "not a valid RFC3339 timestamp: day out of range",
            ),
            (
                Value::string(r#""2017-12-02T00:00:00Z""#),
                r#"not a valid RFC3339 timestamp: cannot use "\"2017-12-02T00:00:00Z\"" as year"#,
            ),
            (
                Value::string("2-12-02T00:00:00Z"),
                // Go parser seems to be trying to parse the whole thing as a year
                // here, producing a confusing error message.
                r#"not a valid RFC3339 timestamp: cannot use "2-12-02T00:00:00Z" as year"#,
            ),
            (
                Value::string("2000-01-01T1:12:34Z"),
                "not a valid RFC3339 timestamp: hour must have exactly two digits",
            ),
            (
                Value::string("2000-01-01T01:1:34Z"),
                "not a valid RFC3339 timestamp: minute must have exactly two digits",
            ),
            (
                Value::string("2000-01-01T01:01:1Z"),
                r#"not a valid RFC3339 timestamp: cannot use "1Z" as second"#,
            ),
            (
                Value::string("2000-01-01T00:00:00,000Z"),
                r#"not a valid RFC3339 timestamp: cannot use "," as timestamp segment"#,
            ),
            (
                Value::string("2000-01-01T00:00:00+24:00"),
                r#"not a valid RFC3339 timestamp: cannot use "+24:00" as UTC offset"#,
            ),
            (
                Value::string("2000-01-01T00:00:00+00:60"),
                r#"not a valid RFC3339 timestamp: cannot use "+00:60" as UTC offset"#,
            ),
        ];

        for (i, (timestamp, want_err)) in parse_err_tests.iter().enumerate() {
            let result = stdlib::format_date(&Value::string(""), timestamp);

            let err = match result {
                Ok(_) => panic!("case {i}: no error; want error {want_err:?}"),
                Err(err) => err,
            };
            assert_eq!(
                err.to_string(),
                *want_err,
                "case {i}: wrong error\ngot:  {err}\nwant: {want_err}"
            );
        }
    }

    // Ported from TestFormatDate (parse success tests):
    // https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/function/stdlib/datetime_test.go#L11
    #[test]
    #[ignore = "not yet implemented"]
    fn format_date_parse_success() {
        let parse_success_tests: Vec<(&str, &str, &str)> = vec![
            (
                "2022-03-01T00:23:45Z",
                "2022-03-01T00:23:45Z",
                "Tuesday, 01-Mar-22 00:23:45 UTC",
            ),
            (
                "2022-03-01T00:23:45+00:00",
                "2022-03-01T00:23:45Z",
                "Tuesday, 01-Mar-22 00:23:45 UTC",
            ),
            (
                "2022-03-01T00:23:45+01:00",
                "2022-03-01T00:23:45+01:00",
                "Tuesday, 01-Mar-22 00:23:45 +0100",
            ),
            (
                "2022-03-01T00:23:45-01:00",
                "2022-03-01T00:23:45-01:00",
                "Tuesday, 01-Mar-22 00:23:45 -0100",
            ),
            (
                "1900-01-01T00:00:00Z",
                "1900-01-01T00:00:00Z",
                "Monday, 01-Jan-00 00:00:00 UTC",
            ),
        ];

        const RFC3339_FORMAT: &str = "YYYY-MM-DD'T'hh:mm:ssZ";
        const RFC850_FORMAT: &str = "EEEE, DD-MMM-YY hh:mm:ss ZZZ";

        for (i, (input, want_rfc3339, want_rfc850)) in parse_success_tests.iter().enumerate() {
            // RFC3339
            let got = stdlib::format_date(&Value::string(RFC3339_FORMAT), &Value::string(*input))
                .unwrap_or_else(|err| panic!("case {i} RFC3339: unexpected error: {err}"));
            assert_eq!(
                got.as_string(),
                *want_rfc3339,
                "case {i} RFC3339: wrong result"
            );

            // RFC850
            let got = stdlib::format_date(&Value::string(RFC850_FORMAT), &Value::string(*input))
                .unwrap_or_else(|err| panic!("case {i} RFC850: unexpected error: {err}"));
            assert_eq!(
                got.as_string(),
                *want_rfc850,
                "case {i} RFC850: wrong result"
            );
        }
    }
}
