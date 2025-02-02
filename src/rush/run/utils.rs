use std::collections::VecDeque;
use std::io;
use std::process::{Command, Stdio};

pub fn tokenize(input: &str) -> VecDeque<String> {
    let mut tokens = VecDeque::new();
    let words: Vec<&str> = input.split_whitespace().collect();
    for word in words {
        if !word.is_empty() {
            tokens.push_back(word.to_string());
        }
    }
    return tokens;
}

pub fn run_program(program: &str, args: &[&str]) -> io::Result<()> {
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

pub fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
