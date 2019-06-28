use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::env::var;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Venue {
    name: String,
    id: Option<u32>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Team {
    id: u32,
    name: String,
    link: String,
    venue: Venue,
    abbreviation: String,
    teamName: String,
}

#[derive(Serialize, Deserialize)]
struct Blob {
    copyright: String,
    teams: Vec<Team>,
}

fn read_file<T: AsRef<Path>>(path: T) -> Option<Blob> {
    File::open(path)
        .ok()
        .and_then(|f| from_reader(BufReader::new(f)).ok())
}

fn string_option<T: ToString>(x: Option<T>, default: String) -> String {
    x.map(|y| y.to_string()).unwrap_or_else(|| default)
}

fn main() {
    if let Some(xs) = var("WD").ok().and_then(|wd| {
        read_file(format!("{}/data/teams-2018-08-01-2019-08-01.json", wd))
    }) {
        for x in xs.teams {
            println!(
                "{},{},{},{}",
                x.id,
                x.name,
                string_option(x.venue.id, "".to_string()),
                x.venue.name,
            )
        }
    };
}
