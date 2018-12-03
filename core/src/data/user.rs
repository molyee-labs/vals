pub struct FromEmail {
    email: String,
    password: String,
}

pub struct FromPhone {
    phone: String,
    password: String,
}

pub enum NewUser {
    FromEmail,
    FromPhone,
    // FromInvite,
    // ..
}