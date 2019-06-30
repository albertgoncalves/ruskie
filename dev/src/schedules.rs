mod blobs;
mod sql;
mod vars;
mod void;

use crate::blobs::read_json;
use crate::sql::connect;
use crate::vars::gather;
use crate::void::{OptionExt, ResultExt};
use reqwest::Client;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct Id {
    id: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct Name {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Team {
    score: u8,
    team: Id,
}

#[derive(Serialize, Deserialize, Debug)]
struct Teams {
    away: Team,
    home: Team,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Game {
    gamePk: u32,
    gameType: String,
    season: String,
    teams: Teams,
    venue: Name,
}

#[derive(Serialize, Deserialize, Debug)]
struct Date {
    date: String,
    games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Schedule {
    dates: Vec<Date>,
}

const QUERY_TEAM_IDS: &str = {
    "SELECT t.id \
     FROM teams t \
     INNER JOIN ledger l ON l.id = t.ledger_id \
     WHERE l.start = DATE(?1) \
     AND l.end = DATE(?2);"
};

const INSERT_SCHEDULES: &str = {
    "INSERT INTO schedules \
     ( id \
     , date \
     , type \
     , season \
     , home_team_id \
     , home_team_score \
     , away_team_id \
     , away_team_score \
     , venue_name \
     ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);"
};

fn get_to_file(url: &str, filename: &Path) {
    if !filename.exists() {
        println!("{}", url);
        let buffer = File::create(filename).map(BufWriter::new).ok();
        let client = Client::new();
        client
            .get(url)
            .send()
            .ok()
            .and_then(|mut r| buffer.and_then(|mut f| r.copy_to(&mut f).ok()))
            .void();
        sleep(Duration::from_millis(500))
    }
}

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
        get_to_file(&u, Path::new(&p));
        read_json(p)
    })
}

fn insert(schedule: Schedule, c: &mut Connection) {
    if let Ok(t) = c.transaction() {
        for date in schedule.dates {
            for game in date.games {
                t.execute(
                    INSERT_SCHEDULES,
                    &[
                        &game.gamePk,
                        &date.date,
                        &game.gameType,
                        &game.season,
                        &game.teams.home.team.id,
                        &game.teams.home.score,
                        &game.teams.away.team.id,
                        &game.teams.away.score,
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
            if let Ok(schedules) = {
                c.prepare(QUERY_TEAM_IDS).and_then(|mut s| {
                    s.query_map(&[&start, &end], |r| {
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
                    .map(|x| x.map(|y| insert(y, &mut c)).void())
                    .collect()
            };
        };
    };
}
