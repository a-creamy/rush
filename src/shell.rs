use crate::engine;
use crate::engine::{lexer, parser, Process};
use std::io::{ErrorKind, Write, stdin, stdout};

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
            .expect("Did not enter a correct string");
        s.trim().to_string()
    }
}

pub fn run() {
    let shell = Shell::new("> ");

    loop {
        let input = shell.ask();

        let tokens = lexer::lex(input);
        let expr = match parser::parse(&tokens) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("rush: {}", e);
                continue;
            }
        };

        let cmd = engine::execute(expr);
        match cmd {
            Ok(Process::Child(mut child)) => {
                let _ = child.wait().map_err(|e| eprintln!("{e}"));
            }
            Err(e) => {
                if e.kind() == ErrorKind::Other {
                    continue;
                }
                eprintln!("{e}");
            }
            _ => {},
        }
    }
}
