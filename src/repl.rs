use crate::engine::{self, error::ShellErrorKind};
use std::{
    env,
    io::{stdin, stdout, Write},
    path::PathBuf,
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

    fn current_dir(&self) -> String {
        let home = env::var("HOME").ok().map(PathBuf::from).unwrap();
        let wd = env::current_dir()
            .unwrap_or_else(|e| panic!("{e}"))
            .to_path_buf();

        if wd.starts_with(&home) {
            return format!("~/{}", wd.strip_prefix(&home).unwrap().to_str().unwrap());
        }

        return wd.to_string_lossy().to_string();
    }

    fn interactive(&self) {
        let mut s = String::new();
        print!(
            "{}",
            self.prompt.replace(r"\w", self.current_dir().as_str())
        );

        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not type in correct string");

        match engine::eval(s.trim()) {
            Ok(()) => (),
            Err(e) if e.kind() == ShellErrorKind::CommandFailure => (),
            Err(e) => eprintln!("rush: {}", e),
        }
    }
}

pub fn run() {
    let shell = Shell::new(r"\w> ");
    loop {
        shell.interactive();
    }
}
