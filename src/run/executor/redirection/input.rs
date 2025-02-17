use crate::run::{error::ShellError, node::Ast};
use std::{
    fs::File,
    process::{Command, Stdio},
};

pub fn execute(lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
    let args = match lhs {
        Ast::Command(args) => args,
        _ => {
            return Err(ShellError::InvalidArgument(
                "'<' Left hand side must be a command".to_string(),
            ))
        }
    };

    let file = match rhs {
        Ast::Command(args) => File::open(args[0].clone())?,
        _ => {
            return Err(ShellError::InvalidArgument(
                "'<' Right hand side must be a file".to_string(),
            ))
        }
    };

    Command::new(args[0].clone())
        .args(&args[1..])
        .stdin(Stdio::from(file))
        .spawn()?
        .wait()?;

    Ok(())
}
