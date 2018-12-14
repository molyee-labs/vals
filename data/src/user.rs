use secure::crypt::password::{Hasher, Password};

pub struct Email {
    raw: String,
}

pub struct Phone {
    raw: String,
}

pub enum NewUser {
    WithEmail { email: Email, password: Vec<u8> },
    WithPhone { phone: Phone, password: Vec<u8> },
    // FromInvite,
    // ..
}

pub type Id = u64;

pub struct Account {
    id: Id,
    email: Option<Email>,
    phone: Option<Phone>,
    password: Password,
}

pub struct Profile {
    id: Id,
    name: Option<String>,
    image: Option<url::Url>,
    dob: Option<std::time::SystemTime>,
}

use self::NewUser::*;

impl From<NewUser> for Account {
    fn from(nu: NewUser) -> Self {
        match nu {
            WithEmail { email, password } => {
                let password = Hasher::new()?.get(password.as_slice)?;
                Account {id: 0, email: Some(email), phone: None, password}
            }
            WithPhone { phone, password } => {
                let password = Hasher::new()?.get(password.as_slice)?;
                Account {id: 0, email: None, phone: Some(phone), password}
            }
        }
    }
}
