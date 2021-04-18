use std::fmt::Formatter;
use std::{error, fmt};

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    EndOfFile,

    /// Hint to avoid relying on exhaustive match which would break if more errors are added.
    #[doc(hidden)]
    __NonExhaustive,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::EndOfFile => write!(f, "End of file reached."),
            ErrorKind::__NonExhaustive => unreachable!(),
        }
    }
}

impl error::Error for Error {}
