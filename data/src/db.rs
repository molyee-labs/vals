use diesel::prelude::*;
use diesel::pg::*;
use std::sync::Arc;
use common::pool::*;
use common::builder::*;

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