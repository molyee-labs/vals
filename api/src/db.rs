use core::db::Db;
use core::result::*;
use postgres::{stmt::Statement, types::*, Connection, TlsMode};

pub struct DbPostgres {
    conn: Connection,
}

impl DbPostgres {
    pub fn new(dsn: &str, dbname: &str) -> Self {
        let conn = Connection::connect(dsn, TlsMode::None)?;
        Db { conn }
    }
}

impl Db for DbPostgres {
    fn func<T>(&self, func: &str, args: &[DbType]) -> Result<T> {
        let stmt = self.conn.prepare_cached("SELECT " + func + "($1,")?;
        stmt.query()
    }

    fn proc<T>(&self, proc: &str, args: &[DbType]) -> Result<T> {
        Err(Error::NotImplemented)
    }

    fn view<T>(&self, view: &str) -> Result<T> {
        Err(Error::NotImplemented)
    }
}
