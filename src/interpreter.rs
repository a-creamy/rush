mod error;
mod lexer;
mod node;
mod parser;
use super::interpreter::{
    error::ShellError,
    lexer::Lexer,
    node::{Ast, LogicType, RedirectType},
    parser::Parser,
};
use std::{
    fs::{File, OpenOptions},
    io::ErrorKind,
    process::{Command, Stdio},
};

pub struct Interpreter {
    // Example environment for future cases
    // Should be used for example: keeping track of variables
    // env: Environment
    debug: bool,
}

impl Interpreter {
    pub fn new(debug: bool) -> Self {
        Interpreter { debug }
    }

    fn command(&self, args: Vec<String>) -> Result<(), ShellError> {
        let cmd = Command::new(&args[0])
            .args(&args[1..])
            .spawn()
            .and_then(|mut child| child.wait());

        if let Err(e) = &cmd {
            if e.kind() == ErrorKind::NotFound {
                return Err(ShellError::CommandNotFound(args[0].clone()));
            }
        }

        if !cmd.as_ref().unwrap().success() {
            return Err(ShellError::CommandFailure(args[0].clone(), cmd.unwrap()));
        }

        Ok(())
    }

    fn logic(&self, lhs: &Ast, rhs: &Ast, logic_type: LogicType) -> Result<(), ShellError> {
        match logic_type {
            LogicType::And => {
                self.execute(lhs)?;
                self.execute(rhs)?;
                Ok(())
            }
            LogicType::Or => match self.execute(lhs) {
                Ok(_) => Ok(()),
                Err(_) => self.execute(rhs),
            },
        }
    }

    fn pipe(&self, lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
        let (left_cmd, left_args) = if let Ast::Command(args) = lhs {
            (&args[0], &args[1..])
        } else {
            return Err(ShellError::InvalidArgument(
                "Left hand side of the pipe must be a command".into(),
            ));
        };

        let (right_cmd, right_args) = if let Ast::Command(args) = rhs {
            (&args[0], &args[1..])
        } else {
            return Err(ShellError::InvalidArgument(
                "Right hand side of the pipe must be a command".into(),
            ));
        };

        let mut left_process = Command::new(left_cmd)
            .args(left_args)
            .stdout(Stdio::piped())
            .spawn()?;

        let left_stdout = left_process
            .stdout
            .take()
            .ok_or("Failed to capture stdout from left command")?;
        let left_status = left_process.wait()?;

        let right_process = Command::new(right_cmd)
            .args(right_args)
            .stdin(Stdio::from(left_stdout))
            .spawn()?
            .wait()?;

        if !left_status.success() {
            return Err(ShellError::CommandFailure(left_cmd.clone(), left_status));
        }

        if !right_process.success() {
            return Err(ShellError::CommandFailure(right_cmd.clone(), right_process));
        }

        Ok(())
    }

    fn redirect(
        &self,
        lhs: &Ast,
        rhs: &Ast,
        redirect_type: RedirectType,
    ) -> Result<(), ShellError> {
        match redirect_type {
            RedirectType::Overwrite => {
                if let Ast::Command(args) = lhs {
                    let filepath = if let Ast::Command(file) = rhs {
                        File::create(&file[0])?
                    } else {
                        return Err(ShellError::InvalidArgument("Unknown Filepath".into()));
                    };

                    Command::new(&args[0])
                        .args(&args[1..])
                        .stdout(Stdio::from(filepath))
                        .spawn()?
                        .wait()?;
                }
            }
            RedirectType::Append => {
                if let Ast::Command(args) = lhs {
                    let filepath = if let Ast::Command(file) = rhs {
                        OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(&file[0])?
                    } else {
                        return Err(ShellError::InvalidArgument("Unknown Filepath".into()));
                    };

                    Command::new(&args[0])
                        .args(&args[1..])
                        .stdout(Stdio::from(filepath))
                        .spawn()?
                        .wait()?;
                }
            }
            RedirectType::Error => {
                if let Ast::Command(args) = lhs {
                    let filepath = if let Ast::Command(file) = rhs {
                        File::create(&file[0])?
                    } else {
                        return Err(ShellError::InvalidArgument("Unknown Filepath".into()));
                    };

                    Command::new(&args[0])
                        .args(&args[1..])
                        .stderr(Stdio::from(filepath))
                        .spawn()?
                        .wait()?;
                }
            }
            _ => {
                return Err(ShellError::InvalidArgument(
                    "Unsupported redirect symbol".into(),
                ))
            }
        };

        return Ok(());
    }

    fn execute(&self, node: &Ast) -> Result<(), ShellError> {
        match node {
            Ast::Command(args) => self.command(args.to_vec()),
            Ast::Logic(lhs, rhs, logic_type) => self.logic(lhs, rhs, logic_type.clone()),
            Ast::Pipe(lhs, rhs) => self.pipe(lhs, rhs),
            Ast::Redirect(lhs, rhs, redirect_type) => {
                self.redirect(lhs, rhs, redirect_type.clone())
            }
            _ => Err(ShellError::InvalidArgument("Unsupported Symbol".into())),
        }
    }

    pub fn interpret<'a>(&self, input: &'a str) {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex();

        if tokens.is_empty() {
            return;
        }

        let mut parser = Parser::new(&tokens);
        let ast = match parser.parse() {
            Ok(result) => result,
            Err(e) => {
                eprintln!("rush: {e}");
                return;
            }
        };

        if self.debug {
            println!("Tokens: {:?}", tokens);
            println!("Ast: {:?}", ast);
        }

        if let Err(e) = self.execute(&ast) {
            eprintln!("rush: {e}");
        }
    }
}
