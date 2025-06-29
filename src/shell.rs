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
    }
}
