use crate::void::OptionExt;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::from_reader;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

#[allow(dead_code)]
pub fn get_to_file(url: &str, filename: &Path, wait: u64) {
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
        sleep(Duration::from_millis(wait))
    }
}

pub fn read_json<P: AsRef<Path>, D: DeserializeOwned>(path: P) -> Option<D> {
    File::open(path)
        .ok()
        .and_then(|f| from_reader(BufReader::new(f)).ok())
}
