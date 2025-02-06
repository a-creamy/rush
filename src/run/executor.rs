use crate::run::node;
use std::process::{Child, Command, Stdio};

pub fn execute(ast: node::AST) {
    match ast {
        node::AST::Command(args) => {
            let mut cmd = Command::new(&args[0]);
            if args.len() > 1 {
                cmd.args(&args[1..]);
            }

            match cmd.stdin(Stdio::inherit()).stdout(Stdio::inherit()).spawn() {
                Ok(mut child) => {
                    child.wait().unwrap();
                }
                Err(e) => {
                    eprintln!("rush: {}", e);
                }
            }
        }
        node::AST::Pipeline(commands) => {
            let mut previous_stdout = None;
            let mut children: Vec<Child> = Vec::new();

            for (i, command) in commands.iter().enumerate() {
                let mut cmd = Command::new(&command[0]);
                if command.len() > 1 {
                    cmd.args(&command[1..]);
                }

                if let Some(stdout) = previous_stdout {
                    cmd.stdin(stdout);
                } else {
                    cmd.stdin(Stdio::inherit());
                }

                let mut child = if i == commands.len() - 1 {
                    match cmd.stdout(Stdio::inherit()).spawn() {
                        Ok(child) => child,
                        Err(e) => {
                            eprintln!("rush: {}", e);
                            return;
                        }
                    }
                } else {
                    match cmd.stdout(Stdio::piped()).spawn() {
                        Ok(child) => child,
                        Err(e) => {
                            eprintln!("rush: {}", e);
                            return;
                        }
                    }
                };

                previous_stdout = child.stdout.take();
                children.push(child);
            }

            for mut child in children {
                child
                    .wait()
                    .expect("rush: Failed to wait for child process");
            }
        }
    }
}
