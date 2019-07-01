mod blobs;
mod sql;
mod theft;
mod vars;
mod void;

use crate::blobs::read_json;
use crate::sql::connect;
use crate::theft::{filename, get_to_file};
use crate::vars::gather;
use crate::void::ResultExt;
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::Number;
use std::collections::HashMap;
use std::path::PathBuf;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Result {
    event: String,
    secondaryType: Option<String>,
    penaltySeverity: Option<String>,
    penaltyMinutes: Option<u8>,
}

#[derive(Deserialize)]
struct Coordinates {
    x: Option<Number>,
    y: Option<Number>,
}

#[derive(Deserialize)]
struct Goals {
    away: u8,
    home: u8,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct About {
    eventIdx: Number,
    eventId: Number,
    period: u8,
    periodTime: String,
    periodTimeRemaining: String,
    goals: Goals,
}

#[derive(Deserialize)]
struct PlayerId {
    id: Number,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Participant {
    player: PlayerId,
    playerType: String,
}

#[derive(Deserialize)]
struct TeamId {
    id: u16,
}

#[derive(Deserialize)]
struct Person {
    id: Number,
    fullName: String,
    shootsCatches: String,
    rosterStatus: String,
}

#[derive(Deserialize)]
struct Position {
    r#type: String,
    abbreviation: String,
}

#[derive(Deserialize)]
struct Player {
    person: Person,
    position: Position,
}

#[derive(Deserialize)]
struct Event {
    result: Result,
    about: About,
    coordinates: Option<Coordinates>,
    team: Option<TeamId>,
    players: Option<Vec<Participant>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Plays {
    allPlays: Vec<Event>,
}

#[derive(Deserialize)]
struct Team {
    team: TeamId,
    players: HashMap<String, Player>,
}

#[derive(Deserialize)]
struct Teams {
    home: Team,
    away: Team,
}

#[derive(Deserialize)]
struct Boxscore {
    teams: Teams,
}

#[derive(Deserialize)]
struct LiveData {
    plays: Plays,
    boxscore: Boxscore,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Events {
    gamePk: Number,
    liveData: LiveData,
}

#[derive(Deserialize)]
struct Shifts {}

const CREATE_PLAYERS: &str = {
    "CREATE TABLE IF NOT EXISTS players \
     ( game_id TEXT \
     , id TEXT NOT NULL \
     , team_id INTEGER \
     , full_name TEXT NOT NULL \
     , shoots_catches TEXT \
     , roster_status TEXT NOT NULL \
     , position_type TEXT NOT NULL \
     , position_abbreviation TEXT NOT NULL \
     , FOREIGN KEY (game_id) REFERENCES games(id) \
     , UNIQUE(id, game_id) \
     ); "
};

const INSERT_PLAYERS: &str = {
    "INSERT INTO players \
     ( game_id \
     , id \
     , team_id \
     , full_name \
     , shoots_catches \
     , roster_status \
     , position_type \
     , position_abbreviation \
     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);"
};

const QUERY_GAME_IDS: &str = {
    "SELECT id \
     FROM games \
     WHERE status_abstract = 'Final' \
     AND status_detailed = 'Final';"
};

#[inline]
fn scrape(wd: &str, id: &str, directory: &str, url: &str) -> PathBuf {
    let x: PathBuf = filename(wd, directory, id);
    get_to_file(url, x.as_path(), 1500);
    x
}

#[inline]
fn events_url(id: &str) -> String {
    format!(
        "https://statsapi.web.nhl.com/api/v1/game/{}/feed/live?site=en_nhl",
        id,
    )
}

#[inline]
fn shifts_url(id: &str) -> String {
    format!(
        "http://www.nhl.com/stats/rest/shiftcharts?cayenneExp=gameId={}",
        id,
    )
}

#[inline]
fn scrape_pair<'a>(
    wd: &'a str,
    id: &Option<String>,
) -> Option<(PathBuf, PathBuf)> {
    id.as_ref().map(|id| {
        (
            scrape(wd, id, "events", &events_url(id)),
            scrape(wd, id, "shifts", &shifts_url(id)),
        )
    })
}

fn insert_events(t: &Connection, events: Events) {
    println!("{}", events.gamePk.to_string());
    for (_, player) in events.liveData.boxscore.teams.home.players.into_iter()
    {
        t.execute(
            INSERT_PLAYERS,
            &[
                &events.gamePk.to_string(),
                &player.person.id.to_string(),
                &events.liveData.boxscore.teams.home.team.id.to_string(),
                &player.person.fullName,
                &player.person.shootsCatches,
                &player.person.rosterStatus,
                &player.position.r#type,
                &player.position.abbreviation,
            ],
        )
        .void();
    }
}

// fn insert_shifts(_t: &Connection, _shifts: Shifts) {
// }

fn main() {
    if let Some((_, _, wd)) = gather() {
        if let Ok(mut c) = connect(&wd) {
            c.execute(CREATE_PLAYERS, &[]).void();
            if let Ok(ids) = c.prepare(QUERY_GAME_IDS).and_then(|mut s| {
                s.query_map(&[], |r| {
                    let id: String = r.get("id");
                    id
                })
                .map(|ids| {
                    let filenames: Vec<Option<(PathBuf, PathBuf)>> =
                        ids.map(|id| (scrape_pair(&wd, &id.ok()))).collect();
                    filenames
                })
            }) {
                if let Ok(t) = c.transaction() {
                    for pair in ids {
                        if let Some((events, _shifts)) = pair {
                            if let (Some(events), Some(_)) = {
                                let events: Option<Events> =
                                    read_json(events.as_path());
                                // let shifts: Option<Shifts> =
                                //     read_json(shifts.as_path());
                                // (events, shifts)
                                (events, Some(()))
                            } {
                                insert_events(&t, events);
                                // insert_shifts(&t, shifts)
                            }
                        }
                    }
                    t.commit().void()
                }
            }
        };
    };
}
