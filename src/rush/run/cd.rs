use libc::{c_char, getpwuid_r, getuid, passwd};
use std::env;
use std::ffi::CStr;
use std::io;
use std::path::Path;
use std::ptr;

pub fn home_directory() -> Option<&'static str> {
    unsafe {
        let uid = getuid();

        let mut pwd: passwd = std::mem::zeroed();
        let mut buf = [0 as c_char; 1024];
        let mut result: *mut passwd = ptr::null_mut();

        if getpwuid_r(uid, &mut pwd, buf.as_mut_ptr(), buf.len(), &mut result) == 0
            && !result.is_null()
        {
            let home_dir = CStr::from_ptr(pwd.pw_dir).to_string_lossy().into_owned();
            let home_dir_static: &'static str = Box::leak(home_dir.into_boxed_str());
            Some(home_dir_static)
        } else {
            None
        }
    }
}

pub fn cd(path: Option<&str>) -> io::Result<()> {
    let new_dir: std::path::PathBuf = match path {
        Some(p) => Path::new(p).to_path_buf(),
        None => match home_directory() {
            Some(home) => Path::new(&home).to_path_buf(),
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Failed to retrieve home directory",
                ))
            }
        },
    };

    if !new_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Directory '{}' does not exist", path.unwrap()),
        ));
    }
    if !new_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("'{}' is not a directory", path.unwrap()),
        ));
    }

    env::set_current_dir(new_dir)?;

    Ok(())
}
