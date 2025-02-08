use std::env;
use std::path::Path;

pub fn cd(arg: &str) -> Result<(), String> {
    let new_path = if arg.is_empty() || arg == "~" {
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else {
        arg.to_string()
    };

    let path = Path::new(&new_path);
    match env::set_current_dir(&path) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("cd {}: {}", path.display(), e)),
    }
}

pub fn exit(code: i32) {
    std::process::exit(code);
}
