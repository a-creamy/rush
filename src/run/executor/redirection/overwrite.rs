use std::{fs::File, io::Write, path::PathBuf, process::Command};
use crate::run::{bic, error::ShellError, node::Ast};

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
                let mut file = File::create(filepath)?;

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


