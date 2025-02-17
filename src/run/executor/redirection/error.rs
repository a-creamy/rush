use crate::run::{bic, error::ShellError, node::Ast};
use std::{
    fs::OpenOptions,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn execute(lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
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
                    OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(filepath)?
                        .write_all(e.to_string().as_bytes())?;

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

            OpenOptions::new()
                .append(true)
                .create(true)
                .open(filepath)?
                .write_all(&output.stdout)?;

            Ok(())
        }
    }
}
