use core::db::Db;
use postgres::{Connection, TlsMode};

pub struct DbPostgres {
    conn: Connection,
}

impl Db for DbPostgres {
    fn new(dsn: &str, dbname: &str) -> Db {
        let conn = Connection::connect(dsn, TlsMode::None)?;
        Db { conn }
    }
}
