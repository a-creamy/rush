use super::interpreter::Interpreter;
use libc::{c_char, gethostname, signal, SIGINT, SIGTSTP, SIG_IGN};
use std::{
    env,
    ffi::CStr,
    io::{stdin, stdout, Write},
    path::Path,
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

    fn get_hostname(&self) -> String {
        let mut buffer = [0 as c_char; 256];

        let result = unsafe { gethostname(buffer.as_mut_ptr(), buffer.len()) };

        if result == 0 {
            let c_str = unsafe { CStr::from_ptr(buffer.as_ptr()) };
            c_str
                .to_str()
                .map(|s| s.to_owned())
                .map_err(|_| "Invalid UTF-8 in hostname".to_string())
                .expect("Invalid C Str")
        } else {
            "Failed to get hostname".to_string()
        }
    }

    fn shorten_dir(&self) -> String {
        match env::current_dir() {
            Ok(path) => {
                let path_str = if let Some(home) = env::var("HOME").unwrap().into() {
                    if let Ok(stripped) = path.strip_prefix(&home) {
                        Path::new("~").join(stripped)
                    } else {
                        path
                    }
                } else {
                    path
                };

                let components: Vec<_> = path_str
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().to_string())
                    .collect();

                if components.len() >= 2 {
                    if components[components.len() - 2].starts_with("/") {
                        return format!("/{}", components[components.len() - 1],);
                    }
                    format!(
                        "{}/{}",
                        components[components.len() - 2],
                        components[components.len() - 1]
                    )
                } else {
                    format!("{}", components[components.len() - 1],)
                }
            }
            Err(_) => String::from("Failed to get current directory."),
        }
    }

    fn interactive(&self) -> String {
        let mut s = String::new();
        print!(
            "{}",
            self.prompt
                .replace(r"\w", self.shorten_dir().as_str())
                .replace(r"\h", self.get_hostname().as_str())
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

    let shell = Shell::new(r"\h :: \w > ");
    let interpreter = Interpreter::new();

    loop {
        interpreter.interpret(shell.interactive().as_str())
    }
}
