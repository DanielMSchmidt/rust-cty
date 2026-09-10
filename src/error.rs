//! Error type shared across the crate.
//!
//! go-cty returns `error` values whose messages are part of the observable behavior
//! (several upstream tests assert on exact message strings). The conformance tests
//! therefore compare `Error`'s `Display` output against the upstream literals.

use std::num::ParseFloatError;

use crate::path::Path;
use thiserror::Error;

/// CtyError is the error type for the cty library
#[derive(Error, Debug)]
pub enum CtyError {
    /// ParseFloat is thrown if the value could not be parsed as a float
    #[error("could not parse value as float")]
    ParseFloat(#[from] ParseFloatError),
    /// InconsistentList is thrown if the list has differently typed values
    #[error("expected all elements of the list to be of type {expected:?}, but found {found:?}")]
    InconsistentList {
        /// The first type we find sets the expectation
        expected: crate::Type,
        /// The type that differed from the expectation
        found: crate::Type,
    },
    /// EmptyList is thrown if a list is being constructed without an element
    #[error("got an empty list, expected at least one element")]
    EmptyList,
    /// Unknown error
    #[error("unknown error")]
    Unknown,
}

impl CtyError {
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
