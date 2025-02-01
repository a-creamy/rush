use std::env;
use std::io;
use std::path::Path;

pub fn cd(path: &str) -> io::Result<()> {
    let new_dir = Path::new(path);

    if !new_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Directory '{}' does not exist", path),
        ));
    }
    if !new_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("'{}' is not a directory", path),
        ));
    }

    env::set_current_dir(new_dir)?;

    Ok(())
}
