#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    EofError,
    Other(String),
    Utf8Error(String),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}
