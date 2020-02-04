use crate::util::SplitArgError;
use serde::{de, ser};

#[cfg(not(feature = "std"))]
use core::{convert, fmt, result};
#[cfg(feature = "std")]
use std::{convert, fmt, result};

/// Alias for a [`Result`] with the error type [`stringly::Error`].
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`stringly::Error`]: enum.Error.html
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    Message(String),
    NotABoolean,
    NotAnInteger,
    NotAnUnsignedInteger,
    NotAFloatingPointNumber,
    NotASingleCharacter,
    NotAnEnum,
    NotAKeyValuePair,
    UnexpectedValueForUnit,
    TooManyElements,
    IndentTooSmall { lineno: usize },
    UnmatchedUnindent { lineno: usize },
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Message(ref msg) => f.write_str(msg),
            Error::NotABoolean => {
                f.write_str("expected a boolean (`true`, `yes`, `false`, `no`; case insensitive)")
            }
            Error::NotAnInteger => f.write_str("expected an integer"),
            Error::NotAnUnsignedInteger => f.write_str("expected an unsigned integer"),
            Error::NotAFloatingPointNumber => f.write_str("expected a floating point number"),
            Error::NotASingleCharacter => f.write_str("expected a single character"),
            Error::NotAnEnum => f.write_str("expected an enum (`VARIANT` or `VARIANT{ARGS}`"),
            Error::NotAKeyValuePair => f.write_str("expected a key-value pair (`KEY=VALUE`)"),
            Error::UnexpectedValueForUnit => f.write_str("unit got an unexpected value"),
            Error::TooManyElements => f.write_str("too many elements"),
            Error::IndentTooSmall { lineno } => write!(
                f,
                "line {}: indentation should be two or more space but got one",
                lineno
            ),
            Error::UnmatchedUnindent { lineno } => write!(
                f,
                "line {}: unindent does not match any outer indentation level",
                lineno
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl convert::From<SplitArgError> for Error {
    fn from(error: SplitArgError) -> Self {
        match error {
            SplitArgError::NotAnEnum => Error::NotAnEnum,
        }
    }
}
