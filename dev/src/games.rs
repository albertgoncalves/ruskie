mod sql;
mod theft;
mod vars;
mod void;

use crate::sql::connect;
use crate::theft::{filename, get_to_file};
use crate::vars::gather;
use std::path::Path;

const QUERY_GAME_IDS: &str = {
    "SELECT id \
     FROM games;"
};

fn scrape(wd: &str, id: &str, directory: &str, url: &str) -> String {
    let x: String = filename(wd, directory, id);
    println!("{}", &x);
    get_to_file(url, Path::new(&x), 1500);
    x
}

fn scrape_both(wd: &str, id: &Option<String>) -> Option<(String, String)> {
    id.as_ref().map(|id| {
        (
            scrape(
                wd,
                id,
                "events",
                &format!(
                    "https://statsapi.web.nhl.com/\
                     api/v1/game/{}/feed/live?site=en_nhl",
                    id,
                ),
            ),
            scrape(
                wd,
                id,
                "shifts",
                &format!(
                    "http://www.nhl.com/\
                     stats/rest/shiftcharts?cayenneExp=gameId={}",
                    id,
                ),
            ),
        )
    })
}

fn main() {
    if let Some((_, _, wd)) = gather() {
        if let Ok(c) = connect(&wd) {
            if let Ok(ids) = c.prepare(QUERY_GAME_IDS).and_then(|mut s| {
                s.query_map(&[], |r| {
                    let id: String = r.get("id");
                    id
                })
                .map(|ids| {
                    let filenames: Vec<Option<(String, String)>> =
                        ids.map(|id| (scrape_both(&wd, &id.ok()))).collect();
                    filenames
                })
            }) {
                ids.into_iter()
                    .map(|id| {
                        if let Some((_, _)) = id {
                            //
                        }
                    })
                    .collect()
            }
        };
    };
}
