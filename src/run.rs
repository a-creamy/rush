mod bic;
mod error;
mod executor;
mod node;
mod parser;
use crate::run::error::ShellError;

pub fn execute(input: &str) {
    match executor::execute(parser::parse(input)) {
        Ok(()) => (),
        Err(ShellError::CommandNotFound(cmd)) => {
            eprintln!("rush: Unknown Command: {}", cmd)
        }
        Err(ShellError::InvalidArgument(msg)) => {
            eprintln!("rush: {}", msg)
        }
        Err(ShellError::BicError(msg)) => {
            eprintln!("{msg}")
        }
        Err(ShellError::IoError(e)) => {
            eprintln!("rush: {}", e)
        }
    }
}
