use super::password::*;
use crate::random;
use scrypt::{scrypt, ScryptParams};

pub struct ScryptHasher {
    params: ScryptParams,
}

use crate::result::*;

impl From<scrypt::errors::InvalidParams> for Error {
    fn from(error: scrypt::errors::InvalidParams) -> Self {
        Error::ScryptInvalidParams(error)
    }
}

impl From<scrypt::errors::InvalidOutputLen> for Error {
    fn from(error: scrypt::errors::InvalidOutputLen) -> Self {
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
    fn get(&self, password: &[u8]) -> Result<Password> {
        let mut salt = [0u8; 16];
        random::next_bytes(&mut salt);
        let mut hash = [0u8; 128];
        scrypt(password, &salt, &self.params, &mut hash)?;
        Ok(Password { hash, salt })
    }

    fn check(&self, password: &[u8], origin: &Password) -> Result<()> {
        let mut hash = [0u8; 128];
        scrypt(password, &origin.salt, &self.params, &mut hash)?;
        if origin.hash[..] == hash[..] {
            Ok(())
        } else {
            Err(Error::IncorrectPassword)
        }
    }
}

fn encode(password: &[u8]) -> Result<Password> {
    ScryptHasher::new()?.get(password)
}
