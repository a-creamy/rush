use std::{
    fs::{File, OpenOptions},
    io::{Read, Result, Write, pipe},
    path::PathBuf,
    process::{ChildStdout, Stdio},
};

#[derive(Clone, Debug)]
pub enum Stream {
    Inherit,
    Piped,
    File(PathBuf, FileOption),
    ChildStdout(String),
    Null,
}

impl Stream {
    pub fn stdio(&self) -> Stdio {
        match self {
            Stream::Inherit => Stdio::inherit(),
            Stream::Piped => Stdio::piped(),
            Stream::File(path, options) => match options {
                FileOption::Overwrite => {
                    Stdio::from(File::create(path).expect("Could not create file"))
                }
                FileOption::Append => Stdio::from(
                    OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(path)
                        .expect("Could not create file"),
                ),
            },
            Stream::ChildStdout(buf) => Stdio::from(
                buf.to_owned().stream_stdio().expect("Could not make String into Stdio"),
            ),
            Stream::Null => Stdio::null(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum FileOption {
    Overwrite,
    Append,
}

pub trait StreamFile {
    fn stream(self, option: FileOption) -> Stream;
}

impl StreamFile for PathBuf {
    fn stream(self, option: FileOption) -> Stream {
        Stream::File(self, option)
    }
}

pub trait StreamChildStdout {
    fn stream(&mut self) -> Stream;
}

impl StreamChildStdout for ChildStdout {
    fn stream(&mut self) -> Stream {
        let mut buf = String::new();
        self.read_to_string(&mut buf).expect("hi");
        return Stream::ChildStdout(buf);
    }
}

trait StreamString {
    fn stream_stdio(self) -> Result<Stdio>;
}

impl StreamString for String {
    fn stream_stdio(self) -> Result<Stdio> {
        let (reader, mut writer) = pipe()?;

        let _ = writer.write_all(self.as_bytes());
        drop(writer);

        Ok(Stdio::from(reader))
    }
}
