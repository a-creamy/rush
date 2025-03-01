// use libc::{fcntl, F_GETFL, F_SETFL, O_NONBLOCK};
use std::{
    env,
    io::{stdin, stdout, Write},
};
use super::interpreter::Interpreter;

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
        /* unsafe {
            let fd = libc::STDIN_FILENO;
            let flags = fcntl(fd, F_GETFL);
            fcntl(fd, F_SETFL, flags | O_NONBLOCK);
        } */

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

        /* loop {
            match stdin().read_line(&mut s) {
                Ok(0) => {}
                Ok(_) => {
                    unsafe {
                        let fd = libc::STDIN_FILENO;
                        let flags = fcntl(fd, F_GETFL);
                        fcntl(fd, F_SETFL, flags & !O_NONBLOCK);
                    }

                    return s.trim().to_string();
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) => {
                    unsafe {
                        let fd = libc::STDIN_FILENO;
                        let flags = fcntl(fd, F_GETFL);
                        fcntl(fd, F_SETFL, flags & !O_NONBLOCK);
                    }
                    panic!("{e}");
                }
            }
        } */
    }
}

pub fn run() {
    let shell = Shell::new(r"\w > ");
    let interpreter = Interpreter::new();

    loop {
        interpreter.interpret(shell.interactive().as_str())
    }
}
