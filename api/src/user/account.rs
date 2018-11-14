use core::identity::{User,UserId};
use core::result::Result;
use core::error::Error;

pub fn add(u:User) -> Result<User> {
    Ok(u)
}

pub fn delete(u:User) -> Result<User> {
    Ok(u)
}

pub fn find(id:UserId) -> Result<User> {
    Err(Error::NotFound)
}

pub fn update(u:User) -> Result<User> {
    Ok(u)
}
