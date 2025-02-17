use crate::run::{error::ShellError, node::Ast};
use std::{
    fs::File,
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn execute(lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
    let args = match lhs {
        Ast::Command(args) => args,
        _ => {
            return Err(ShellError::InvalidArgument(
                "'&>' Does not support this type of command".to_string(),
            ))
        }
    };

    let file = match rhs {
        Ast::Command(filepath) => File::create(PathBuf::from(&filepath[0]))?,
        _ => {
            return Err(ShellError::InvalidArgument(
                "'&>' Does not support this type of file".to_string(),
            ))
        }
    };

    let stdout_file = file.try_clone()?;
    let stderr_file = file.try_clone()?;

    Command::new(&args[0]).args(&args[1..])
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .spawn()?
        .wait()?;

    Ok(())
}
