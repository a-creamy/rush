use crate::run::node::Ast;
use std::process::{Command, Stdio};
use crate::run::ShellError;

pub fn execute(lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
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

