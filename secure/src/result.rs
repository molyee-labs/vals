pub enum Error {
    IncorrectPassword,
    ScryptInvalidParams(scrypt::errors::InvalidParams),
    ScryptInvalidOutputLen(scrypt::errors::InvalidOutputLen),
}

pub type Result<T> = std::result::Result<T, Error>;
