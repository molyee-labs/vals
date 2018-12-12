use crate::data::user::{Id, Profile};
use crate::result::*;
use std::time;


pub fn update(p: Profile) -> Result<()> {
    Ok(())
}

pub fn get(uid: Id) -> Result<Profile> {
    Err(Error::NotFound)
}
