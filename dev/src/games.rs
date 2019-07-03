mod blobs;
mod scrape;
mod sql;
mod test;
mod void;

use crate::blobs::read_json;
use crate::scrape::{filename, get_to_file};
use crate::sql::connect;
use crate::void::ResultExt;
use rusqlite::Connection;
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

#[derive(Deserialize, Copy, Clone)]
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

#[derive(Deserialize, Copy, Clone)]
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
    "SELECT DISTINCT s.id \
     FROM schedule s \
     LEFT JOIN events e \
     ON s.id = e.game_id \
     LEFT JOIN shifts sh \
     ON s.id = sh.game_id \
     WHERE e.game_id IS NULL \
     AND sh.game_id IS NULL \
     AND s.status_abstract = 'Final' \
     AND s.status_detailed = 'Final' \
     AND s.type IN ('R', 'P');"
};

const CREATE_PLAYERS: &str = {
    "CREATE TABLE IF NOT EXISTS players \
     ( id TEXT NOT NULL \
     , game_id TEXT \
     , team_id INTEGER NOT NULL \
     , full_name TEXT NOT NULL \
     , shoots_catches TEXT \
     , roster_status TEXT NOT NULL \
     , position_type TEXT NOT NULL \
     , position_abbreviation TEXT NOT NULL \
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
     , position_type \
     , position_abbreviation \
     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);"
};

const CREATE_EVENTS: &str = {
    "CREATE TABLE IF NOT EXISTS events \
     ( id INTEGER NOT NULL \
     , game_id TEXT \
     , team_id TEXT NOT NULL \
     , player_id TEXT \
     , player_type TEXT NOT NULL \
     , event TEXT NOT NULL \
     , secondary_type TEXT \
     , penality_severity TEXT \
     , penality_minutes INTEGER \
     , period INTEGER NOT NULL \
     , period_type TEXT NOT NULL \
     , period_time INTEGER \
     , period_time_remaining INTEGER \
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
    "CREATE INDEX index_events_event ON events(event);";

const INSERT_EVENTS: &str = {
    "INSERT INTO events
     ( id \
     , game_id \
     , team_id \
     , player_id \
     , player_type \
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
     ( ?1 \
     , ?2 \
     , ?3 \
     , ?4 \
     , ?5 \
     , ?6 \
     , ?7 \
     , ?8 \
     , ?9 \
     , ?10 \
     , ?11 \
     , ?12 \
     , ?13 \
     , ?14 \
     , ?15 \
     , ?16 \
     , ?17 \
     );"
};

const CREATE_SHIFTS: &str = {
    "CREATE TABLE IF NOT EXISTS shifts \
     ( game_id TEXT \
     , team_id INTEGER NOT NULL \
     , player_id TEXT NOT NULL \
     , period INTEGER NOT NULL \
     , start_time INTEGER \
     , end_time INTEGER \
     , duration INTEGER \
     , shift_number INTEGER NOT NULL \
     , event TEXT \
     , FOREIGN KEY (game_id) REFERENCES schedule(id) \
     , UNIQUE(game_id, player_id, period, start_time, end_time, shift_number) \
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

#[inline]
fn insert_player(t: &Connection, game_id: &str, team: Team) {
    for (_, player) in team.players {
        t.execute(
            INSERT_PLAYERS,
            &[
                &player.person.id.to_string(),
                &game_id,
                &team.team.id,
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

#[inline]
fn insert_players(t: &Connection, game_id: &str, away: Team, home: Team) {
    insert_player(t, game_id, away);
    insert_player(t, game_id, home);
}

#[inline]
fn parse_time(t: &str) -> Option<u16> {
    if let [minutes, seconds] = t.split(':').collect::<Vec<&str>>().as_slice()
    {
        return minutes
            .parse::<u16>()
            .and_then(|m| seconds.parse::<u16>().map(|s| (m * 60) + s))
            .ok();
    } else {
        return None;
    }
}

#[inline]
fn insert_events(t: &Connection, game_id: &str, events: Events) {
    for play in events.liveData.plays.allPlays {
        if let Some(players) = play.players {
            let event_id: u16 = play.about.eventId;
            let team_id: Option<u16> = play.team.map(|t| t.id);
            let event: String = play.result.event;
            let secondary_type: Option<String> = play.result.secondaryType;
            let penality_severity: Option<String> =
                play.result.penaltySeverity;
            let penality_minutes: Option<u8> = play.result.penaltyMinutes;
            let period: u8 = play.about.period;
            let period_type: String = play.about.periodType;
            let period_time: Option<u16> = parse_time(&play.about.periodTime);
            let period_time_remaining: Option<u16> =
                parse_time(&play.about.periodTimeRemaining);
            let goals_away: u8 = play.about.goals.away;
            let goals_home: u8 = play.about.goals.home;
            let x: Option<f64> = play.coordinates.and_then(|c| c.x);
            let y: Option<f64> = play.coordinates.and_then(|c| c.y);
            for player in players {
                t.execute(
                    INSERT_EVENTS,
                    &[
                        &event_id,
                        &game_id,
                        &team_id,
                        &player.player.id.to_string(),
                        &player.playerType,
                        &event,
                        &secondary_type,
                        &penality_severity,
                        &penality_minutes,
                        &period,
                        &period_type,
                        &period_time,
                        &period_time_remaining,
                        &goals_away,
                        &goals_home,
                        &x,
                        &y,
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
            c.execute(INDEX_PLAYERS_GAME_ID, &[]).void();
            c.execute(INDEX_PLAYERS_TEAM_ID, &[]).void();
            c.execute(CREATE_EVENTS, &[]).void();
            c.execute(INDEX_EVENTS_GAME_ID, &[]).void();
            c.execute(INDEX_EVENTS_TEAM_ID, &[]).void();
            c.execute(INDEX_EVENTS_PLAYER_ID, &[]).void();
            c.execute(INDEX_EVENTS_EVENT, &[]).void();
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
                        if let Some((events, shifts)) = pair {
                            if let (Some(events), Some(shifts)) = {
                                let events: Option<Events> =
                                    read_json(events.as_path());
                                let shifts: Option<Shifts> =
                                    read_json(shifts.as_path());
                                (events, shifts)
                            } {
                                let away: Team = events
                                    .liveData
                                    .boxscore
                                    .teams
                                    .away
                                    .clone();
                                let home: Team = events
                                    .liveData
                                    .boxscore
                                    .teams
                                    .home
                                    .clone();
                                let game_id: String =
                                    events.gamePk.to_string();
                                insert_players(&t, &game_id, away, home);
                                insert_events(&t, &game_id, events);
                                insert_shifts(&t, shifts)
                            }
                        }
                    }
                    t.commit().void()
                }
            }
        };
    };
}
