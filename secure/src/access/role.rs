use super as user;

pub type Id = u32;

pub struct Role {
    id: Id,
    parent: Option<Id>,
    name: String,
}

pub fn create(name:String) -> Result<Role> {

}

pub fn inherit(parent:Id, name:String) -> Result<Role> {

}

pub fn assign(uid:user::Id, rid:Id) -> Result<()> {
    Ok(())
}

pub fn unassign(uid:user::Id, rid:Id) -> Result<()> {
    Ok(())
}

pub fn assignments(uid:user::Id) -> Result<Vec<Role>> {
    Ok(())
}