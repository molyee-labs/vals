use crate::random;
use scrypt::{scrypt, ScryptParams};

pub struct Password {
    hash: [u8; 128],
    salt: [u8; 16],
}

pub struct Hasher {
    params: ScryptParams
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

impl PartialEq<&[u8]> for [u8; 128] {
    fn eq(&self, other: &[u8]) -> bool {
        &self.len() == other.len() &&
            &self.iter().zip(other).all(|(a, b)| { a == b })
    }
}

impl Hasher {
    pub fn new() -> Result<Self> {
        let params = ScryptParams::new(15, 8, 1)?;
        Ok(Hasher { params })
    }

    pub fn get(&self, password: &[u8]) -> Result<Password> {
        let mut salt = [0u8; 16];
        random::next_bytes(&mut salt);
        let mut hash = [0u8; 128];
        scrypt(password, &salt, &self.params, &mut hash)?;
        Ok(Password { hash, salt })
    }

    pub fn check(&self, password: &[u8], origin: Password) -> Result<()> {
        let mut hash = [0u8; 128];
        scrypt(password, &origin.salt, &self.params, &mut hash)?;
        if origin.hash.iter().zip(hash).all() {
            Ok(())
        } else {
            Err(Error::IncorrectPassword)
        }
    }
}
