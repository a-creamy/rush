use crate::run::{executor, node::Ast, ShellError};
use std::thread;

pub fn execute(node: &Ast) -> Result<(), ShellError> {
    match node {
        Ast::Command(args) => {
            let args = args.clone();
            thread::spawn(move || match executor::execute(&Ast::Command(args)) {
                Ok(_) => Ok(()),
                Err(ShellError::BicError(msg)) => {
                    eprint!("{}", msg);
                    Err(ShellError::BicError(msg))
                }
                Err(e) => {
                    eprintln!("rush: {}", e);
                    Err(e)
                }
            });

            Ok(())
        }
        _ => Err(ShellError::InvalidArgument(
            "'&' Operator only supports commands".to_string(),
        )),
    }
}
