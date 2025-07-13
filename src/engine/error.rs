use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum ShellError {
    Parser(String),
    Unnecassary,
    IoError(Error),
}

#[derive(Debug, PartialEq)]
pub enum ShellErrorKind {
    Parser,
    Unnecassary,
    IoError,
}

impl ShellError {
    pub fn kind(&self) -> ShellErrorKind {
        match self {
            ShellError::Parser(_) => ShellErrorKind::Parser,
            ShellError::Unnecassary => ShellErrorKind::Unnecassary,
            ShellError::IoError(_) => ShellErrorKind::IoError,
        }
    }
}

impl std::error::Error for ShellError {}

impl std::fmt::Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellError::Parser(msg) => write!(f, "{msg}"),
            ShellError::Unnecassary => write!(f, "Unnecassary Error"),
            ShellError::IoError(e) => write!(f, "{e}"),
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
