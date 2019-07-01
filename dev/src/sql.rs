use rusqlite::{Connection, Result};

pub fn connect(wd: &str) -> Result<Connection> {
    Connection::open(format!("{}/ruskie.db", &wd))
}
