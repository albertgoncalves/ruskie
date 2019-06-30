use serde::de::DeserializeOwned;
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn read_json<P: AsRef<Path>, D: DeserializeOwned>(path: P) -> Option<D> {
    File::open(path)
        .ok()
        .and_then(|f| from_reader(BufReader::new(f)).ok())
}
