use super as user;
use std::time;

pub struct Profile {
    id: user::Id,
    name: String,
    image: String,
    dob: time::SystemTime,
}

pub fn update(p:Profile) -> Result<()> {

}

pub fn get(uid:user::Id) -> Result<Profile> {

}
