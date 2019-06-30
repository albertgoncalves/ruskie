mod blobs;
mod sql;
mod test_schedules;
mod theft;
mod vars;
mod void;

use crate::blobs::read_json;
use crate::sql::{connect, query_ledger_id};
use crate::theft::{filename, get_to_file};
use crate::vars::gather;
use crate::void::{OptionExt, ResultExt};
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::Number;
use std::path::Path;

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

const QUERY_TEAM_IDS: &str = {
    "SELECT t.id \
     FROM teams t \
     INNER JOIN ledger l ON l.id = t.ledger_id \
     WHERE l.id = ?1;"
};

const INSERT_SCHEDULES: &str = {
    "INSERT INTO schedules \
     ( id \
     , ledger_id \
     , status_abstract \
     , status_detailed \
     , status_start_time_tbd \
     , date \
     , type \
     , season \
     , home_team_id \
     , away_team_id \
     , venue_name \
     ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11);"
};

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

fn scrape(
    start: &str,
    end: &str,
    wd: &str,
    id: Option<u32>,
) -> Option<Schedule> {
    id.and_then(|id| {
        let x: String = filename(&wd, "schedules", id, &start, &end);
        println!("{}", &x);
        get_to_file(&url(id, &start, &end), Path::new(&x), 500);
        read_json(x)
    })
}

fn insert(schedule: Schedule, ledger_id: u32, c: &mut Connection) {
    if let Ok(t) = c.transaction() {
        for date in schedule.dates {
            for game in date.games {
                t.execute(
                    INSERT_SCHEDULES,
                    &[
                        &game.gamePk.to_string(),
                        &ledger_id,
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
    if let Some((start, end, wd)) = gather() {
        if let Ok(mut c) = connect(&wd) {
            if let Some(ledger_id) = query_ledger_id(&start, &end, &c) {
                if let Ok(schedules) = {
                    c.prepare(QUERY_TEAM_IDS).and_then(|mut s| {
                        s.query_map(&[&ledger_id], |r| {
                            let id: u32 = r.get("id");
                            id
                        })
                        .map(|ids| {
                            let schedules: Vec<Option<Schedule>> = ids
                                .map(|id| scrape(&start, &end, &wd, id.ok()))
                                .collect();
                            schedules
                        })
                    })
                } {
                    schedules
                        .into_iter()
                        .map(|schedule| {
                            schedule
                                .map(|schedule| {
                                    insert(schedule, ledger_id, &mut c)
                                })
                                .void()
                        })
                        .collect()
                };
            };
        };
    };
}
