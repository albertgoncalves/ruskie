mod blobs;
mod scrape;
mod sql;
mod test;
mod void;

use crate::blobs::read_json;
use crate::scrape::{filename, get_to_file};
use crate::sql::connect;
use crate::void::ResultExt;
use rayon::prelude::*;
use rusqlite::{Connection, ToSql, NO_PARAMS};
use serde::Deserialize;
use serde_json::Number;
use std::collections::HashMap;
use std::env::var;
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
    x: Option<f64>,
    y: Option<f64>,
}

#[derive(Deserialize)]
struct Goals {
    away: u8,
    home: u8,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct About {
    eventId: u16,
    period: u8,
    periodType: String,
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

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Person {
    id: Number,
    fullName: String,
    shootsCatches: Option<String>,
    rosterStatus: String,
}

#[derive(Deserialize)]
struct Position {
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

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Shift {
    gameId: Number,
    teamId: u16,
    playerId: Number,
    period: u8,
    startTime: String,
    endTime: String,
    duration: Option<String>,
    shiftNumber: u8,
    eventDescription: Option<String>,
}

#[derive(Deserialize)]
struct Shifts {
    data: Vec<Shift>,
}

const QUERY_GAME_IDS: &str = {
    "SELECT id \
     FROM schedule \
     WHERE type IN ('R', 'P') \
     AND status_abstract = 'Final' \
     AND status_detailed = 'Final';"
};

const CREATE_PLAYERS: &str = {
    "CREATE TABLE IF NOT EXISTS players \
     ( id TEXT NOT NULL \
     , game_id TEXT \
     , team_id INTEGER NOT NULL \
     , full_name TEXT NOT NULL \
     , shoots_catches TEXT \
     , roster_status TEXT NOT NULL \
     , position TEXT NOT NULL \
     , FOREIGN KEY (game_id) REFERENCES schedule(id) \
     , UNIQUE(id, game_id) \
     );"
};

const INDEX_PLAYERS_GAME_ID: &str =
    "CREATE INDEX index_players_game_id ON players(game_id);";

const INDEX_PLAYERS_TEAM_ID: &str =
    "CREATE INDEX index_players_team_id ON players(team_id);";

const INSERT_PLAYERS: &str = {
    "INSERT INTO players \
     ( id \
     , game_id \
     , team_id \
     , full_name \
     , shoots_catches \
     , roster_status \
     , position \
     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);"
};

const CREATE_EVENTS: &str = {
    "CREATE TABLE IF NOT EXISTS events \
     ( id INTEGER NOT NULL \
     , game_id TEXT \
     , team_id TEXT NOT NULL \
     , player_id TEXT NOT NULL \
     , player_type TEXT NOT NULL \
     , event TEXT NOT NULL \
     , secondary_type TEXT \
     , penalty_severity TEXT \
     , penalty_minutes INTEGER \
     , period INTEGER NOT NULL \
     , period_type TEXT NOT NULL \
     , period_time INTEGER NOT NULL \
     , period_time_remaining INTEGER NOT NULL \
     , away_score INTEGER NOT NULL \
     , home_score INTEGER NOT NULL \
     , x REAL \
     , y REAL \
     , FOREIGN KEY (game_id) REFERENCES schedule(id) \
     , UNIQUE(id, game_id, player_id) \
     );"
};

const INDEX_EVENTS_GAME_ID: &str =
    "CREATE INDEX index_events_game_id ON events(game_id);";

const INDEX_EVENTS_TEAM_ID: &str =
    "CREATE INDEX index_events_team_id ON events(team_id);";

const INDEX_EVENTS_PLAYER_ID: &str =
    "CREATE INDEX index_events_player_id ON events(player_id);";

const INDEX_EVENTS_EVENT: &str =
    "CREATE INDEX index_events_event ON events(event, player_type);";

const INDEX_EVENTS_PENALTY_SEVERITY: &str =
    "CREATE INDEX index_events_penalty_severity ON events(penalty_severity);";

const INDEX_EVENTS_MULTIPLE: &str = {
    "CREATE INDEX index_events_multiple \
     ON events(game_id, period, period_time, event, player_type, x, y);"
};

const INSERT_EVENTS: &str = {
    "INSERT INTO events
     ( id \
     , game_id \
     , team_id \
     , player_id \
     , player_type \
     , event \
     , secondary_type \
     , penalty_severity \
     , penalty_minutes \
     , period \
     , period_type \
     , period_time \
     , period_time_remaining \
     , away_score \
     , home_score \
     , x \
     , y \
     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14 \
     , ?15, ?16, ?17);"
};

const CREATE_SHIFTS: &str = {
    "CREATE TABLE IF NOT EXISTS shifts \
     ( game_id TEXT \
     , team_id INTEGER NOT NULL \
     , player_id TEXT NOT NULL \
     , period INTEGER NOT NULL \
     , start_time INTEGER NOT NULL \
     , end_time INTEGER NOT NULL \
     , duration INTEGER \
     , shift_number INTEGER NOT NULL \
     , event TEXT NOT NULL \
     , FOREIGN KEY (game_id) REFERENCES schedule(id) \
     , UNIQUE(game_id, player_id, period, start_time, end_time, event) \
     );"
};

const INSERT_SHIFTS: &str = {
    "INSERT INTO shifts \
     ( game_id \
     , team_id \
     , player_id \
     , period \
     , start_time \
     , end_time \
     , duration \
     , shift_number \
     , event \
     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);"
};

const INDEX_SHIFTS_GAME_ID: &str =
    "CREATE INDEX index_shifts_game_id ON shifts(game_id);";

const INDEX_SHIFTS_TEAM_ID: &str =
    "CREATE INDEX index_shifts_team_id ON shifts(team_id);";

const INDEX_SHIFTS_PLAYER_ID: &str =
    "CREATE INDEX index_shifts_player_id ON shifts(player_id);";

const INDEX_SHIFTS_MULTIPLE: &str = {
    "CREATE INDEX index_shifts_multiple \
     ON shifts(event, period, start_time, end_time);"
};

fn scrape(wd: &str, id: &str, directory: &str, url: &str) -> PathBuf {
    let x: PathBuf = filename(wd, directory, id);
    get_to_file(url, x.as_path(), 1500);
    x
}

fn events_url(id: &str) -> String {
    format!(
        "https://statsapi.web.nhl.com/api/v1/game/{}/feed/live?site=en_nhl",
        id,
    )
}

fn shifts_url(id: &str) -> String {
    format!(
        "http://www.nhl.com/stats/rest/shiftcharts?cayenneExp=gameId={}",
        id,
    )
}

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

fn insert_player(t: &Connection, game_id: &str, team: &Team) {
    if let Ok(mut p) = t.prepare(INSERT_PLAYERS) {
        for player in team.players.values() {
            p.execute(&[
                &player.person.id.to_string() as &dyn ToSql,
                &game_id,
                &team.team.id,
                &player.person.fullName,
                &player.person.shootsCatches,
                &player.person.rosterStatus,
                &player.position.abbreviation,
            ])
            .void()
        }
    }
}

fn insert_players(t: &Connection, game_id: &str, away: &Team, home: &Team) {
    insert_player(t, game_id, away);
    insert_player(t, game_id, home);
}

fn parse_time(t: &str) -> Option<u16> {
    if let [minutes, seconds] = t.split(':').collect::<Vec<&str>>().as_slice()
    {
        minutes
            .parse::<u16>()
            .and_then(|m| seconds.parse::<u16>().map(|s| (m * 60) + s))
            .ok()
    } else {
        None
    }
}

fn insert_events(t: &Connection, game_id: &str, events: &Events) {
    if let Ok(mut p) = t.prepare(INSERT_EVENTS) {
        for play in &events.liveData.plays.allPlays {
            if let (
                Some(players),
                Some(period_time),
                Some(period_time_remaining),
            ) = (
                &play.players,
                parse_time(&play.about.periodTime),
                parse_time(&play.about.periodTimeRemaining),
            ) {
                let team_id: Option<u16> = play.team.as_ref().map(|t| t.id);
                let x: Option<f64> =
                    play.coordinates.as_ref().and_then(|c| c.x);
                let y: Option<f64> =
                    play.coordinates.as_ref().and_then(|c| c.y);
                for player in players.iter() {
                    p.execute(&[
                        &play.about.eventId as &dyn ToSql,
                        &game_id,
                        &team_id,
                        &player.player.id.to_string(),
                        &player.playerType,
                        &play.result.event,
                        &play.result.secondaryType,
                        &play.result.penaltySeverity,
                        &play.result.penaltyMinutes,
                        &play.about.period,
                        &play.about.periodType,
                        &period_time,
                        &period_time_remaining,
                        &play.about.goals.away,
                        &play.about.goals.home,
                        &x,
                        &y,
                    ])
                    .void()
                }
            }
        }
    }
}

fn insert_shifts(t: &Connection, shifts: Shifts) {
    if let Ok(mut p) = t.prepare(INSERT_SHIFTS) {
        for shift in shifts.data {
            if let (Some(start_time), Some(end_time)) =
                (parse_time(&shift.startTime), parse_time(&shift.endTime))
            {
                p.execute(&[
                    &shift.gameId.to_string() as &dyn ToSql,
                    &shift.teamId,
                    &shift.playerId.to_string(),
                    &shift.period,
                    &start_time,
                    &end_time,
                    &shift.duration.and_then(|d| parse_time(&d)),
                    &shift.shiftNumber,
                    &shift.eventDescription.unwrap_or_else(|| "".to_owned()),
                ])
                .void()
            }
        }
    }
}

fn main() {
    if let Ok(wd) = var("WD") {
        if let Ok(mut c) = connect(&wd) {
            let xs: [&str; 15] = [
                CREATE_PLAYERS,
                INDEX_PLAYERS_GAME_ID,
                INDEX_PLAYERS_TEAM_ID,
                CREATE_EVENTS,
                INDEX_EVENTS_GAME_ID,
                INDEX_EVENTS_TEAM_ID,
                INDEX_EVENTS_PLAYER_ID,
                INDEX_EVENTS_EVENT,
                INDEX_EVENTS_PENALTY_SEVERITY,
                INDEX_EVENTS_MULTIPLE,
                CREATE_SHIFTS,
                INDEX_SHIFTS_GAME_ID,
                INDEX_SHIFTS_TEAM_ID,
                INDEX_SHIFTS_PLAYER_ID,
                INDEX_SHIFTS_MULTIPLE,
            ];
            for x in &xs {
                c.execute(x, NO_PARAMS).void();
            }
            if let Ok(ids) = c.prepare(QUERY_GAME_IDS).and_then(|mut s| {
                s.query_map(NO_PARAMS, |r| r.get("id")).map(|ids| {
                    ids.map(|id| (scrape_pair(&wd, &id.ok())))
                        .collect::<Vec<Option<(PathBuf, PathBuf)>>>()
                })
            }) {
                let pairs: Vec<Option<(Events, Shifts)>> = ids
                    .par_iter()
                    .map(|pair| {
                        pair.as_ref().and_then(|(events, shifts)| {
                            let events: Option<Events> =
                                read_json(events.as_path());
                            let shifts: Option<Shifts> =
                                read_json(shifts.as_path());
                            events.and_then(|e| shifts.map(|s| (e, s)))
                        })
                    })
                    .collect();
                if let Ok(t) = c.transaction() {
                    for pair in pairs {
                        if let Some((events, shifts)) = pair {
                            let game_id: String = events.gamePk.to_string();
                            let away: &Team =
                                &events.liveData.boxscore.teams.away;
                            let home: &Team =
                                &events.liveData.boxscore.teams.home;
                            insert_players(&t, &game_id, away, home);
                            insert_events(&t, &game_id, &events);
                            insert_shifts(&t, shifts)
                        }
                    }
                    t.commit().void()
                }
            };
            c.execute("PRAGMA optimize;", NO_PARAMS).void();
        };
    };
}
