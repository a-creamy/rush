use libc::{getpwuid, getuid};
use std::{env, ffi::CStr, io::{self, Error, ErrorKind}, path::PathBuf};

pub fn home_dir() -> io::Result<String> {
    unsafe {
        let uid = getuid();
        let passwd = getpwuid(uid);

        if passwd.is_null() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "Failed to get password entry",
            ));
        }

        let home_dir = (*passwd).pw_dir;

        if home_dir.is_null() {
            return Err(Error::new(ErrorKind::NotFound, "Home directory not found"));
        }

        CStr::from_ptr(home_dir)
            .to_str()
            .map(String::from)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }
}

pub fn current() -> io::Result<PathBuf> {
    match env::current_dir() {
        Ok(path) => {
            let home = PathBuf::from(home_dir()?);

            if path == home {
                return Ok(PathBuf::from("~"));
            }

            if let Ok(relative_path) = path.strip_prefix(&home) {
                let mut new_path = PathBuf::from("~");
                new_path.push(relative_path);
                return Ok(new_path);
            }

            Ok(path)
        }
        Err(e) => Err(e),
    }
}
