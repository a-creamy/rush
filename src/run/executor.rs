use crate::run::bic;
use crate::run::node;
use std::fs::{File, OpenOptions};
use std::process::{Child, Command, Stdio};

pub fn execute(ast: node::AST) {
    match ast {
        node::AST::Command(args, output) => {
            if args[0] == "cd" {
                let arg = if args.len() > 1 {
                    &args[1]
                } else {
                    "~"
                };

                match bic::cd(arg) {
                    Ok(_) => return,
                    Err(e) => eprintln!("{e}")
                }
            } else if args[0] == "exit" {
                let code = if args.len() > 1 {
                    args[1].parse::<i32>().unwrap_or(0)
                } else {
                    0
                };
                bic::exit(code);
            }

            let mut cmd = Command::new(&args[0]);
            if args.len() > 1 {
                cmd.args(&args[1..]);
            }

            cmd.stdin(Stdio::inherit());

            if let Some((file, append)) = output {
                let file = if append {
                    OpenOptions::new().append(true).create(true).open(file)
                } else {
                    File::create(file)
                }
                .expect("rush: Failed to open output file");

                cmd.stdout(Stdio::from(file));
            } else {
                cmd.stdout(Stdio::inherit());
            }

            cmd.spawn()
                .expect("rush: Failed to execute command")
                .wait()
                .unwrap();
        }
        node::AST::Pipeline(commands, output) => {
            let mut previous_stdout = None;
            let mut children: Vec<Child> = Vec::new();

            for (i, command) in commands.iter().enumerate() {
                if command[0] == "cd" {
                    let arg = if command.len() > 1 {
                        &command[1]
                    } else {
                        "~"
                    };

                    match bic::cd(arg) {
                        Ok(_) => continue,
                        Err(e) => eprintln!("{e}")
                    }
                } else if command[0] == "exit" {
                    let code = if command.len() > 1 {
                        command[1].parse::<i32>().unwrap_or(0)
                    } else {
                        0
                    };
                    bic::exit(code);
                }

                let mut cmd = Command::new(&command[0]);
                if command.len() > 1 {
                    cmd.args(&command[1..]);
                }

                if let Some(stdout) = previous_stdout {
                    cmd.stdin(stdout);
                } else {
                    cmd.stdin(Stdio::inherit());
                }

                if i == commands.len() - 1 {
                    if let Some((file, append)) = &output {
                        let file = if *append {
                            OpenOptions::new().append(true).create(true).open(file)
                        } else {
                            File::create(file)
                        }
                        .expect("rush: Failed to open output file");

                        cmd.stdout(Stdio::from(file));
                    } else {
                        cmd.stdout(Stdio::inherit());
                    }
                } else {
                    cmd.stdout(Stdio::piped());
                }

                let mut child = cmd.spawn().expect("rush: Failed to execute command");
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
