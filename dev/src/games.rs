mod sql;
mod theft;
mod vars;
mod void;

use crate::sql::connect;
use crate::theft::{filename, get_to_file};
use crate::vars::gather;
use std::path::PathBuf;

const QUERY_GAME_IDS: &str = {
    "SELECT id \
     FROM games \
     WHERE status_abstract = 'Final' \
     AND status_detailed = 'Final';"
};

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

fn main() {
    if let Some((_, _, wd)) = gather() {
        if let Ok(c) = connect(&wd) {
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
                ids.into_iter()
                    .map(|pair| {
                        if let Some((_, _)) = pair {
                            //
                        }
                    })
                    .collect()
            }
        };
    };
}
