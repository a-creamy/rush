use libc::{c_char, geteuid, gethostname, getpwuid};
use std::env::{self};
use std::ffi::CStr;
use std::io::{self, Write};
use std::path::PathBuf;

pub mod run;

pub struct Rush {
    prompt: String,
}

impl Rush {
    pub fn new(prompt: Option<&str>) -> Rush {
        let username = get_username().unwrap_or_else(|e| {
            eprintln!("Failed to retrieve user information: {}", e);
            "unknown".into()
        });

        let hostname = get_hostname().unwrap_or_else(|e| {
            eprintln!("Failed to get hostname: {}", e);
            "unknown".into()
        });

        let dir = match env::current_dir() {
            Ok(path) => {
                if path == PathBuf::from(run::dir::home_directory().unwrap()) {
                    PathBuf::from("~")
                } else {
                    path
                }
            }
            Err(e) => {
                eprintln!("Failed to get current directory: {}", e);
                let default_prompt = format!("[{}@{} unknown]$", username, hostname);
                return Rush {
                    prompt: default_prompt,
                };
            }
        };

        let default_prompt = format!(
            "[{}@{} {}]$",
            username,
            hostname,
            dir.file_name()
                .unwrap_or_else(|| dir.as_os_str())
                .to_string_lossy()
        );
        match prompt {
            Some(p) => Rush {
                prompt: p.to_string(),
            },
            None => Rush {
                prompt: default_prompt,
            },
        }
    }

    pub fn input(&self) -> io::Result<String> {
        let mut s = String::new();
        print!("{} ", self.prompt);
        io::stdout().flush()?;
        io::stdin().read_line(&mut s)?;
        Ok(s.trim().to_string())
    }

    pub fn update_dir(&mut self, updated_dir: PathBuf) {
        let username = get_username().unwrap_or_else(|e| {
            eprintln!("Failed to retrieve user information: {}", e);
            "unknown".into()
        });

        let hostname = get_hostname().unwrap_or_else(|e| {
            eprintln!("Failed to get hostname: {}", e);
            "unknown".into()
        });

        self.prompt = format!(
            "[{}@{} {}]$",
            username,
            hostname,
            updated_dir
                .file_name()
                .unwrap_or_else(|| updated_dir.as_os_str())
                .to_string_lossy()
        );
    }
}

pub fn get_username() -> Result<String, String> {
    unsafe {
        let uid = geteuid();
        let passwd_entry = getpwuid(uid);

        if passwd_entry.is_null() {
            return Err("passwd entry is null".into());
        }

        Ok(CStr::from_ptr((*passwd_entry).pw_name)
            .to_string_lossy()
            .into_owned())
    }
}

pub fn get_hostname() -> Result<String, String> {
    let mut buffer = [0 as c_char; 64];
    unsafe {
        if gethostname(buffer.as_mut_ptr(), buffer.len()) != 0 {
            return Err("gethostname failed".into());
        }

        Ok(CStr::from_ptr(buffer.as_ptr())
            .to_string_lossy()
            .into_owned())
    }
}
