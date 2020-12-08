use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Unparseable line {0}")]
    UnparseableLine(String),
    #[error("Unknown instruction {0}")]
    UnknownInstruction(String),
    #[error("Missing parameter for instruction {0}")]
    MissingParameter(String),
    #[error("Unparseable parameter {1} for instruction {0}")]
    UnparseableParameter(String, String),
}

#[derive(Error, Debug)]
pub enum CodeParseError {
    #[error("{error} at line {line}")]
    AtLine { line: usize, error: ParseError },
    #[error("cannot load file {path} because of {error}")]
    IOError {
        path: PathBuf,
        error: std::io::Error,
    },
}

impl CodeParseError {
    pub fn at_line(line: usize, error: ParseError) -> CodeParseError {
        CodeParseError::AtLine { line, error }
    }

    pub fn from_io<P: AsRef<Path>>(path: P, error: std::io::Error) -> CodeParseError {
        CodeParseError::IOError {
            path: path.as_ref().to_owned(),
            error,
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum ExecutionError {
    #[error("The program tried to access instruction at {0} but it is not valid")]
    InvalidAccess(usize),
    #[error("Infinite loop detected at instruction {0}")]
    InfiniteLoop(usize),
}
