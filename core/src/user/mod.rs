use ::result::*;
use chrono::prelude::*;
use std::collections::HashSet;
pub type Id = u64;

pub type GroupId = u32;

pub type RoleId = u32;

pub struct Account {
    id: Id,
    email: String,
    phone: String,
    password: String,
}

pub struct Profile {
    id: Id,
    name: String,
    image: String,
    dob: Date,
}

pub struct Role {
    id: RoleId,
    name: String,
}

pub struct Group {
    id: GroupId,
    name: String,
    owner: Id,
}

pub struct Membership {
    uid: Id,
    gid: GroupId,
    rid: RoleId,
}

pub fn add(u:&Account) -> Result<Account> {
    Ok(u)
}

pub fn delete(u:&Account) -> Result<Account> {
    Ok(u)
}

pub fn find(uid:Id) -> Result<Account> {
    Err(Error::NotFound)
}

pub fn find_members(gid:GroupId) -> Result<Vec<Account>> {
    Err(Error::NotFound)
}

pub fn find_by_email(email:&str) -> Result<Account> {
    Err(Error::NotFound)
}

pub fn find_by_phone(phone:&str) -> Result<Account> {
    Err(Error::NotFound)
}

pub fn memberships(uid:Id) -> Result<Vec<Membership>> {
    Err(Error::NotFound)
}

pub fn roles(uid:Id, gid:GroupId) -> Result<Vec<Role>> {
    Err(Error::NotFound)
}

pub fn groups(uid:Id) -> Result<Vec<Group>> {
    Err(Error::NotFound)
}

pub fn join(uid:Id, gid:GroupId, roles:&[RoleId]) -> Result<Vec<Membership>> {
    Err(Error::NotFound)
}

pub fn leave(uid:Id, gid:GroupId) -> Result<Vec<Membership>> {
    Ok(())
}

pub fn assign(uid:Id, gid:GroupId, rid:RoleId) -> Result<Membership> {
    Ok(())
}

pub fn unassign(uid:Id, gid:GroupId, rid:RoleId) -> Result<Membership> {
    Ok(Membership{uid, gid, rid})
}
