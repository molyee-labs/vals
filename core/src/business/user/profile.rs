use crate::result::*;
use data::user::{Id, Profile};

pub fn update(p: Profile) -> Result<()> {
    Ok(())
}

pub fn get(uid: Id) -> Result<Profile> {
    Err(Error::NotFound)
}
