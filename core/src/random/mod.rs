use rand::prelude::*;

pub fn next_bytes(len: u32) -> [u8; len] {
    let mut rng = thread_rng();
    let mut salt = [0u8; len];
    rng.fill(&salt);
    salt
}
