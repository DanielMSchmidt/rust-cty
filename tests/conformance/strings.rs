//! Conformance tests transcribed from go-cty
//! (github.com/zclconf/go-cty @ a918e1174fcf2a25b7a222e7e78b00ea40ace26c):
//!   cty/ctystrings/prefix_test.go
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::strings::safe_known_prefix;

// Ported from TestSafeKnownPrefix:
// https://github.com/zclconf/go-cty/blob/a918e1174fcf2a25b7a222e7e78b00ea40ace26c/cty/ctystrings/prefix_test.go#L7
#[test]
#[ignore = "not yet implemented"]
fn safe_known_prefix_test() {
    // (input, want)
    let tests: Vec<(&str, &str)> = vec![
        // NOTE: Under future improvements to SafeKnownPrefix the "Want"
        // results for all of these tests can safely get longer, thereby
        // describing a more precise constraint, but we should avoid making
        // them shorter because that will weaken existing constraints from
        // older versions.
        // (We might make exceptions for behaviors that are found to be
        // clearly wrong, but consider the consequences carefully.)
        ("", ""),
        // The "a" is discarded because it might combine with diacritics to follow
        ("a", ""),
        // The final o is discarded because it might combine with diacritics to follow
        ("boo", "bo"),
        // The final \r is discarded because it could combine with \r\n to produce a single grapheme cluster
        ("boop\r", "boop"),
        // Hangul syllables can combine arbitrarily, so we must trim of trailing ones
        // (U+AC00 is the Hangul syllable "가")
        ("hello \u{ac00}", "hello "),
        // We conservatively trim the whole emoji sequence because other emoji
        // modifiers might come in later unicode specs
        // (the sequence is "🤷🏽‍♂️": person shrugging, medium skin tone,
        // zero-width joiner, male sign, variation selector-16)
        ("hello \u{1f937}\u{1f3fd}\u{200d}\u{2642}\u{fe0f}", "hello "),
        // A subsequent character avoids the need to trim
        (
            "hello \u{1f937}\u{1f3fd}\u{200d}\u{2642}\u{fe0f} ",
            "hello \u{1f937}\u{1f3fd}\u{200d}\u{2642}\u{fe0f} ",
        ),
        // "Person Shrugging" (U+1F937, "🤷") can potentially combine with
        // subsequent skin tone modifiers or ZWJ followed by gender
        // presentation modifiers
        ("hello \u{1f937}", "hello "),
        // A subsequent character avoids the need to trim
        ("hello \u{1f937} ", "hello \u{1f937} "),
        // U+200D is "zero width joiner"; the "Person Shrugging" followed by
        // zero with joiner anticipates a subsequent modifier to join with
        ("hello \u{1f937}\u{200d}", "hello "),
        // U+1F1E6 is the beginning of a "regional indicator symbol", which are
        // supposed to appear in pairs but we only have one here. The symbol
        // was discarded because we can't know what character it represents
        // until we have both parts.
        ("hello \u{1f1e6}", "hello "),
        // This is a regional indicator symbol "AA", which happens to be Aruba
        // but it's not important exactly which country we're encoding. The
        // text segmentation spec allows any number of consecutive regional
        // indicators, so we must always discard any number of them at the end.
        ("hello \u{1f1e6}\u{1f1e6}", "hello "),
        // A subsequent character avoids the need to trim
        ("hello \u{1f1e6}\u{1f1e6} ", "hello \u{1f1e6}\u{1f1e6} "),
        // The following all rely on our additional heuristic about certain
        // commonly-used delimiters that we know can never be the beginning
        // of a combined grapheme cluster sequence. We make these exceptions
        // because cty tends to be used more often for constructing strings
        // for use by machines than for constructing text for human consumption.
        // e.g. prefix of an Amazon EC2 object identifier
        ("ami-", "ami-"),
        // e.g. prefix of a variable name
        ("foo_", "foo_"),
        // e.g. prefix of a JSON object
        (r#"{"foo":"#, r#"{"foo":"#),
        // e.g. prefix of a program in a C-like language?
        ("beep();", "beep();"),
        // e.g. prefix of a URL with a known scheme
        ("https://", "https://"),
        // e.g. windows filesystem path with a known drive letter
        (r"c:\", r"c:\"),
        // e.g. prefix of a JSON document that includes a partially-known array
        (r#"["foo","#, r#"["foo","#),
        // e.g. prefix of a traversal through attributes
        ("foo.bar.", "foo.bar."),
        // e.g. prefix of a program in a C-like language?
        ("beep(", "beep("),
        // e.g. prefix of a program in a C-like language?
        ("beep()", "beep()"),
        // e.g. prefix of a JSON object
        ("{", "{"),
        // e.g. fragment of JSON
        ("[{}", "[{}"),
        // e.g. prefix of a JSON array
        ("[", "["),
        // e.g. fragment of JSON
        ("[[]", "[[]"),
        // e.g. partial Unix-style command line
        ("whatever |", "whatever |"),
        // e.g. prefix of a URL with a query string
        ("https://example.com/foo?", "https://example.com/foo?"),
        // dunno but seems weird to have ? without !
        ("boop!", "boop!"),
        // A reference to somebody's home directory
        ("ls ~", "ls ~"),
        // A space always disambiguates whether our suffix is safe
        ("a ", "a "),
        // A tab always disambiguates whether our suffix is safe
        ("a\t", "a\t"),
        // e.g. incomplete email address
        ("username@", "username@"),
        // e.g. start of a single-linecomment in some machine languages, or a "hashtag"
        ("#", "#"),
        // e.g. start of a reference to a Perl scalar
        ("print $", "print $"),
        // e.g. start of a reference to a Perl hash
        ("print %", "print %"),
        // e.g. start of a pessimistic version constraint in some version constraint syntaxes
        ("^", "^"),
        // e.g. the "address of" operator in some programming languages
        ("foo(&", "foo(&"),
        // e.g. multiplying by something
        ("foo *", "foo *"),
        // e.g. addition
        ("foo +", "foo +"),
        // e.g. we know it's a JSON string but we don't know the content yet
        (r#"[""#, r#"[""#),
        // e.g. a string in a JSON-like language that also supports single quotes!
        ("['", "['"),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got = safe_known_prefix(input);
        assert_eq!(
            got, *want,
            "case {i}: wrong result\ninput: {input:?}\ngot:   {got:?}\nwant:  {want:?}"
        );
    }
}
