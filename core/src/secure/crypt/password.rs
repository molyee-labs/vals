use crate::data::user::Password;
use crate::result::*;
use crate::random;
use scrypt::{scrypt, ScryptParams};

lazy_static! {
    static SCRYPT_PARAMS = ScryptParams::new(15, 8, 1).unwrap();
}

pub fn generate(password: &[u8], f: HashFunc, hash_len: u32, salt_len: u32) -> ([u8; hash_len], [u8; salt_len]) {
    let salt: [u8; salt_len] = random::next_bytes(salt_len);
    let mut output: [u8; 128] = [];
    let params = SCRYPT_PARAMS;
    scrypt(password, &salt, &params, &output);
    (output, salt)
}