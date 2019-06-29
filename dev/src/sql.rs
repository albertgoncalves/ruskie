pub const CREATE_LEDGER: &str = {
    "CREATE TABLE IF NOT EXISTS ledger \
     ( id INTEGER PRIMARY KEY \
     , start DATE NOT NULL \
     , end DATE NOT NULL \
     , UNIQUE(start, end)
     );"
};

pub const CREATE_TABLE: &str = {
    "CREATE TABLE IF NOT EXISTS teams \
     ( id INTEGER PRIMARY KEY \
     , ledger_id INTEGER \
     , team_id INTEGER NOT NULL \
     , team_name TEXT NOT NULL \
     , abbreviation TEXT NOT NULL \
     , venue_name TEXT NOT NULL \
     , FOREIGN KEY (ledger_id) REFERENCES ledger(id) \
     , UNIQUE(ledger_id, team_id)
     );"
};

pub const INSERT_LEDGER: &str =
    "INSERT INTO ledger (start, end) values (?1, ?2);";

pub const INSERT_TEAM: &str = {
    "INSERT INTO teams \
     ( ledger_id \
     , team_id \
     , team_name \
     , abbreviation \
     , venue_name \
     ) values (?1, ?2, ?3, ?4, ?5);"
};

pub const QUERY_LEDGER_ID: &str = {
    "SELECT * \
     FROM ledger \
     WHERE start = DATE(?1) \
     AND end = DATE(?2);"
};

pub struct Ledger {
    pub id: u32,
}
