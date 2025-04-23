use std::{
    io::{Error, ErrorKind},
    process::ExitStatus,
};

#[derive(Debug)]
pub enum ShellError {
    CommandNotFound(String),
    CommandFailure(String, ExitStatus),
    InvalidArgument(String),
    LexerError(String),
    ParserError(String),
    IoError(Error),
}

#[derive(Debug, PartialEq)]
pub enum ShellErrorKind {
    CommandNotFound,
    CommandFailure,
    InvalidArgument,
    LexerError,
    ParserError,
    IoError,
}

impl ShellError {
    pub fn kind(&self) -> ShellErrorKind {
        match self {
            ShellError::CommandNotFound(_) => ShellErrorKind::CommandNotFound,
            ShellError::CommandFailure(_, _) => ShellErrorKind::CommandFailure,
            ShellError::InvalidArgument(_) => ShellErrorKind::InvalidArgument,
            ShellError::LexerError(_) => ShellErrorKind::LexerError,
            ShellError::ParserError(_) => ShellErrorKind::ParserError,
            ShellError::IoError(_) => ShellErrorKind::IoError,
        }
    }
}

impl std::error::Error for ShellError {}

impl std::fmt::Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellError::CommandNotFound(cmd) => write!(f, "Unknown Command: {}", cmd),
            ShellError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            ShellError::CommandFailure(cmd, exit_status) => {
                write!(f, "'{}' Failed: Exit Code: {}", cmd, exit_status)
            }
            ShellError::LexerError(msg) => write!(f, "Error occured during lexing: {}", msg),
            ShellError::ParserError(msg) => write!(f, "Error occured during parsing: {}", msg),
            ShellError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<&str> for ShellError {
    fn from(error: &str) -> Self {
        ShellError::IoError(Error::new(ErrorKind::Other, error))
    }
}

impl From<Error> for ShellError {
    fn from(error: Error) -> Self {
        ShellError::IoError(error)
    }
}
