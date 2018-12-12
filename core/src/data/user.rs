use crate::secure::password::Password;

pub struct Email {
    raw: String,
}

pub struct Phone {
    raw: String,
}

pub enum NewUser {
    WithEmail { email: Email, password: String },
    WithPhone { phone: Phone, password: String },
    // FromInvite,
    // ..
}

pub type Id = u64;

pub struct Account {
    id: Id,
    email: Email,
    phone: Phone,
    password: Password,
}

pub struct Profile {
    id: Id,
    name: String,
    image: url::Url,
    dob: std::time::SystemTime,
}
