use std::process::Command;

use crate::run::{error::ShellError, node::Ast};

pub fn execute(lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
    let left = match lhs {
        Ast::Command(args) => args,
        _ => return Err(ShellError::InvalidArgument("';' Expected a command".to_string())),
    };

    let right = match rhs {
        Ast::Command(args) => args,
        _ => return Err(ShellError::InvalidArgument("';' Expected a command".to_string())),
    };

    Command::new(left[0].as_str())
        .args(&left[1..])
        .spawn()?.wait()?;

    Command::new(right[0].as_str())
        .args(&right[1..])
        .spawn()?.wait()?;

    Ok(())
}
