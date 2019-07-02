mod blobs;
mod sql;
mod theft;
mod void;

use crate::blobs::read_json;
use crate::sql::connect;
use crate::theft::{filename, get_to_file};
use crate::void::ResultExt;
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::Number;
use std::collections::HashMap;
use std::env::var;
use std::path::PathBuf;

#[allow(non_snake_case)]
#[derive(Deserialize, Clone)]
struct Result {
    event: String,
    secondaryType: Option<String>,
    penaltySeverity: Option<String>,
    penaltyMinutes: Option<u8>,
}

#[derive(Deserialize, Clone)]
struct Coordinates {
    x: Option<Number>,
    y: Option<Number>,
}

#[derive(Deserialize, Clone)]
struct Goals {
    away: u8,
    home: u8,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone)]
struct About {
    eventId: u16,
    period: u8,
    periodType: String,
    periodTime: String,
    periodTimeRemaining: String,
    goals: Goals,
}

#[derive(Deserialize, Clone)]
struct PlayerId {
    id: Number,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone)]
struct Participant {
    player: PlayerId,
    playerType: String,
}

#[derive(Deserialize, Clone)]
struct TeamId {
    id: u16,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone)]
struct Person {
    id: Number,
    fullName: String,
    shootsCatches: Option<String>,
    rosterStatus: String,
}

#[derive(Deserialize, Clone)]
struct Position {
    r#type: String,
    abbreviation: String,
}

#[derive(Deserialize, Clone)]
struct Player {
    person: Person,
    position: Position,
}

#[derive(Deserialize, Clone)]
struct Event {
    result: Result,
    about: About,
    coordinates: Option<Coordinates>,
    team: Option<TeamId>,
    players: Option<Vec<Participant>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone)]
struct Plays {
    allPlays: Vec<Event>,
}

#[derive(Deserialize, Clone)]
struct Team {
    team: TeamId,
    players: HashMap<String, Player>,
}

#[derive(Deserialize, Clone)]
struct Teams {
    home: Team,
    away: Team,
}

#[derive(Deserialize, Clone)]
struct Boxscore {
    teams: Teams,
}

#[derive(Deserialize, Clone)]
struct LiveData {
    plays: Plays,
    boxscore: Boxscore,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone)]
struct Events {
    gamePk: Number,
    liveData: LiveData,
}

#[derive(Deserialize)]
struct Shifts {}

const QUERY_GAME_IDS: &str = {
    "SELECT id \
     FROM games \
     WHERE status_abstract = 'Final' \
     AND status_detailed = 'Final' \
     AND type = 'R' \
     ORDER BY DATE(date) ASC \
     LIMIT 20;"
};

const CREATE_PLAYERS: &str = {
    "CREATE TABLE IF NOT EXISTS players \
     ( game_id TEXT \
     , team_id INTEGER NOT NULL \
     , id TEXT NOT NULL \
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
     , team_id \
     , id \
     , full_name \
     , shoots_catches \
     , roster_status \
     , position_type \
     , position_abbreviation \
     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);"
};

const CREATE_EVENTS: &str = {
    "CREATE TABLE IF NOT EXISTS events \
     ( game_id TEXT \
     , team_id TEXT NOT NULL \
     , player_id TEXT \
     , player_type TEXT NOT NULL \
     , id INTEGER NOT NULL \
     , event TEXT NOT NULL \
     , secondary_type TEXT \
     , penality_severity TEXT \
     , penality_minutes INTEGER \
     , period INTEGER NOT NULL \
     , period_type TEXT NOT NULL \
     , period_time TEXT NOT NULL \
     , period_time_remaining TEXT NOT NULL \
     , away_score INTEGER NOT NULL \
     , home_score INTEGER NOT NULL \
     , x REAL \
     , y REAL \
     , FOREIGN KEY (player_id, game_id) REFERENCES players(id, game_id) \
     , UNIQUE(game_id, player_id, id) \
     ); "
};

const INSERT_EVENTS: &str = {
    "INSERT INTO events
     ( game_id \
     , team_id \
     , player_id \
     , player_type \
     , id \
     , event \
     , secondary_type \
     , penality_severity \
     , penality_minutes \
     , period \
     , period_type \
     , period_time \
     , period_time_remaining \
     , away_score \
     , home_score \
     , x \
     , y \
     ) VALUES \
     ( ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16
     , ?17
     );"
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

fn insert_player(t: &Connection, game_id: &str, team: Team) {
    for (_, player) in team.players.into_iter() {
        t.execute(
            INSERT_PLAYERS,
            &[
                &game_id,
                &team.team.id,
                &player.person.id.to_string(),
                &player.person.fullName,
                &player.person.shootsCatches,
                &player.person.rosterStatus,
                &player.position.r#type,
                &player.position.abbreviation,
            ],
        )
        .void()
    }
}

fn insert_players(t: &Connection, game_id: &str, away: Team, home: Team) {
    insert_player(t, game_id, away);
    insert_player(t, game_id, home);
}

fn insert_events(t: &Connection, game_id: &str, events: Events) {
    for play in events.liveData.plays.allPlays {
        if let Some(players) = play.players {
            for player in players {
                t.execute(
                    INSERT_EVENTS,
                    &[
                        &game_id,
                        &play.team.clone().map(|t| t.id),
                        &player.player.id.to_string(),
                        &player.playerType,
                        &play.about.eventId,
                        &play.result.event,
                        &play.result.secondaryType,
                        &play.result.penaltySeverity,
                        &play.result.penaltyMinutes,
                        &play.about.period,
                        &play.about.periodType,
                        &play.about.periodTime,
                        &play.about.periodTimeRemaining,
                        &play.about.goals.away,
                        &play.about.goals.home,
                        &play
                            .coordinates
                            .clone()
                            .and_then(|c| c.x.and_then(|x| x.as_f64())),
                        &play
                            .coordinates
                            .clone()
                            .and_then(|c| c.y.and_then(|y| y.as_f64())),
                    ],
                )
                .void()
            }
        }
    }
}

// fn insert_shifts(_t: &Connection, _shifts: Shifts) {
// }

fn main() {
    if let Ok(wd) = var("WD") {
        if let Ok(mut c) = connect(&wd) {
            c.execute(CREATE_PLAYERS, &[]).void();
            c.execute(CREATE_EVENTS, &[]).void();
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
                                let teams: Teams =
                                    events.liveData.boxscore.teams.clone();
                                let away: Team = teams.away.clone();
                                let home: Team = teams.home;
                                let game_id: String =
                                    events.gamePk.to_string();
                                println!("{}", &game_id);
                                insert_players(&t, &game_id, away, home);
                                insert_events(&t, &game_id, events);
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
