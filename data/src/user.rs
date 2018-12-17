use secure::crypt::password;

pub struct Email {
    raw: String,
}

pub struct Phone {
    raw: String,
}

pub struct Password {
    hash: [u8; 128],
    salt: [u8; 16],
}

pub enum NewUser {
    WithEmail { email: Email, password: Password },
    WithPhone { phone: Phone, password: Password },
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
            WithEmail { email, password } => Account {
                id: 0,
                email: Some(email),
                phone: None,
                password,
            },
            WithPhone { phone, password } => Account {
                id: 0,
                email: None,
                phone: Some(phone),
                password,
            },
        }
    }
}
