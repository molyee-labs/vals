use user::account::Account;
use result::Result;
use std::convert::From;

pub enum DbType {
    SMALLINT(i16),
    INT(i32),
    BIGINT(i64),
    TEXT(String),
    REAL(f32),
    DOUBLE(f64),
}

impl From<i16> for DbType {
    fn from(v: i16) -> Self {
        DbType::SMALLINT(v)
    }
}

impl From<i32> for DbType {
    fn from(v: i32) -> Self {
        DbType::INT(v)
    }
}

impl From<i64> for DbType {
    fn from(v: i64) -> Self {
        DbType::BIGINT(v)
    }
}

impl From<f32> for DbType {
    fn from(v: f32) -> Self {
        DbType::REAL(v)
    }
}

impl From<f64> for DbType {
    fn from(v: f64) -> Self {
        DbType::DOUBLE(v)
    }
}

impl From<&str> for DbType {
    fn from(v: &str) -> Self {
        DbType::TEXT(v.to_string())
    }
}

pub trait Db {
    fn exec<T>(&self, func:&str, args:&[DbType]) -> Result<T>;
    fn call(&self, proc:&str, args:&[DbType]) -> Result<()>;
    fn view<T>(&self, view:&str) -> Result<T>;
}

