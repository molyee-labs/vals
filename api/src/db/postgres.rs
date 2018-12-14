use core::db::*;
use core::result::*;
use postgres::{types, stmt::Statement, Connection, TlsMode};

pub struct PgDb {
    conn: Connection,
}

impl PgDb {
    pub fn new(dsn: &str, dbname: &str) -> Self {
        let conn = Connection::connect(dsn, TlsMode::None)?;
        Db { conn }
    }
}

pub struct PgQuery {
    source: String,
    args: Vec<ToSql>,
}

impl Query for PgQuery {
    fn str(&self) -> &str {
        &self.source
    }

    fn put(&self, )
}

impl Database for PgDb {
    fn get<T>(&self, q: &Query) -> Result<T> {
        &self.conn.query
    }

    fn call<T>(&self, f: &Function) -> Result<T> {

    }
}
