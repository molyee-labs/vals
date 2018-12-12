use crate::result::*;
use crate::data::user::*;

pub fn add(nu: NewUser) -> Result<Account> {
    let password
    Err(Error::NotFound)
}

pub fn delete(uid: Id) -> Result<Account> {
    Err(Error::NotFound)
}

pub fn find(uid: Id) -> Result<Account> {
    Err(Error::NotFound)
}

pub fn find_by_email(email: &str) -> Result<Account> {
    Err(Error::NotFound)
}

pub fn find_by_phone(phone: &str) -> Result<Account> {
    Err(Error::NotFound)
}

pub fn change_email(email: &str) -> Result<String> { Err(Error::NotFound) }

pub fn change_phone(phone: &str) -> Result<String> { Err(Error::NotFound) }
