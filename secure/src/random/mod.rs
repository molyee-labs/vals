use rand::prelude::*;

pub fn next_bytes(buf: &mut [u8]) {
    let mut rng = thread_rng();
    rng.fill_bytes(buf)
}
