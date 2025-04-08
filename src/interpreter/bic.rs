use super::error::ShellError;
use std::{env, path::Path, process::Command, time::Instant};

fn cd(args: &[String]) -> Result<(), ShellError> {
    let path_str = if args.is_empty() || args[0] == "~" {
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else {
        args[0].to_string()
    };

    let path = Path::new(&path_str);
    match env::set_current_dir(path) {
        Ok(()) => Ok(()),
        Err(_) => Err(ShellError::BicError(format!(
            "cd: The directory '{}' does not exist\n",
            path.display()
        ))),
    }
}

fn exit(args: &[String]) -> ! {
    let code = if !args.is_empty() {
        args[0].parse().unwrap_or(0)
    } else {
        0
    };
    std::process::exit(code);
}

fn pwd() -> Result<(), ShellError> {
    if let Ok(current_dir) = env::current_dir() {
        println!("{}", current_dir.display());
        Ok(())
    } else {
        Err(ShellError::from("Could not get current directory"))
    }
}

fn time(args: &[String]) -> Result<(), ShellError> {
    let start = Instant::now();
    if !args.is_empty() {
        Command::new(&args[0]).args(&args[1..]).spawn()?.wait()?;
    }
    let time = start.elapsed();
    println!("Finished! {} ms", time.as_millis());
    Ok(())
}

pub fn is_bic(cmd: &str) -> bool {
    matches!(cmd, "cd" | "exit" | "pwd" | "time")
}

pub fn execute(args: Vec<String>) -> Result<(), ShellError> {
    if args.is_empty() {
        return Ok(());
    }

    match &args[0][..] {
        "cd" => cd(&args[1..]),
        "exit" => exit(&args[1..]),
        "pwd" => pwd(),
        "time" => time(&args[1..]),
        _ => Err(ShellError::CommandNotFound(format!(
            "Unknown Built In Command:{}",
            &args[0]
        ))),
    }
}
