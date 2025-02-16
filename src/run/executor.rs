use super::error::ShellError;
use super::node::Ast;
use crate::run::bic;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn execute(node: &Ast) -> Result<(), ShellError> {
    match node {
        Ast::Command(args) => execute_command(args),
        Ast::Pipeline(lhs, rhs) => execute_pipeline(lhs, rhs),
        Ast::AndLogical(lhs, rhs) => execute_and(lhs, rhs),
        Ast::OverwriteRedirection(lhs, rhs) => execute_redirection(lhs, rhs, true),
        Ast::AppendRedirection(lhs, rhs) => execute_redirection(lhs, rhs, false),
        Ast::ErrorRedirection(lhs, rhs) => execute_error_redirection(lhs, rhs),
    }
}

fn log_error(filepath: &PathBuf, output: &str) -> Result<(), ShellError> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filepath)?;

    file.write_all(output.as_bytes())?;
    Ok(())
}

fn execute_error_redirection(lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
    let filepath = match rhs {
        Ast::Command(args) => PathBuf::from(&args[0]),
        _ => {
            return Err(ShellError::InvalidArgument(
                "Expected a filepath".to_string(),
            ));
        }
    };

    let args = match lhs {
        Ast::Command(cmd) => cmd,
        _ => {
            return Err(ShellError::InvalidArgument(
                "Expected a command for redirection".to_string(),
            ));
        }
    };

    match args[0].as_str() {
        "cd" => {
            let path = if args.len() > 1 { &args[1] } else { "" };
            match bic::cd(path) {
                Ok(_) => Ok(()),
                Err(e) => {
                    log_error(&filepath, &e.to_string())?;
                    Ok(())
                }
            }
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
            let output = Command::new(&args[0])
                .args(&args[1..])
                .stderr(Stdio::piped())
                .output()?;

            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(filepath)?;

            file.write_all(&output.stderr)?;
            Ok(())
        }
    }
}

fn execute_redirection(lhs: &Ast, rhs: &Ast, overwrite: bool) -> Result<(), ShellError> {
    let filepath = match rhs {
        Ast::Command(args) => PathBuf::from(&args[0]),
        _ => {
            return Err(ShellError::InvalidArgument(
                "Expected a filepath".to_string(),
            ));
        }
    };

    let args = match lhs {
        Ast::Command(cmd) => cmd,
        _ => {
            return Err(ShellError::InvalidArgument(
                "Expected a command for redirection".to_string(),
            ));
        }
    };

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
            let output = Command::new(&args[0]).args(&args[1..]).output()?;

            if output.status.success() {
                let mut file = if !overwrite {
                    OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(filepath)?
                } else {
                    File::create(filepath)?
                };

                file.write_all(&output.stdout)?;
                return Ok(());
            }

            Err(ShellError::CommandFailure(
                String::from_utf8_lossy(&output.stderr).to_string(),
                output.status,
            ))
        }
    }
}

fn execute_command(args: &[String]) -> Result<(), ShellError> {
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
        _ => execute_external_command(args),
    }
}

fn execute_external_command(args: &[String]) -> Result<(), ShellError> {
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
fn execute_pipeline(lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
    let lhs_command = match lhs {
        Ast::Command(args) => args,
        _ => {
            return Err(ShellError::InvalidArgument(
                "Pipeline left hand side must be a command".to_string(),
            ))
        }
    };
    let rhs_command = match rhs {
        Ast::Command(args) => args,
        _ => {
            return Err(ShellError::InvalidArgument(
                "Pipeline right hand side must be a command".to_string(),
            ))
        }
    };

    let lhs_process = Command::new(&lhs_command[0])
        .args(&lhs_command[1..])
        .stdout(Stdio::piped())
        .spawn()?;

    let rhs_process = Command::new(&rhs_command[0])
        .args(&rhs_command[1..])
        .stdin(lhs_process.stdout.unwrap())
        .status()?;

    if !rhs_process.success() {
        return Err(ShellError::CommandFailure(
            rhs_command[0].to_string(),
            rhs_process,
        ));
    }
    Ok(())
}

fn execute_and(lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
    execute(lhs)?;
    execute(rhs)?;
    Ok(())
}
