use result::*;

pub type Id = u64;

pub struct Account {
    id: Id,
    email: String,
    phone: String,
    password: String,
}

pub fn add(u:&Account) -> Result<&Account> {
    Ok(u)
}

pub fn delete(u:&Account) -> Result<&Account> {
    Ok(u)
}

pub fn find(uid:Id) -> Result<Account> {
    Err(Error::NotFound)
}

pub fn find_by_email(email:&str) -> Result<Account> {
    Err(Error::NotFound)
}

pub fn find_by_phone(phone:&str) -> Result<Account> {
    Err(Error::NotFound)
}
