use user::account::Account;
use result::Result;

pub trait Db {
    fn create_user(&self, nu:&NewUser) -> Result<Account>;
}
