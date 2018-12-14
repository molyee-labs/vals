use crate::random;
use crate::result::Result;

pub struct Password {
    pub hash: [u8; 128],
    pub salt: [u8; 16],
}

pub trait Hasher {
    fn get(&self, password: &[u8]) -> Result<Password>;
    fn check(&self, password: &[u8], origin: &Password) -> Result<()>;
}
