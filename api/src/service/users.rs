use core::context::Context;
use core::db::Db;
use core::user::account::Account;
use data::user;
use env::Env;
use result::*;

struct Service {
    db: Db,
    env: Env,
}

impl Service {
    pub fn add_user(&self, c:&Context, nu:&user::NewUser) -> Result<()> {
        let u:Account = &self.db.create_user(&nu)?;
    }

    pub fn find_user(&self, c:&Context) -> Result<Account> {

    }
}