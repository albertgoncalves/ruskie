mod sql;
mod vars;
mod void;

use crate::sql::connect;
use crate::vars::gather;
use crate::void::{OptionExt, ResultExt};
use reqwest::Client;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

const QUERY_TEAM_IDS: &str = {
    "SELECT t.id \
     FROM teams t \
     INNER JOIN ledger l ON l.id = t.ledger_id \
     WHERE l.start = DATE(?1) \
     AND l.end = DATE(?2);"
};

fn scrape(url: &str, filename: &Path) {
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
        sleep(Duration::from_millis(500))
    }
}

fn filename(wd: &str, id: u32, start: &str, end: &str) -> String {
    format!("{}/data/schedule-{}-{}-{}.json", wd, id, start, end)
}

fn url(id: u32, start: &str, end: &str) -> String {
    format!(
        "https://statsapi.web.nhl.com/api/v1/schedule?\
         teamId={}&\
         startDate={}&\
         endDate={}",
        id,     //
        &start, //
        &end,
    )
}

fn main() {
    if let Some((start, end, wd)) = gather() {
        if let Ok(c) = connect(&wd) {
            if let Ok(mut s) = c.prepare(QUERY_TEAM_IDS) {
                s.query_map(&[&start, &end], |r| {
                    let id: u32 = r.get("id");
                    id
                })
                .map(|ids| {
                    for id in ids {
                        if let Ok(x) = id {
                            let u: String = url(x, &start, &end);
                            let p: String = filename(&wd, x, &start, &end);
                            println!("{}", &p);
                            scrape(&u, Path::new(&p))
                        }
                    }
                })
                .void()
            };
        }
    }
}
