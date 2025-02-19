use crate::run::{executor, node::Ast, ShellError};
use std::thread;

pub fn execute(lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
    match lhs {
        Ast::Command(args) => {
            let args = args.clone();
            thread::spawn(move || {
                if let Err(e) = executor::execute(&Ast::Command(args.clone())) {
                    eprintln!("rush: {}", e);
                }
            });
        }
        Ast::Background(left, right) => {
            execute(left, right)?;
        }
        ast => {
            executor::execute(ast)?;
        }
    }

    match rhs {
        Ast::Command(args) => {
            executor::execute(&Ast::Command(args.clone()))?;
        }
        Ast::Background(left, right) => {
            execute(left, right)?;
        }
        ast => {
            executor::execute(ast)?;
        }
    }

    Ok(())
}
