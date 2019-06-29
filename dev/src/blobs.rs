use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Venue {
    pub name: String,
    pub id: Option<u16>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Team {
    pub id: u16,
    pub name: String,
    pub venue: Venue,
    pub abbreviation: String,
}

#[derive(Serialize, Deserialize)]
pub struct Teams {
    pub teams: Vec<Team>,
}

pub fn read_json<T: AsRef<Path>>(path: T) -> Option<Teams> {
    File::open(path)
        .ok()
        .and_then(|f| from_reader(BufReader::new(f)).ok())
}
