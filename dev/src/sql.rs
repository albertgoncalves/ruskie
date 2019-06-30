use rusqlite::{Connection, Result};

const QUERY_LEDGER_ID: &str = {
    "SELECT * \
     FROM ledger \
     WHERE start = DATE(?1) \
     AND end = DATE(?2);"
};

pub fn query_ledger_id(start: &str, end: &str, c: &Connection) -> Option<u32> {
    c.query_row(
        QUERY_LEDGER_ID, //
        &[&start, &end], //
        |r| {
            let id: u32 = r.get("id");
            id
        },
    )
    .ok()
}

pub fn connect(wd: &str) -> Result<Connection> {
    Connection::open(format!("{}/ruskie.db", &wd))
}
