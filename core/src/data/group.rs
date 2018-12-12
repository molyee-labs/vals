use super::user;

pub type Id = u32;

pub struct Group {
    id: Id,
    name: String,
    owner: user::Id,
}
