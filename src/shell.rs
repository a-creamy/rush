use super::interpreter::Interpreter;
use std::{
    env,
    io::{stdin, stdout, Write},
};

struct Shell {
    prompt: String,
}

impl Shell {
    fn new(prompt: &str) -> Shell {
        Shell {
            prompt: prompt.to_string(),
        }
    }

    fn interactive(&self) -> String {
        let mut s = String::new();
        print!(
            "{}",
            self.prompt.replace(
                r"\w",
                env::current_dir()
                    .unwrap_or_else(|e| panic!("{e}"))
                    .to_str()
                    .unwrap()
            )
        );

        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not type in correct string");
        s.trim().to_string()
    }
}

pub fn run() {
    let shell = Shell::new(r"\w > ");
    let interpreter = Interpreter::new(true);

    loop {
        interpreter.interpret(shell.interactive().as_str())
    }
}
