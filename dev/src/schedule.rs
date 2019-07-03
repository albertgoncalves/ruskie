mod blobs;
mod scrape;
mod sql;
mod void;

use crate::blobs::read_json;
use crate::scrape::{filename, get_to_file};
use crate::sql::connect;
use crate::void::{OptionExt, ResultExt};
use rayon::prelude::*;
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::Number;
use std::env::var;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Id {
    id: u16,
}

#[derive(Deserialize)]
struct Name {
    name: String,
}

#[derive(Deserialize)]
struct Team {
    team: Id,
}

#[derive(Deserialize)]
struct Teams {
    away: Team,
    home: Team,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Status {
    abstractGameState: String,
    detailedState: String,
    startTimeTBD: bool,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Game {
    gamePk: Number,
    gameType: String,
    season: String,
    status: Status,
    teams: Teams,
    venue: Name,
}

#[derive(Deserialize)]
struct Date {
    date: String,
    games: Vec<Game>,
}

#[derive(Deserialize)]
struct Schedule {
    dates: Vec<Date>,
}

const CREATE_GAMES: &str = {
    "CREATE TABLE IF NOT EXISTS schedule \
     ( id TEXT PRIMARY KEY \
     , status_abstract TEXT NOT NULL \
     , status_detailed TEXT NOT NULL \
     , status_start_time_tbd BOOLEAN NOT NULL \
     , date DATE NOT NULL \
     , type TEXT NOT NULL \
     , season TEXT NOT NULL \
     , home_team_id INTEGER \
     , away_team_id INTEGER \
     , venue_name TEXT NOT NULL \
     );"
};

const INDEX_HOME_TEAM_ID: &str =
    "CREATE INDEX index_schedule_home_team_id ON schedule(home_team_id);";

const INDEX_AWAY_TEAM_ID: &str =
    "CREATE INDEX index_schedule_away_team_id ON schedule(away_team_id);";

const INDEX_TYPE: &str = "CREATE INDEX index_schedule_type ON schedule(type);";

const INDEX_SEASON: &str =
    "CREATE INDEX index_schedule_season ON schedule(season);";

const QUERY_TEAM_IDS: &str = {
    "SELECT t.id \
     FROM teams t;"
};

const INSERT_GAMES: &str = {
    "INSERT INTO schedule \
     ( id \
     , status_abstract \
     , status_detailed \
     , status_start_time_tbd \
     , date \
     , type \
     , season \
     , home_team_id \
     , away_team_id \
     , venue_name \
     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10);"
};

#[inline]
fn url(id: u32, start: &str, end: &str) -> String {
    format!(
        "https://statsapi.web.nhl.com/api/v1/schedule?\
         teamId={}&\
         startDate={}&\
         endDate={}",
        id,     //
        &start, //
        &end,
    )
}

#[inline]
fn scrape(
    start: &str,
    end: &str,
    wd: &str,
    id: Option<u32>,
) -> Option<PathBuf> {
    id.map(|id| {
        let x: PathBuf =
            filename(&wd, "schedule", format!("{}-{}-{}", &start, &end, id));
        get_to_file(&url(id, &start, &end), x.as_path(), 500);
        x
    })
}

#[inline]
fn insert(schedule: Schedule, c: &mut Connection) {
    if let Ok(t) = c.transaction() {
        for date in schedule.dates {
            for game in date.games {
                t.execute(
                    INSERT_GAMES,
                    &[
                        &game.gamePk.to_string(),
                        &game.status.abstractGameState,
                        &game.status.detailedState,
                        &game.status.startTimeTBD,
                        &date.date,
                        &game.gameType,
                        &game.season,
                        &game.teams.home.team.id,
                        &game.teams.away.team.id,
                        &game.venue.name,
                    ],
                )
                .void()
            }
        }
        t.commit().void()
    }
}

fn main() {
    if let (Ok(start), Ok(end), Ok(wd)) = (
        var("START"), //
        var("END"),   //
        var("WD"),
    ) {
        if let Ok(mut c) = connect(&wd) {
            let xs: [&str; 5] = [
                CREATE_GAMES,
                INDEX_HOME_TEAM_ID,
                INDEX_AWAY_TEAM_ID,
                INDEX_TYPE,
                INDEX_SEASON,
            ];
            for x in &xs {
                c.execute(x, &[]).void();
            }
            if let Ok(schedules) = {
                c.prepare(QUERY_TEAM_IDS).and_then(|mut s| {
                    s.query_map(&[], |r| {
                        let id: u32 = r.get("id");
                        id
                    })
                    .map(|ids| {
                        ids.map(|id| scrape(&start, &end, &wd, id.ok()))
                            .collect::<Vec<Option<PathBuf>>>()
                    })
                })
            } {
                schedules
                    .par_iter()
                    .map(|s| s.as_ref().and_then(|s| read_json(s.as_path())))
                    .collect::<Vec<Option<Schedule>>>()
                    .into_iter()
                    .map(|schedule| {
                        schedule
                            .map(|schedule| insert(schedule, &mut c))
                            .void()
                    })
                    .collect()
            };
        };
    };
}
