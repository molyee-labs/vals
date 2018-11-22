use super as user;

pub type Id = u32;

pub struct Group {
    id: Id,
    name: String,
    owner: user::Id,
}

pub fn create(name:String, owner:user::Id) -> Result<Group> {

}

pub fn set_owner(gid:Id, owner:user::Id) -> Result<()> {

}

pub fn members(gid:Id) -> Result<Vec<user::Account>> {
    Err(Error::NotFound)
}

pub fn membership(uid:user::Id) -> Result<Vec<Group>> {
    Err(Error::NotFound)
}

pub fn join(uid:user::Id, gid:Id) -> Result<()> {
    Err(Error::NotFound)
}

pub fn leave(uid:user::Id, gid:Id) -> Result<()> {
    Ok(())
}