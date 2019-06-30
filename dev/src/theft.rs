use crate::void::OptionExt;
use reqwest::Client;
use std::fmt::Display;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

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

pub fn filename<T: Display>(
    wd: &str,
    directory: &str,
    id: T,
    start: &str,
    end: &str,
) -> String {
    format!("{}/data/{}/{}-{}-{}.json", wd, directory, id, start, end)
}
