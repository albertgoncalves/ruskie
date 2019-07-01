use crate::void::OptionExt;
use reqwest::{Client, StatusCode};
use std::fmt::Display;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

pub fn filename<T: Display>(wd: &str, directory: &str, id: T) -> String {
    format!("{}/data/{}/{}.json", wd, directory, id)
}

pub fn get_to_file(url: &str, filename: &Path, wait: u64) {
    if !filename.exists() {
        println!("{}", url);
        if let Ok(mut response) = Client::new().get(url).send() {
            if let StatusCode::OK = response.status() {
                let buffer = File::create(filename).map(BufWriter::new).ok();
                buffer
                    .and_then(|mut f| response.copy_to(&mut f).ok())
                    .void()
            }
        };
        sleep(Duration::from_millis(wait))
    }
}
