pub enum Error {
    Unknown,
    NotFound,
    Undefined
}

pub type Result<T> = std::result::Result<T,Error>;
