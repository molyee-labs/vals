use common::builder::*;
use common::pool::*;
use diesel::pg::*;
use diesel::prelude::*;
use std::sync::Arc;

pub struct Conn(PgConnection);

pub struct Db {
    pool: Pool<Conn>,
}

impl Db {
    fn new(dsn: &str) -> Self {
        let dsn = dsn.to_string();
        let f = move || Conn(PgConnection::establish(&dsn).unwrap());
        let pool = Pool::with_closure(f);
        Db { pool }
    }
}
