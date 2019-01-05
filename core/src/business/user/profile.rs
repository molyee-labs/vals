use data::user::{Id, Profile};
use crate::result::*;

pub fn update(p: Profile) -> Result<()> {
    Ok(())
}

pub fn get(uid: Id) -> Result<Profile> {
    Err(Error::NotFound)
}
