use std::collections::VecDeque;
use std::io;
use std::process::{Command, Stdio};
pub mod dir;

fn tokenize(input: &str) -> VecDeque<String> {
    let mut tokens = VecDeque::new();
    let words: Vec<&str> = input.split_whitespace().collect();
    for word in words {
        if !word.is_empty() {
            tokens.push_back(word.to_string());
        }
    }
    return tokens;
}

fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn run_program(program: &str, args: &[&str]) -> io::Result<()> {
    let mut command = Command::new(program);
    command.args(args);

    command.stdin(Stdio::inherit());
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());

    let status = command.status()?;

    if !status.success() {
        eprintln!("rush: {} exited with error: {:?}", program, status);
    }

    Ok(())
}

pub fn execute(cmd: &str) {
    let args = tokenize(cmd);
    if args.is_empty() {
        return;
    }

    if &args[0] == "cd" {
        let arg = if args.len() > 1 {
            Some(args[1].as_str())
        } else {
            None
        };
        match dir::cd(arg) {
            Ok(()) => return,
            Err(e) => {
                eprintln!("cd: {}", e);
                return;
            }
        }
    }

    if command_exists(&args[0]) {
        let args_slice: Vec<&str> = args.iter().skip(1).map(|s| s.as_str()).collect();
        let _ = run_program(&args[0], &args_slice);
    } else {
        eprintln!("rush: {}: command not found", &args[0]);
    }
}
