use super::user;

pub type Id = u32;

pub struct Group {
    id: Id,
    name: String,
    owner: user::Id,
}

impl Group {
    pub fn new(id: Id, name: String, owner: user::Id) -> Self {
        Group { id, name, owner }
    }
}
