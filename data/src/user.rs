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
