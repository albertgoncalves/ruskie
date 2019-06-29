mod blobs;
mod sql;
mod void;

use crate::blobs::{read_json, Team};
use crate::sql::{
    Ledger, CREATE_LEDGER, CREATE_TABLE, INSERT_LEDGER, INSERT_TEAM,
    QUERY_LEDGER_ID,
};
use crate::void::ResultExt;
use rusqlite::Connection;
use std::env::var;

fn inject_teams(c: &mut Connection, ledger_id: u32, teams: &[Team]) {
    if let Ok(t) = c.transaction() {
        for team in teams {
            t.execute(
                INSERT_TEAM,
                &[
                    &ledger_id,
                    &team.id,
                    &team.name,
                    &team.abbreviation,
                    &team.venue.name,
                ],
            )
            .void();
        }
        t.commit().void()
    }
}

fn main() {
    let start = "2018-08-01";
    let end = "2019-08-01";
    if let Ok(wd) = var("WD") {
        Connection::open(format!("{}/ruskie.db", wd))
            .map(|mut c| {
                c.execute(CREATE_LEDGER, &[]).void();
                c.execute(CREATE_TABLE, &[]).void();
                c.execute(INSERT_LEDGER, &[&start, &end]).void();
                if let Ok(ledger) = c.query_row(
                    QUERY_LEDGER_ID, //
                    &[&start, &end], //
                    |r| Ledger { id: r.get("id") },
                ) {
                    if let Some(xs) = read_json(format!(
                        "{}/data/teams-{}-{}.json",
                        wd,     //
                        &start, //
                        &end,
                    )) {
                        inject_teams(&mut c, ledger.id, &xs.teams);
                    };
                }
            })
            .void()
    }
}
