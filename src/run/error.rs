use std::{io, process::ExitStatus};

#[derive(Debug)]
pub enum ShellError {
    CommandNotFound(String),
    ExpectedCommand,
    CommandFailure(String, ExitStatus),
    InvalidArgument(String),
    BicError(String),
    IoError(io::Error),
}

impl std::error::Error for ShellError {}

impl std::fmt::Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellError::CommandNotFound(cmd) => write!(f, "Unknown Command: {}", cmd),
            ShellError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            ShellError::ExpectedCommand => write!(f, "Expected command"),
            ShellError::CommandFailure(cmd, exit_status) => write!(f, "Command '{}' failed with status code: {}", cmd, exit_status),
            ShellError::BicError(msg) => write!(f, "{}", msg),
            ShellError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<io::Error> for ShellError {
    fn from(error: io::Error) -> Self {
        ShellError::IoError(error)
    }
}
