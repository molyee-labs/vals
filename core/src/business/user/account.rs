use crate::result::*;
use crate::data::user::*;
use crate::secure::crypt::password::Password;

pub fn add(nu: NewUser) -> Result<Account> {
    Err(Error::NotImplemented)
}

pub fn delete(uid: Id) -> Result<Account> {
    Err(Error::NotImplemented)
}

pub fn find(uid: Id) -> Result<Account> {
    Err(Error::NotImplemented)
}

pub fn find_by_email(email: &str) -> Result<Account> {
    Err(Error::NotImplemented)
}

pub fn find_by_phone(phone: &str) -> Result<Account> {
    Err(Error::NotImplemented)
}

pub fn change_email(email: &str) -> Result<String> {
    Err(Error::NotImplemented)
}

pub fn change_phone(phone: &str) -> Result<String> {
    Err(Error::NotImplemented)
}
