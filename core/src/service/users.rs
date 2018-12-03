use context::Context;
use db::Db;
use data::user;
use env::Env;
use user::account::Account;
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