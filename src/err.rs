use std::{io, error};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum SolverError {
    InvalidPuzzle(String),
    DictionaryNotFound(String),
    ReadError(io::Error),
}

use SolverError::*;

impl error::Error for SolverError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            InvalidPuzzle(_) => None,
            DictionaryNotFound(_) => None,
            ReadError(ref cause) => Some(cause),
        }
    }
}

impl Display for SolverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            InvalidPuzzle(ref reason) => {
                write!(f, "Puzzle is not valid. {reason}")
            }
            DictionaryNotFound(ref path) => {
                write!(f, "Dictionary not found: '{path}'")
            }
            ReadError(ref err) => {
                err.fmt(f)
            }
        }
    }
}

 impl From<io::Error> for SolverError {
    fn from(err: io::Error) -> SolverError {
        ReadError(err)
    }
 }
