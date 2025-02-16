use std::{fs::OpenOptions, io::Write, path::PathBuf};
use crate::run::error::ShellError;

pub fn log_error(filepath: &PathBuf, output: &str) -> Result<(), ShellError> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filepath)?;

    file.write_all(output.as_bytes())?;
    Ok(())
}
