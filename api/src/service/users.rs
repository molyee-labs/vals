use core::context::Context;
use core::env::Env;
use core::result::*;
use data::db::Db;
use data::user::*;

struct Service {
    db: Db,
    env: Env,
}

impl Service {
    pub fn add_user(&self, c: &Context, nu: &NewUser) -> Result<()> {
        unimplemented!()
    }

    pub fn find_user(&self, c: &Context) -> Result<Account> {
        unimplemented!()
    }
}
