use libc::{c_char, geteuid, gethostname, getpwuid};
use std::ffi::CStr;
use std::io::{self, Write};

struct Shell {
    prompt: String,
}

impl Shell {
    fn new(prompt: Option<&str>) -> Shell {
        let username = get_username().unwrap_or_else(|e| {
            eprintln!("Failed to retrieve user information: {}", e);
            "unknown".into()
        });

        let hostname = get_hostname().unwrap_or_else(|e| {
            eprintln!("Failed to get hostname: {}", e);
            "unknown".into()
        });

        let default_prompt = format!("[{}@{}]$", username, hostname);
        match prompt {
            Some(p) => Shell {
                prompt: p.to_string(),
            },
            None => Shell {
                prompt: default_prompt,
            },
        }
    }

    fn input(&self) -> io::Result<String> {
        let mut s = String::new();
        print!("{} ", self.prompt);
        io::stdout().flush()?;
        io::stdin().read_line(&mut s)?;
        Ok(s.trim().to_string())
    }
}

fn get_username() -> Result<String, String> {
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

fn get_hostname() -> Result<String, String> {
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

fn main() {
    let shell = Shell::new(None);
    loop {
        match shell.input() {
            Ok(user_input) => println!("{}", user_input),
            Err(e) => eprintln!("Error reading input: {}", e),
        }
    }
}
