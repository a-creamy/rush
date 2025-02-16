use crate::run::{error::ShellError, node::Ast};
use std::process::Command;

pub fn execute(lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
    let left_args = match lhs {
        Ast::Command(args) => args,
        _ => {
            return Err(ShellError::InvalidArgument(
                "Or logical right hand side must be a command".to_string(),
            ))
        }
    };

    let mut binding = Command::new(&left_args[0]);
    let left = binding.args(&left_args[1..]);

    let right_args = match rhs {
        Ast::Command(args) => args,
        _ => {
            return Err(ShellError::InvalidArgument(
                "Or logical right hand side must be a command".to_string(),
            ))
        }
    };

    let mut binding = Command::new(&right_args[0]);
    let right = binding.args(&right_args[1..]);

    let mut left_cmd = left.spawn()?;

    if !left_cmd.wait()?.success() {
        let mut right_cmd = right.spawn()?;
        right_cmd.wait()?;
    }

    Ok(())
}
