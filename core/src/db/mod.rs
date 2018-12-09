use ::result::Result;
use std::convert::From;


pub trait Db {
    fn func<T>(&self, func: &str, args: &[DbType]) -> Result<T>;
    fn proc<T>(&self, proc: &str, args: &[DbType]) -> Result<T>;
    fn view<T>(&self, view: &str) -> Result<T>;
}
