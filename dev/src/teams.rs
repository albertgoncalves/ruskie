mod blobs;
mod sql;
mod vars;
mod void;

use crate::blobs::read_json;
use crate::sql::{connect, query_ledger_id};
use crate::vars::gather;
use crate::void::ResultExt;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Venue {
    name: String,
    id: Option<u16>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Team {
    id: u16,
    name: String,
    venue: Venue,
    abbreviation: String,
}

#[derive(Serialize, Deserialize)]
struct Teams {
    teams: Vec<Team>,
}

const CREATE_LEDGER: &str = {
    "CREATE TABLE IF NOT EXISTS ledger \
     ( id INTEGER PRIMARY KEY \
     , start DATE NOT NULL \
     , end DATE NOT NULL \
     , UNIQUE(start, end) \
     );"
};

const CREATE_TEAMS: &str = {
    "CREATE TABLE IF NOT EXISTS teams \
     ( id INTEGER PRIMARY KEY \
     , ledger_id INTEGER \
     , abbreviation TEXT NOT NULL \
     , name TEXT NOT NULL \
     , venue_name TEXT NOT NULL \
     , FOREIGN KEY (ledger_id) REFERENCES ledger(id) \
     , UNIQUE(id, ledger_id) \
     );"
};

const CREATE_SCHEDULES: &str = {
    "CREATE TABLE IF NOT EXISTS schedules \
     ( id INTEGER PRIMARY KEY \
     , ledger_id INTEGER \
     , status_abstract TEXT NOT NULL \
     , status_detailed TEXT NOT NULL \
     , status_start_time_tbd BOOLEAN NOT NULL \
     , date DATE NOT NULL \
     , type TEXT NOT NULL \
     , season TEXT NOT NULL \
     , home_team_id INTEGER \
     , away_team_id INTEGER \
     , venue_name TEXT NOT NULL \
     , FOREIGN KEY (ledger_id) REFERENCES ledger(id) \
     , FOREIGN KEY (home_team_id) REFERENCES teams(id) \
     , FOREIGN KEY (away_team_id) REFERENCES teams(id) \
     , UNIQUE(id, ledger_id) \
     );"
};

const INSERT_LEDGER: &str = "INSERT INTO ledger (start, end) values (?1, ?2);";

const INSERT_TEAMS: &str = {
    "INSERT INTO teams \
     ( ledger_id \
     , id \
     , abbreviation \
     , name \
     , venue_name \
     ) values (?1, ?2, ?3, ?4, ?5);"
};

fn insert_teams(c: &mut Connection, ledger_id: u32, teams: &[Team]) {
    if let Ok(t) = c.transaction() {
        for team in teams {
            println!("{}", team.abbreviation);
            t.execute(
                INSERT_TEAMS,
                &[
                    &ledger_id,
                    &team.id,
                    &team.abbreviation,
                    &team.name,
                    &team.venue.name,
                ],
            )
            .void();
        }
        t.commit().void()
    }
}

fn main() {
    gather()
        .map(|(start, end, wd)| {
            connect(&wd)
                .map(|mut c| {
                    c.execute(CREATE_LEDGER, &[]).void();
                    c.execute(CREATE_TEAMS, &[]).void();
                    c.execute(CREATE_SCHEDULES, &[]).void();
                    c.execute(INSERT_LEDGER, &[&start, &end]).void();
                    if let (Some(ledger_id), Some(teams)) =
                        (query_ledger_id(&start, &end, &c), {
                            let teams: Option<Teams> = read_json(format!(
                                "{}/data/teams-{}-{}.json",
                                &wd,    //
                                &start, //
                                &end,
                            ));
                            teams
                        })
                    {
                        insert_teams(&mut c, ledger_id, &teams.teams);
                    }
                })
                .void()
        })
        .void()
}
