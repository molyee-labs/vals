use crate::result::*;
use crate::random;
use scrypt::{scrypt, ScryptParams, errors::*};

pub trait Hasher {
    fn encode(&self, password: &[u8], hash: &mut [u8], salt: &mut [u8]) -> Result<()>;
    fn check(&self, password: &[u8], hash: &[u8], salt: &[u8]) -> Result<()>;
}

pub struct ScryptHasher {
    params: ScryptParams,
}

impl From<InvalidParams> for Error {
    fn from(error: InvalidParams) -> Self {
        Error::ScryptInvalidParams(error)
    }
}

impl From<InvalidOutputLen> for Error {
    fn from(error: InvalidOutputLen) -> Self {
        Error::ScryptInvalidOutputLen(error)
    }
}

impl ScryptHasher {
    pub fn new() -> Result<Self> {
        let params = ScryptParams::new(15, 8, 1)?;
        Ok(ScryptHasher { params })
    }
}

impl Hasher for ScryptHasher {
    fn encode(&self, password: &[u8], hash: &mut [u8], salt: &mut [u8]) -> Result<()> {
        random::next_bytes(salt);
        scrypt(password, salt, &self.params, hash)?;
        Ok(())
    }

    fn check(&self, password: &[u8], hash: &[u8], salt: &[u8]) -> Result<()> {
        let mut output = Vec::with_capacity(hash.len());
        scrypt(password, salt, &self.params, &mut output)?;
        if output == hash {
            Ok(())
        } else {
            Err(Error::IncorrectPassword)
        }
    }
}
