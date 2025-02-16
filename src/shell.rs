use crate::run;
use std::{io::{self, stdin, stdout, Write}, str};
mod dir;

struct Shell {
    prompt: String,
}

impl Shell {
    fn new(prompt: Option<&str>) -> Shell {
        match prompt {
            Some(_) => Shell {
                prompt: prompt.unwrap().to_string(),
            },

            None => match dir::current() {
                Ok(path) => Shell {
                    prompt: format!("{} > ", path.display()),
                },
                Err(e) => panic!("rush: {e}"),
            },
        }
    }

    fn input(&self) -> io::Result<String> {
        print!("{}", self.prompt);
        let mut s = String::new();
        stdout().flush()?;
        stdin().read_line(&mut s)?;
        Ok(s.trim().to_string())
    }

    fn handle_dir(&mut self) {
        self.prompt = match dir::current() {
            Ok(path) => format!("{} > ", path.display()),
            Err(e) => panic!("rush: {e}"),
        }
    }
}

pub fn run() {
    let mut shell = Shell::new(None);
    loop {
        shell.handle_dir();
        match shell.input() {
            Ok(input) => run::execute(input.as_str()),
            Err(e) => eprintln!("{e}"),
        }
    }
}
