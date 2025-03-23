mod bic;
mod error;
mod lexer;
mod node;
mod parser;
use super::interpreter::{
    error::{ShellError, ShellErrorKind},
    lexer::Lexer,
    node::{Ast, LogicType, RedirectType},
    parser::Parser,
};
use std::{
    fs::{File, OpenOptions},
    io::ErrorKind,
    process::{Command, Stdio},
    thread,
};

#[derive(Clone)]
pub struct Interpreter {
    // Example environment for future cases
    debug: bool,
}

impl Interpreter {
    pub fn new(debug: bool) -> Self {
        Interpreter { debug }
    }

    fn command(&self, args: Vec<String>) -> Result<(), ShellError> {
        if args.is_empty() {
            return Ok(()); // Just don't interpret it
        }

        if let Err(e) = bic::execute(args.clone()) {
            if let ShellErrorKind::CommandNotFound = e.kind() {
                let cmd = Command::new(&args[0])
                    .args(&args[1..])
                    .spawn()
                    .and_then(|mut child| child.wait());

                match &cmd {
                    Err(e) if e.kind() == ErrorKind::NotFound => {
                        return Err(ShellError::CommandNotFound(args[0].clone()));
                    }
                    Ok(status) if !status.success() => {
                        return Err(ShellError::CommandFailure(args[0].clone(), *status));
                    }
                    _ => return Ok(()),
                }
            } else {
                return Err(e);
            }
        }

        Ok(())
    }

    fn logic(&self, lhs: &Ast, rhs: &Ast, logic_type: LogicType) -> Result<(), ShellError> {
        match logic_type {
            LogicType::And => {
                self.execute(lhs)?;
                self.execute(rhs)
            }
            LogicType::Or => self.execute(lhs).or_else(|_| self.execute(rhs)),
        }
    }

    fn extract_command_args<'a>(
        &self,
        ast: &'a Ast,
        side: &str,
    ) -> Result<(&'a String, &'a [String]), ShellError> {
        if let Ast::Command(args) = ast {
            if args.is_empty() {
                return Err(ShellError::InvalidArgument(
                    format!("{} hand side of the pipe cannot be empty", side).into(),
                ));
            }
            Ok((&args[0], &args[1..]))
        } else {
            Err(ShellError::InvalidArgument(
                format!("{} hand side of the pipe must be a command", side).into(),
            ))
        }
    }

    fn pipe(&self, lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
        let (left_cmd, left_args) = self.extract_command_args(lhs, "Left")?;
        let (right_cmd, right_args) = self.extract_command_args(rhs, "Right")?;

        if bic::is_bic(left_cmd.as_str()) || bic::is_bic(right_cmd.as_str()) {
            return Err(ShellError::InvalidArgument(
                "Cannot pipe to or from built-in commands".into(),
            ));
        }

        let mut left_process = Command::new(left_cmd)
            .args(left_args)
            .stdout(Stdio::piped())
            .spawn()?;

        let left_stdout = left_process
            .stdout
            .take()
            .ok_or_else(|| ShellError::from("Failed to capture stdout from left command"))?;

        let left_status = left_process.wait()?;

        let right_status = Command::new(right_cmd)
            .args(right_args)
            .stdin(Stdio::from(left_stdout))
            .spawn()?
            .wait()?;

        if !left_status.success() {
            return Err(ShellError::CommandFailure(left_cmd.clone(), left_status));
        }

        if !right_status.success() {
            return Err(ShellError::CommandFailure(right_cmd.clone(), right_status));
        }

        Ok(())
    }

    fn redirect(
        &self,
        lhs: &Ast,
        rhs: &Ast,
        redirect_type: RedirectType,
    ) -> Result<(), ShellError> {
        if let Ast::Command(args) = lhs {
            if args.is_empty() {
                return Err(ShellError::InvalidArgument(
                    "Empty command for redirection".into(),
                ));
            }

            let filepath = if let Ast::Command(file) = rhs {
                if file.is_empty() {
                    return Err(ShellError::InvalidArgument("Empty file path".into()));
                }

                match redirect_type {
                    RedirectType::Overwrite => File::create(&file[0])?,
                    RedirectType::Append => OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&file[0])?,
                    RedirectType::Error => File::create(&file[0])?,
                    RedirectType::Input => File::open(&file[0])?,
                }
            } else {
                return Err(ShellError::InvalidArgument("Unknown Filepath".into()));
            };

            let mut binding = Command::new(&args[0]);
            let mut cmd = binding.args(&args[1..]);

            cmd = match redirect_type {
                RedirectType::Overwrite | RedirectType::Append => cmd.stdout(Stdio::from(filepath)),
                RedirectType::Error => cmd.stderr(Stdio::from(filepath)),
                RedirectType::Input => cmd.stdin(Stdio::from(filepath)),
            };

            let status = cmd.spawn()?.wait()?;

            if !status.success() {
                return Err(ShellError::CommandFailure(args[0].clone(), status));
            }
        }

        Ok(())
    }

    fn background(&self, lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
        match lhs {
            Ast::Command(args) => {
                let args = args.clone();
                let self_clone = (*self).clone();
                thread::spawn(move || {
                    if let Err(e) = self_clone.command(args.to_vec()) {
                        eprintln!("{e}");
                    }
                });
            }
            Ast::Background(lhs, rhs) => {
                self.background(lhs, rhs)?;
            }
            ast => {
                self.execute(ast)?;
            }
        }

        match rhs {
            Ast::Command(args) => {
                self.command(args.to_vec())?;
            }
            Ast::Background(lhs, rhs) => {
                self.background(lhs, rhs)?;
            }
            ast => {
                self.execute(ast)?;
            }
        }
        Ok(())
    }

    fn execute(&self, node: &Ast) -> Result<(), ShellError> {
        match node {
            Ast::Command(args) => self.command(args.to_vec()),
            Ast::Logic(lhs, rhs, logic_type) => self.logic(lhs, rhs, logic_type.clone()),
            Ast::Pipe(lhs, rhs) => self.pipe(lhs, rhs),
            Ast::Redirect(lhs, rhs, redirect_type) => {
                self.redirect(lhs, rhs, redirect_type.clone())
            }
            Ast::Background(lhs, rhs) => self.background(lhs, rhs),
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
