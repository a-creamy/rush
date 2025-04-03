use super::interpreter::Interpreter;
use std::{
    env,
    io::{stdin, stdout, Write},
    path::PathBuf,
};
    use libc::{signal, SIGINT, SIGTSTP, SIG_IGN};

struct Shell {
    prompt: String,
}

impl Shell {
    fn new(prompt: &str) -> Shell {
        Shell {
            prompt: prompt.to_string(),
        }
    }

    fn current_dir(&self) -> PathBuf {
        let home = env::var("HOME").ok().map(PathBuf::from).unwrap();
        let current = env::current_dir()
            .unwrap_or_else(|e| panic!("{e}"))
            .to_path_buf();
        if home == current {
            return PathBuf::from("~");
        } else {
            return current;
        }
    }

    fn interactive(&self) -> String {
        let mut s = String::new();
        print!(
            "{}",
            self.prompt
                .replace(r"\w", self.current_dir().to_str().unwrap())
        );

        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not type in correct string");
        s.trim().to_string()
    }
}

pub fn run() {
    unsafe {
        signal(SIGINT, SIG_IGN);
        signal(SIGTSTP, SIG_IGN);
    }

    let shell = Shell::new(r"\w > ");
    let interpreter = Interpreter::new(false);

    loop {
        interpreter.interpret(shell.interactive().as_str())
    }
}
