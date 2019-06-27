use serde_json::{from_reader, to_string_pretty, Value};
use std::env::var;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn read_file<T: AsRef<Path>>(path: T) -> Option<Value> {
    File::open(path)
        .ok()
        .and_then(|f| from_reader(BufReader::new(f)).ok())
}

fn main() {
    var("WD")
        .ok()
        .and_then(|wd| {
            read_file(format!("{}/data/teams-2018-08-01-2019-08-01.json", wd))
        })
        .and_then(|xs| {
            xs.get("teams")
                .as_ref()
                .and_then(|x| to_string_pretty(x).ok())
                .map(|y| println!("{}", y))
        });
}
