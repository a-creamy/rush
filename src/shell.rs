use crate::engine;
use crate::engine::{lexer, parser};
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
            .expect("Did not enter a correct string");
        s.trim().to_string()
    }
}

pub fn run() {
    let shell = Shell::new("> ");

    loop {
        let input = shell.ask();
        println!("{}", input);

        let tokens = lexer::lex(input);
        let expr = match parser::parse(&tokens) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("rush: {}", e);
                continue;
            }
        };

        let child = engine::execute(expr);
        match child.and_then(|mut c| c.wait()) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("rush: {}", e);
                continue;
            }
        }
    }
}
