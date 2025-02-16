use crate::run::{bic, ShellError};
use std::{io, process::Command};

pub fn execute(args: &[String]) -> Result<(), ShellError> {
    if args.is_empty() {
        return Err(ShellError::ExpectedCommand);
    }

    match args[0].as_str() {
        "cd" => {
            let path = if args.len() > 1 { &args[1] } else { "" };
            bic::cd(path).map_err(ShellError::BicError)
        }
        "exit" => {
            let code = if args.len() > 1 {
                args[1].parse().unwrap_or(0)
            } else {
                0
            };
            bic::exit(code);
            Ok(())
        }
        _ => {
            let mut command = Command::new(&args[0]);
            if args.len() > 1 {
                command.args(&args[1..]);
            }

            match command.status() {
                Ok(status) => {
                    if !status.success() {
                        return Err(ShellError::CommandFailure(args[0].to_string(), status));
                    }
                    Ok(())
                }
                Err(e) if e.kind() == io::ErrorKind::NotFound => {
                    Err(ShellError::CommandNotFound(args[0].clone()))
                }
                Err(e) => Err(ShellError::IoError(e)),
            }
        }
    }
}
