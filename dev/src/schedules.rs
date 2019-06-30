mod blobs;
mod sql;
mod vars;
mod void;

use crate::blobs::{get_to_file, read_json};
use crate::sql::{connect, query_ledger_id};
use crate::vars::gather;
use crate::void::{OptionExt, ResultExt};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Id {
    id: u16,
}

#[derive(Serialize, Deserialize)]
struct Name {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Team {
    score: u8,
    team: Id,
}

#[derive(Serialize, Deserialize)]
struct Teams {
    away: Team,
    home: Team,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Status {
    abstractGameState: String,
    detailedState: String,
    startTimeTBD: bool,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Game {
    gamePk: u32,
    gameType: String,
    season: String,
    status: Status,
    teams: Teams,
    venue: Name,
}

#[derive(Serialize, Deserialize)]
struct Date {
    date: String,
    games: Vec<Game>,
}

#[derive(Serialize, Deserialize)]
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
     , ledger_id
     , status_abstract
     , status_detailed
     , status_start_time_tbd
     , date \
     , type \
     , season \
     , home_team_id \
     , away_team_id \
     , venue_name \
     ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11);"
};

fn filename(wd: &str, id: u32, start: &str, end: &str) -> String {
    format!("{}/data/schedule-{}-{}-{}.json", wd, id, start, end)
}

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
    id.and_then(|x| {
        let u: String = url(x, &start, &end);
        let p: String = filename(&wd, x, &start, &end);
        println!("{}", &p);
        get_to_file(&u, Path::new(&p), 500);
        read_json(p)
    })
}

fn insert(schedule: Schedule, ledger_id: u32, c: &mut Connection) {
    if let Ok(t) = c.transaction() {
        for date in schedule.dates {
            for game in date.games {
                t.execute(
                    INSERT_SCHEDULES,
                    &[
                        &game.gamePk,
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
