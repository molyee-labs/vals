use diesel::prelude::*;
use diesel::pg::*;
use std::sync::Arc;
use common::sync::pool::*;

pub trait Conn : diesel::Connection + Sized {}

const POOL: Arc<Pool<Conn>> = Arc::new(Pool::new());

pub fn get() -> Holder<Conn> {
    POOL
}
