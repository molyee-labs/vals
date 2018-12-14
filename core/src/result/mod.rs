pub enum Error {
    NotImplemented,
    Unknown,
    NotFound,
    Undefined,
    IoError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
