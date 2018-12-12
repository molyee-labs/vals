use crate::data::user::Password;
use crate::result::*;
use crate::random;

pub struct Password {
    hash: [u8; 128],
    salt: [u8; 128],
}

impl Password {
    fn new(source: Vec<u8>) -> Self {
        let salt = random::next_bytes(128);

    }

    fn validate<T: AsBytesArray>(&self, phrase: T) -> Result<()> {

    }
}