use crate::result::*;
use crate::data::user::*;
use crate::secure::crypt::hasher;
use crate::db;

pub fn add(nu: NewUser) -> Result<Account> {
    let mut u: Account = Account::from(nu);
    let s = db::open()?;
    s.
    Err(Error::NotImplemented)
}

pub fn delete(uid: Id) -> Result<Account> {
    Err(Error::NotImplemented)
}

pub fn find(uid: Id) -> Result<Account> {
    Err(Error::NotImplemented)
}

pub fn find_by_email(email: &Email) -> Result<Account> {
    Err(Error::NotImplemented)
}

pub fn find_by_phone(phone: &Phone) -> Result<Account> {
    Err(Error::NotImplemented)
}

pub fn set_email(uid: Id, email: &Email) -> Result<()> {
    Err(Error::NotImplemented)
}

pub fn set_phone(uid: Id, phone: &Phone) -> Result<()> {
    Err(Error::NotImplemented)
}

pub fn set_password(uid: Id, password: &[u8]) -> Result<()> {
    Err(Error::NotImplemented)
}
