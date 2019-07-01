mod blobs;
mod sql;
mod vars;
mod void;

use crate::blobs::read_json;
use crate::sql::connect;
use crate::vars::gather;
use crate::void::ResultExt;
use rusqlite::Connection;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct Venue {
    name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Team {
    id: u16,
    name: String,
    venue: Venue,
    abbreviation: String,
}

#[derive(Deserialize)]
struct Teams {
    teams: Vec<Team>,
}

const CREATE_TEAMS: &str = {
    "CREATE TABLE IF NOT EXISTS teams \
     ( id INTEGER PRIMARY KEY \
     , abbreviation TEXT NOT NULL \
     , name TEXT NOT NULL \
     , venue_name TEXT NOT NULL \
     );"
};

const INSERT_TEAMS: &str = {
    "INSERT INTO teams \
     ( id \
     , abbreviation \
     , name \
     , venue_name \
     ) VALUES (?1, ?2, ?3, ?4);"
};

fn insert(c: &mut Connection, teams: &[Team]) {
    if let Ok(t) = c.transaction() {
        for team in teams {
            t.execute(
                INSERT_TEAMS,
                &[&team.id, &team.abbreviation, &team.name, &team.venue.name],
            )
            .void();
        }
        t.commit().void()
    }
}

fn main() {
    if let Some((_, _, wd)) = gather() {
        if let Ok(mut c) = connect(&wd) {
            c.execute(CREATE_TEAMS, &[]).void();
            if let Some(teams) = {
                let teams: Option<Teams> =
                    read_json(Path::new(&format!("{}/data/teams.json", &wd)));
                teams
            } {
                insert(&mut c, &teams.teams);
            }
        }
    }
}
