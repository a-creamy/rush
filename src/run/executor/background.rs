use crate::run::{executor, node::Ast, ShellError};
use std::thread;

pub fn execute(lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
    match lhs {
        Ast::Command(args) => {
            let handle = spawn_thread(args.clone());
            drop(handle);
        }
        Ast::Background(left, right) => {
            execute(left, right)?;
        }
        _ => {
            return Err(ShellError::InvalidArgument(
                "'&' Operator only supports commands".to_string(),
            ));
        }
    }

    match rhs {
        Ast::Command(args) => {
            executor::execute(&Ast::Command(args.clone()))?;
            Ok(())
        }
        Ast::Background(left, right) => execute(left, right),
        _ => Err(ShellError::InvalidArgument(
            "'&' Operator only supports commands".to_string(),
        )),
    }
}

fn spawn_thread(args: Vec<String>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        if let Err(e) = executor::execute(&Ast::Command(args)) {
            eprintln!("rush: {}", e);
        }
    })
}
