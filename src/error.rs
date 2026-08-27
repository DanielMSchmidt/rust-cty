//! Error type shared across the crate.
//!
//! go-cty returns `error` values whose messages are part of the observable behavior
//! (several upstream tests assert on exact message strings). The conformance tests
//! therefore compare `Error`'s `Display` output against the upstream literals.

use crate::path::Path;

/// An error produced by any fallible cty operation.
///
/// The `Display` implementation yields the user-facing message, which for ported
/// behavior must match the corresponding go-cty error string exactly.
#[derive(Debug, Clone)]
pub struct Error {
    _priv: (),
}

impl Error {
    /// Creates a new error with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        let _ = message.into();
        todo!()
    }

    /// Creates an error annotated with the path at which it occurred, mirroring
    /// go-cty's `cty.PathError`.
    pub fn new_at_path(path: Path, message: impl Into<String>) -> Self {
        let _ = (path, message.into());
        todo!()
    }

    /// The path this error is associated with, if any (go-cty: `cty.PathError.Path`).
    pub fn path(&self) -> Option<&Path> {
        todo!()
    }

    /// For argument errors produced by the function system, the index of the
    /// offending argument (go-cty: `function.ArgError.Index`).
    pub fn arg_index(&self) -> Option<usize> {
        todo!()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        todo!()
    }
}

impl std::error::Error for Error {}
