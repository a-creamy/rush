use crate::engine::{self, error::ShellErrorKind, lexer, parser};
use std::io::{Write, stdin, stdout};

struct Shell {
    prompt: String,
}

impl Shell {
    pub fn new(prompt: &str) -> Self {
        Shell {
            prompt: prompt.to_string(),
        }
    }

    pub fn ask(&self) -> String {
        let mut s = String::new();
        print!("{}", self.prompt);
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter correct string");
        s.trim().to_string()
    }
}

pub fn run() {
    let shell = Shell::new("> ");

    loop {
        let input = shell.ask();

        let tokens = match lexer::lex(input) {
            Ok(tokens) => tokens,
            Err(e) => {
                eprintln!("rush: {e}");
                continue;
            }
        };

        let expr = match parser::parse(&tokens) {
            Ok(expr) => expr,
            Err(e) => {
                eprintln!("rush: {e}");
                continue;
            }
        };

        let cmd = engine::execute(expr, None);
        match cmd {
            Ok(mut child) => {
                if let Err(e) = child.wait() {
                    eprintln!("rush: {e}");
                }
            }
            Err(e) => {
                if e.kind() == ShellErrorKind::Unnecassary {
                    continue;
                }
                eprintln!("rush: {e}");
            }
        }
    }
}
