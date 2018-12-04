use ::user::account as user;
use ::result::*;
use std::time;

pub struct Profile {
    id: user::Id,
    name: String,
    image: String,
    dob: time::SystemTime,
}

pub fn update(p:Profile) -> Result<()> {
    Ok(())
}

pub fn get(uid:user::Id) -> Result<Profile> {
    Err(Error::NotFound)
}
