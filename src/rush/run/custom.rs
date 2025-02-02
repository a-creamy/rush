use crate::run::dir;
use std::collections::VecDeque;
use std::process;

pub fn execute(args: &VecDeque<String>) -> bool {
    if &args[0] == "cd" {
        let arg = if args.len() > 1 {
            Some(args[1].as_str())
        } else {
            None
        };
        match dir::cd(arg) {
            Ok(()) => return true,
            Err(e) => {
                eprintln!("cd: {}", e);
                return true;
            }
        }
    }

    if &args[0] == "exit" {
        let arg = if args.len() > 1 {
            match args[1].parse::<i32>() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("rush: invalid exit code: {}", args[1]);
                    1
                }
            }
        } else {
            0
        };
        process::exit(arg);
    }

    return false;
}
