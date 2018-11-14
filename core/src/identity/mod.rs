pub type Id = u64;

pub type UserId = Id;

pub struct User {
    id: UserId,
    name: String,
    email: String,
    password: String,
    phone: String
}

pub type RoleId = Id;

pub struct Role {
    id: RoleId,
    name: String
}
