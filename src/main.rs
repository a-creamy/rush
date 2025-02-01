use crate::rush::run::cd;
use std::env::{self};
use std::path::PathBuf;
mod rush;

fn main() {
    let mut shell = rush::Rush::new(None);
    let mut original_dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            return;
        }
    };

    loop {
        match shell.input() {
            Ok(user_input) => {
                rush::run::execute(&user_input);
                let updated_dir = match env::current_dir() {
                    Ok(path) => path,
                    Err(e) => {
                        eprintln!("Failed to get current directory: {}", e);
                        return;
                    }
                };
                if original_dir != updated_dir {
                    if updated_dir.to_str() == cd::home_directory() {
                        shell.update_dir(PathBuf::from("~"));
                    } else {
                        shell.update_dir(updated_dir.clone());
                    }
                    original_dir = updated_dir;
                }
            }
            Err(e) => eprintln!("Error reading input: {}", e),
        }
    }
}
